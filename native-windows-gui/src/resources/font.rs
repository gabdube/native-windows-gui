use winapi::shared::windef::HFONT;
use winapi::um::winnt::HANDLE;
use crate::win32::resources_helper as rh;
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::NwgError;
use std::ptr;

use std::sync::Mutex;


lazy_static! {
    /// Default font to use when creating controls. Set using `Font::set_global_default` && get using `Font::global_default()`
    static ref DEFAULT_FONT: Mutex<Option<Font>> = {
        Mutex::new(None)
    };
}

pub struct MemFont(pub HANDLE);

/** 
Represent a font parameters. Returned by the font dialog when the user selected a font.
Can also be used to create a Font resource using `Font::from_info`
For more information on the parameters see: https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfonta
*/
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


/**

Represent a system font.

Can be used with any controls that draws text. Due to the very limited way win32 can draw text,
only family, size and weight can be configured.

Example:

```rust
use native_windows_gui as nwg;

fn build_font() -> nwg::Font {
    let mut font = nwg::Font::default();

    nwg::Font::builder()
        .size(16)
        .family("Arial")
        .weight(1000)
        .build(&mut font);

    font
}

```

*/
#[derive(PartialEq, Eq, Debug)]
pub struct Font {
    pub handle: HFONT
}

impl Font {

    pub fn builder<'a>() -> FontBuilder<'a> {
        FontBuilder::new() 
    }

    /// Set the default (application global!) font that will be used when creating controls and return the old one
    pub fn set_global_default(font: Option<Font>) -> Option<Font> {
        let mut global_font = DEFAULT_FONT.lock().unwrap();
        let old = global_font.take();
        *global_font = font;
        old
    }

    /// Set the default (application global!) font that will be used when creating controls
    /// This is a shortcut over `Font::set_global_default`
    pub fn set_global_family(family: &str) -> Result<Option<Font>, NwgError> {
        let mut font = Font::default();

        Font::builder()
            .family(family)
            .build(&mut font)?;

        Ok(Font::set_global_default(Some(font)))
    }

    /// Return the default font that was previously set using `Font::set_default`
    pub fn global_default() -> Option<Font> {
        DEFAULT_FONT.lock()
            .unwrap()
            .as_ref()
            .map(|f| Font { handle: f.handle } )
    }

    /** 
        Add a font to the system font table. Don't forget to call `Font::remove_font(path)` once you're done.
        Returns `false` if the font could not be added. Windows won't tell you why though. 

        Other info:
        - The value of `path` can be a `ttf` or a `otf` font. 
        - Adding the same font multiple time increase the internal refcount
        - Use `Font::families()` to return the available system font families
    */
    pub fn add_font(path: &str) -> bool {
        use winapi::um::wingdi::AddFontResourceW;

        unsafe {
            let path = to_utf16(path);
            AddFontResourceW(path.as_ptr()) > 0
        }
    }

    /// Remove a font that was previously added by `Font::add_font`
    pub fn remove_font(path: &str) {
        use winapi::um::wingdi::RemoveFontResourceW;

        unsafe {
            let path = to_utf16(path);
            RemoveFontResourceW(path.as_ptr());
        }
    }

    /**
        Add a font resource from a binary source. Returns a memory font handle if the font was loaded succesfully.
        Send the handle to `remove_memory_font` at the end of your program to free the font from memory.
    */
    pub fn add_memory_font(bin: &mut [u8]) -> Result<MemFont, ()> {
        use winapi::um::wingdi::AddFontMemResourceEx;

        let bin_len = bin.len();
        let mut num_fonts = 0;
        let handle = unsafe {
            AddFontMemResourceEx(
                bin.as_mut_ptr() as _,
                bin_len as _,
                ptr::null_mut(),
                &mut num_fonts,
            )
        };

        if num_fonts > 0 {
            Ok(MemFont(handle))
        } else {
            Err(())
        }
    }

    /// Remove a font that was previously added by `Font::add_memory_font`
    pub fn remove_memory_font(font: MemFont) {
        use winapi::um::wingdi::RemoveFontMemResourceEx;

        unsafe {
            RemoveFontMemResourceEx(font.0);
        }
    }

    /// Returns all the font families loaded on the OS. 
    /// Probably pretty slow, so cache the value if possible
    pub fn families() -> Vec<String> {
        use winapi::um::wingdi::{LOGFONTW, TEXTMETRICW, DEFAULT_CHARSET, EnumFontFamiliesExW};
        use winapi::um::winuser::GetDC;
        use winapi::shared::minwindef::{DWORD, LPARAM};
        use std::mem;
        
        let mut families = Vec::with_capacity(16);

        unsafe extern "system" fn callback(font_ptr: *const LOGFONTW, _txt: *const TEXTMETRICW, _font_type: DWORD, lparam: LPARAM) -> i32 {
            let families_ptr = lparam as *mut Vec<String>;
            let families = &mut *families_ptr;

            let font = &*font_ptr;
            let family_text = from_utf16(&font.lfFaceName);
            if !families.iter().any(|f| f == &family_text) {
                families.push(family_text);
            }

            1
        }

        unsafe {
            let hdc = GetDC(ptr::null_mut());
            let mut font: LOGFONTW = mem::zeroed();
            font.lfCharSet = DEFAULT_CHARSET as u8;

            EnumFontFamiliesExW(hdc, &mut font, Some(callback), (&mut families as *mut Vec<String>) as _, 0);
        }

        families.shrink_to_fit();
        families
    }

}

impl Default for Font {

    fn default() -> Font {
        Font { handle: ptr::null_mut() }
    }

}

/**
Builds a font struct

Parameters:
    - size: Size of the font. The font mapper transforms this value into device units and matches it against the cell height of the available fonts. 
    - size_absolute:  Size of the font. The font mapper transforms this value into device units and matches its absolute value against the character height of the available fonts. 
    - weight: Weight of the font. A value betweem 0 and 1000. 0 use the system default, 100 is very thin, 1000 is very bold.
    - family: Family name of the font (ex: Arial). Can be None to use the system default.
*/
pub struct FontBuilder<'a> {
    size: Option<i32>,
    weight: u32,
    family: Option<&'a str>
}

impl<'a> FontBuilder<'a> {

    pub fn new() -> FontBuilder<'a> {
        FontBuilder {
            size: None,
            weight: 0,
            family: None,
        }
    }

    pub fn size(mut self, size: u32) -> FontBuilder<'a> {
        self.size = Some(size as i32);
        self
    }

    pub fn size_absolute(mut self, size: u32) -> FontBuilder<'a> {
        self.size = Some(-(size as i32));
        self
    }

    pub fn weight(mut self, weight: u32) -> FontBuilder<'a> {
        self.weight = weight;
        self
    }

    pub fn family(mut self, fam: &'a str) -> FontBuilder<'a> {
        self.family = Some(fam);
        self
    }

    pub fn build(self, font: &mut Font) -> Result<(), NwgError> {
        

        font.handle = unsafe { rh::build_font(
            self.size.unwrap_or(0),
            self.weight,
            [false, false, false],
            self.family
        ) }?;

        Ok(())
    }

}

unsafe impl Send for Font {}
unsafe impl Sync for Font {}

unsafe impl Send for MemFont {}
unsafe impl Sync for MemFont {}
