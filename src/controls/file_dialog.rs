/*!
    File/Directory dialog control definition
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::hash::Hash;
use std::any::TypeId;
use std::ptr;
use std::mem;

use winapi::{S_OK, HWND, IShellItem, FILEOPENDIALOGOPTIONS};
use winapi::shobjidl::IFileDialog;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::{Error, SystemError};
use events::{Event, Destroyed};
use defs::FileDialogAction;
use low::other_helper::to_utf16;

/**
    A template that creates a standard file dialog

    Events:  
    `Destroyed`  

    Members:  
    • `title`: The title of the dialog  
    • `parent`: The dialog parent window. While the dialog is running, the parent will be disabled.  
    • `action`: The action that the dialog will execute. It can be `Open`, `OpenDirectory` or `Save`  
    • `multiselect`: The user can select more than one choice.  
    • `default_folder`: If defined, this defines the default folder that is openened when `run` is called  
    • `filters`: If defined, filter the files that the user can select (In a `Open` dialog) or which extension to add to the saved file (in a `Save` dialog)

    Failures:  
    • `Error::UserError`: if the default folder do not identify a folder  
    • `Error::UserError`: if the extension filter format is not valid  
    • `Error::System`: if the folder do not exists  
    • `Error::System`: if the extensions filters could not be set  

*/
#[derive(Clone)]
pub struct FileDialogT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub title: S,
    pub parent: Option<ID>,
    pub action: FileDialogAction,
    pub multiselect: bool,
    pub default_folder: Option<S>,
    pub filters: Option<S>
}

impl<S1: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for FileDialogT<S1, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<FileDialog>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use ole32::{CoCreateInstance};
        use winapi::{CLSCTX_INPROC_SERVER, FOS_FORCEFILESYSTEM, FOS_PICKFOLDERS, FOS_ALLOWMULTISELECT};
        use low::clsid::{CLSID_FileOpenDialog, CLSID_FileSaveDialog, UUIDOF_IFileOpenDialog, UUIDOF_IFileDialog};
        use low::window_helper::handle_of_window;

        let parent = match self.parent.as_ref() {
            Some(id) =>  
            match handle_of_window(ui, id, "The parent of a FileDialog must be a window-like control.") {
                Ok(h) => h,
                Err(e) => { return Err(e); }
            },
            None => ptr::null_mut()
        };
        
        let (clsid, uuid) = match self.action {
            FileDialogAction::Save => (CLSID_FileSaveDialog(), UUIDOF_IFileDialog()),
            _ => (CLSID_FileOpenDialog(), UUIDOF_IFileOpenDialog())
        };
        
        let mut handle: *mut IFileDialog = ptr::null_mut();
        let r = unsafe { CoCreateInstance(&clsid, ptr::null_mut(), CLSCTX_INPROC_SERVER, &uuid, mem::transmute(&mut handle) ) };
        if r != S_OK {
            return Err(Error::System(SystemError::ComInstanceCreation("FileDialog".to_string())));
        }

        unsafe {
            let pfd = &mut *handle;
            let mut flags: FILEOPENDIALOGOPTIONS = FILEOPENDIALOGOPTIONS(0);

            // Set dialog options
            if pfd.GetOptions(&mut flags) != S_OK {
                pfd.Release(); 
                return Err(Error::System(SystemError::ComError("Failed to get the file dialog options".to_string())));
            }

            let use_dir = if self.action == FileDialogAction::OpenDirectory { FOS_PICKFOLDERS } else { FILEOPENDIALOGOPTIONS(0) };
            let multiselect = if self.multiselect { FOS_ALLOWMULTISELECT } else { FILEOPENDIALOGOPTIONS(0) };
            if pfd.SetOptions(flags | FOS_FORCEFILESYSTEM | use_dir | multiselect) != S_OK {
                pfd.Release();
                return Err(Error::System(SystemError::ComError("Failed to set the file dialog options".to_string())));
            }

            // Set the default folder
            match &self.default_folder {
                &Some(ref f) => match set_default_folder(pfd, f) {
                    Ok(_) => (),
                    Err(e) => { pfd.Release(); return Err(e); }
                },
                &None => ()
            }

            // Set the default filters
            match &self.filters {
                &Some(ref f) => match set_filters(pfd, f) {
                    Ok(_) => (),
                    Err(e) => { pfd.Release(); return Err(e); }
                },
                &None => ()
            }
        }
        
        Ok(Box::new(
            FileDialog{
                handle: handle, parent: parent,
                action: self.action.clone(),
                multiselect: self.multiselect
            }
        )as Box<Control>)
    }
}

/**
    A file dialog control
*/
pub struct FileDialog {
    parent: HWND,
    handle: *mut IFileDialog,
    action: FileDialogAction,
    multiselect: bool
}


impl FileDialog {

