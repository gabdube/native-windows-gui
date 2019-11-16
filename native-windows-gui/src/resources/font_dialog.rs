use winapi::shared::minwindef::DWORD;
use winapi::um::commdlg::{CHOOSEFONTW, ChooseFontW};
use winapi::um::wingdi::LOGFONTW;
use crate::controls::ControlHandle;
use crate::SystemError;
use std::cell::{RefCell};
use std::{ptr, mem};
use std::pin::Pin;


struct InnerFontDialog {
    font: Pin<Box<LOGFONTW>>,
    dialog: CHOOSEFONTW,
}

/// Represent a font parameters. Returned by the font dialog when the user selected a font.
/// Can also be used to create a Font resource using `Font::from_info`
#[derive(Debug, Clone)]
pub struct FontInfo {

}


/// The Font dialog box lets the user choose attributes for a logical font, such as
/// font family and associated font style, point size, effects (underline, strikeout, and text color),
/// and a script (or character set).
pub struct FontDialog {
    data: RefCell<InnerFontDialog>,
}

impl FontDialog {

    pub fn builder() -> FontDialogBuilder {
        FontDialogBuilder {}
    }

    /// Execute the font dialog.
    /// This function will return `true` if the user select a font or `false` if the dialog is cancelled
    pub fn show<C: Into<ControlHandle>>(&self, owner: Option<C>) -> bool {
        if owner.is_some() {
            let ownder_handle = owner.unwrap().into();
            self.data.borrow_mut().dialog.hwndOwner = ownder_handle.hwnd().expect("Color dialog must be a window control");
        }

        unsafe {
            let mut data = self.data.borrow_mut();
            let data_ref = &mut data.dialog;
            ChooseFontW(data_ref as *mut CHOOSEFONTW) > 0
        }
    }

    /// Return a `FontInfo` structure that describe the font selected by the user.
    pub fn font(&self) -> FontInfo {
        FontInfo {

        }
    }

}


/// The builder for a `FontDialog` object. Use `FontDialog::builder` to create one.
pub struct FontDialogBuilder {
}

impl FontDialogBuilder {

    pub fn build(self, _out: &mut FontDialog) -> Result<(), SystemError> {
        Ok(())
    }

}

impl Default for FontDialog {

    fn default() -> FontDialog {
        let dialog = CHOOSEFONTW {
            lStructSize: mem::size_of::<CHOOSEFONTW>() as DWORD,
            hwndOwner: ptr::null_mut(),
            hDC: ptr::null_mut(),
            lpLogFont: ptr::null_mut(),
            iPointSize: 0,
            Flags: 0,
            rgbColors: 0,
            lCustData: 0,
            lpfnHook: None,
            lpTemplateName: ptr::null(),
            hInstance: ptr::null_mut(),
            lpszStyle: ptr::null_mut(),
            nFontType: 0,
            ___MISSING_ALIGNMENT__: 0,
            nSizeMin: 0,
            nSizeMax: 0
        };

        let font: LOGFONTW = unsafe { mem::zeroed() };

        let mut inner = InnerFontDialog {
            font: Box::pin(font),
            dialog,
        };

        let mut font = inner.font.as_mut();
        let cols_ref: &mut LOGFONTW = &mut font;
        inner.dialog.lpLogFont = cols_ref as *mut LOGFONTW;

        FontDialog {
            data: RefCell::new(inner)
        }
    }

}
