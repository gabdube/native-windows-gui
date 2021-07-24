use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_CURSOR;
use crate::win32::resources_helper as rh;
use crate::{OemCursor, OemImage, NwgError};
use std::ptr;

#[cfg(feature = "embed-resource")]
use super::EmbedResource;

/**
A wrapper over a cursor file (*.cur)

Cursor resources can be used with the `cursor` feature

Example:

```rust
use native_windows_gui as nwg;

fn load_cursor() -> nwg::Cursor {
    nwg::Cursor::from_file("Hello.cur", true).unwrap()
}

fn load_cursor_builder() -> nwg::Cursor {
    let mut cursor = nwg::Cursor::default();

    nwg::Cursor::builder()
        .source_file(Some("Hello.cur"))
        .strict(true)
        .build(&mut cursor)
        .unwrap();

    cursor
}

*/
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

            #[cfg(feature = "embed-resource")]
            source_embed: None,

            #[cfg(feature = "embed-resource")]
            source_embed_id: 0,

            #[cfg(feature = "embed-resource")]
            source_embed_str: None,
            
            strict: false
        }
    }

    pub fn from_system(cursor: OemCursor) -> Cursor {
        let mut out = Self::default();

        // Default cursor creation cannot fail
        Self::builder()
            .source_system(Some(cursor))
            .build(&mut out)
            .unwrap();

        out
    }

    /**
        Single line helper function over the cursor builder api.

        Use a file resource.
    */
    pub fn from_file(path: &str, strict: bool) -> Result<Cursor, NwgError> {
        let mut cursor = Cursor::default();

        Cursor::builder()
            .source_file(Some(path))
            .strict(strict)
            .build(&mut cursor)?;

        Ok(cursor)
    }

    /**
        Single line helper function over the cursor builder api.

        Use an embedded resource. Either `embed_id` or `embed_str` must be defined, not both.

        Requires the `embed-resource` feature.
    */
    #[cfg(feature = "embed-resource")]
    pub fn from_embed(embed: &EmbedResource, embed_id: Option<usize>, embed_str: Option<&str>) -> Result<Cursor, NwgError> {
        let mut bitmap = Cursor::default();

        Cursor::builder()
            .source_embed(Some(embed))
            .source_embed_id(embed_id.unwrap_or(0))
            .source_embed_str(embed_str)
            .build(&mut bitmap)?;

        Ok(bitmap)
    }

}

pub struct CursorBuilder<'a> {
    source_text: Option<&'a str>,
    source_system: Option<OemCursor>,
    size: Option<(u32, u32)>,

    #[cfg(feature = "embed-resource")]
    source_embed: Option<&'a EmbedResource>,

    #[cfg(feature = "embed-resource")]
    source_embed_id: usize,

    #[cfg(feature = "embed-resource")]
    source_embed_str: Option<&'a str>,

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

    #[cfg(feature = "embed-resource")]
    pub fn source_embed(mut self, em: Option<&'a EmbedResource>) -> CursorBuilder<'a> {
        self.source_embed = em;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed_id(mut self, id: usize) -> CursorBuilder<'a> {
        self.source_embed_id = id;
        self
    }

    #[cfg(feature = "embed-resource")]
    pub fn source_embed_str(mut self, id: Option<&'a str>) -> CursorBuilder<'a> {
        self.source_embed_str = id;
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
        if let Some(src) = self.source_text {
            let handle = unsafe { rh::build_image(src, self.size, self.strict, IMAGE_CURSOR)? };
            *b = Cursor { handle, owned: true };
        } else if let Some(src) = self.source_system {
            let handle = unsafe { rh::build_oem_image(OemImage::Cursor(src), self.size)? };
            *b = Cursor { handle, owned: true };
        } else {
            #[cfg(feature = "embed-resource")]
            fn build_embed(builder: CursorBuilder) -> Result<Cursor, NwgError> {
                match builder.source_embed {
                    Some(embed) => {
                        match builder.source_embed_str {
                            Some(src) => embed.cursor_str(src)
                                .ok_or_else(|| NwgError::resource_create(format!("No cursor in embed resource identified by {}", src))),
                            None => embed.cursor(builder.source_embed_id)
                                .ok_or_else(|| NwgError::resource_create(format!("No cursor in embed resource identified by {}", builder.source_embed_id)))
                        }
                    },
                    None => Err(NwgError::resource_create("No source provided for Cursor"))
                }
            }

            #[cfg(not(feature = "embed-resource"))]
            fn build_embed(_builder: CursorBuilder) -> Result<Cursor, NwgError> {
                Err(NwgError::resource_create("No source provided for Cursor"))
            }

            *b = build_embed(self)?;
        }

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

impl Drop for Cursor {

    fn drop(&mut self) {
        if self.owned && !self.handle.is_null() {
            rh::destroy_cursor(self.handle);
        }
    }

}
