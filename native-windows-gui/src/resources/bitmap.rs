use winapi::um::winnt::HANDLE;
use winapi::um::winuser::IMAGE_BITMAP;
use crate::win32::resources_helper as rh;
use crate::{OemBitmap, OemImage, NwgError};
use std::ptr;


/// A wrapper over a bitmap file (*.bmp)
#[allow(unused)]
pub struct Bitmap {
    pub handle: HANDLE,
    pub(crate) owned: bool
}

impl Bitmap {

    pub fn builder<'a>() -> BitmapBuilder<'a> {
        BitmapBuilder {
            source_text: None,
            source_bin: None,
            source_system: None,
            transparency_key: None,
            size: None,
            strict: false
        }
    }

}

pub struct BitmapBuilder<'a> {
    source_text: Option<&'a str>,
    source_bin: Option<&'a [u8]>,
    source_system: Option<OemBitmap>,
    transparency_key: Option<[u8; 3]>,
    size: Option<(u32, u32)>,
    strict: bool,
}

impl<'a> BitmapBuilder<'a> {

    pub fn source_file(mut self, t: Option<&'a str>) -> BitmapBuilder<'a> {
        self.source_text = t;
        self
    }

    pub fn source_bin(mut self, t: Option<&'a [u8]>) -> BitmapBuilder<'a> {
        self.source_bin = t;
        self
    }

    pub fn source_system(mut self, t: Option<OemBitmap>) -> BitmapBuilder<'a> {
        self.source_system = t;
        self
    }

    pub fn size(mut self, s: Option<(u32, u32)>) -> BitmapBuilder<'a> {
        self.size = s;
        self
    }

    pub fn strict(mut self, s: bool) -> BitmapBuilder<'a> {
        self.strict = s;
        self
    }

    pub fn transparency_key(mut self, k: Option<[u8; 3]>) -> BitmapBuilder<'a> {
        self.transparency_key = k;
        self
    }

    pub fn build(self, b: &mut Bitmap) -> Result<(), NwgError> {
        let mut handle;
        
        if let Some(src) = self.source_text {
            handle = unsafe { rh::build_image(src, self.size, self.strict, IMAGE_BITMAP) };
        } else if let Some(src) = self.source_system {
            handle = unsafe { rh::build_oem_image(OemImage::Bitmap(src), self.size) };
        } else if let Some(src) = self.source_bin { 
            handle = unsafe { bitmap_from_memory(src) };
        } else {
            return Err(NwgError::resource_create("No source provided for Bitmap"));
        }

        if let Some(key) = self.transparency_key {
            let size = match self.size {
                Some((x, y)) => (x as i32, y as i32),
                None => (0, 0)
            };

            handle = unsafe { rh::make_bitmap_transparent(handle?, size, key) };
        }
        
        *b = Bitmap { handle: handle?, owned: true };
    
        Ok(())
    }

}


impl Default for Bitmap {

    fn default() -> Bitmap {
        Bitmap {
            handle: ptr::null_mut(),
            owned: false
        }
    }

}

impl PartialEq for Bitmap {

    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }

}

impl Drop for Bitmap {

    fn drop(&mut self) {
        use winapi::um::wingdi::DeleteObject;
        if self.owned {
            unsafe { DeleteObject(self.handle); }
        }
    }

}


/** 
    Create a bitmap from memory.
    The memory must contain the whole file (including the bitmap header).
*/
unsafe fn bitmap_from_memory(source: &[u8]) -> Result<HANDLE, NwgError> {
    use winapi::um::wingdi::{CreateCompatibleBitmap, CreateCompatibleDC, SetDIBits, BITMAPFILEHEADER, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, BI_RGB, RGBQUAD};
    use winapi::shared::{ntdef::LONG, minwindef::DWORD};
    use winapi::um::winuser::{GetDC, ReleaseDC};
    use winapi::ctypes::c_void;
    use std::mem;

    // Check the header size requirement
    let fheader_size = mem::size_of::<BITMAPFILEHEADER>();
    let iheader_size = mem::size_of::<BITMAPINFOHEADER>();
    let header_size = fheader_size + iheader_size;
    if source.len() < header_size {
        let msg = format!("Invalid source. The source size ({} bytes) is smaller than the required headers size ({} bytes).", source.len(), header_size);
        return Err(NwgError::ResourceCreationError(msg));
    }

    // Read the bitmap file header
    let src: *const u8 = source.as_ptr();
    let fheader_ptr = src as *const BITMAPFILEHEADER;
    let fheader: BITMAPFILEHEADER = ptr::read( fheader_ptr );

    // Read the bitmap info header
    let iheader_ptr = src.offset(fheader_size as isize) as *const BITMAPINFOHEADER;
    let iheader: BITMAPINFOHEADER = ptr::read( iheader_ptr );

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

    let quad = RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 };
    let info = BITMAPINFO {
        bmiHeader: header,
        bmiColors: [quad],
    };

    let data_ptr = source.as_ptr().offset(fheader.bfOffBits as isize) as *const c_void;
    if 0 == SetDIBits(hdc, bitmap, 0, h as u32, data_ptr, &info, DIB_RGB_COLORS) {
        let msg = "SetDIBits failed.".to_string();
        return Err(NwgError::ResourceCreationError(msg));
    }

    return Ok(bitmap as HANDLE);
}
