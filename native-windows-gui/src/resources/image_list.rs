use winapi::um::commctrl::{HIMAGELIST, ImageList_AddMasked};
use winapi::shared::windef::{HICON, HBITMAP};
use crate::{Bitmap, Icon, NwgError};
use std::ptr;


const NOT_BOUND: &'static str = "ImageList is not yet bound to a winapi object";

/**
An image list is a collection of images of the same size, each of which can be referred to by its index.
Image lists are used in controls such as tabs container and tree view in order to add icon next to the items.

There are two kinds of image list in Winapi: masked. This is a wrapper over the masked type.

Image list and the method that use them in controls are behind the "image-list" feature. 

**Builder parameters:**
  * `size`:    The size size of the images in the image list. Default `(32, 32)`
  * `initial`: The initial size (in images) of the image list. Default `5`
  * `grow`:    The number of images by which the image list can grow when the system needs to make room for new images. Default `5`

```rust
use native_windows_gui as nwg;
fn build_image_list(list: &mut nwg::ImageList) {
    nwg::ImageList::builder()
        .size((64, 64))
        .initial(10)
        .grow(1)
        .build(list);
}
```

*/
pub struct ImageList {
    pub handle: HIMAGELIST,
    pub owned: bool
}

impl ImageList {

    pub fn builder() -> ImageListBuilder {
        ImageListBuilder {
            size: (32, 32),
            initial: 5,
            grow: 5
        }
    }

    /// Returns the size of the images in the image list
    pub fn size(&self) -> (i32, i32) {
        use winapi::um::commctrl::ImageList_GetIconSize;

        if self.handle.is_null() { panic!("{}", NOT_BOUND); }

        let mut size = (0, 0);
        unsafe { ImageList_GetIconSize(self.handle, &mut size.0, &mut size.1); }

        size
    }

    /// Sets the size of the image list. This clears all current image data.
    pub fn set_size(&self, size: (i32, i32)) {
        use winapi::um::commctrl::ImageList_SetIconSize;

        if self.handle.is_null() { panic!("{}", NOT_BOUND); }

        let (w, h) = size;
        unsafe { ImageList_SetIconSize(self.handle, w, h); }
    }

    /// Returns the number of images in the image list
    pub fn len(&self) -> u32 {
        use winapi::um::commctrl::ImageList_GetImageCount;

        if self.handle.is_null() { panic!("{}", NOT_BOUND); }

        unsafe { ImageList_GetImageCount(self.handle) as u32 }
    }

    /// Adds a new bitmap to the image list. Returns the index to the image. Panics if the bitmap was not initialized
    pub fn add_bitmap(&self, bitmap: &Bitmap) -> i32 {
        if self.handle.is_null() { panic!("{}", NOT_BOUND); }
        if bitmap.handle.is_null() { panic!("Bitmap was not initialized"); }

        unsafe { ImageList_AddMasked(self.handle, bitmap.handle as HBITMAP, 0) }
    }

    /**
        Adds a bitmap directly from a filename. The image is resized to the image list size.
        Returns the index to the image or an error if the image could not be loaded
    */
    pub fn add_bitmap_from_filename(&self, filename: &str) -> Result<i32, NwgError> {
        if self.handle.is_null() { panic!("{}", NOT_BOUND); }

        let (w, h) = self.size();
        let mut bitmap = Bitmap::default();
        Bitmap::builder()
            .source_file(Some(filename))
            .size(Some((w as u32, h as u32)))
            .strict(true)
            .build(&mut bitmap)?;

        unsafe { Ok(ImageList_AddMasked(self.handle, bitmap.handle as HBITMAP, 0)) }
    }