    /**
        Return the item selected in the dialog by the user. 
        
        Failures:  
        • `Error::System` if the dialog was not called  
        • `Error::System` if there was a system error while reading the selected item  
        • `Error::UserError` if the dialog has the `multiselect` flag  
    */
    pub fn get_selected_item(&self) -> Result<String, Error> { unsafe {
        if self.multiselect {
            return Err(Error::UserError("FileDialog have the multiselect flag".to_string()))
        }
        
        let handle = &mut *self.handle;
        let mut _item: *mut IShellItem = ptr::null_mut();

        if handle.GetResult(&mut _item) != S_OK {
            return Err(Error::System(SystemError::ComError("Failed to get result".to_string())));
        }

        let text = get_ishellitem_path(&mut *_item);
        (&mut *_item).Release();

        text
    }}

    /**
        Return the selected items in the dialog by the user.

        Failures:  
        • `Error::System` if the dialog was not called  
        • `Error::System` if there was a system error while reading the selected items  
        • `Error::UserError` if the dialog has `Save` for action  
    */
    pub fn get_selected_items(&self) -> Result<Vec<String>, Error> { unsafe{
        use winapi::{DWORD, IFileOpenDialog};
        use low::defs::IShellItemArray;

        if self.action == FileDialogAction::Save {
            return Err(Error::UserError("Save dialog cannot have more than one item selected".to_string()));
        }

        let handle: &mut IFileOpenDialog = mem::transmute(self.handle);
        let mut _item: *mut IShellItem = ptr::null_mut();
        let mut _items: *mut IShellItemArray = ptr::null_mut();

        if handle.GetResults( mem::transmute(&mut _items) ) != S_OK {
            return Err(Error::System(SystemError::ComError("Failed to get results".to_string())));
        }

        let items = &mut *_items;
        let mut count: DWORD = 0;
        items.GetCount(&mut count);
        
        let mut item_names: Vec<String> = Vec::with_capacity(count as usize);
        for i in 0..count {
            items.GetItemAt(i, &mut _item);
            match get_ishellitem_path(&mut *_item) {
                Ok(s) => item_names.push(s),
                Err(_) => {}
            }
        }

        items.Release();

        Ok(item_names)
    }}
    
    /// Return the action type executed by this dialog
    pub fn action(&self) -> FileDialogAction {
        self.action.clone()
    }

    /// Return `true` if the dialog accepts multiple values or `false` otherwise
    pub fn get_multiselect(&self) -> bool {
        self.multiselect
    }

    /**
        Set the multiselect flag of the dialog. 

        Failures:  
        • `Error::System` if there was a system error while setting the new flag value  
        • `Error::UserError` if the dialog has `Save` for action  
    */
    pub fn set_multiselect(&mut self, multiselect: bool) -> Result<(), Error> {
        use winapi::FOS_ALLOWMULTISELECT;

        if self.action == FileDialogAction::Save {
            return Err(Error::UserError("Cannot set multiselect flag for a save file dialog".to_string()));
        }

        match unsafe{ toggle_dialog_flags(&mut *self.handle, FOS_ALLOWMULTISELECT, multiselect) } {
            Ok(_) => { self.multiselect=multiselect; Ok(())}
            Err(e) => Err(e)
        }
    }

    /**
        Set the first opened folder when the dialog is shown. This value is overriden by the user after the dialog ran.  
        Call `clear_client_data` to fix that.

        Failures:
        • `Error::UserError`: if the default folder do not identify a folder  
        • `Error::System`: if the folder do not exists  
    */
    pub fn set_default_folder<'a>(&self, folder: &'a str) -> Result<(), Error> {
        unsafe{ 
            let handle = &mut *self.handle;
            set_default_folder(handle, &folder) 
        }
    }

    /**
        Filter the files that the user can select (In a `Open` dialog) in the dialog or which extension to add to the saved file (in a `Save` dialog).  
        This can only be set ONCE (the initialization counts) and won't work if the dialog is `OpenDirectory`.  
       
        The `filters` value must be a '|' separated string having this format: "Test(*.txt;*.rs)|Any(*.*)"  
        Where the fist part is the "human name" and the second part is a filter for the system.
    */
    pub fn set_filters<'a>(&self, filters: &'a str) -> Result<(), Error> {
        unsafe{ 
            let handle = &mut *self.handle;
            set_filters(handle, &filters) 
        }
    }

    /// Change the dialog title
    pub fn set_title<'a>(&self, title: &'a str) { unsafe{
        let handle = &mut *self.handle;
        let title = to_utf16(title);

        handle.SetTitle(title.as_ptr());
    }}

    /// Instructs the dialog to clear all persisted state information (such as the last folder visited).
    pub fn clear_client_data(&self) { unsafe{
        let handle =  &mut *self.handle;
        handle.ClearClientData();
    }}

    /// Display the dialog. Return true if the dialog was accepted or false if it was cancelled
    pub fn run(&self) -> bool { unsafe{
        (&mut *self.handle).Show(self.parent) == S_OK
    }}
}

