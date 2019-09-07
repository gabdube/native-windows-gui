use winapi::shared::windef::HWND;
use winapi::um::shobjidl::IFileDialog;
use crate::win32::resources_helper as rh;
use crate::controls::ControlBase;
use crate::{SystemError};
use std::fmt;
use std::ptr;


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
*/
pub struct FileDialog {
    parent: HWND,
    handle: *mut IFileDialog,
    action: FileDialogAction,
    multiselect: bool
}

impl FileDialog {

    pub fn builder() -> FileDialogBuilder {
        FileDialogBuilder::new()
    }

    /// Display the dialog. Return true if the dialog was accepted or false if it was cancelled
    pub fn run(&self) -> bool { 
        use winapi::shared::winerror::S_OK;

        unsafe { (&mut *self.handle).Show(self.parent) == S_OK }
    }

}


impl fmt::Debug for FileDialog {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileDialog {{ action: {:?}, multiselect: {:?} }}", self.action, self.multiselect)
    }

}

impl Default for FileDialog {
    fn default() -> FileDialog {
        FileDialog {
            parent: ptr::null_mut(),
            handle: ptr::null_mut(),
            action: FileDialogAction::Open,
            multiselect: false
        }
    }
}

impl PartialEq for FileDialog {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.action == other.action && self.multiselect == other.multiselect
    }
}

impl Eq for FileDialog {}

/*
    Structure that hold the state required to build a file dialog
*/
pub struct FileDialogBuilder {
    pub title: Option<String>,
    pub parent: Option<HWND>,
    pub action: FileDialogAction,
    pub multiselect: bool,
    pub default_folder: Option<String>,
    pub filters: Option<String>
}

impl FileDialogBuilder {

    pub fn new() -> FileDialogBuilder {
        FileDialogBuilder {
            title: None,
            parent: None,
            action: FileDialogAction::Save,
            multiselect: true,
            default_folder: None,
            filters: None
        }
    }

    pub fn parent(mut self, p: Option<&ControlBase>) -> FileDialogBuilder {
        self.parent = p.map(|p| p.handle.hwnd()).unwrap_or(None);
        self
    }

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

    pub fn build(self) -> Result<FileDialog, SystemError> {
        unsafe {
            let mut dialog = FileDialog::default();
            dialog.parent = self.parent.unwrap_or(ptr::null_mut());
            dialog.action = self.action;
            dialog.multiselect = self.multiselect;

            let handle = rh::create_file_dialog(
                self.action,
                self.multiselect,
                self.default_folder,
                self.filters
            )?;

            dialog.handle = handle;

            Ok(dialog)
        }
    }

}

