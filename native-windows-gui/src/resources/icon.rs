/*!
    A wrapper over a icon file (*.ico)
*/
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_ICON;
use crate::win32::resources_helper as rh;
use crate::SystemError;
use std::ptr;


/// A wrapper over a icon file (*.ico)
/// See module documentation
#[allow(unused)]
pub struct Icon {
    pub handle: HANDLE,
    pub(crate) owned: bool
}

impl Icon {

    pub fn builder<'a>() -> IconBuilder<'a> {
        IconBuilder {
            source_text: None,
            source_bin: None,
            size: None,
            strict: false
        }
    }

}

pub struct IconBuilder<'a> {
    source_text: Option<&'a str>,
    source_bin: Option<&'a [u8]>,
    size: Option<(u32, u32)>,
    strict: bool,
}

impl<'a> IconBuilder<'a> {

    pub fn source_file(mut self, t: Option<&'a str>) -> IconBuilder<'a> {
        self.source_text = t;
        self
    }

    pub fn source_bin(mut self, t: Option<&'a [u8]>) -> IconBuilder<'a> {
        self.source_bin = t;
        self
    }

    pub fn size(mut self, s: Option<(u32, u32)>) -> IconBuilder<'a> {
        self.size = s;
        self
    }

    pub fn strict(mut self, s: bool) -> IconBuilder<'a> {
        self.strict = s;
        self
    }

    pub fn build(self, b: &mut Icon) -> Result<(), SystemError> {
        let handle;
        
        if let Some(src) = self.source_text {
            handle = unsafe { rh::build_image(src, self.size, self.strict, IMAGE_ICON).ok() };
        } else {
            panic!("No source provided for Cursor. TODO ERROR");
        }

        *b = Icon { handle: handle.unwrap(), owned: true };
    
        Ok(())
    }

}


impl Default for Icon {

    fn default() -> Icon {
        Icon {
            handle: ptr::null_mut(),
            owned: false
        }
    }

}

impl PartialEq for Icon {

    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }

}

