use winapi::shared::minwindef::{HINSTANCE, HRSRC, HGLOBAL};
use winapi::um::winuser::{LoadImageW, LR_DEFAULTSIZE, LR_CREATEDIBSECTION};
use winapi::ctypes::c_void;
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::NwgError;
use super::{Icon, Bitmap, Cursor};
use std::{ptr, slice};


/// Raw resource type that can be stored into an embedded resource.
#[derive(Copy, Clone, Debug)]
pub enum RawResourceType {
    Cursor,
    Bitmap,
    Icon,
    Menu,
    Dialog,
    String,
    FontDir,
    Font,
    Accelerator,
    RawData,
    MessageTable,
    Version,
    DlgInclude,
    PlugPlay,
    Vxd,
    AnimatedCursor,
    AnimatedIcon,
    Html,
    Manifest,
    Other(&'static str)
}

/**
    Represents a raw handle to a embed resource. Manipulating raw resources is inherently unsafe.
    `RawResources` are loaded using `EmbedResource::raw` and `EmbedResource::raw_str`

    In order to access the raw resource data use `as_mut_ptr()` or `as_mut_slice()` and cast the pointer to your data type.
*/
pub struct RawResource {
    module: HINSTANCE,
    handle: HRSRC,
    data_handle: HGLOBAL,
    ty: RawResourceType,
}

impl RawResource {

    /// Returns the system handle for the resource
    pub fn handle(&self) -> HRSRC {
        self.handle
    }

    /// Returns the system handle for the resource data
    pub fn data_handle(&self) -> HGLOBAL {
        self.data_handle
    }

    /// Returns the resource type set during texture loading
    pub fn resource_type(&self) -> RawResourceType {
        self.ty
    }

    /// Returns the size in bytes of the resource
    pub fn len(&self) -> usize {
        use winapi::um::libloaderapi::SizeofResource;

        unsafe {
            SizeofResource(self.module, self.handle) as usize
        }
    }

    /// Return a const pointer to the resource.
    pub unsafe fn as_mut_ptr(&mut self) -> *mut c_void {
        self.lock()
    }

    /// Return the resource data as a byte slice. This is equivalent to using `slice::from_raw_parts_mut`
    pub unsafe fn as_mut_slice(&self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.lock() as *mut u8, self.len())
    }

    fn lock(&self) -> *mut c_void {
        use winapi::um::libloaderapi::LockResource;
        unsafe { LockResource(self.data_handle) }
    }

}

/**
EmbedResource represent an embed resource file (".rc") inside on the executable module.

By default (without any arguments), the embed resources wraps the executable. If the embed resources
are in a dll, it's also possible to load them by setting the "module" parameter to the dll name.

**Builder parameters:**
    * `module`:  The name of the module that owns the embed resources. If `None`, use the executable name.

```rust
use native_windows_gui as nwg;
fn build_embed1() -> nwg::EmbedResource {
    nwg::EmbedResource::load(None).unwrap()
}

fn build_embed2() -> nwg::EmbedResource {
    nwg::EmbedResource::load(Some("external.dll")).unwrap()
}
```
*/
pub struct EmbedResource {
    pub hinst: HINSTANCE,
}

impl EmbedResource {

    /// Returns an embed resource that wraps the current executable. Shortcut for the builder API.
    pub fn load(name: Option<&str>) -> Result<EmbedResource, NwgError> {
        let mut embed = EmbedResource::default();
        EmbedResource::builder()
            .module(name)
            .build(&mut embed)?;

        Ok(embed)
    }

    /// Creates a `EmbedResourceBuilder`. `EmbedResource::load` can also be used to skip the builder api
    pub fn builder() -> EmbedResourceBuilder {
        EmbedResourceBuilder {
            module: None
        }
    }

    /// Load a string the the RC file STRINGTABLE. Returns `None` if `id` does not map to a string.
    pub fn string(&self, id: u32) -> Option<String> {
        use winapi::um::libloaderapi::LoadStringW;
        unsafe {
            let mut str_ptr = ptr::null_mut();
            let ccount = LoadStringW(self.hinst, id, (&mut str_ptr) as *mut *mut u16 as _, 0);
            match ccount {
                0 => None,
                count => {
                    let str_slice = slice::from_raw_parts(str_ptr, count as usize);
                    Some(from_utf16(str_slice))
                }
            }
        }
    }

    /// Load an icon from the rc file. Returns `None` if `id` does not map to a icon.
    /// For more feature, use the `Icon::builder` with the `embed` parameter.
    pub fn icon(&self, id: usize, size: Option<(u32, u32)>) -> Option<Icon> {
        use winapi::um::winuser::IMAGE_ICON;

        unsafe {
            let id_rc = id as _;
            let icon = match size {
                None => LoadImageW(self.hinst, id_rc, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE),
                Some((w, h)) =>  LoadImageW(self.hinst, id_rc, IMAGE_ICON, w as _, h as _, 0),
            };

            if icon.is_null() {
                None
            } else {
                Some(Icon { handle: icon as _, owned: true } )
            }
        }
    }

    /// Load an icon identified by a string in a resource file. Returns `None` if `id` does not map to a icon.
    pub fn icon_str(&self, id: &str, size: Option<(u32, u32)>) -> Option<Icon> {
        let name = to_utf16(id);
        self.icon(name.as_ptr() as usize, size)
    }