    /// Adds a new icon to the image list. Returns the index to the image. Panics if the icon was not initialized
    pub fn add_icon(&self, icon: &Icon) -> i32 {
        use winapi::um::winuser::{GetIconInfo, ICONINFO};
        use winapi::um::wingdi::DeleteObject;

        if self.handle.is_null() { panic!("{}", NOT_BOUND); }
        if icon.handle.is_null() { panic!("Icon was not initialized"); }

        // Extract the bitmap from the icon
        // Can't use `ImageList_AddIcon` because it doesn't always guess the mask
        unsafe {
            let mut info: ICONINFO = ::std::mem::zeroed();
            GetIconInfo(icon.handle as _, &mut info);
            
            let i = ImageList_AddMasked(self.handle, info.hbmColor, 0);

            DeleteObject(info.hbmMask as _);
            DeleteObject(info.hbmColor as _);

            i
        }
    }

    /**
        Adds a icon directly from a filename. The image is resized to the image list size.
        Returns the index to the image or an error if the image could not be loaded
    */
    pub fn add_icon_from_filename(&self, filename: &str) -> Result<i32, NwgError> {
        if self.handle.is_null() { panic!("{}", NOT_BOUND); }

        let (w, h) = self.size();
        let mut icon = Icon::default();
        Icon::builder()
            .source_file(Some(filename))
            .size(Some((w as u32, h as u32)))
            .strict(true)
            .build(&mut icon)?;

        Ok(self.add_icon(&icon))
    }

    /**
        Removes the image at the specified index

        When an image is removed, the indexes of the remaining images are adjusted so that the image indexes
        always range from zero to one less than the number of images in the image list. For example, if you
        remove the image at index 0, then image 1 becomes image 0, image 2 becomes image 1, and so on.
    */
    pub fn remove(&self, index: i32) {
        use winapi::um::commctrl::ImageList_Remove;

        if self.handle.is_null() { panic!("{}", NOT_BOUND); }

        unsafe { ImageList_Remove(self.handle, index); }
    }

    /// Replaces an image in the image list. Panics if the bitmap was not initialized
    pub fn replace_bitmap(&self, index: i32, bitmap: &Bitmap) {
        use winapi::um::commctrl::ImageList_Replace;

        if self.handle.is_null() { panic!("{}", NOT_BOUND); }
        if bitmap.handle.is_null() { panic!("Bitmap was not initialized"); }
        
        unsafe { ImageList_Replace(self.handle, index, bitmap.handle as HBITMAP, ptr::null_mut()); }
    }

    /// Replaces an image in the image list by an icon. Panics if the icon was not initialized
    pub fn replace_icon(&self, index: i32, icon: &Icon) {
        use winapi::um::commctrl::ImageList_ReplaceIcon;

        if self.handle.is_null() { panic!("{}", NOT_BOUND); }
        if icon.handle.is_null() { panic!("Icon was not initialized"); }

        unsafe { ImageList_ReplaceIcon(self.handle, index, icon.handle as HICON); }
    }

}

impl Drop for ImageList {
    fn drop(&mut self) {
        use winapi::um::commctrl::ImageList_Destroy;
        unsafe {
            if self.owned && !self.handle.is_null() {
                ImageList_Destroy(self.handle);
            }
        }
    }
}

impl Default for ImageList {

    fn default() -> ImageList {
        ImageList {
            handle: ptr::null_mut(),
            owned: false
        }
    }

}

impl PartialEq for ImageList {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

pub struct ImageListBuilder {
    size: (i32, i32),
    initial: i32,
    grow: i32,
}

impl ImageListBuilder {

    pub fn size(mut self, size: (i32, i32)) -> ImageListBuilder {
        self.size = size;
        self
    }

    pub fn initial(mut self, initial: i32) -> ImageListBuilder {
        self.initial = initial;
        self
    }

    pub fn grow(mut self, grow: i32) -> ImageListBuilder {
        self.grow = grow;
        self
    }

    pub fn build(self, list: &mut ImageList) -> Result<(), NwgError> {
        use winapi::um::commctrl::{ImageList_Create, ILC_COLOR32, ILC_MASK};

        unsafe {
            let (w, h) = self.size;
            let handle = ImageList_Create(w, h, ILC_COLOR32 | ILC_MASK, self.initial, self.grow);
            if handle.is_null() {
                return Err(NwgError::resource_create("Failed to create image list"));
            }

            list.handle = handle;
            list.owned = true;
        }
        
        Ok(())
    }

}
