use winapi::shared::windef::HFONT;
use crate::win32::resources_helper as rh;
use crate::{NwgError};
use std::ptr;


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
    pub(crate) handle: HFONT
}

impl Font {

    pub fn builder<'a>() -> FontBuilder<'a> {
        FontBuilder::new() 
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
    - size: Size of the font
    - weight: Weight of the font. A value betweem 0 and 1000. 0 use the system default, 100 is very thin, 1000 is very bold.
    - family: Family name of the font (ex: Arial). Can be None to use the system default.
*/
pub struct FontBuilder<'a> {
    size: u32,
    weight: u32,
    family: Option<&'a str>
}

impl<'a> FontBuilder<'a> {

    pub fn new() -> FontBuilder<'a> {
        FontBuilder {
            size: 16,
            weight: 0,
            family: None,
        }
    }

    pub fn size(mut self, size: u32) -> FontBuilder<'a> {
        self.size = size;
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
            self.size,
            self.weight,
            [false, false, false],
            self.family
        ) }?;

        Ok(())
    }

}

