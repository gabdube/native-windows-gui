/*!
    Wrapper types over some DirectDraw enums
*/
use winapi::um::dwrite::{DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STYLE_OBLIQUE, DWRITE_FONT_STYLE_ITALIC};


#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum FontStyle {
    Normal = DWRITE_FONT_STYLE_NORMAL,
    Oblique = DWRITE_FONT_STYLE_OBLIQUE,
    Italic = DWRITE_FONT_STYLE_ITALIC
}
