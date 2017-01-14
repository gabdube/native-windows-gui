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

use winapi::{HWND, IShellItem};
use winapi::shobjidl::IFileDialog;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::{Error, SystemError};
use events::Event;
use defs::FileDialogAction;

/**
    A template that creates a standard file dialog

    Members:  
    • `title`: The title of the dialog  
    • `parent`: The dialog parent window. While thwe dialog is running, the window will be disabled.  
    • `action`: The action that the dialog will execute. It can be `Open`, `OpenDirectory` or `Save`  
    • `multiselect`: The user can select more than one choice. The names returned by open will be separated by `|`. Ex: File1|File2
*/
#[derive(Clone)]
pub struct FileDialogT<S: Clone+Into<String>, ID: Hash+Clone> {
    pub title: S,
    pub parent: Option<ID>,
    pub action: FileDialogAction,
    pub multiselect: bool
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for FileDialogT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<FileDialog>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use ole32::{CoCreateInstance};
        use winapi::{FILEOPENDIALOGOPTIONS, S_OK, CLSCTX_INPROC_SERVER, FOS_FORCEFILESYSTEM, FOS_PICKFOLDERS, FOS_ALLOWMULTISELECT};
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

            if pfd.GetOptions(&mut flags) != S_OK {
                return Err(Error::System(SystemError::ComError("Failed to get the file dialog options".to_string())));
            }

            let use_dir = if self.action == FileDialogAction::OpenDirectory { FOS_PICKFOLDERS } else { FILEOPENDIALOGOPTIONS(0) };
            let multiselect = if self.multiselect { FOS_ALLOWMULTISELECT } else { FILEOPENDIALOGOPTIONS(0) };
            if pfd.SetOptions(flags | FOS_FORCEFILESYSTEM | use_dir | multiselect) != S_OK {
                return Err(Error::System(SystemError::ComError("Failed to set the file dialog options".to_string())));
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
            return Err(Error::UserError("FileDialog do not have the multiselect flag".to_string()))
        }
        
        let handle = &mut *self.handle;
        let mut _item: *mut IShellItem = ptr::null_mut();

        if handle.GetResult(&mut _item) != ::winapi::S_OK {
            return Err(Error::System(SystemError::ComError("Failed to get result".to_string())));
        }

        let text = get_ishellitem_path(&mut *_item);
        (&mut *_item).Release();

        text
    }}

    /**
        Return the selected items in the dialog by the user. Currently won't work. `GetResults.GetResults` do not return the IShellItemArray.

        Failures:  
        • `Error::System` if the dialog was not called  
        • `Error::System` if there was a system error while reading the selected items
        • `Error::UserError` if the dialog has `Save` for action
    */
    pub fn get_selected_items(&self) -> Result<Vec<String>, Error> { unsafe{
        use winapi::{DWORD, S_OK, IFileOpenDialog};
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

        println!("{:?}", _items);
        let items = &mut *_items;
        let mut count: DWORD = 0;
        items.GetCount(&mut count);
        println!("{:?}", count);
        
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
    

    /**
        Display the dialog. Return true if the dialog was accepted or false if it was cancelled
    */
    pub fn run(&self) -> bool { unsafe{
        (&mut *self.handle).Show(self.parent) == ::winapi::S_OK
     }}
}

impl Control for FileDialog {

    fn handle(&self) -> AnyHandle {
        AnyHandle::Custom(TypeId::of::<FileDialog>(), 0)
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