/*!
    Wrapper over a `IDWriteFactory`. Used to create all subsequent DirectWrite objects. This interface is the root factory interface for all DirectWrite objects.

    Winapi documentation: https://docs.microsoft.com/en-us/windows/win32/api/dwrite/nn-dwrite-idwritefactory
*/
use winapi::um::dwrite::IDWriteFactory;
use winapi::um::unknwnbase::IUnknown;
use winapi::shared::winerror::S_OK;
use winapi::Interface;
use super::WriteError;
use std::{ptr, fmt};


pub struct WriteFactory {
    handle: *mut IDWriteFactory
}

impl WriteFactory {

    /// Creates a DirectWrite factory object that is used for subsequent creation of individual DirectWrite objects.
    pub fn new() -> Result<WriteFactory, WriteError> {
        use winapi::um::dwrite::DWriteCreateFactory;
        use winapi::um::dwrite::DWRITE_FACTORY_TYPE_SHARED;

        let mut handle: *mut IDWriteFactory = ptr::null_mut();
        let result = unsafe { 
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, &IDWriteFactory::uuidof(), (&mut handle as *mut *mut IDWriteFactory) as *mut *mut IUnknown)
        };

        match result {
            S_OK => Ok(WriteFactory { handle }),
            e => Err(WriteError::Unknown(e))
        }
    }

    /// Check if the write factory is initialized
    pub fn is_null(&self) -> bool { self.handle.is_null() }

}

impl Default for WriteFactory {

    fn default() -> WriteFactory {
        WriteFactory {  handle: ptr::null_mut() }
    }

}

impl fmt::Debug for WriteFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WriteFactory")
    }
}

impl Clone for WriteFactory {

    fn clone(&self) -> WriteFactory {
        match self.is_null() {
            true => WriteFactory{ handle: ptr::null_mut() },
            false => unsafe {
                (&*self.handle).AddRef();
                WriteFactory{ handle: self.handle }
            }
        }
    }

}

impl Drop for WriteFactory {

    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { (&*self.handle).Release(); }
            self.handle = ptr::null_mut();
        }
    }

}
