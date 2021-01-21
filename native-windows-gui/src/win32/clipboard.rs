use crate::controls::ControlHandle;
use super::base_helper::{to_utf16};
use winapi::um::winuser::{CF_BITMAP, CF_TEXT, CF_UNICODETEXT};
use winapi::um::winnt::HANDLE;


#[derive(Copy, Clone)]
pub enum ClipboardFormat {
    /// ANSI text. You probably want to use `UnicodeText`.
    Text,

    /// UnicodeText. Equivalent to a OsString
    UnicodeText,

    /// A bitmap file
    Bitmap,

    /// Global clipboard format to share data between applications
    /// The format name comparison is case-insensitive.
    Global(&'static str)
}


impl ClipboardFormat {

    fn into_raw(&self) -> u32 {
        use ClipboardFormat::*;
        use winapi::um::winuser::RegisterClipboardFormatW;

        match self {
            Text => CF_TEXT,
            UnicodeText => CF_UNICODETEXT,
            Bitmap => CF_BITMAP,
            Global(v) => unsafe {
                let v = to_utf16(v);
                RegisterClipboardFormatW(v.as_ptr())
            }
        }
    }

}

/// Wrapper over a clipboard global allocation handle.
/// This value should be released with `release` or dropped before closing the clipboard.
pub struct ClipboardData(HANDLE);

impl ClipboardData {
    pub unsafe fn cast<D: Copy>(&self) -> *const D { self.0 as *const D }
    pub fn release(self) { /* See drop implementation */ }
}

impl Drop for ClipboardData {
    fn drop(&mut self) {
        unsafe { ::winapi::um::winbase::GlobalUnlock(self.0); }
    }
}


/**
A global object that wraps the system clipboard. It can be used to set or get the system cliboard content.

It's important to keep in mind that there is no way to validate data sent through the clipboard API, as such this wrapper
is still mostly unsafe and you must validate the data when reading.

Note that NWG clipboard is intentionally keeps things simple and close to the metal. If you want to more robust API, then I recommend you look into https://github.com/DoumanAsh/clipboard-win

Requires the feature "clipboard"

Writing / Reading text

```rust
use native_windows_gui as nwg;

fn clipboard_text(window: &nwg::Window) {
    nwg::Clipboard::set_data_text(window, "Hello!");

    let text = nwg::Clipboard::data_text(window);
    assert!(text.is_some());
    assert!(&text.unwrap() == &"Hello!");
}
```


Writing / Reading custom data

```rust
use native_windows_gui as nwg;

#[repr(C)]
#[derive(Clone, Copy)]
struct Hello {
    foo: usize,
    bar: [u16; 3]
}

fn write_custom_data(window: &nwg::Window) {
    let data = Hello {
        foo: 6529,
        bar: [0, 100, 20]
    };

    nwg::Clipboard::open(window);
    nwg::Clipboard::empty();
    unsafe {
        nwg::Clipboard::set_data(
            nwg::ClipboardFormat::Global("Hello"),
            &data as *const Hello,
            1
        );
    }

    nwg::Clipboard::close();
}

fn read_custom_data(window: &nwg::Window) -> Option<Hello> {
    unsafe {
        nwg::Clipboard::open(window);
        let data = nwg::Clipboard::data(nwg::ClipboardFormat::Global("Hello"));
        nwg::Clipboard::close();
        data
    }
}

fn read_custom_data_handle(window: &nwg::Window) -> Option<Hello> {
    unsafe {
        nwg::Clipboard::open(window);
        let handle = nwg::Clipboard::data_handle(nwg::ClipboardFormat::Global("Hello"));
        let data = match handle {
            Some(h) => {
                let data_ptr: *const Hello = h.cast();
                let data = *data_ptr;
                h.release();
                Some(data)
            },
            None => None
        };

        nwg::Clipboard::close();
        data
    }
}

```
*/
pub struct Clipboard;

impl Clipboard {

