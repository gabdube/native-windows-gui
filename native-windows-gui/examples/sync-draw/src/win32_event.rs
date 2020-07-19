use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::CloseHandle;
use winapi::shared::winerror::WAIT_TIMEOUT;
use winapi::um::winbase::{WAIT_OBJECT_0, WAIT_FAILED};
use winapi::um::winnt::{SYNCHRONIZE, EVENT_MODIFY_STATE};
use winapi::um::synchapi::{CreateEventW, OpenEventW, SetEvent, WaitForSingleObject};
use std::ptr;


pub enum Win32EventWaitResult {
    Signaled,
    Timout,
    Failed
}

/// A wrapper over win32 events
pub struct Win32Event {
    handle: HANDLE
}

impl Win32Event {

    pub fn create(name: &str) -> Result<Win32Event, ()> {
        let name = to_utf16(name);
        let handle = unsafe { CreateEventW(ptr::null_mut(), 0, 0, name.as_ptr()) };
        match handle.is_null() {
            true => Err(()),
            false => Ok(Win32Event{ handle })
        }
    }

    pub fn open(name: &str) -> Result<Win32Event, ()> {
        let name = to_utf16(name);
        let handle = unsafe { OpenEventW(SYNCHRONIZE | EVENT_MODIFY_STATE, 0, name.as_ptr()) };
        match handle.is_null() {
            true => Err(()),
            false => Ok(Win32Event{ handle })
        }
    }

    pub fn wait(&self, timeout: u32) -> Win32EventWaitResult {
        match unsafe { WaitForSingleObject(self.handle, timeout) } {
            WAIT_OBJECT_0 => Win32EventWaitResult::Signaled,
            WAIT_TIMEOUT => Win32EventWaitResult::Timout,
            WAIT_FAILED => Win32EventWaitResult::Failed,
            _ => unreachable!()
        }
    }

    pub fn set(&self) {
        unsafe { SetEvent(self.handle); }
    }

    pub fn close(&self) {
        unsafe { CloseHandle(self.handle); }
    }

}

unsafe impl Send for Win32Event {}
unsafe impl Sync for Win32Event {}


impl Default for Win32Event {

    fn default() -> Win32Event {
        Win32Event {
            handle: ptr::null_mut()
        }
    }

}

fn to_utf16(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s)
      .encode_wide()
      .chain(Some(0u16).into_iter())
      .collect()
}