impl Control for FileDialog {

    fn handle(&self) -> AnyHandle {
        let handle_usize = unsafe{ mem::transmute(self.handle) };
        AnyHandle::Custom(TypeId::of::<FileDialog>(), handle_usize)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::FileDialog 
    }

    fn free(&mut self) {
        unsafe{
            let handle = &mut*self.handle;
            handle.Release(); 
        }
    }

}

#[inline(always)]
unsafe fn get_ishellitem_path(item: &mut IShellItem) -> Result<String, Error> {
    use winapi::{S_OK, SIGDN_FILESYSPATH, PWSTR};
    use ole32::CoTaskMemFree;
    use low::other_helper::from_wide_ptr;

    let mut item_path: PWSTR = ptr::null_mut();
    if item.GetDisplayName(SIGDN_FILESYSPATH, &mut item_path) != S_OK {
        return Err(Error::System(SystemError::ComError("Failed to get display name".to_string())));
    }

    let text = from_wide_ptr(item_path);

    CoTaskMemFree(mem::transmute(item_path));

    Ok(text)
}

#[inline(always)]
unsafe fn set_default_folder<S: Clone+Into<String>>(dialog: &mut IFileDialog, folder_name: &S) -> Result<(), Error> {
    use low::defs::{SHCreateItemFromParsingName, SFGAO_FOLDER};
    use low::clsid::IID_IShellItem;
    use winapi::{IShellItem, SFGAOF, S_FALSE};

    let mut shellitem: *mut IShellItem = ptr::null_mut();
    let path_s = folder_name.clone().into();
    let path = to_utf16(&path_s);

    if SHCreateItemFromParsingName(path.as_ptr(), ptr::null_mut(), &IID_IShellItem(), mem::transmute(&mut shellitem) ) != S_OK {
        let msg = format!("Failed to open the following folder: {}", path_s);
        return Err(Error::System(SystemError::ComError(msg)));
    }

    let shellitem = &mut *shellitem;
    let mut file_properties: SFGAOF = 0;
    
    let results = shellitem.GetAttributes(SFGAO_FOLDER, &mut file_properties);

    if results != S_OK && results != S_FALSE {
        shellitem.Release();
        let msg = format!("There was an error while reading the file properties");
        return Err(Error::System(SystemError::ComError(msg)));
    }

    if file_properties & SFGAO_FOLDER != SFGAO_FOLDER {
        shellitem.Release();
        let msg = format!("File {} do not identify a folder", path_s);
        return Err(Error::UserError(msg));
    }

    if dialog.SetDefaultFolder(shellitem) != S_OK {
        shellitem.Release();
        let msg = format!("Failed to set the dialog default folder {:?}", path_s);
        return Err(Error::System(SystemError::ComError(msg)));
    }

    shellitem.Release();

    Ok(())
}

#[inline(always)]
unsafe fn set_filters<S: Clone+Into<String>>(dialog: &mut IFileDialog, filters: &S) -> Result<(), Error> {
    use winapi::{COMDLG_FILTERSPEC, UINT};

    let filters = filters.clone().into();
    let mut raw_filters: Vec<COMDLG_FILTERSPEC> = Vec::with_capacity(3);
    let mut keep_alive: Vec<(Vec<u16>, Vec<u16>)> = Vec::with_capacity(3);

    for f in filters.split('|') {
        let end = f.rfind('(');
        if end.is_none() {
            let msg = format!("Bad extension filter format: {:?}", filters);
            return Err(Error::UserError(msg));
        }

        let (_name, _filter) = f.split_at(end.unwrap());
        let (name, filter) = (to_utf16(_name), to_utf16(&_filter[1.._filter.len()-1]));
        
        raw_filters.push(COMDLG_FILTERSPEC{ pszName: name.as_ptr(), pszSpec: filter.as_ptr() });
        keep_alive.push( (name, filter) );
    }

    let filters_count = raw_filters.len() as UINT;
    if dialog.SetFileTypes(filters_count, raw_filters.as_ptr()) == S_OK {
        Ok(())
    } else {
        let msg = format!("Failed to set the filters using {:?}", filters);
        Err(Error::System(SystemError::ComError(msg)))
    }
}

#[inline(always)]
unsafe fn toggle_dialog_flags(dialog: &mut IFileDialog, flag: FILEOPENDIALOGOPTIONS, enabled: bool) -> Result<(), Error> {
    let mut flags: FILEOPENDIALOGOPTIONS = FILEOPENDIALOGOPTIONS(0);
    if dialog.GetOptions(&mut flags) != S_OK {
        return Err(Error::System(SystemError::ComError("Failed to get the file dialog options".to_string())));
    }

    flags = match enabled {
        true => flags | flag,
        false => flags & (!flag)
    };

    if dialog.SetOptions(flags) != S_OK {
        return Err(Error::System(SystemError::ComError("Failed to set the file dialog options".to_string())));
    } else {
        Ok(())
    }
}