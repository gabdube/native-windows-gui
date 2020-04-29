/*!
    Font resources creation
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use std::any::TypeId;
use std::hash::Hash;

use winapi::{c_int, DWORD, HFONT};

use controls::AnyHandle;
use defs::{FONT_DECO_ITALIC, FONT_DECO_STRIKEOUT, FONT_DECO_UNDERLINE};
use error::{Error, SystemError};
use resources::{Resource, ResourceT};
use ui::Ui;

/**
    A template that can create a font resource

    Params:
    • `size`: The height, in logical units, of the font's character cell or character. 0 means default height.
    • `weight`: The weight of the font in the range 0 through 1000. For example, 400 is normal and 700 is bold. See the FONT_WEIGHT_* constants for convenience
    • `decoration`: Extra style for the font. A bitwise combination of the FONT_DECO_* constants. Ex: FONT_DECO_ITALIC | FONT_DECO_UNDERLINE | FONT_DECO_STRIKEOUT
*/
#[derive(Clone)]
pub struct FontT<S: Clone + Into<String>> {
    pub family: S,
    pub size: c_int,
    pub weight: c_int,
    pub decoration: u32,
}

impl<ID: Clone + Hash, S: Clone + Into<String>> ResourceT<ID> for FontT<S> {
    fn resource_type_id(&self) -> TypeId {
        TypeId::of::<Font>()
    }

    #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Resource>, Error> {
        use gdi32::CreateFontW;
        use low::other_helper::to_utf16;
        use winapi::{
            CLEARTYPE_QUALITY, CLIP_DEFAULT_PRECIS, DEFAULT_CHARSET, OUT_DEFAULT_PRECIS,
            VARIABLE_PITCH,
        };

        let use_italic = ((self.decoration & FONT_DECO_ITALIC) != 0) as DWORD;
        let use_underline = ((self.decoration & FONT_DECO_UNDERLINE) != 0) as DWORD;
        let use_strikeout = ((self.decoration & FONT_DECO_STRIKEOUT) != 0) as DWORD;

        let family_name = to_utf16(self.family.clone().into().as_ref());

        let handle = unsafe {
            CreateFontW(
                self.size as c_int, // nHeight
                0,
                0,
                0,                    // nWidth, nEscapement, nOrientation
                self.weight,          // fnWeight
                use_italic,           // fdwItalic
                use_underline,        // fdwUnderline
                use_strikeout,        // fdwStrikeOut
                DEFAULT_CHARSET,      // fdwCharSet
                OUT_DEFAULT_PRECIS,   // fdwOutputPrecision
                CLIP_DEFAULT_PRECIS,  // fdwClipPrecision
                CLEARTYPE_QUALITY,    // fdwQuality
                VARIABLE_PITCH,       // fdwPitchAndFamily
                family_name.as_ptr(), // lpszFace
            )
        };

        if handle.is_null() {
            Err(Error::System(SystemError::FontCreation))
        } else {
            Ok(Box::new(Font { handle: handle }))
        }
    }
}

/**
    A font resource
*/
pub struct Font {
    handle: HFONT,
}

impl Resource for Font {
    fn handle(&self) -> AnyHandle {
        AnyHandle::HFONT(self.handle)
    }

    fn free(&mut self) {
        use gdi32::DeleteObject;
        unsafe {
            DeleteObject(::std::mem::transmute(self.handle));
        }
    }
}
