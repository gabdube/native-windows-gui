use winapi::um::wincodec::{IWICImagingFactory, IWICBitmapDecoder};
use winapi::ctypes::c_void;
use winapi::shared::winerror::S_OK;
use winapi::Interface;
use crate::{NwgError};
use std::ptr;


unsafe fn create_image_factory() -> Result<*mut IWICImagingFactory, NwgError> {
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

unsafe fn create_decoder_from_file<'a>(fact: &IWICImagingFactory, path: &'a str) -> Result<*mut IWICBitmapDecoder, NwgError> {
    use winapi::um::wincodec::{WICDecodeMetadataCacheOnDemand};
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
        return Err(NwgError::resource_create("Failed to create a jpeg decoder"));
    }

    Ok(decoder)
}
