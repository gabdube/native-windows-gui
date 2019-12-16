/*!
    A wrapper over a cursor file (*.cur)
*/
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_CURSOR;
use crate::win32::resources_helper as rh;
use crate::{OemCursor, OemImage, NwgError};
use std::ptr;


/// A wrapper over a cursor file (*.cur)
/// See module documentation
#[allow(unused)]
pub struct Cursor {
    pub handle: HANDLE,
    pub(crate) owned: bool
}

impl Cursor {

    pub fn builder<'a>() -> CursorBuilder<'a> {
        CursorBuilder {
            source_text: None,
            source_system: None,
            size: None,
            strict: false
        }
    }

}

pub struct CursorBuilder<'a> {
    source_text: Option<&'a str>,
    source_system: Option<OemCursor>,
    size: Option<(u32, u32)>,
    strict: bool,
}

impl<'a> CursorBuilder<'a> {

    pub fn source_file(mut self, t: Option<&'a str>) -> CursorBuilder<'a> {
        self.source_text = t;
        self
    }

    pub fn source_system(mut self, t: Option<OemCursor>) -> CursorBuilder<'a> {
        self.source_system = t;
        self
    }

    pub fn size(mut self, s: Option<(u32, u32)>) -> CursorBuilder<'a> {
        self.size = s;
        self
    }

    pub fn strict(mut self, s: bool) -> CursorBuilder<'a> {
        self.strict = s;
        self
    }

    pub fn build(self, b: &mut Cursor) -> Result<(), NwgError> {
        let handle;
        
        if let Some(src) = self.source_text {
            handle = unsafe { rh::build_image(src, self.size, self.strict, IMAGE_CURSOR) };
        } else if let Some(src) = self.source_system {
            handle = unsafe { rh::build_oem_image(OemImage::Cursor(src), self.size) };
        } else {
            panic!("No source provided for Cursor. TODO ERROR");
        }

        *b = Cursor { handle: handle?, owned: true };
    
        Ok(())
    }

}


impl Default for Cursor {

    fn default() -> Cursor {
        Cursor {
            handle: ptr::null_mut(),
            owned: false
        }
    }

}

impl PartialEq for Cursor {

    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }

}
