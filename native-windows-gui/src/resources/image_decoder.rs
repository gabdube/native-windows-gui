use winapi::um::wincodec::{IWICImagingFactory, IWICBitmapDecoder, IWICBitmapFrameDecode, WICPixelFormatGUID};
use winapi::shared::winerror::S_OK;
use crate::win32::image_decoder::{create_image_factory, create_decoder_from_file};
use crate::NwgError;
use std::{ptr, mem};


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

    /**
        Return the number of frame in the image. For most format (ex: PNG), this will be 1.
        It might be more than 1 in animated image formats (such as GIFs).
    */
    pub fn frame_count(&self) -> u32 {
        let mut frame_count = 0;
        unsafe { (&*self.decoder).GetFrameCount(&mut frame_count); }
        frame_count
    }

    /**
        Return the image data of the requested frame in a BitmapFrame object.
    */
    pub fn frame(&self, index: u32) -> Result<BitmapFrame, NwgError> {
        let mut bitmap = BitmapFrame { frame: ptr::null_mut() };
        let hr = unsafe { (&*self.decoder).GetFrame(index, &mut bitmap.frame) };
        match hr {
            S_OK => Ok(bitmap),
            err => Err(NwgError::image_decoder(err, "Could not read image frame"))
        }
    }

    /*  Retrieves the container format of the bitmap source. 

        See https://docs.microsoft.com/en-us/windows/win32/wic/-wic-guids-clsids#container-formats
    */
    pub fn container_format(&self) -> ContainerFormat {
        use ContainerFormat::*;
        let mut container = unsafe { mem::zeroed() };
        unsafe { (&*self.decoder).GetContainerFormat(&mut container) };

        // Just use the first field of the GUID because the winapi type does no implement EQ
        match container.Data1 {
            0xf3ff6d0d => Adng,
            0xaf1d87e => Bmp,
            0x1b7cfaf4 => Png,
            0xa3a860c4 => Ico,
            0x19e4a5aa => Jpeg,
            0x163bcc30 => Tiff,
            0x1f8a5601 => Gif,
            0x57a37caa => Wmp,
            _ => Unknown
        }

    }

}


/**
    Represents a source of pixel that can be read, but cannot be written back to.
*/
pub struct BitmapFrame {
    frame: *mut IWICBitmapFrameDecode
}

impl BitmapFrame {

    /// Retrieves the sampling rate between pixels and physical world measurements.
    pub fn resolution(&self) -> (f64, f64) {
        let (mut rx, mut ry) = (0.0, 0.0);
        unsafe { (&*self.frame).GetResolution(&mut rx, &mut ry) };
        (rx, ry)
    }  
    
    /// Retrieves the pixel width and height of the bitmap.
    pub fn size(&self) -> (u32, u32) {
        let (mut sx, mut sy) = (0, 0);
        unsafe { (&*self.frame).GetSize(&mut sx, &mut sy) };
        (sx, sy)
    }

    /*  Retrieves the pixel format of the bitmap source. Returns a GUID, Match it to the GUID defined in the link below:

        See https://docs.microsoft.com/en-us/windows/win32/wic/-wic-codec-native-pixel-formats#undefined-pixel-formats
    */
    pub fn pixel_format(&self) -> WICPixelFormatGUID {
        let mut fmt = unsafe { mem::zeroed() };
        unsafe { (&*self.frame).GetPixelFormat(&mut fmt) };
        fmt
    }

}

/// A list of container format implemented in WIC
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContainerFormat {
    Unknown,
    Adng,
    Bmp,
    Png,
    Ico,
    Jpeg,
    Tiff,
    Gif,
    Wmp,
}

//
// IMPL
//

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


/**
    A blank builder for the image decoder
*/
pub struct ImageDecoderBuilder {
}

impl ImageDecoderBuilder {
    pub fn build(self, out: &mut ImageDecoder) -> Result<(), NwgError> {
        let factory = unsafe { create_image_factory() }?;
        *out = ImageDecoder { factory };
        Ok(())
    }
}
