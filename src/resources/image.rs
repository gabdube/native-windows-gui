/*!
    Image resources creation
*/
use std::any::TypeId;
use std::hash::Hash;
use std::ptr;
use std::mem;

use winapi::{HANDLE, HBITMAP, c_int};

use ui::Ui;
use controls::{AnyHandle, HandleSpec};
use resources::{ResourceT, Resource};
use error::{Error, SystemError};
use defs::{ImageType, OemImage};
use low::other_helper::to_utf16;

/**
    A template that creates a image resource from a global system resource.

    Params:  
    • `source`: A OemImage image identifier.  
    • `size`: The size of the image to load. If left to (0, 0), use the original resource size.  
*/
pub struct OemImageT {
    pub source: OemImage,
    pub size: (c_int, c_int)
}

impl<ID: Clone+Hash> ResourceT<ID> for OemImageT {
    fn type_id(&self) -> TypeId { TypeId::of::<Image>() }

    #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Resource>, Error> {
        use winapi::{LPCWSTR, LR_DEFAULTSIZE, LR_SHARED, IMAGE_BITMAP, IMAGE_CURSOR, IMAGE_ICON};
        use user32::LoadImageW;

        let (width, height) = self.size;
        let (res_type, c_res_type, res_identifier) = match &self.source {
            &OemImage::Bitmap(b) => {
                (ImageType::Bitmap, IMAGE_BITMAP, (b as usize) as LPCWSTR)
            },
            &OemImage::Cursor(c) => {
                (ImageType::Cursor, IMAGE_CURSOR, (c as usize) as LPCWSTR)
            },
            &OemImage::Icon(i) => {
                (ImageType::Icon, IMAGE_ICON, (i as usize) as LPCWSTR)
            }
        };

        let flags = if self.size == (0, 0) {
            LR_DEFAULTSIZE|LR_SHARED
        } else {
            LR_SHARED
        };

        let handle = unsafe{ LoadImageW(ptr::null_mut(), res_identifier, c_res_type, width, height, flags) };

        if handle.is_null() {
            Err(Error::System(SystemError::ImageCreation))
        } else {
            Ok( Box::new( Image{ handle: handle, image_type: res_type } ) )
        }

    }
}

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
    A template a create a image from a bitmap file that was loaded in memory.
    Both the bitmap file header and the bitmap information must be present in
    the source array
*/
#[derive(Clone)]
pub struct MemoryImageT {
    pub source: Vec<u8>,
}


impl<ID: Clone+Hash> ResourceT<ID> for MemoryImageT {
    fn type_id(&self) -> TypeId { TypeId::of::<Image>() }

    #[allow(unused_variables)]
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Resource>, Error> {
        match unsafe{ create_dib(self) } {
            Ok(h) => Ok( Box::new( Image{ handle: h as HANDLE, image_type: ImageType::Bitmap } ) ),
            Err(e) => Err(e)
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


// Private functions

unsafe fn create_dib(i: &MemoryImageT) -> Result<HBITMAP, Error> {
    use winapi::{BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, BI_RGB, LONG, DWORD};
    use gdi32::{CreateCompatibleBitmap, CreateCompatibleDC, SetDIBits};
    use user32::{GetDC, ReleaseDC};
    use low::defs::BITMAPFILEHEADER;
    use std::os::raw::c_void;

    // Check the header size requirement
    let fheader_size = mem::size_of::<BITMAPFILEHEADER>();
    let iheader_size = mem::size_of::<BITMAPINFOHEADER>();
    let header_size = fheader_size + iheader_size;
    if i.source.len() < header_size {
        let msg = format!("Invalid source. The source size ({} bytes) is smaller than the required headers size ({} bytes).", i.source.len(), header_size);
        return Err(Error::BadResource(msg));
    }

    // Read the bitmap file header
    let src: *const u8 = i.source.as_ptr();
    let fheader_ptr: *const BITMAPFILEHEADER = mem::transmute(src);
    let fheader: BITMAPFILEHEADER = ptr::read( fheader_ptr );

    // Read the bitmap info header
    let iheader_ptr: *const BITMAPINFOHEADER = mem::transmute(src.offset(fheader_size as isize));
    let iheader: BITMAPINFOHEADER = ptr::read( iheader_ptr );

    println!("{:?}", fheader);
    println!("In BI header: {:?}", iheader);

    let (w, h) = (iheader.biWidth, iheader.biHeight);

    let screen_dc = GetDC(ptr::null_mut());
    let hdc = CreateCompatibleDC(screen_dc);
    let bitmap = CreateCompatibleBitmap(screen_dc, w, h);
    ReleaseDC(ptr::null_mut(), screen_dc);

    let header = BITMAPINFOHEADER {
        biSize: mem::size_of::<BITMAPINFOHEADER>() as DWORD,
        biWidth: w as LONG, biHeight: h as LONG, 
        biPlanes: 1, biBitCount: 24, biCompression: BI_RGB,
        biSizeImage: (w * h * 3) as u32,
        biXPelsPerMeter: 0, biYPelsPerMeter: 0,
        biClrUsed: 0, biClrImportant: 0
    };
    println!("Created BI header: {:?}", header);

    let info = BITMAPINFO {
        bmiHeader: header,
        bmiColors: [],
    };

    let data_ptr = i.source.as_ptr().offset(fheader.bfOffBits as isize) as *const c_void;
    if 0 == SetDIBits(hdc, bitmap, 0, h as u32, data_ptr, &info, DIB_RGB_COLORS) {
        let msg = "SetDIBits failed.".to_string();
        return Err(Error::BadResource(msg));
    }

    return Ok(bitmap);
}
