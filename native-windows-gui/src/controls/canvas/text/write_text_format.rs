/*!
Wrapper over a IDWriteTextFormat interface. 
The TextFormat interface describes the font and paragraph properties used to format text, and it describes locale information.

```
use native_windows_gui as nwg;

fn test(fact: &nwg::WriteFactory) {
    nwg::WriteTextFormat::builder(&fact)
        .family("Arial")
        .size(19.0)
        .build();
}
```

Winapi documentation: https://docs.microsoft.com/en-us/windows/win32/api/dwrite/nn-dwrite-idwritetextformat
*/

use winapi::um::dwrite::IDWriteTextFormat;
use winapi::shared::winerror::S_OK;
use super::{WriteError, WriteFactory};
use crate::win32::base_helper::to_utf16;
use std::{ptr, fmt};


/// See module level documentation
pub struct WriteTextFormat {
    handle: *mut IDWriteTextFormat
}

impl WriteTextFormat {

    pub fn builder<'a>(fact: &'a WriteFactory) -> WriteTextFormatBuilder<'a> {
        use winapi::um::dwrite::{DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STRETCH_NORMAL};

        WriteTextFormatBuilder {
            fact,
            family: None,
            font_weight: DWRITE_FONT_WEIGHT_NORMAL,
            font_style: DWRITE_FONT_STYLE_NORMAL,
            font_stretch: DWRITE_FONT_STRETCH_NORMAL,
            size: 20.0,
            locale: Some("en-us"),
        }
    }

    /// Check if the write text format is initialized
    pub fn is_null(&self) -> bool { self.handle.is_null() }

}


/// A builder for a WriteTextFormat object
pub struct WriteTextFormatBuilder<'a> {
    fact: &'a WriteFactory,
    family: Option<&'a str>,
    locale: Option<&'a str>,
    font_weight: u32,
    font_style: u32,
    font_stretch: u32,
    size: f32,
}

impl<'a> WriteTextFormatBuilder<'a> {

    /// The name of the font family. Ex: "Arial"
    pub fn family(mut self, fam: &'a str) -> WriteTextFormatBuilder<'a> {
        self.family = Some(fam);
        self
    }

    /// The locale name. Ex: "en-us". Default to "en-us"
    pub fn locale(mut self, loc: Option<&'a str>) -> WriteTextFormatBuilder<'a> {
        self.locale = loc;
        self
    }

    /// Weight of the font. A value between 100 and 950. Default to "normal" (400)
    /// See: https://docs.microsoft.com/en-us/windows/win32/api/dwrite/ne-dwrite-dwrite_font_weight
    pub fn font_weight(mut self, w: u32) -> WriteTextFormatBuilder<'a> {
        self.font_weight = w;
        self
    }

    /// Represents the style of a font face as normal, italic, or oblique. Default to `Normal`
    pub fn font_style(mut self, s: u32) -> WriteTextFormatBuilder<'a> {
        self.font_style = s;
        self
    }

    /// The font stretch for the text object created by this method. A value between 1 and 9. Default to "normal" (5)
    /// See: https://docs.microsoft.com/en-us/windows/win32/api/dwrite/ne-dwrite-dwrite_font_stretch
    pub fn font_stretch(mut self, s: u32) -> WriteTextFormatBuilder<'a> {
        self.font_stretch = s;
        self
    }

    /// The logical size of the font in DIP ("device-independent pixel") units. A DIP equals 1/96 inch.
    /// Default size is `20.0`
    pub fn size(mut self, s: f32) -> WriteTextFormatBuilder<'a> {
        self.size = s;
        self
    }

    pub fn build(self) -> Result<WriteTextFormat, WriteError> {

        let family = match self.family {
            Some(f) => to_utf16(f),
            None => { return Err(WriteError::MissingParameter("family")) }
        };

        let locale = match self.locale {
            Some(l) => to_utf16(l),
            None => { return Err(WriteError::MissingParameter("locale")) }
        };

        let mut handle: *mut IDWriteTextFormat = ptr::null_mut();
        let result = unsafe { 
            (&*self.fact.handle).CreateTextFormat(
                family.as_ptr(),
                ptr::null_mut(),
                self.font_weight,
                self.font_style,
                self.font_stretch,
                self.size,
                locale.as_ptr(),
                &mut handle
            )
        };
        
        match result {
            S_OK => Ok(WriteTextFormat { handle }),
            e => Err(WriteError::Unknown(e))
        }
    }

}


impl Default for WriteTextFormat {

    fn default() -> WriteTextFormat {
        WriteTextFormat {  handle: ptr::null_mut() }
    }

}

impl fmt::Debug for WriteTextFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WriteTextFormat")
    }
}

impl Clone for WriteTextFormat {

    fn clone(&self) -> WriteTextFormat {
        match self.is_null() {
            true => WriteTextFormat{ handle: ptr::null_mut() },
            false => unsafe {
                (&*self.handle).AddRef();
                WriteTextFormat{ handle: self.handle }
            }
        }
    }

}

impl Drop for WriteTextFormat {

    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { (&*self.handle).Release(); }
            self.handle = ptr::null_mut();
        }
    }

}
