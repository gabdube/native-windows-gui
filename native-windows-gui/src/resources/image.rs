use crate::win32::resources_helper as rh;
use crate::{SystemError};
use winapi::um::winnt::HANDLE;
use std::ptr;


#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq)]
pub struct Image {
    pub(crate) handle: HANDLE,
}

impl Image {

    /// Create a new image from an icon (*.ico)
    ///  - source: Path to the icon
    ///  - size: Size of the image. If None, use the original image size
    ///  - strict: If set to `true`, the image creation will fail if the source cannot be read.  
    ///            If not, the resource creation will not fails and the Windows `Error` default icon will be loaded instead.
    pub fn icon<'a>(source: &'a str, size: Option<(u32, u32)>, strict: bool) -> Result<Image, SystemError> {
        unsafe { rh::build_image(source, size, strict, ::winapi::um::winuser::IMAGE_ICON).map(|i| Image { handle: i } ) }
    }

    /// Create a new image from a bitmap image (*.bmp)
    ///  - source: Path to the icon
    ///  - size: Size of the image. If None, use the original image size
    ///  - strict: If set to `true`, the image creation will fail if the source cannot be read.  
    ///            If not, the resource creation will not fails and the Windows `Error` default icon will be loaded instead.
    pub fn bitmap<'a>(source: &'a str, size: Option<(u32, u32)>, strict: bool) -> Result<Image, SystemError> {
        unsafe { rh::build_image(source, size, strict, ::winapi::um::winuser::IMAGE_BITMAP).map(|i| Image { handle: i } ) }
    }

    /// Create a new image from a cursor (*.cur)
    ///  - source: Path to the icon
    ///  - size: Size of the image. If None, use the original image size
    ///  - strict: If set to `true`, the image creation will fail if the source cannot be read.  
    ///            If not, the resource creation will not fails and the Windows `Error` default icon will be loaded instead.
    pub fn cursor<'a>(source: &'a str, size: Option<(u32, u32)>, strict: bool) -> Result<Image, SystemError> {
        unsafe { rh::build_image(source, size, strict, ::winapi::um::winuser::IMAGE_CURSOR).map(|i| Image { handle: i } ) }
    }

}

impl Default for Image {

    fn default() -> Image {
        Image { handle: ptr::null_mut() }
    }

}
