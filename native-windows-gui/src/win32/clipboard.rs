use crate::controls::ControlHandle;
use super::base_helper::to_utf16;


pub enum ClipboardFormat {
    /// ANSI text. You probably want to use `UnicodeText`.
    Text,

    /// UnicodeText. Equivalent to a OsString
    UnicodeText,

    /// A bitmap file
    Bitmap
}


/**
    A global object that wraps the system clipbord. It can be used to set or get the system cliboard content.
    This object must not be instanced. The methods should be used this way:

    ```rust
        use native_windows_gui as nwg;
        let (x,y) = nwg::Clipboard::position();
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
        It is recommended to use a higher level function such as `set_text` instead.

        Note 1: `data` is copied into a global system allocation.
        Note 2: When copying text, the null byte must be included.
    */
    pub unsafe fn set_data<D: Copy>(fmt: ClipboardFormat, data: *const D, count: usize) {
        use winapi::um::winuser::{SetClipboardData, CF_BITMAP, CF_TEXT, CF_UNICODETEXT};
        use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
        use winapi::um::winnt::HANDLE;
        use winapi::shared::basetsd::SIZE_T;
        use ClipboardFormat::*;
        use std::{mem, ptr};

        let fmt = match fmt {
            Text => CF_TEXT,
            UnicodeText => CF_UNICODETEXT,
            Bitmap => CF_BITMAP,
        };

        let alloc_size = (mem::size_of::<D>() * count) as SIZE_T;
        let alloc = GlobalAlloc(GMEM_MOVEABLE, alloc_size);

        ptr::copy_nonoverlapping(data, GlobalLock(alloc) as *mut D, count);
        GlobalUnlock(alloc);

        SetClipboardData(fmt, alloc as HANDLE);    
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
