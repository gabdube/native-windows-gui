use winapi::shared::minwindef::DWORD;
use winapi::um::commdlg::{CHOOSEFONTW, ChooseFontW};
use winapi::um::wingdi::{LOGFONTW};
use super::FontInfo;
use crate::controls::ControlHandle;
use crate::NwgError;
use std::cell::{RefCell};
use std::{ptr, mem};
use std::pin::Pin;


struct InnerFontDialog {
    font: Pin<Box<LOGFONTW>>,
    dialog: CHOOSEFONTW,
}


/// The Font dialog box lets the user choose attributes for a logical font, such as
/// font family and associated font style, point size, effects (underline, strikeout),
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
    pub fn run<C: Into<ControlHandle>>(&self, owner: Option<C>) -> bool {
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
        let data: &InnerFontDialog = &self.data.borrow();
        let font: &LOGFONTW = &data.font;

        let end = font.lfFaceName.iter().position(|&c| c == 0).unwrap_or(0);
        let name = String::from_utf16(&font.lfFaceName[0..end]).unwrap_or("ERROR".to_string());

        FontInfo {
            point_size: data.dialog.iPointSize as u32,
            height: font.lfHeight as i32,
            width: font.lfWidth as i32,
            escapement: font.lfEscapement as i32,
            orientation: font.lfOrientation as i32,
            weight: font.lfWeight as i32,
            italic: font.lfItalic == 1,
            underline: font.lfUnderline == 1,
            strike_out: font.lfStrikeOut == 1,
            char_set: font.lfCharSet as u8,
            out_precision: font.lfOutPrecision as u8,
            clip_precision: font.lfClipPrecision as u8,
            quality: font.lfQuality as u8,
            pitch_and_family: font.lfPitchAndFamily as u8,
            name
        }
    }

}


/// The builder for a `FontDialog` object. Use `FontDialog::builder` to create one.
pub struct FontDialogBuilder {
}

impl FontDialogBuilder {

    pub fn build(self, _out: &mut FontDialog) -> Result<(), NwgError> {
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
