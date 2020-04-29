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

use std::mem;
use std::ptr;

use winapi::DWORD;

use defs::{MessageButtons, MessageChoice, MessageIcons, MessageParams};

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

    let null_index = s.iter().position(|&i| i == 0).unwrap_or(s.len());
    let os_string = OsString::from_wide(&s[0..null_index]);

    os_string
        .into_string()
        .unwrap_or("Decoding error".to_string())
}

/**
    Read a string from a wide char pointer. Undefined behaviour if [ptr] is not null terminated.
*/
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
    use kernel32::{FormatMessageW, GetLastError};
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use winapi::{FORMAT_MESSAGE_FROM_SYSTEM, LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT};

    let code = GetLastError();
    let lang = MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as DWORD;
    let mut buf: Vec<u16> = Vec::with_capacity(1024);
    buf.set_len(1024);
    FormatMessageW(
        FORMAT_MESSAGE_FROM_SYSTEM,
        ptr::null(),
        code,
        lang,
        buf.as_mut_ptr(),
        1024,
        ptr::null_mut(),
    );

    let end = buf.iter().position(|&i| i == 0).unwrap_or(1024);
    let error_message = OsString::from_wide(&buf[..end])
        .into_string()
        .unwrap_or("Error while decoding system error message".to_string());

    (code, error_message)
}

/**
  Enable the Windows visual style in the application without having to use a manifest
*/
pub unsafe fn enable_visual_styles() {
    use comctl32::InitCommonControlsEx;
    use kernel32::{ActivateActCtx, CreateActCtxW, GetSystemDirectoryW};
    use low::defs::{
        ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID, ACTCTX_FLAG_RESOURCE_NAME_VALID,
        ACTCTX_FLAG_SET_PROCESS_DEFAULT,
    };
    use winapi::{
        ACTCTXW, ICC_DATE_CLASSES, ICC_PROGRESS_CLASS, ICC_STANDARD_CLASSES, ICC_WIN95_CLASSES,
        INITCOMMONCONTROLSEX, MAX_PATH, ULONG, ULONG_PTR,
    };

    let mut sys_dir: Vec<u16> = Vec::with_capacity(MAX_PATH);
    sys_dir.set_len(MAX_PATH);
    GetSystemDirectoryW(sys_dir.as_mut_ptr(), MAX_PATH as u32);

    let mut source = to_utf16("shell32.dll");

    let mut activation_cookie: ULONG_PTR = 0;
    let mut act_ctx = ACTCTXW {
        cbSize: mem::size_of::<ACTCTXW> as ULONG,
        dwFlags: ACTCTX_FLAG_RESOURCE_NAME_VALID
            | ACTCTX_FLAG_SET_PROCESS_DEFAULT
            | ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID,
        lpSource: source.as_mut_ptr(),
        wProcessorArchitecture: 0,
        wLangId: 0,
        lpAssemblyDirectory: sys_dir.as_mut_ptr(),
        lpResourceName: mem::transmute(124usize), // ID_MANIFEST
        lpApplicationName: ptr::null_mut(),
        hModule: ptr::null_mut(),
    };

    let handle = CreateActCtxW(&mut act_ctx);
    ActivateActCtx(handle, &mut activation_cookie);

    let controls_classes = INITCOMMONCONTROLSEX {
        dwSize: mem::size_of::<INITCOMMONCONTROLSEX> as DWORD,
        dwICC: ICC_DATE_CLASSES | ICC_STANDARD_CLASSES | ICC_PROGRESS_CLASS | ICC_WIN95_CLASSES,
    };

    InitCommonControlsEx(&controls_classes);
}

/**
   Initializes the COM library for use by the calling thread,
*/
pub unsafe fn enable_com() {
    use ole32::CoInitializeEx;
    use winapi::{COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE};
    CoInitializeEx(
        ptr::null_mut(),
        COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE,
    );
}

/**
    Create an application wide message box

    Parameters:
    * params: A `MessageParams` structure that defines how the message box should look
*/
pub fn message<'a>(params: &MessageParams) -> MessageChoice {
    use low::defs::{
        IDABORT, IDCANCEL, IDCONTINUE, IDIGNORE, IDNO, IDOK, IDRETRY, IDTRYAGAIN, IDYES,
    };
    use user32::MessageBoxW;
    use winapi::{
        MB_ABORTRETRYIGNORE, MB_CANCELTRYCONTINUE, MB_ICONEXCLAMATION, MB_ICONINFORMATION,
        MB_ICONQUESTION, MB_ICONSTOP, MB_OK, MB_OKCANCEL, MB_RETRYCANCEL, MB_YESNO, MB_YESNOCANCEL,
    };

    let text = to_utf16(params.content);
    let title = to_utf16(params.title);

    let buttons = match params.buttons {
        MessageButtons::AbortTryIgnore => MB_ABORTRETRYIGNORE,
        MessageButtons::CancelTryContinue => MB_CANCELTRYCONTINUE,
        MessageButtons::Ok => MB_OK,
        MessageButtons::OkCancel => MB_OKCANCEL,
        MessageButtons::RetryCancel => MB_RETRYCANCEL,
        MessageButtons::YesNo => MB_YESNO,
        MessageButtons::YesNoCancel => MB_YESNOCANCEL,
    };

    let icons = match params.icons {
        MessageIcons::Error => MB_ICONSTOP,
        MessageIcons::Info => MB_ICONINFORMATION,
        MessageIcons::None => 0,
        MessageIcons::Question => MB_ICONQUESTION,
        MessageIcons::Warning => MB_ICONEXCLAMATION,
    };

    let answer = unsafe {
        MessageBoxW(
            ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            buttons | icons,
        )
    };
    match answer {
        IDABORT => MessageChoice::Abort,
        IDCANCEL => MessageChoice::Cancel,
        IDCONTINUE => MessageChoice::Continue,
        IDIGNORE => MessageChoice::Ignore,
        IDNO => MessageChoice::No,
        IDOK => MessageChoice::Ok,
        IDRETRY => MessageChoice::Retry,
        IDTRYAGAIN => MessageChoice::TryAgain,
        IDYES => MessageChoice::Yes,
        _ => MessageChoice::Cancel,
    }
}

/**
    Display a message box and then panic. The message box has for style `MessageButtons::Ok` and `MessageIcons::Error` .

    Parameters:
    * title: The message box title
    * content: The message box message
*/
pub fn fatal_message<'a>(title: &'a str, content: &'a str) -> ! {
    error_message(title, content);
    panic!("{} - {}", title, content);
}

/**
    Display a simple error message box. The message box has for style `MessageButtons::Ok` and `MessageIcons::Error` .

    Parameters:
    * title: The message box title
    * content: The message box message
*/
pub fn error_message<'a>(title: &'a str, content: &'a str) -> MessageChoice {
    let params = MessageParams {
        title: title,
        content: content,
        buttons: MessageButtons::Ok,
        icons: MessageIcons::Error,
    };

    message(&params)
}

/**
    Display a simple message box. The message box has for style `MessageButtons::Ok` and `MessageIcons::Info` .

    Parameters:
    * title: The message box title
    * content: The message box message
*/
pub fn simple_message<'a>(title: &'a str, content: &'a str) -> MessageChoice {
    let params = MessageParams {
        title: title,
        content: content,
        buttons: MessageButtons::Ok,
        icons: MessageIcons::Info,
    };

    message(&params)
}
