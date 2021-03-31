use winapi::um::shobjidl::IFileDialog;
use crate::win32::resources_helper as rh;

use crate::win32::base_helper::to_utf16;
use crate::{ControlHandle, NwgError};
use std::{fmt, ptr, mem, ffi::OsString};


/**
    A enum that dictates how a file dialog should behave
    Members:  
    * `Open`: User can select a file that is not a directory  
    * `OpenDirectory`: User can select a directory  
    * `Save`: User select the name of a file. If it already exists, a confirmation message will be raised  
*/
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FileDialogAction {
    Open,
    OpenDirectory,
    Save,
}

/**
    A file dialog control

    The file dialog builders accepts the following parameters:
    * title: The title of the dialog
    * action: The action to execute. Open, OpenDirectory for Save
    * multiselect: Whether the user can select more than one file. Only supported with the Open action
    * default_folder: Default folder to show in the dialog.
    * filters: If defined, filter the files that the user can select (In a Open dialog) or which extension to add to the saved file (in a Save dialog)
    The `filters` value must be a '|' separated string having this format: "Test(*.txt;*.rs)|Any(*.*)"  

    ```rust
        use native_windows_gui as nwg;
        fn layout(dialog: &mut nwg::FileDialog) {
            nwg::FileDialog::builder()
                .title("Hello")
                .action(nwg::FileDialogAction::Open)
                .multiselect(true)
                .build(dialog);
        }
    ```
*/
pub struct FileDialog {
    handle: *mut IFileDialog,
    action: FileDialogAction
}

impl FileDialog {

    pub fn builder() -> FileDialogBuilder {
        FileDialogBuilder {
            title: None,
            action: FileDialogAction::Save,
            multiselect: false,
            default_folder: None,
            filters: None
        }
    }

    /// Return the action type executed by this dialog
    pub fn action(&self) -> FileDialogAction {
        self.action
    }

    /** 
        Display the dialog. Return true if the dialog was accepted or false if it was cancelled
        If the dialog was accepted, `get_selected_item` or `get_selected_items` can be used to find the selected file(s)
    
        It's important to note that `run` blocks the current thread until the user as chosen a file (similar to `dispatch_thread_events`)

        The parent argument must be a window control otherwise the method will panic.
    */
    pub fn run<C: Into<ControlHandle>>(&self, parent: Option<C>) -> bool { 
        use winapi::shared::winerror::S_OK;

        let parent_handle = match parent {
            Some(p) => p.into().hwnd().expect("File dialog parent must be a window control"),
            None => ptr::null_mut()
        };

        unsafe { (&mut *self.handle).Show(parent_handle) == S_OK }
    }

    /**
        Return the item selected in the dialog by the user. 
        
        Failures:  
        • if the dialog was not called  
        • if there was a system error while reading the selected item  
        • if the dialog has the `multiselect` flag  
    */
    pub fn get_selected_item(&self) -> Result<OsString, NwgError> { 
        
        if self.multiselect() {
            return Err(NwgError::file_dialog("FileDialog have the multiselect flag"));
        }

        unsafe {
            rh::filedialog_get_item(&mut *self.handle)
        }
    }

    /**
        Return the selected items in the dialog by the user.
        Failures:  
        • if the dialog was not called  
        • if there was a system error while reading the selected items  
        • if the dialog has `Save` for action  
    */
    pub fn get_selected_items(&self) -> Result<Vec<OsString>, NwgError> {
        if self.action == FileDialogAction::Save {
            return Err(NwgError::file_dialog("Save dialog cannot have more than one item selected"));
        }

        unsafe {
            rh::filedialog_get_items(mem::transmute(&mut *self.handle))
        }
    }

    /// Return `true` if the dialog accepts multiple values or `false` otherwise
    pub fn multiselect(&self) -> bool {
        use winapi::um::shobjidl::FOS_ALLOWMULTISELECT;

        unsafe {
            let flags = rh::file_dialog_options(&mut *self.handle).unwrap_or(0);
            flags & FOS_ALLOWMULTISELECT == FOS_ALLOWMULTISELECT
        }
    }

