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

use winapi::{HWND, HANDLE};

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle, HandleSpec};
use error::Error;
use events::{Event, Destroyed, Moved, Resized};
use events::image_frame::{Click, DoubleClick};

/**
    A template that creates a ImageFrame control.

    Rvents:  
    `Destroyed, Moved, Resized, Any`

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

impl<ID: Hash+Clone> ControlT<ID> for ImageFrameT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<ImageFrame>() }

    fn events(&self) -> Vec<Event> {
        vec![Destroyed, Moved, Resized, Click, DoubleClick, Event::Any]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{WindowParams, build_window, handle_of_window};
        use low::defs::{SS_NOTIFY, SS_BITMAP};
        use winapi::{DWORD, WS_VISIBLE, WS_DISABLED, WS_CHILD};
        use std::ptr;

        let flags: DWORD = WS_CHILD | SS_NOTIFY |  SS_BITMAP | 
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
                Ok(AnyHandle::HANDLE(h, spec)) => match spec {
                    HandleSpec::Bitmap => h
                },
                Ok(h) => { return Err(Error::BadResource(format!("Image frame image must be a bitmap, got {:?}", h))); },
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
                unsafe{ set_image(h, image); }
                Ok( Box::new(ImageFrame{handle: h}) )
            },
            Err(e) => Err(Error::System(e))
        }
    }
}


/**
    A frame that display a bitmap image loaded from a Image resource.
*/
pub struct ImageFrame {
    handle: HWND
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
unsafe fn set_image(hwnd: HWND, image: HANDLE) {
    use user32::SendMessageW;
    use winapi::{IMAGE_BITMAP, LPARAM, WPARAM};
    use low::defs::STM_SETIMAGE;

    SendMessageW(hwnd, STM_SETIMAGE, IMAGE_BITMAP as WPARAM, image as LPARAM);
}