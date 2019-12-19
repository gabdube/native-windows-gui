/*!
    A wrapper over a jpeg file (*.jpg)
*/
use winapi::Interface;
use crate::NwgError;
use std::ptr;



/// A wrapper over a jpeg file (*.jpg)
#[allow(unused)]
#[derive(Default)]
pub struct Jpeg {
}

impl Jpeg {

    pub fn builder<'a>() -> JpegBuilder<'a> {
        JpegBuilder {
            source_text: None
        }
    }

}


pub struct JpegBuilder<'a> {
    source_text: Option<&'a str>
}

impl<'a> JpegBuilder<'a> {

    pub fn source_file(mut self, t: Option<&'a str>) -> JpegBuilder<'a> {
        self.source_text = t;
        self
    }

    pub fn build(self, img: &Jpeg) -> Result<(), NwgError> {
        unsafe {
            let factory = create_image_factory()?;
            let decoder = create_decoder(&*factory)?;

            (&*decoder).Release();
            (&*factory).Release();
        }

        Ok(())
    }

}


use winapi::um::wincodec::{IWICImagingFactory, IWICBitmapDecoder};
use winapi::ctypes::c_void;
use winapi::shared::winerror::S_OK;


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

unsafe fn create_decoder(fact: &IWICImagingFactory) -> Result<*mut IWICBitmapDecoder, NwgError> {
    use winapi::um::wincodec::{GUID_ContainerFormatJpeg};

    let mut decoder: *mut IWICBitmapDecoder = ptr::null_mut();
    let result = fact.CreateDecoder(
        &GUID_ContainerFormatJpeg,
        ptr::null(),
        (&mut decoder as *mut *mut IWICBitmapDecoder) as *mut *mut IWICBitmapDecoder
    );

    if result != S_OK {
        return Err(NwgError::resource_create("Failed to create a jpeg decoder"));
    }

    Ok(decoder)
}
