use winapi::shared::minwindef::DWORD;
use winapi::um::commdlg::{CHOOSEFONTW, ChooseFontW};
use winapi::um::wingdi::{LOGFONTW};
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
/// For more information on the parameters see: https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfonta
#[derive(Debug, Clone)]
pub struct FontInfo {
    /// The size of the selected font, in units of 1/10 of a point
    pub point_size: u32,
    /// Specifies the height, in logical units, of the font's character cell or character.
    pub height: i32,
    /// Specifies the width, in logical units, of characters in the font.
    pub width: i32,
    /// Contains the angle, in tenths of degrees, between the escapement vector and the x-axis of the device. The escapement vector is parallel to the base line of a row of text.
    pub escapement: i32,
    /// Specifies the angle, in tenths of degrees, between each character's base line and the x-axis of the device.
    pub orientation: i32,
    /// Specifies the weight of the font in the range from 0 through 1000.
    pub weight: i32,
    /// Specifies an italic font if set to TRUE
    pub italic: bool,
    /// Specifies an underlined font if set to TRUE.
    pub underline: bool,
    /// Specifies a strikeout font if set to TRUE.
    pub strike_out: bool,
    /// Specifies the character set.
    pub char_set: u8,
    /// Specifies the output precision. The output precision defines how closely the output must match the requested font's height, width, character orientation, escapement, pitch, and font type.
    pub out_precision: u8,
    /// specifies the clipping precision. The clipping precision defines how to clip characters that are partially outside the clipping region.
    pub clip_precision: u8,
    /// specifies the output quality. The output quality defines how carefully the GDI must attempt to match the logical-font attributes to those of an actual physical font.
    pub quality: u8,
    /// Specifies the pitch and family of the font.
    pub pitch_and_family: u8,
    /// Contains a null-terminated string that specifies the typeface name of the font. 
    pub name: String
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
