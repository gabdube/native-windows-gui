use crate::controls::ControlHandle;
use super::base_helper::to_utf16;
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
    Global(&'static str)
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
    A global object that wraps the system clipbord. It can be used to set or get the system cliboard content.
    
    It's important to keep in mind that there is no way to validate data sent through the clipboard API, as such this wrapper
    is still mostly unsafe and you must validate the data when reading.

    Writing / Reading text

    ```rust

    ```


    Writing / Reading custom data

    ```rust
    use native_windows_gui as nwg;

    #[repr(C)]
    #[derive(Copy)]
    struct Hello {
        foo: usize,
        bar: [u16; 3]
    }

    fn write_custom_data() {
        let data = Hello {
            foo: 6529,
            bar: [0, 100, 20]
        }

        nwg::Clipboard::set_data(
            nwg::ClipboardFormat::Global("Hello"),
            &data as *const Hello,
            1
        )
    }

    fn read_custom_data() -> Hello {

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
    pub fn set_text<'a, C: Into<ControlHandle>>(handle: C, text: &'a str) {
        Clipboard::open(handle);
        Clipboard::empty();

        let text = to_utf16(text);
        
        unsafe { Clipboard::set_data(ClipboardFormat::UnicodeText, text.as_ptr(), text.len()) };

        Clipboard::close();
    }

    /**
        Return the current text value in the clipboard (if there is one).
        This function will return the text if the clipboard has either the `UnicodeText` format or the `Text` format.

        If the clipboard do not have a text format OR the text data is not a valid utf-8 sequence, this function will return `None`.
    */
    pub fn get_text<C: Into<ControlHandle>>(handle: C) -> Option<String> {
        use ClipboardFormat::*;
        let mut data = None;

        Clipboard::open(handle);
        
        if Clipboard::has_format(UnicodeText) {

        } else if Clipboard::has_format(Text) {
            
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
    
        This method is unsafe because there is no way to ensure that data is safe.
        If possible, it is recommended to use a higher level function such as `set_text` instead.

        If the data will be used across applications, make sure that D has `repr(C)` for compatibility.

        * Note 1: `data` is copied into a global system allocation. It's ok to discard the data as soon as this function returns.
        * Note 2: When copying text, the null byte must be included.
    */
    pub unsafe fn set_data<D: Copy>(fmt: ClipboardFormat, data: *const D, count: usize) {
        use winapi::um::winuser::SetClipboardData;
        use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
        use winapi::shared::basetsd::SIZE_T;
        use ClipboardFormat::*;
        use std::{mem, ptr};

        let fmt = match fmt {
            Text => CF_TEXT,
            UnicodeText => CF_UNICODETEXT,
            Bitmap => CF_BITMAP,
            _ => unimplemented!()
        };

        let alloc_size = (mem::size_of::<D>() * count) as SIZE_T;
        let alloc = GlobalAlloc(GMEM_MOVEABLE, alloc_size);

        ptr::copy_nonoverlapping(data, GlobalLock(alloc) as *mut D, count);
        GlobalUnlock(alloc);

        SetClipboardData(fmt, alloc as HANDLE);    
    }

    /**
        Check if the selected format is available in the clipboard. The clipboard must be open.
    */
    pub fn has_format(fmt: ClipboardFormat) -> bool {
        use winapi::um::winuser::EnumClipboardFormats;
        use ClipboardFormat::*;

        let selected_format = match fmt {
            Text => CF_TEXT,
            UnicodeText => CF_UNICODETEXT,
            Bitmap => CF_BITMAP,
            _ => unimplemented!()
        };

        let mut format = 0;
        let next_format = unsafe {
            |fmt: &mut u32| { let f = EnumClipboardFormats(*fmt); *fmt = f; f }
        };

        let mut found = false;
        while next_format(&mut format) != 0 {
            if format == selected_format {
                found = true;
                break;
            } 
        }

        found
    }

    /**
        Get the handle to the clipboard data and copy it's data into a new value of type `D`.
        This function is very unsafe because it assumes that the handle points to the correct type.

        If no data is found with the selected clipboard format, `None` is returned.
    */
    pub unsafe fn get_data<D: Copy>(fmt: ClipboardFormat) -> Option<D> {
        None
    }

    /**
        Gets the data handle of the clipboard, lock the memory and return it in a `ClipboardData` wrapper.
        The returned data is read-only. The application should copy the data and release the handle as soon as possible.

        If no data is found with the selected clipboard format, `None` is returned.
    */
    pub unsafe fn get_data_handle(fmt: ClipboardFormat) -> Option<ClipboardData> {
        None
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

}
