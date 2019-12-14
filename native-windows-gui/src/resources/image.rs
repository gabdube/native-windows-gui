use crate::win32::resources_helper as rh;
use crate::{SystemError};
use super::system_images::{OemImage, OemIcon, OemBitmap, OemCursor};
use winapi::um::winnt::HANDLE;


/// An image resource. This can be an icon, a bitmap or a cursor.
/// To display an Image onto a window, see the ImageFrame control.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub(crate) handle: HANDLE,
}

impl Image {

    /// Create a new image from a system icon defined under `OemIcon`
    /// Will return a `SystemError` if the image could not be loaded
    ///
    ///  - source: Resource identifier
    ///  - size: Size of the image. If None, use the original image size
    pub fn oem_icon(source: OemIcon, size: Option<(u32, u32)>) -> Result<Image, SystemError> {
        unsafe { rh::build_oem_image(OemImage::Icon(source), size).map(|i| Image { handle: i } ) }
    }

    /// Create a new image from a system bitmap defined under `OemBitmap`
    /// Will return a `SystemError` if the image could not be loaded
    ///
    ///  - source: Resource identifier
    ///  - size: Size of the image. If None, use the original image size
    pub fn oem_bitmap(source: OemBitmap, size: Option<(u32, u32)>) -> Result<Image, SystemError> {
        unsafe { rh::build_oem_image(OemImage::Bitmap(source), size).map(|i| Image { handle: i } ) }
    }

    /// Create a new image from a system cursor defined under `OemCursor`
    /// Will return a `SystemError` if the image could not be loaded
    ///
    ///  - source: Resource identifier
    ///  - size: Size of the image. If None, use the original image size
    pub fn oem_cursor(source: OemCursor, size: Option<(u32, u32)>) -> Result<Image, SystemError> {
        unsafe { rh::build_oem_image(OemImage::Cursor(source), size).map(|i| Image { handle: i } ) }
    }

}
