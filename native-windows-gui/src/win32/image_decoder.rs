use winapi::um::wincodec::{IWICImagingFactory, IWICBitmapDecoder};
use winapi::um::objidlbase::IStream;
use winapi::ctypes::{c_void, c_uint};
use winapi::shared::winerror::S_OK;
use winapi::Interface;
use crate::{NwgError, Bitmap, ImageData};
use std::ptr;


pub unsafe fn create_image_factory() -> Result<*mut IWICImagingFactory, NwgError> {
    use winapi::um::combaseapi::CoCreateInstance;
    use winapi::um::wincodec::CLSID_WICImagingFactory;
    use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;

    let mut image_factory: *mut IWICImagingFactory = ptr::null_mut();
    let result = CoCreateInstance(
        &CLSID_WICImagingFactory,
        ptr::null_mut(),
        CLSCTX_INPROC_SERVER,
        &IWICImagingFactory::uuidof(),
        (&mut image_factory as *mut *mut IWICImagingFactory) as *mut *mut c_void
    );

    if result != S_OK {
        return Err(NwgError::resource_create("Failed to create a image factory"));
    }

    Ok(image_factory)
}

pub unsafe fn create_decoder_from_file<'a>(fact: &IWICImagingFactory, path: &'a str) -> Result<*mut IWICBitmapDecoder, NwgError> {
    use winapi::um::wincodec::WICDecodeMetadataCacheOnDemand;
    use winapi::um::winnt::GENERIC_READ;
    use crate::win32::base_helper::to_utf16;

    let path = to_utf16(path);

    let mut decoder: *mut IWICBitmapDecoder = ptr::null_mut();
    let result = fact.CreateDecoderFromFilename(
        path.as_ptr(),
        ptr::null(),
        GENERIC_READ,
        WICDecodeMetadataCacheOnDemand,
        (&mut decoder as *mut *mut IWICBitmapDecoder) as *mut *mut IWICBitmapDecoder
    );

    if result != S_OK {
        return Err(NwgError::resource_create("Failed to create a bitmap decoder"));
    }

    Ok(decoder)
}

// This function is not declared by winapi yet. But winapi have import libraries
// for shlwapi (when the shellapi feature is enabled), so we can use it if we
// declare it ourselves.
extern "system" {
    fn SHCreateMemStream(p_init: *const u8, cb_init: c_uint) -> *mut IStream;
}

pub unsafe fn create_decoder_from_stream(fact: &IWICImagingFactory, data: &[u8]) -> Result<*mut IWICBitmapDecoder, NwgError> {
    use winapi::um::wincodec::WICDecodeMetadataCacheOnDemand;
    use std::convert::TryInto;

    let stream = SHCreateMemStream(data.as_ptr(), data.len().try_into().map_err(|_| {
        NwgError::resource_create("Failed to create memory stream, stream is too long")
    })?);
    if stream.is_null() {
        return Err(NwgError::resource_create("Failed to create memory stream, allocation failure"));
    }

    let mut decoder: *mut IWICBitmapDecoder = ptr::null_mut();
    let r = fact.CreateDecoderFromStream(
        stream,
        ptr::null(),
        WICDecodeMetadataCacheOnDemand,
        (&mut decoder as *mut *mut IWICBitmapDecoder) as *mut *mut IWICBitmapDecoder
    );

    (*stream).Release();

    if r != S_OK {
        return Err(NwgError::resource_create("Failed to create decoder from stream"));
    }

    Ok(decoder)
}

pub unsafe fn create_bitmap_from_wic(image: &ImageData) -> Result<Bitmap, NwgError> {
    use winapi::um::wincodec::{WICConvertBitmapSource, IWICBitmapSource, GUID_WICPixelFormat32bppPBGRA};
    use winapi::um::wingdi::{DeleteObject, CreateDIBSection, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, BI_RGB, RGBQUAD};
    use winapi::shared::{ntdef::LONG, minwindef::DWORD, windef::HBITMAP};
    use winapi::um::winuser::{GetDC, ReleaseDC};
    use std::mem;

    // First convert the image into a bitmap compatible format
    let frame_ptr = (&*image.frame) as &IWICBitmapSource as *const IWICBitmapSource;
    let mut converted = ptr::null_mut();
    let hr = WICConvertBitmapSource(&GUID_WICPixelFormat32bppPBGRA, frame_ptr, &mut converted);

    if hr != S_OK {
        return Err(NwgError::image_decoder(hr, "Could not convert image pixels"))
    }

    // Converted size
    let (mut width, mut height) = (0, 0);
    (&*converted).GetSize(&mut width, &mut height);

    // Prepare the bitmap
    let header = BITMAPINFOHEADER {
        biSize: mem::size_of::<BITMAPINFOHEADER>() as DWORD,
        biWidth: width as LONG, biHeight: -(height as LONG),
        biPlanes: 1, biBitCount: 32, biCompression: BI_RGB,
        biSizeImage: (width * height * 3) as u32,
        biXPelsPerMeter: 0, biYPelsPerMeter: 0,
        biClrUsed: 0, biClrImportant: 0
    };

    let quad = RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 };
    let bitmap_info = BITMAPINFO {
        bmiHeader: header,
        bmiColors: [quad],
    };

    // Create a DIB
    let mut bits = ptr::null_mut();
    let screen_dc = GetDC(ptr::null_mut());
    let bitmap = CreateDIBSection(screen_dc, &bitmap_info, DIB_RGB_COLORS, &mut bits, ptr::null_mut(), 0) as HBITMAP;
    ReleaseDC(ptr::null_mut(), screen_dc);

    if bitmap.is_null() {
        return Err(NwgError::image_decoder(hr, "Could not create a bitmap"));
    }

    // Write the DIB data
    let stride = width * 4;
    let bitmap_size = width * height * 4;
    let hr = (&*converted).CopyPixels(ptr::null(), stride, bitmap_size, bits as *mut u8);
    if hr != S_OK {
        DeleteObject(bitmap as _);
        return Err(NwgError::image_decoder(hr, "Could not write to bitmap"));
    }

    // Free up the converted image
    (&*converted).Release();

    Ok(
        Bitmap {
            handle: bitmap as _,
            owned: true
        }
    )
}

pub unsafe fn resize_bitmap(fact: &IWICImagingFactory, image: &ImageData, new_size: [u32;2]) -> Result<ImageData, NwgError> {
    use winapi::um::wincodec::{IWICBitmapScaler, IWICBitmapSource, WICBitmapInterpolationModeCubic};

    let mut scaler: *mut IWICBitmapScaler = ptr::null_mut();
    let result = fact.CreateBitmapScaler(&mut scaler);
    if result != S_OK {
        return Err(NwgError::image_decoder(result, "Could not create bitmap scaler"));
    }

    let [w, h] = new_size;
    let image_source = image.frame as *const IWICBitmapSource;
    let result = (&*scaler).Initialize(image_source, w, h, WICBitmapInterpolationModeCubic);
    if result != S_OK {
        return Err(NwgError::image_decoder(result, "Could not initialize bitmap scaler"));
    }

    Ok(ImageData { frame: scaler as *mut IWICBitmapSource })
}