    /**
        Fill the clipboard with the selected text.
        The data use the `ClipboardFormat::UnicodeText` format.

        This is a high level function that handles `open` and `close`
    */
    pub fn set_data_text<'a, C: Into<ControlHandle>>(handle: C, text: &'a str) {
        use winapi::um::winuser::SetClipboardData;
        use winapi::um::stringapiset::MultiByteToWideChar;
        use winapi::um::winnls::CP_UTF8;
        use winapi::shared::basetsd::SIZE_T;
        use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalFree, GlobalUnlock, GMEM_MOVEABLE};
        use core::{mem, ptr};

        let size = unsafe {
            MultiByteToWideChar(CP_UTF8, 0, text.as_ptr() as *const _, text.len() as _, ptr::null_mut(), 0)
        };

        if size == 0 {
            return;
        }

        let alloc_size = (mem::size_of::<u16>() * (size as usize + 1)) as SIZE_T;
        let alloc = unsafe { GlobalAlloc(GMEM_MOVEABLE, alloc_size) };

        unsafe {
            let locked_ptr = GlobalLock(alloc) as *mut u16;
            assert!(!locked_ptr.is_null());
            MultiByteToWideChar(CP_UTF8, 0, text.as_ptr() as *const _, text.len() as _, locked_ptr, size);
            ptr::write(locked_ptr.offset(size as isize), 0);
            GlobalUnlock(alloc);
        }

        Clipboard::open(handle);
        Clipboard::empty();

        unsafe {
            if SetClipboardData(CF_UNICODETEXT, alloc as _).is_null() {
                GlobalFree(alloc);
            }
        }

