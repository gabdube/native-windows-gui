use winapi::um::wincodec::{IWICImagingFactory, IWICBitmapDecoder, IWICBitmapFrameDecode};
use crate::win32::image_decoder::{create_image_factory, create_decoder_from_file};
use crate::NwgError;
use std::ptr;


/**
    A image decoder. Can load an extended number of image file format from a filename, from a file handle, or from a stream.

    ImageDecoder do not take any parameter to build, but it still provides a builder API to match the other component of NWG.
    You can also use "ImageDecoder::new" to avoid the builder API.

    There's not much reason to have more than 1 image decoder per application.

    ```rust
    use native_windows_gui as nwg;
    fn open_image(decoder: &nwg::ImageDecoder) -> Result<nwg::BitmapFrame, nwg::NwgError> {
        decoder
            .from_filename("poop.png")?
            .frame(0)
    }
    ```

    ```rust
    use native_windows_gui as nwg;
    fn build_image_decode(decode: &mut nwg::ImageDecoder) -> Result<(), nwg::NwgError> {
        nwg::ImageDecoder::builder()
            .build(decode)
    }
    ```
*/
pub struct ImageDecoder {
    factory: *mut IWICImagingFactory,
}

impl ImageDecoder {
    pub fn new() -> Result<ImageDecoder, NwgError> {
        let factory = unsafe { create_image_factory() }?;
        Ok(ImageDecoder { factory })
    }

    pub fn builder() -> ImageDecoderBuilder {
        ImageDecoderBuilder {
        }
    }

    /**
        Try to read an image from a file path.
        The file type can be any of the native WIC codecs (https://docs.microsoft.com/en-us/windows/win32/wic/native-wic-codecs)

        * If there is an error during the decoding, returns a NwgError.
        * If the image decoder was not initialized, this method panics

        This method returns a BitmapSource object.
    */
    pub fn from_filename<'a>(&self, path: &'a str) -> Result<BitmapSource, NwgError> {
        if self.factory.is_null() {
            panic!("ImageDecoder is not yet bound to a winapi object");
        }

        let decoder = unsafe { create_decoder_from_file(&*self.factory, path) }?;

        Ok(BitmapSource { decoder })
    }

}


/**
    Represents a bitmap data source in read only mode.
*/
pub struct BitmapSource {
    decoder: *mut IWICBitmapDecoder
}

impl BitmapSource {

    pub fn frame(&self, index: u32) -> Result<BitmapFrame, NwgError> {
        unimplemented!();
    }

}

pub struct BitmapFrame {
    frame: *mut IWICBitmapFrameDecode
}

impl Default for ImageDecoder {
    fn default() -> ImageDecoder {
        ImageDecoder {
            factory: ptr::null_mut()
        }
    }
}

impl Drop for ImageDecoder {
    fn drop(&mut self) {
        if !self.factory.is_null() {
            unsafe { (&*self.factory).Release(); }
        }
    }
}

impl Drop for BitmapSource {
    fn drop(&mut self) {
        unsafe { (&*self.decoder).Release(); }
    }
}

impl Drop for BitmapFrame {
    fn drop(&mut self) {
        unsafe { (&*self.frame).Release(); }
    }
}



pub struct ImageDecoderBuilder {
}

impl ImageDecoderBuilder {
    pub fn build(self, out: &mut ImageDecoder) -> Result<(), NwgError> {
        let factory = unsafe { create_image_factory() }?;
        *out = ImageDecoder { factory };
        Ok(())
    }
}
