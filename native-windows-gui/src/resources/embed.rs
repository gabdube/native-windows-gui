use winapi::shared::minwindef::HINSTANCE;
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::NwgError;
use super::Icon;
use std::{ptr, slice};

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

    /// Load an icon from the rc file. Returns `None` if `id` does not map to a string.
    /// For more feature, use the `Icon::builder` with the `embed` parameter.
    pub fn icon(&self, id: u32) -> Option<Icon> {
        use winapi::um::winuser::{LoadImageW, IMAGE_ICON, LR_DEFAULTSIZE};

        unsafe {
            let id_rc = (id as usize) as _;
            let icon = LoadImageW(self.hinst, id_rc, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE);
            if icon.is_null() {
                None
            } else {
                Some(Icon { handle: icon as _, owned: true } )
            }
        }
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