        Clipboard::close();
    }

    /**
        Return the current text value in the clipboard (if there is one).
        This function will return the text if the clipboard has either the `UnicodeText` format or the `Text` format.

        If the clipboard do not have a text format OR the text data is not a valid utf-8 sequence, this function will return `None`.
    */
    pub fn data_text<C: Into<ControlHandle>>(handle: C) -> Option<String> {
        use ClipboardFormat::*;
        let mut data = None;

        Clipboard::open(handle);

        unsafe {
            if Clipboard::has_format(UnicodeText) {
                let handle = Clipboard::data_handle(UnicodeText).unwrap();
                data = from_wide_ptr(handle.cast());
                handle.release();
            } else if Clipboard::has_format(Text) {
                let handle = Clipboard::data_handle(Text).unwrap();
                data = from_ptr(handle.cast());
                handle.release();
            }
        }

        Clipboard::close();

        data
    }

    /**
        Remove the current data in the clipboard
    */
    pub fn clear<C: Into<ControlHandle>>() {
        use winapi::um::winuser::{OpenClipboard, EmptyClipboard, CloseClipboard};
        use std::ptr;
        unsafe {
            OpenClipboard(ptr::null_mut());
            EmptyClipboard();
            CloseClipboard();
        }
    }

    /**
        Opens the clipboard for examination and prevents other applications from modifying the clipboard content.
        Another call to `close` should be made as soon as the application is done with the clipboard.

        Parameters:
            handle: A window control that will be identified as the current "owner" of the clipboard

        This function will panic if the control is not HWND based.
    */
    pub fn open<C: Into<ControlHandle>>(handle: C) {
        use winapi::um::winuser::OpenClipboard;
        let handle = handle.into().hwnd().expect("Control should be a window");
        unsafe { OpenClipboard(handle); }
    }

    /**
        Places data on the clipboard in a specified clipboard format.

        This method is unsafe because there is no way to ensure that data is valid.
        If possible, it is recommended to use a higher level function such as `set_data_text` instead.

        If the data will be used across applications, make sure that D has `repr(C)` for compatibility.

        The clipboard must be open when calling this function.

        * Note 1: `data` is copied into a global system allocation. It's ok to discard the data as soon as this function returns.
        * Note 2: When copying text, the null byte must be included.
    */
    pub unsafe fn set_data<D: Copy>(fmt: ClipboardFormat, data: *const D, count: usize) {
        use winapi::um::winuser::SetClipboardData;
        use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalFree, GlobalUnlock, GMEM_MOVEABLE};
        use winapi::shared::basetsd::SIZE_T;
        use std::{mem, ptr};

        let fmt = fmt.into_raw();
        let alloc_size = (mem::size_of::<D>() * count) as SIZE_T;
        let alloc = GlobalAlloc(GMEM_MOVEABLE, alloc_size);

        ptr::copy_nonoverlapping(data, GlobalLock(alloc) as *mut D, count);
        GlobalUnlock(alloc);

        if SetClipboardData(fmt, alloc as HANDLE).is_null() {
            GlobalFree(alloc);
        }
    }

    /**
        Check if the selected format is available in the clipboard.
    */
    pub fn has_format(fmt: ClipboardFormat) -> bool {
        use winapi::um::winuser::IsClipboardFormatAvailable;

        let selected_format = fmt.into_raw();
        unsafe { IsClipboardFormatAvailable(selected_format) != 0 }
    }

    /**
        Get the handle to the clipboard data and copy it's data into a new value of type `D`.
        This function is very unsafe because it assumes that the handle points to the correct type.

        The clipboard must be open when calling this function.

        If no data is found with the selected clipboard format, `None` is returned.
    */
    pub unsafe fn data<D: Copy>(fmt: ClipboardFormat) -> Option<D> {
        use winapi::um::winuser::GetClipboardData;
        use winapi::um::winbase::{GlobalLock, GlobalUnlock};
        use std::{ptr, mem};

        let fmt = fmt.into_raw();
        let handle = GetClipboardData(fmt);
        if handle.is_null() {
            return None;
        }

        let mut data = mem::zeroed();
        ptr::copy_nonoverlapping(GlobalLock(handle) as *const D, &mut data, 1);
        GlobalUnlock(handle);

        Some(data)
    }

    /**
        Gets the data handle of the clipboard, lock the memory and return it in a `ClipboardData` wrapper.
        The returned data is read-only. The application should copy the data and release the handle as soon as possible.

        The clipboard must be open when calling this function.

        If no data is found with the selected clipboard format, `None` is returned.
    */
    pub unsafe fn data_handle(fmt: ClipboardFormat) -> Option<ClipboardData> {
        use winapi::um::winuser::GetClipboardData;
        use winapi::um::winbase::GlobalLock;

        let fmt = fmt.into_raw();
        let handle = GetClipboardData(fmt);
        match handle.is_null() {
            true => None,
            false => Some(ClipboardData(GlobalLock(handle)))
        }
    }

    /**
        A window can place more than one clipboard object on the clipboard, each representing the same information in a different clipboard format.
        Retrieves the number of different data formats currently on the clipboard.
    */
    pub fn count_clipboard_formats() -> u32 {
        use winapi::um::winuser::CountClipboardFormats;
        unsafe { CountClipboardFormats() as u32 }
    }

    /**
        Empty the clipboard data.
        This is a low-level function and `open` must have been called first.
        To only clear the clipboard data use `clear`
    */
    pub fn empty() {
        use winapi::um::winuser::EmptyClipboard;
        unsafe { EmptyClipboard(); }
    }

    /**
        Close the clipboard after it was opened with the `open` function.
    */
    pub fn close() {
        use winapi::um::winuser::CloseClipboard;
        unsafe { CloseClipboard(); }
    }

    /**
        Return the handle of the window that owns the clipboard
    */
    pub fn ownder() -> ControlHandle {
        let handle = unsafe { ::winapi::um::winuser::GetClipboardOwner() };
        ControlHandle::Hwnd(handle)
    }

}


unsafe fn from_wide_ptr(ptr: *const u16) -> Option<String> {
    use std::slice::from_raw_parts;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let mut length: isize = 0;
    while *&*ptr.offset(length) != 0 {
        length += 1;
    }

    let array: &[u16] = from_raw_parts(ptr, length as usize);

    OsString::from_wide(&array)
        .into_string()
        .ok()
}

unsafe fn from_ptr(ptr: *const u8) -> Option<String> {
    use std::slice::from_raw_parts;
    use std::str;

    let mut length: isize = 0;
    while *&*ptr.offset(length) != 0 {
        length += 1;
    }

    let array: &[u8] = from_raw_parts(ptr, length as usize);

    str::from_utf8(array)
        .map(|s| s.into())
        .ok()
}
