/*!
    A frame that can display a Bitmap image
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use std::hash::Hash;
use std::any::TypeId;
use std::ptr;

use winapi::{HWND, HANDLE, WPARAM, IMAGE_BITMAP, IMAGE_ICON};
use user32::SendMessageW;

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle, HandleSpec};
use defs::ImageType;
use error::Error;

/**
    A template that creates a ImageFrame control.

    Control specific events:  
    `image_frame::Click, image_frame::DoubleClick`

    Members:  
    • `position`: The start position of the label  
    • `size`: The start size of the label  
    • `visible`: If the label should be visible to the user  
    • `disabled`: If the user can or can't click on the label  
    • `image`: The image resource to display in the frame  
    • `parent`: The label parent  
*/
#[derive(Clone)]
pub struct ImageFrameT<ID: Hash+Clone> {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub image: Option<ID>,
    pub parent: ID
}

impl<ID: Hash+Clone+'static> ControlT<ID> for ImageFrameT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<ImageFrame>() }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, handle_of_window};
        use low::defs::{SS_NOTIFY, SS_BITMAP, SS_CENTERIMAGE, SS_ICON};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD};

        let mut flags: DWORD = WS_CHILD | SS_NOTIFY | 
        if self.visible    { WS_VISIBLE }   else { 0 } |
        if self.disabled   { WS_DISABLED }  else { 0 };

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of an image frame must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the image handle
        let image = if let &Some(ref img) = &self.image {
            match ui.handle_of(img) {
                Ok(AnyHandle::HANDLE(h, HandleSpec::Bitmap)) =>  { flags |= SS_CENTERIMAGE | SS_BITMAP; h},
                Ok(AnyHandle::HICON(h)) => { flags |= SS_CENTERIMAGE | SS_ICON; h as HANDLE }
                Ok(h) => { return Err(Error::BadResource(format!("Image frame image must be a bitmap or icon, got {:?}", h))); },
                Err(e) => { return Err(e); }
            }
        } else {
            ptr::null_mut()
        };

        let params = WindowParams {
            title: "",
            class_name: "STATIC",
            position: self.position.clone(),
            size: self.size.clone(),
            flags: flags,
            ex_flags: Some(0),
            parent: parent
        };

        match unsafe{ build_window(params) } {
            Ok(h) => {
                let image_type = if flags & SS_ICON == SS_ICON {
                    ImageType::Icon
                } else {
                    ImageType::Bitmap
                };
                
                unsafe{ set_image(h, image, &image_type); }
                Ok( Box::new(ImageFrame{handle: h, image_type: image_type}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}


/**
    A frame that display a Image resource. Once created, the type of the image resource cannot be changed.  
    For example: an ImageFrame that can display icons will not be able to display bitmaps
*/
pub struct ImageFrame {
    handle: HWND,
    image_type: ImageType,
}

impl ImageFrame {
    pub fn get_visibility(&self) -> bool { unsafe{ ::low::window_helper::get_window_visibility(self.handle) } }
    pub fn set_visibility(&self, visible: bool) { unsafe{ ::low::window_helper::set_window_visibility(self.handle, visible); }}
    pub fn get_position(&self) -> (i32, i32) { unsafe{ ::low::window_helper::get_window_position(self.handle) } }
    pub fn set_position(&self, x: i32, y: i32) { unsafe{ ::low::window_helper::set_window_position(self.handle, x, y); }}
    pub fn get_size(&self) -> (u32, u32) { unsafe{ ::low::window_helper::get_window_size(self.handle) } }
    pub fn set_size(&self, w: u32, h: u32) { unsafe{ ::low::window_helper::set_window_size(self.handle, w, h, false); } }
    pub fn get_enabled(&self) -> bool { unsafe{ ::low::window_helper::get_window_enabled(self.handle) } }
    pub fn set_enabled(&self, e:bool) { unsafe{ ::low::window_helper::set_window_enabled(self.handle, e); } }

    /// Set the image of image frame. The new image resource must match the image type of the frame
    /// Pass `None` as an argument to remove the image.
    pub fn set_image<ID: Hash+Clone>(&self, ui: &Ui<ID>, img: Option<&ID>) -> Result<(), Error> {
        if !ui.has_handle(&self.handle()) {
            return Err(Error::BadUi("Image resource and control must be in the same Ui.".to_string()));
        }

        let img_handle = if let Some(id) = img {
            match ui.handle_of(id) {
                Ok(AnyHandle::HANDLE(h, HandleSpec::Bitmap)) => h,
                Ok(AnyHandle::HICON(h)) => h as HANDLE,
                Ok(h) => { return Err(Error::BadResource(format!("An Image resource is required, got {:?}", h))) },
                Err(e) => { return Err(e); }
            }
        } else {
            ptr::null_mut()
        };

        unsafe{
            set_image(self.handle, img_handle, &self.image_type);
        }

        Ok(())
    }

    /// Return the resource identifier of the control image    
    /// Return `None` if the control do not have an image. Will also return `None` if the UI passed as a parameter is unrelated to the control.
    pub fn get_image<ID: Hash+Clone>(&self, ui: &Ui<ID>) -> Option<ID> {
        use low::defs::STM_GETIMAGE;
        use winapi::HICON;

        let w_type = match &self.image_type {
            &ImageType::Bitmap => IMAGE_BITMAP,
            &ImageType::Icon => IMAGE_ICON,
            _ => unreachable!() // unimplemented
        };

        let image_handle = unsafe{ SendMessageW(self.handle, STM_GETIMAGE, w_type as WPARAM, 0) as HANDLE };
        if image_handle.is_null() {
            return None;
        }

        let handle = match &self.image_type {
            &ImageType::Bitmap => AnyHandle::HANDLE(image_handle, HandleSpec::Bitmap),
            &ImageType::Icon => AnyHandle::HICON(image_handle as HICON),
            _ => unreachable!() // unimplemented
        };

        match ui.id_from_handle(&handle) {
            Ok(id) => Some(id),
            Err(_) => None
        }
    }

    /// Get the type of the image resource that the frame can display
    pub fn get_image_type(&self) -> ImageType {
        self.image_type.clone()
    }
}

impl Control for ImageFrame {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::ImageFrame 
    }

    fn free(&mut self) {
        use user32::DestroyWindow;
        unsafe{ DestroyWindow(self.handle) };
    }

}

/**
    Set the image of an image frame
*/
unsafe fn set_image(hwnd: HWND, image: HANDLE, _type: &ImageType) {
    use winapi::LPARAM;
    use low::defs::STM_SETIMAGE;

    let w_type = match _type {
        &ImageType::Bitmap => IMAGE_BITMAP,
        &ImageType::Icon => IMAGE_ICON,
        _ => unreachable!() // unimplemented
    };

    SendMessageW(hwnd, STM_SETIMAGE, w_type as WPARAM, image as LPARAM);
}