/*!
    A wrapper over a bitmap file (*.bmp)
*/
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_BITMAP;
use crate::win32::resources_helper as rh;
use crate::{OemBitmap, OemImage, NwgError};
use std::ptr;


/// A wrapper over a bitmap file (*.bmp)
/// See module documentation
#[allow(unused)]
pub struct Bitmap {
    pub handle: HANDLE,
    pub(crate) owned: bool
}

impl Bitmap {

    pub fn builder<'a>() -> BitmapBuilder<'a> {
        BitmapBuilder {
            source_text: None,
            source_bin: None,
            source_system: None,
            transparency_key: None,
            size: None,
            strict: false
        }
    }

}

pub struct BitmapBuilder<'a> {
    source_text: Option<&'a str>,
    source_bin: Option<&'a [u8]>,
    source_system: Option<OemBitmap>,
    transparency_key: Option<[u8; 3]>,
    size: Option<(u32, u32)>,
    strict: bool,
}

impl<'a> BitmapBuilder<'a> {

    pub fn source_file(mut self, t: Option<&'a str>) -> BitmapBuilder<'a> {
        self.source_text = t;
        self
    }

    pub fn source_bin(mut self, t: Option<&'a [u8]>) -> BitmapBuilder<'a> {
        self.source_bin = t;
        self
    }

    pub fn source_system(mut self, t: Option<OemBitmap>) -> BitmapBuilder<'a> {
        self.source_system = t;
        self
    }

    pub fn size(mut self, s: Option<(u32, u32)>) -> BitmapBuilder<'a> {
        self.size = s;
        self
    }

    pub fn strict(mut self, s: bool) -> BitmapBuilder<'a> {
        self.strict = s;
        self
    }

    pub fn transparency_key(mut self, k: Option<[u8; 3]>) -> BitmapBuilder<'a> {
        self.transparency_key = k;
        self
    }

    pub fn build(self, b: &mut Bitmap) -> Result<(), NwgError> {
        let mut handle;
        
        if let Some(src) = self.source_text {
            handle = unsafe { rh::build_image(src, self.size, self.strict, IMAGE_BITMAP) };
        } else if let Some(src) = self.source_system {
            handle = unsafe { rh::build_oem_image(OemImage::Bitmap(src), self.size) };
        } else {
            panic!("No source provided for Bitmap. TODO ERROR");
        }

        if let Some(key) = self.transparency_key {
            let size = match self.size {
                Some((x, y)) => (x as i32, y as i32),
                None => (0, 0)
            };

            handle = unsafe { rh::make_bitmap_transparent(handle?, size, key) };
        }
        
        *b = Bitmap { handle: handle?, owned: true };
    
        Ok(())
    }

}


impl Default for Bitmap {

    fn default() -> Bitmap {
        Bitmap {
            handle: ptr::null_mut(),
            owned: false
        }
    }

}

impl PartialEq for Bitmap {

    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }

}
