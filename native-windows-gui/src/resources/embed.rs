use winapi::shared::minwindef::HINSTANCE;
use crate::win32::base_helper::to_utf16;
use crate::{NwgError};
use std::ptr;

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
    hinst: HINSTANCE,
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
