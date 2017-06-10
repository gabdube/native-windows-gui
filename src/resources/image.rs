/*!
    Image resources creation
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
use std::any::TypeId;
use std::hash::Hash;
use std::ptr;

use winapi::{HANDLE, c_int};

use ui::Ui;
use controls::{AnyHandle, HandleSpec};
use resources::{ResourceT, Resource};
use error::{Error, SystemError};
use defs::ImageType;
use low::other_helper::to_utf16;

/**
    A template that creates a image resource

    Params:  
    • `source`: The path to the image resource  
    • `strict`: If set to `true`, the image creation will fail if the source cannot be read.  
                If not, the resource creation will not fails and the Windows `Error` default icon will be loaded instead.
    • `_type`: The type of the resource to load  
    • `size`: The size of the image to load. If left to (0, 0), use the original resource size.  
*/
#[derive(Clone)]
pub struct ImageT<S: Clone+Into<String>> {
    pub source: S,
    pub strict: bool,
    pub image_type: ImageType,
    pub size: (c_int, c_int)
}

impl<ID: Clone+Hash, S: Clone+Into<String>> ResourceT<ID> for ImageT<S> {
    fn type_id(&self) -> TypeId { TypeId::of::<Image>() }

    #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Resource>, Error> {
        use winapi::{LR_LOADFROMFILE, LR_DEFAULTSIZE, LR_SHARED, IMAGE_BITMAP, IMAGE_CURSOR, IMAGE_ICON};
        use user32::LoadImageW;
        use low::defs::OIC_HAND;
        use low::other_helper::get_system_error;

        let filepath = to_utf16(self.source.clone().into().as_ref());
        let (width, height) = self.size;
        let mut res_type = self.image_type.clone();
        let c_res_type = match self.image_type {
            ImageType::Bitmap => IMAGE_BITMAP,
            ImageType::Cursor => IMAGE_CURSOR,
            ImageType::Icon => IMAGE_ICON
        };

        let mut handle = unsafe{ LoadImageW(ptr::null_mut(), filepath.as_ptr(), c_res_type, width, height, LR_LOADFROMFILE) };
        if handle.is_null() {
            let (code, _) = unsafe{ get_system_error() } ;
            if code == 2 && !self.strict {
                // If the file was not found (err code: 2) and the loading is not strict, replace the image by the system error icon
                let hand_resource = (OIC_HAND as usize) as *const u16;
                handle = unsafe{ LoadImageW(ptr::null_mut(), hand_resource, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE|LR_SHARED) };
                res_type = ImageType::Icon;
            }
        }

        if handle.is_null() {
            Err(Error::System(SystemError::ImageCreation))
        } else {
            Ok( Box::new( Image{ handle: handle, image_type: res_type } ) )
        }

    }
}


/**
    An image resource. May represent a bitmap, an icon or a cursor
*/
pub struct Image {
    handle: HANDLE,
    image_type: ImageType
}

impl Image {
    pub fn resource_type(&self) -> ImageType {
        self.image_type.clone()
    }
}

impl Resource for Image {
    fn handle(&self) -> AnyHandle { 
        use winapi::{HICON, HCURSOR};
        
        match self.image_type {
            ImageType::Bitmap => AnyHandle::HANDLE(self.handle, HandleSpec::Bitmap),
            ImageType::Cursor => AnyHandle::HCURSOR(self.handle as HCURSOR),
            ImageType::Icon => AnyHandle::HICON(self.handle as HICON)
        }
    }

    fn free(&mut self) {
        use gdi32::DeleteObject;
        use user32::{DestroyCursor, DestroyIcon};
        use std::mem;

        unsafe{
            match self.image_type {
                ImageType::Bitmap => DeleteObject(mem::transmute(self.handle)),
                ImageType::Cursor => DestroyCursor(mem::transmute(self.handle)),
                ImageType::Icon => DestroyIcon(mem::transmute(self.handle))
            };
        }
    }
}