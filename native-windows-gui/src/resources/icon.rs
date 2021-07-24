use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_ICON;
use crate::win32::resources_helper as rh;
use crate::{OemImage, OemIcon, NwgError};
use std::ptr;

#[cfg(feature = "embed-resource")]
use super::EmbedResource;


/**
A wrapper over a icon file (*.ico)

Windows icons are a legacy thing and should only be used when winapi forces you to use them (ex: when setting a window icon).
The `Bitmap` object of NWG not only supports transparency if "image-decoder" is enabled but it can also be create from multiple
different sources (including ".ico" files).

To display a icon in an application, see the `ImageFrame` control.

Note: Loading an icon from binary source (source_bin) REQUIRES the "image-decoder" feature.

**Builder parameters:**
  * `source_file`:      The source of the icon if it is a file.
  * `source_bin`:       The source of the icon if it is a binary blob. For example using `include_bytes!("my_icon.ico")`.
  * `source_system`:    The source of the icon if it is a system resource (see OemIcon)
  * `source_embed`:     The source of the icon if it is stored in an embedded file
  * `source_embed_id`:  The number identifier of the icon in the embedded file
  * `source_embed_str`: The string identifier of the icon in the embedded file
  * `size`:             Optional. Resize the image to this size.
  * `strict`:           Use a system placeholder instead of panicking if the image source do no exists.

Example:

```rust
use native_windows_gui as nwg;

fn load_icon() -> nwg::Icon {
    nwg::Icon::from_file("hello.ico", true).unwrap()
}

fn load_icon_builder() -> nwg::Icon {
    let mut icon = nwg::Icon::default();

    nwg::Icon::builder()
        .source_file(Some("hello.ico"))
        .strict(true)
        .build(&mut icon);

    icon
}

*/
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
        Single line helper function over the icon builder api.

        Use system resources.
    */
    pub fn from_system(sys_icon: OemIcon) -> Icon {
        let mut icon = Self::default();

        // Default icon creation cannot fail
        Self::builder()
            .source_system(Some(sys_icon))
            .build(&mut icon)
            .unwrap();

        icon
    }

    /**
        Single line helper function over the icon builder api.

        Use a file resource.
    */
    pub fn from_file(path: &str, strict: bool) -> Result<Icon, NwgError> {
        let mut icon = Icon::default();

        Icon::builder()
            .source_file(Some(path))
            .strict(strict)
            .build(&mut icon)?;

        Ok(icon)
    }

    /**
        Single line helper function over the icon builder api.

        Use a binary resource.
    */
    pub fn from_bin(bin: &[u8]) -> Result<Icon, NwgError> {
        let mut icon = Icon::default();

        Icon::builder()
            .source_bin(Some(bin))
            .build(&mut icon)?;

        Ok(icon)
    }

    /**
        Single line helper function over the icon builder api.

        Use an embedded resource. Either `embed_id` or `embed_str` must be defined, not both.

        Requires the `embed-resource` feature.
    */
    #[cfg(feature = "embed-resource")]
    pub fn from_embed(embed: &EmbedResource, embed_id: Option<usize>, embed_str: Option<&str>) -> Result<Icon, NwgError> {
        let mut icon = Icon::default();

        Icon::builder()
            .source_embed(Some(embed))
            .source_embed_id(embed_id.unwrap_or(0))
            .source_embed_str(embed_str)
            .build(&mut icon)?;

        Ok(icon)
    }

}

pub struct IconBuilder<'a> {
    source_text: Option<&'a str>,
    source_bin: Option<&'a [u8]>,
    source_system: Option<OemIcon>,

    #[cfg(feature = "embed-resource")]
    source_embed: Option<&'a EmbedResource>,

    #[cfg(feature = "embed-resource")]
    source_embed_id: usize,

    #[cfg(feature = "embed-resource")]
    source_embed_str: Option<&'a str>,

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

    pub fn source_system(mut self, t: Option<OemIcon>) -> IconBuilder<'a> {
        self.source_system = t;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed(mut self, em: Option<&'a EmbedResource>) -> IconBuilder<'a> {
        self.source_embed = em;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed_id(mut self, id: usize) -> IconBuilder<'a> {
        self.source_embed_id = id;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed_str(mut self, id: Option<&'a str>) -> IconBuilder<'a> {
        self.source_embed_str = id;
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

    pub fn build(self, b: &mut Icon) -> Result<(), NwgError> {
        if let Some(src) = self.source_text {
            let handle = unsafe { rh::build_image(src, self.size, self.strict, IMAGE_ICON)? };
            *b = Icon { handle, owned: true };
        } else if let Some(src) = self.source_system {
            let handle = unsafe { rh::build_oem_image(OemImage::Icon(src), self.size)? };
            *b = Icon { handle, owned: true };
        } else if let Some(src) = self.source_bin {
            let handle = unsafe { rh::icon_from_memory(src, self.strict, self.size)? };
            *b = Icon { handle, owned: true };
        } else {
            #[cfg(feature = "embed-resource")]
            fn build_embed(builder: IconBuilder) -> Result<Icon, NwgError> {
                match builder.source_embed {
                    Some(embed) => {
                        match builder.source_embed_str {
                            Some(src) => embed.icon_str(src, builder.size)
                                .ok_or_else(|| NwgError::resource_create(format!("No icon in embed resource identified by {}", src))),
                            None => embed.icon(builder.source_embed_id, builder.size)
                                .ok_or_else(|| NwgError::resource_create(format!("No icon in embed resource identified by {}", builder.source_embed_id)))
                        }
                    },
                    None => Err(NwgError::resource_create("No source provided for Icon"))
                }
            }

            #[cfg(not(feature = "embed-resource"))]
            fn build_embed(_builder: IconBuilder) -> Result<Icon, NwgError> {
                Err(NwgError::resource_create("No source provided for Icon"))
            }

            *b = build_embed(self)?;
        }
    
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

impl Drop for Icon {

    fn drop(&mut self) {
        if self.owned && !self.handle.is_null() {
            rh::destroy_icon(self.handle);
        }
    }

}

impl PartialEq for Icon {

    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }

}

