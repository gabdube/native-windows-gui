use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_BITMAP;
use crate::win32::resources_helper as rh;
use crate::{OemBitmap, OemImage, NwgError};
use std::ptr;

#[cfg(feature = "embed-resource")]
use super::EmbedResource;

/** 
A wrapper over a bitmap file (*.bmp)

Note that Bitmap object are only used as display resources (ie: it's impossible to read pixels or resized it).
If those features are needed, see the `image-decoder` feature.

To display a bitmap in an application, see the `ImageFrame` control.

By default, bitmap resources do not support transparency BUT if `image-decoder` is enabled, bitmaps can be loaded
from any file type supported natively by Windows: `JPEG, PNG, BMP, ICO, DDS, TIFF`.

Bitmaps can be converted to icons using the "copy_as_icon" function.


**Builder parameters:**
  * `source_file`:      The source of the bitmap if it is a file.
  * `source_bin`:       The source of the bitmap if it is a binary blob. For example using `include_bytes!("my_icon.bmp")`.
  * `source_system`:    The source of the bitmap if it is a system resource (see OemBitmap)
  * `source_embed`:     The source of the bitmap if it is stored in an embedded file
  * `source_embed_id`:  The number identifier of the icon in the embedded file
  * `source_embed_str`: The string identifier of the icon in the embedded file
  * `size`:             Optional. Resize the image to this size.
  * `strict`:           Use a system placeholder instead of panicking if the image source do no exists.

Example:

```rust
use native_windows_gui as nwg;

fn load_bitmap() -> nwg::Bitmap {
    nwg::Bitmap::from_file("Hello.bmp", true).unwrap()
}

fn load_bitmap_builder() -> nwg::Bitmap {
    let mut bitmap = nwg::Bitmap::default();

    nwg::Bitmap::builder()
        .source_file(Some("Hello.bmp"))
        .strict(true)
        .build(&mut bitmap)
        .unwrap();

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

            #[cfg(feature = "embed-resource")]
            source_embed: None,

            #[cfg(feature = "embed-resource")]
            source_embed_id: 0,

            #[cfg(feature = "embed-resource")]
            source_embed_str: None,

            size: None,
            strict: false
        }
    }

    /**
        Single line helper function over the bitmap builder api.

        Use system resources.
    */
    pub fn from_system(sys_bitmap: OemBitmap) -> Bitmap {
        let mut bitmap = Self::default();

        // Default cursor creation cannot fail
        Self::builder()
            .source_system(Some(sys_bitmap))
            .build(&mut bitmap)
            .unwrap();

            bitmap
    }

    /**
        Single line helper function over the bitmap builder api.

        Use a file resource.
    */
    pub fn from_file(path: &str, strict: bool) -> Result<Bitmap, NwgError> {
        let mut bitmap = Bitmap::default();

        Bitmap::builder()
            .source_file(Some(path))
            .strict(strict)
            .build(&mut bitmap)?;

        Ok(bitmap)
    }

    /**
        Single line helper function over the bitmap builder api.

        Use a binary resource.
    */
    pub fn from_bin(bin: &[u8]) -> Result<Bitmap, NwgError> {
        let mut bitmap = Bitmap::default();

        Bitmap::builder()
            .source_bin(Some(bin))
            .build(&mut bitmap)?;

        Ok(bitmap)
    }

    /**
        Single line helper function over the bitmap builder api.

        Use an embedded resource. Either `embed_id` or `embed_str` must be defined, not both.

        Requires the `embed-resource` feature.
    */
    #[cfg(feature = "embed-resource")]
    pub fn from_embed(embed: &EmbedResource, embed_id: Option<usize>, embed_str: Option<&str>) -> Result<Bitmap, NwgError> {
        let mut bitmap = Bitmap::default();

        Bitmap::builder()
            .source_embed(Some(embed))
            .source_embed_id(embed_id.unwrap_or(0))
            .source_embed_str(embed_str)
            .build(&mut bitmap)?;

        Ok(bitmap)
    }

    /**
        Creates a new icon from the bitmap data.
        
        Panics if the bitmap is not initialized
    */
    pub fn copy_as_icon(&self) -> crate::Icon {
        use winapi::um::winuser::CreateIconIndirect;
        use winapi::um::winuser::ICONINFO;

        if self.handle.is_null() {
            panic!("Bitmap was not initialized");
        }

        let mut icon_info = ICONINFO {
            fIcon: 1,
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: self.handle as _,
            hbmColor: self.handle as _
        };

        let icon = unsafe { CreateIconIndirect(&mut icon_info) };

        crate::Icon {
            handle: icon as _,
            owned: true
        }
    }

}

