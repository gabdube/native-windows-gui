use std::{ptr};
use winapi::shared::minwindef::DWORD;


pub fn to_utf16<'a>(s: &'a str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s)
      .encode_wide()
      .chain(Some(0u16).into_iter())
      .collect()
}

/**
    Decode a raw utf16 string. Should be null terminated.
*/
pub fn from_utf16(s: &[u16]) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let null_index = s.iter().position(|&i| i==0).unwrap_or(s.len());
    let os_string = OsString::from_wide(&s[0..null_index]);

    os_string.into_string().unwrap_or("Decoding error".to_string())
}

/**
    Read a string from a wide char pointer. Undefined behaviour if [ptr] is not null terminated.
*/
#[cfg(feature = "file-dialog")]
pub unsafe fn from_wide_ptr(ptr: *mut u16) -> String {
    use std::slice::from_raw_parts;

    let mut length: isize = 0;
    while *&*ptr.offset(length) != 0 {
        length += 1;
    }

    let array: &[u16] = from_raw_parts(ptr, length as usize);
    from_utf16(array)
}

/**
    Return a formatted output of the last system error that was raised.

    (ERROR ID, Error message localized)
*/
pub unsafe fn get_system_error() -> (DWORD, String) { 
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::winbase::{FormatMessageW, FORMAT_MESSAGE_FROM_SYSTEM};
    use winapi::um::winnt::{MAKELANGID, LANG_NEUTRAL, SUBLANG_DEFAULT};
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let code = GetLastError();
    let lang = MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as DWORD;
    let mut buf: Vec<u16> = Vec::with_capacity(1024);
    buf.set_len(1024);
    FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM, ptr::null(), code, lang, buf.as_mut_ptr(), 1024, ptr::null_mut());

    let end = buf.iter().position(|&i| i==0).unwrap_or(1024);
    let error_message = OsString::from_wide(&buf[..end])
        .into_string()
        .unwrap_or("Error while decoding system error message".to_string());

    (code, error_message)
}