    /**
        Set the multiselect flag of the dialog. 
        Failures:  
        • if there was a system error while setting the new flag value  
        • if the dialog has `Save` for action  
    */
    pub fn set_multiselect(&self, multiselect: bool) -> Result<(), NwgError> {
        use winapi::um::shobjidl::FOS_ALLOWMULTISELECT;

        if self.action == FileDialogAction::Save {
            return Err(NwgError::file_dialog("Cannot set multiselect flag for a save file dialog"));
        }

        let result = unsafe{ rh::toggle_dialog_flags(&mut *self.handle, FOS_ALLOWMULTISELECT, multiselect) };
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    /**
        Set the first opened folder when the dialog is shown. This value is overriden by the user after the dialog ran.  
        Call `clear_client_data` to fix that.
        Failures:
        • if the default folder do not identify a folder  
        • if the folder do not exists  
    */
    pub fn set_default_folder<'a>(&self, folder: &'a str) -> Result<(), NwgError> {
        unsafe{ 
            let handle = &mut *self.handle;
            rh::file_dialog_set_default_folder(handle, &folder) 
        }
    }

    /**
        Filter the files that the user can select (In a `Open` dialog) in the dialog or which extension to add to the saved file (in a `Save` dialog).  
        This can only be set ONCE (the initialization counts) and won't work if the dialog is `OpenDirectory`.  
       
        The `filters` value must be a '|' separated string having this format: "Test(*.txt;*.rs)|Any(*.*)"  
        Where the fist part is the "human name" and the second part is a filter for the system.
    */
    pub fn set_filters<'a>(&self, filters: &'a str) -> Result<(), NwgError> {
        unsafe{ 
            let handle = &mut *self.handle;
            rh::file_dialog_set_filters(handle, &filters) 
        }
    }

    /// Change the dialog title
    pub fn set_title<'a>(&self, title: &'a str) {
        unsafe {
            let title = to_utf16(title);
            let handle = &mut *self.handle;
            handle.SetTitle(title.as_ptr());
        }
    }

    /// Instructs the dialog to clear all persisted state information (such as the last folder visited).
    pub fn clear_client_data(&self) { 
        unsafe{
            let handle =  &mut *self.handle;
            handle.ClearClientData();
        }
    }

}


impl fmt::Debug for FileDialog {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileDialog {{ action: {:?} }}", self.action)
    }

}

impl Default for FileDialog {
    fn default() -> FileDialog {
        FileDialog {
            handle: ptr::null_mut(),
            action: FileDialogAction::Open
        }
    }
}

impl PartialEq for FileDialog {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle && self.action == other.action
    }
}

impl Eq for FileDialog {}

/*
    Structure that hold the state required to build a file dialog
*/
pub struct FileDialogBuilder {
    pub title: Option<String>,
    pub action: FileDialogAction,
    pub multiselect: bool,
    pub default_folder: Option<String>,
    pub filters: Option<String>
}

impl FileDialogBuilder {

    pub fn title<S: Into<String>>(mut self, t: S) -> FileDialogBuilder {
        self.title = Some(t.into());
        self
    }

    pub fn default_folder<S: Into<String>>(mut self, t: S) -> FileDialogBuilder {
        self.default_folder = Some(t.into());
        self
    }

    pub fn filters<S: Into<String>>(mut self, t: S) -> FileDialogBuilder {
        self.filters = Some(t.into());
        self
    }

    pub fn action(mut self, a: FileDialogAction) -> FileDialogBuilder {
        self.action = a;
        self
    }

    pub fn multiselect(mut self, m: bool) -> FileDialogBuilder {
        self.multiselect = m;
        self
    }

    pub fn build(self, out: &mut FileDialog) -> Result<(), NwgError> {
        unsafe {
            out.handle = rh::create_file_dialog(
                self.action,
                self.multiselect,
                self.default_folder,
                self.filters
            )?;
        }

        out.action = self.action;
        
        if let Some(title) = self.title {
            out.set_title(&title);
        }

        Ok(())
    }

}

