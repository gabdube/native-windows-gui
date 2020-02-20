use winapi::um::commctrl::HIMAGELIST;
use winapi::shared::windef::{HICON, HBITMAP};
use crate::{Bitmap, Icon, NwgError};
use std::ptr;


/**
An image list is a collection of images of the same size, each of which can be referred to by its index.
Image lists are used in controls such as tabs container and tree view in order to add icon next to the items.

There are two kinds of image list in Winapi: nonmasked and masked. This is a wrapper over the nonmasked type.

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
    pub handle: HIMAGELIST
}

impl ImageList {

    pub fn builder() -> ImageListBuilder {
        ImageListBuilder {
            size: (32, 32),
            initial: 5,
            grow: 5
        }
    }

    /// Add a new bitmap to the image list. Returns the index to the image.
    pub fn add(&self, bitmap: &Bitmap) -> i32 {
        use winapi::um::commctrl::ImageList_Add;
        unsafe { ImageList_Add(self.handle, bitmap.handle as HBITMAP, ptr::null_mut()) }
    }

    /// Add a new icon to the image list. Returns the index to the image.
    pub fn add_icon(&self, icon: &Icon) -> i32 {
        use winapi::um::commctrl::ImageList_AddIcon;
        unsafe { ImageList_AddIcon(self.handle, icon.handle as HICON) }
    }

    /**
        Remove the image at the specified index

        When an image is removed, the indexes of the remaining images are adjusted so that the image indexes
        always range from zero to one less than the number of images in the image list. For example, if you
        remove the image at index 0, then image 1 becomes image 0, image 2 becomes image 1, and so on.
    */
    pub fn remove(&self, index: i32) {
        use winapi::um::commctrl::ImageList_Remove;
        unsafe { ImageList_Remove(self.handle, index); }
    }

    /// Replace an image in the image list
    pub fn replace(&self, index: i32, bitmap: &Bitmap) {
        use winapi::um::commctrl::ImageList_Replace;
        unsafe { ImageList_Replace(self.handle, index, bitmap.handle as HBITMAP, ptr::null_mut()); }
    }

    /// Replace an image in the image list by an icon
    pub fn replace_icon(&self, index: i32, icon: &Icon) {
        use winapi::um::commctrl::ImageList_ReplaceIcon;
        unsafe { ImageList_ReplaceIcon(self.handle, index, icon.handle as HICON); }
    }

}

impl Drop for ImageList {
    fn drop(&mut self) {
        use winapi::um::commctrl::ImageList_Destroy;
        unsafe {
            ImageList_Destroy(self.handle);
        }
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
        use winapi::um::commctrl::{ImageList_Create, ILC_COLOR32};

        unsafe {
            let (w, h) = self.size;
            let handle = ImageList_Create(w, h, ILC_COLOR32, self.initial, self.grow);
            if handle.is_null() {
                return Err(NwgError::resource_create("Failed to create image list"));
            }

            list.handle = handle;
        }
        
        Ok(())
    }

}
