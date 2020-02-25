use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_BITMAP;
use crate::win32::resources_helper as rh;
use crate::{OemBitmap, OemImage, NwgError};
use std::ptr;


/** 
A wrapper over a bitmap file (*.bmp)

Note that Bitmap object are only used as display resources (ie: it's impossible to read pixels or resized it).
If those features are needed, see the `image-decoder` feature.

To display a bitmap in an application, see the `ImageFrame` control.

By default, bitmap resources do not support transparency BUT if `image-decoder` is enabled, bitmaps can be loaded
from any file type supported by NWG (JPEG, PNG, BMP, ICO, DDS, TIFF).

**Builder parameters:**
  * `source_file`:   The source of the bitmap if it is a file.
  * `source_bin`:    The source of the bitmap if it is a binary blob.
  * `source_system`: The source of the bitmap if it is a system resource (see OemBitmap)
  * `size`:          Optional. Resize the image to this size.
  * `strict`:        Use a system placeholder instead of panicking if the image source do no exists.

Example:

```rust
use native_windows_gui as nwg;

fn load_bitmap() -> nwg::Bitmap {
    let mut bitmap = nwg::Bitmap::default();

    nwg::Bitmap::builder()
        .source_file(Some("Hello.bmp"))
        .strict(true)
        .build(&mut bitmap);

    bitmap
}

```

*/
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
            size: None,
            strict: false
        }
    }

}

pub struct BitmapBuilder<'a> {
    source_text: Option<&'a str>,
    source_bin: Option<&'a [u8]>,
    source_system: Option<OemBitmap>,
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

    pub fn build(self, b: &mut Bitmap) -> Result<(), NwgError> {
        let handle;
        
        if let Some(src) = self.source_text {
            handle = unsafe { 
                #[cfg(feature="image-decoder")]
                let handle = rh::build_image_decoder(src, self.size, self.strict, IMAGE_BITMAP);

                #[cfg(not(feature="image-decoder"))]
                let handle = rh::build_image(src, self.size, self.strict, IMAGE_BITMAP);

                handle
            };
        } else if let Some(src) = self.source_system {
            handle = unsafe { rh::build_oem_image(OemImage::Bitmap(src), self.size) };
        } else if let Some(src) = self.source_bin { 
            handle = unsafe { rh::bitmap_from_memory(src) };
        } else {
            return Err(NwgError::resource_create("No source provided for Bitmap"));
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

impl Drop for Bitmap {

    fn drop(&mut self) {
        use winapi::um::wingdi::DeleteObject;
        if self.owned {
            unsafe { DeleteObject(self.handle); }
        }
    }

}
