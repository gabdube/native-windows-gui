use winapi::um::wincodec::{IWICImagingFactory};
use crate::win32::image_decoder::{create_image_factory};
use crate::NwgError;
use std::ptr;


/**
    A image decoder. Can load an extended number of image file format from a filename, from a file handle, or from a stream.

    ImageDecoder do not take any parameter to build, but it still provides a builder API to match the other component of NWG.

    You can also use `ImageDecoder::new` to avoid the builder API.

```rust
use native_windows_gui as nwg;
fn build_image_decode() -> nwg::ImageDecoder {
    nwg::ImageDecoder::builder()
        .build()
}
```
*/
pub struct ImageDecoder {
    factory: *mut IWICImagingFactory,
}

impl ImageDecoder {
    pub fn new() -> Result<ImageDecoder, NwgError> {
        let factory = unsafe { create_image_factory() }?;
        Ok(ImageDecoder { factory })
    }

    pub fn builder() -> ImageDecoderBuilder {
        ImageDecoderBuilder {
        }
    }
}

impl Default for ImageDecoder {
    fn default() -> ImageDecoder {
        ImageDecoder {
            factory: ptr::null_mut()
        }
    }
}

pub struct ImageDecoderBuilder {
}

impl ImageDecoderBuilder {
    pub fn build(self, out: &mut ImageDecoder) -> Result<(), NwgError> {
        let factory = unsafe { create_image_factory() }?;
        *out = ImageDecoder { factory };
        Ok(())
    }
}