    /// Load a bitmap file from the rc file. Returns `None` if `id` does not map to a bitmap.
    pub fn bitmap(&self, id: usize, size: Option<(u32, u32)>) -> Option<Bitmap> {
        use winapi::um::winuser::IMAGE_BITMAP;

        unsafe {
            let id_rc = id as _;
            let bitmap = match size {
                None => LoadImageW(self.hinst, id_rc, IMAGE_BITMAP, 0, 0, LR_DEFAULTSIZE),
                Some((w, h)) =>  LoadImageW(self.hinst, id_rc, IMAGE_BITMAP, w as _, h as _, LR_CREATEDIBSECTION),
            };

            if bitmap.is_null() {
                None
            } else {
                Some(Bitmap { handle: bitmap as _, owned: true } )
            }
        }
    }

    /// Load a bitmap file from the rc file. Returns `None` if `id` does not map to a bitmap.
    pub fn bitmap_str(&self, id: &str, size: Option<(u32, u32)>) -> Option<Bitmap> {
        let name = to_utf16(id);
        self.bitmap(name.as_ptr() as usize, size)
    }

    
    #[cfg(feature="image-decoder")]
    /// Load an image from the embed files and returns a bitmap. An image is defined this way: `IMAGE_NAME IMAGE "../path/my_image.bmp"`
    /// This method can load any image type supported by the image decoder.
    pub fn image(&self, id: usize, size: Option<(u32, u32)>) -> Option<Bitmap> {
        use crate::win32::resources_helper as rh;

        match self.raw(id, RawResourceType::Other("Image")) {
            None => None,
            Some(raw) => {
                let src = unsafe { raw.as_mut_slice() };
                let handle = unsafe { rh::build_image_decoder_from_memory(src, size) };
                match handle {
                    Ok(handle) => Some(Bitmap { handle, owned: true }),
                    Err(e) => {
                        println!("{:?}", e);
                        None
                    }
                }
            }
        }
    }

    #[cfg(feature="image-decoder")]
    /// Load a image using a string name. See `EmbedResource::image`
    pub fn image_str(&self, id: &str, size: Option<(u32, u32)>) -> Option<Bitmap> {
        let name = to_utf16(id);
        self.image(name.as_ptr() as usize, size)
    }

    /// Load a cursor file from the rc file. Returns `None` if `id` does not map to a cursor.
    pub fn cursor(&self, id: usize) -> Option<Cursor> {
        use winapi::um::winuser::IMAGE_CURSOR;

        unsafe {
            let id_rc = id as _;
            let cursor = LoadImageW(self.hinst, id_rc, IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE);
            if cursor.is_null() {
                None
            } else {
                Some(Cursor { handle: cursor as _, owned: true } )
            }
        }
    }

    /// Load a cursor file from the rc file. Returns `None` if `id` does not map to a cursor.
    pub fn cursor_str(&self, id: &str) -> Option<Cursor> {
        let name = to_utf16(id);
        self.cursor(name.as_ptr() as usize)
    }

    /// Return a wrapper over the data of an embed resource. Return `None` `id` does not map to a resource.
    pub fn raw(&self, id: usize, ty: RawResourceType) -> Option<RawResource> {
        use winapi::um::libloaderapi::{FindResourceW, LoadResource};
        use RawResourceType::*;

        unsafe {
            let data_u16;
            let ty_value = match ty {
                Cursor => 1,
                Bitmap => 2,
                Icon => 3,
                Menu => 4,
                Dialog => 5,
                String => 6,
                FontDir => 7,
                Font => 8,
                Accelerator => 9,
                RawData => 10,
                MessageTable => 11,
                Version => 16,
                DlgInclude => 17,
                PlugPlay => 19,
                Vxd => 20,
                AnimatedCursor => 21,
                AnimatedIcon => 22,
                Html => 23,
                Manifest => 24,
                Other(value) => {
                    data_u16 = Some(to_utf16(value));
                    data_u16.as_ref().map(|v| v.as_ptr() as usize).unwrap()
                }
            };

            let handle = FindResourceW(self.hinst as _, id as _, ty_value as _);
            if handle.is_null() {
                return None;
            }

            let data_handle = LoadResource(self.hinst as _, handle);

            Some(RawResource {
                module: self.hinst,
                handle,
                data_handle,
                ty
            })
        }
    }

    /// Return a wrapper over the data of an embed resource. Return `None` `id` does not map to a resource.
    pub fn raw_str(&self, id: &str, ty: RawResourceType) ->  Option<RawResource> {
        let name = to_utf16(id);
        self.raw(name.as_ptr() as usize, ty)
    }

}


impl Default for EmbedResource {

    fn default() -> EmbedResource {
        EmbedResource {
            hinst: ptr::null_mut()
        }
    }

}


/**
    The EmbedResource builder. See `EmbedResource` docs.
*/
pub struct EmbedResourceBuilder {
    module: Option<String>
}

impl EmbedResourceBuilder {

    pub fn module(mut self, module: Option<&str>) -> EmbedResourceBuilder {
        self.module = module.map(|s| s.to_string());
        self
    }

    pub fn build(self, out: &mut EmbedResource) -> Result<(), NwgError> {
        use winapi::um::libloaderapi::GetModuleHandleW;

        let hinst = match self.module.as_ref() {
            Some(name) => {
                let name = to_utf16(name);
                unsafe { GetModuleHandleW(name.as_ptr()) as HINSTANCE }
            },
            None => unsafe { GetModuleHandleW(ptr::null_mut()) as HINSTANCE }
        };

        if hinst.is_null() {
            let name = self.module.as_ref().map(|name| name as &str ).unwrap_or("");
            return Err(NwgError::resource_create(format!("No module named \"{}\" in application", name)));
        }

        *out = EmbedResource {
            hinst
        };

        Ok(())
    }

}