pub struct BitmapBuilder<'a> {
    source_text: Option<&'a str>,
    source_bin: Option<&'a [u8]>,
    source_system: Option<OemBitmap>,

    #[cfg(feature = "embed-resource")]
    source_embed: Option<&'a EmbedResource>,

    #[cfg(feature = "embed-resource")]
    source_embed_id: usize,

    #[cfg(feature = "embed-resource")]
    source_embed_str: Option<&'a str>,
    
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

    #[cfg(feature = "embed-resource")]
    pub fn source_embed(mut self, em: Option<&'a EmbedResource>) -> BitmapBuilder<'a> {
        self.source_embed = em;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed_id(mut self, id: usize) -> BitmapBuilder<'a> {
        self.source_embed_id = id;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed_str(mut self, id: Option<&'a str>) -> BitmapBuilder<'a> {
        self.source_embed_str = id;
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
        if let Some(src) = self.source_text {
            let handle = unsafe { 
                #[cfg(feature="image-decoder")]
                let handle = rh::build_image_decoder(src, self.size, self.strict, IMAGE_BITMAP);

                #[cfg(not(feature="image-decoder"))]
                let handle = rh::build_image(src, self.size, self.strict, IMAGE_BITMAP);

                handle?
            };

            *b = Bitmap { handle, owned: true };
        } else if let Some(src) = self.source_system {
            let handle = unsafe { rh::build_oem_image(OemImage::Bitmap(src), self.size)? };
            *b = Bitmap { handle, owned: true };
        } else if let Some(src) = self.source_bin { 
            let handle = unsafe { rh::bitmap_from_memory(src)? };

            *b = Bitmap { handle, owned: true };
        } else {
            #[cfg(all(feature = "embed-resource", feature="image-decoder"))]
            fn build_embed(builder: BitmapBuilder) -> Result<Bitmap, NwgError> {
                match builder.source_embed {
                    Some(embed) => {
                        match builder.source_embed_str {
                            Some(src) => embed.image_str(src, builder.size)
                                .ok_or_else(|| NwgError::resource_create(format!("No bitmap in embed resource identified by {}", src))),
                            None => embed.image(builder.source_embed_id, builder.size)
                                .ok_or_else(|| NwgError::resource_create(format!("No bitmap in embed resource identified by {}", builder.source_embed_id)))
                        }
                    },
                    None => Err(NwgError::resource_create("No source provided for Bitmap"))
                }
            }

            #[cfg(feature = "embed-resource")]
            #[cfg(not(feature = "image-decoder"))]
            fn build_embed(builder: BitmapBuilder) -> Result<Bitmap, NwgError> {
                match builder.source_embed {
                    Some(embed) => {
                        match builder.source_embed_str {
                            Some(src) => embed.bitmap_str(src, builder.size)
                                .ok_or_else(|| NwgError::resource_create(format!("No bitmap in embed resource identified by {}", src))),
                            None => embed.bitmap(builder.source_embed_id, builder.size)
                                .ok_or_else(|| NwgError::resource_create(format!("No bitmap in embed resource identified by {}", builder.source_embed_id)))
                        }
                    },
                    None => Err(NwgError::resource_create("No source provided for Bitmap"))
                }
            }

            #[cfg(not(feature = "embed-resource"))]
            fn build_embed(_builder: BitmapBuilder) -> Result<Bitmap, NwgError> {
                Err(NwgError::resource_create("No source provided for Bitmap"))
            }

            *b = build_embed(self)?;
        }
    
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
        if self.owned && !self.handle.is_null() {
            rh::destroy_obj(self.handle);
        }
    }

}
