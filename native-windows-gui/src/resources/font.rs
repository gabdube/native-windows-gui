use winapi::shared::windef::HFONT;
use crate::win32::resources_helper as rh;
use crate::{SystemError};
use std::ptr;


/// A system font
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

/// Builds a font struct
///
/// - size: Size of the font
/// - weight: Weight of the font. A value betweem 0 and 1000. 0 use the system default, 100 is very thin, 1000 is very bold.
/// - family: Family name of the font (ex: Arial). Can be None to use the system default.
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

    pub fn build(self, font: &mut Font) -> Result<(), SystemError> {
        font.handle = unsafe { rh::build_font(
            self.size,
            self.weight,
            [false, false, false],
            self.family
        ) }?;

        Ok(())
    }

}

