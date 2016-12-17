/*!
    Functionalities that cannot fit in any specific categories
*/

/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::ptr;
use std::mem;

use winapi::DWORD;

/**
    Encode a string value into a utf16 string. Adds a null char at the end of the string.
*/
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
    Return a formatted output of the last system error that was raised.

    (ERROR ID, Error message localized)
*/
pub unsafe fn get_system_error() -> (DWORD, String) { 
  use kernel32::{GetLastError, FormatMessageW};
  use winapi::{FORMAT_MESSAGE_FROM_SYSTEM, MAKELANGID, LANG_NEUTRAL, SUBLANG_DEFAULT};
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

/**
  Enable the Windows visual style in the application without having to use a manifest
*/
pub unsafe fn enable_visual_styles() {
    use kernel32::{ActivateActCtx, CreateActCtxW, GetSystemDirectoryW};
    use winapi::{MAX_PATH, ULONG, ACTCTXW, ULONG_PTR};
    use comctl32::InitCommonControls;
    use low::defs::{ACTCTX_FLAG_RESOURCE_NAME_VALID, ACTCTX_FLAG_SET_PROCESS_DEFAULT, ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID};

    let mut sys_dir: Vec<u16> = Vec::with_capacity(MAX_PATH);
    sys_dir.set_len(MAX_PATH);
    GetSystemDirectoryW(sys_dir.as_mut_ptr(), MAX_PATH as u32);

    let mut source = to_utf16("shell32.dll");

    let mut activation_cookie: ULONG_PTR = 0;
    let mut act_ctx = ACTCTXW {
        cbSize: mem::size_of::<ACTCTXW> as ULONG,
        dwFlags: ACTCTX_FLAG_RESOURCE_NAME_VALID | ACTCTX_FLAG_SET_PROCESS_DEFAULT | ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID,
        lpSource: source.as_mut_ptr(),
        wProcessorArchitecture: 0,
        wLangId: 0,
        lpAssemblyDirectory: sys_dir.as_mut_ptr(),
        lpResourceName: mem::transmute(124usize), // ID_MANIFEST
        lpApplicationName: ptr::null_mut(),
        hModule: ptr::null_mut()
    };

    let handle = CreateActCtxW(&mut act_ctx);
    ActivateActCtx(handle, &mut activation_cookie);

    // Init common controls
    InitCommonControls();
}
