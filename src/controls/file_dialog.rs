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

use winapi::{HWND, HFONT};

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::Event;

/**
    A template that creates a standard file dialog

    Members:  
    • `title`: The title of the dialog 
*/
#[derive(Clone)]
pub struct FileDialogT<S: Clone+Into<String>, ID: Hash+Clone> {
    title: S,
    parent: Option<ID>,
}

impl<S: Clone+Into<String>, ID: Hash+Clone> ControlT<ID> for FileDialogT<S, ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<FileDialog>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use ole32::{CoCreateInstance};
        use winapi::{FILEOPENDIALOGOPTIONS, S_OK, CLSCTX_INPROC_SERVER, FOS_FORCEFILESYSTEM};
        use low::clsid::{CLSID_FileOpenDialog, UUIDOF_IFileDialog};
        use low::window_helper::handle_of_window;

        let parent = match self.parent.as_ref() {
            Some(id) =>  
            match handle_of_window(ui, id, "The parent of a FileDialog must be a window-like control.") {
                Ok(h) => h,
                Err(e) => { return Err(e); }
            },
            None => ptr::null_mut()
        };
        
        /*
        let mut handle: *mut IFileDialog = ptr::null_mut();
        let r = unsafe { CoCreateInstance(&CLSID_FileOpenDialog(), ptr::null_mut(), CLSCTX_INPROC_SERVER, &UUIDOF_IFileDialog(), mem::transmute(&mut handle) ) };
        if r != S_OK {
            return Err(Error::System(SystemError::DialogCreation));
        }

        unsafe {
            let pfd = &mut *handle;
            let mut flags: FILEOPENDIALOGOPTIONS = FILEOPENDIALOGOPTIONS(0);

            if pfd.GetOptions(&mut flags) != S_OK {
                return Err(Error::System(SystemError::DialogCreation));
            }

            if pfd.SetOptions(flags | FOS_FORCEFILESYSTEM) != S_OK {
                return Err(Error::System(SystemError::DialogCreation));
            }
        }
        
        Ok(Box::new(FileDialog{handle: handle, parent: parent}) as Box<AnyControlBase>)
    */

    }
}

/**
    A file dialog control
*/
pub struct FileDialog {
    parent: HWND
}


// impl FileDialog {
//     pub fn open(&self) -> Result<String, Error> {
//         unsafe{
//             use winapi::{S_OK, IShellItem, SIGDN_FILESYSPATH, PWSTR};
//             use ole32::CoTaskMemFree;
//             use std::mem;
//             use base::string_from_wide_ptr;

//             let handle = &mut *self.handle;
//             if handle.Show(self.parent) != S_OK {
//                 return Ok("".to_string());
//             }

//             let mut _item: *mut IShellItem = ptr::null_mut();
//             if handle.GetResult(&mut _item) != S_OK {
//                 return Err(Error::Unknown);
//             }

//             let item = &mut *_item;
//             let mut item_path: PWSTR = ptr::null_mut();
//             if item.GetDisplayName(SIGDN_FILESYSPATH, &mut item_path) != S_OK {
//                 return Err(Error::Unknown);
//             }

//             let text = string_from_wide_ptr(item_path);

//             CoTaskMemFree(mem::transmute(item_path));
//             item.Release();

//             Ok(text)
//         }
//     }
// }

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