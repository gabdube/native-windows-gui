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
    let mut cursor = nwg::Cursor::default();

    nwg::Cursor::builder()
        .source_file(Some("Hello.cur"))
        .strict(true)
        .build(&mut cursor);

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
