use super::base_helper::to_utf16;
use crate::controls::ControlHandle;
use winapi::shared::windef::HWND;
use std::ptr;


/**
    Enum of message box buttons (to use with `MessageParams` )
*/
#[derive(Clone, PartialEq, Debug)]
pub enum MessageButtons {
    AbortTryIgnore,
    CancelTryContinue,
    Ok,
    OkCancel,
    RetryCancel,
    YesNo,
    YesNoCancel,
}

/**
    Enum of message box icons (to use with `MessageParams` )
*/
#[derive(Clone, PartialEq, Debug)]
pub enum MessageIcons {
    Warning,
    Info,
    Question,
    Error,
    None
}

/**
    Return value of `message`. Define the button that the user clicked. If the user 
    cancelled the message box by clicking on X button of the window, `MessageChoice::Cancel` is returned.
*/
#[derive(Clone, PartialEq, Debug)]
pub enum MessageChoice {
    Abort,
    Cancel,
    Continue,
    Ignore,
    No,
    Ok,
    Retry,
    TryAgain,
    Yes
}

/**
    A structure that defines how a messagebox should look and behave. 

    Members:  
    * `title`: The title of the message box  
    * `content`: The message of the message box  
    * `buttons`: The button of the message box  
    * `icons`: The message box icon  
*/
#[derive(Clone, PartialEq, Debug)]
pub struct MessageParams<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub buttons: MessageButtons,
    pub icons: MessageIcons
}


/// Inner function used by the message box function
fn inner_message(parent: HWND, params: &MessageParams) -> MessageChoice {
    use winapi::um::winuser::{MB_ABORTRETRYIGNORE, MB_CANCELTRYCONTINUE, MB_OK, MB_OKCANCEL, MB_RETRYCANCEL, MB_YESNO,
        MB_YESNOCANCEL, MB_ICONSTOP, MB_ICONINFORMATION, MB_ICONQUESTION, MB_ICONEXCLAMATION};
   
       use winapi::um::winuser::{IDABORT, IDCANCEL, IDCONTINUE, IDIGNORE, IDNO, IDOK, IDRETRY, IDTRYAGAIN, IDYES};
       use winapi::um::winuser::MessageBoxW;
   
       let text = to_utf16(params.content);
       let title = to_utf16(params.title);
   
       let buttons = match params.buttons {
           MessageButtons::AbortTryIgnore => MB_ABORTRETRYIGNORE,
           MessageButtons::CancelTryContinue => MB_CANCELTRYCONTINUE,
           MessageButtons::Ok => MB_OK,
           MessageButtons::OkCancel => MB_OKCANCEL,
           MessageButtons::RetryCancel => MB_RETRYCANCEL,
           MessageButtons::YesNo => MB_YESNO,
           MessageButtons::YesNoCancel => MB_YESNOCANCEL
       };
   
       let icons = match params.icons {
           MessageIcons::Error => MB_ICONSTOP,
           MessageIcons::Info => MB_ICONINFORMATION,
           MessageIcons::None => 0,
           MessageIcons::Question => MB_ICONQUESTION,
           MessageIcons::Warning => MB_ICONEXCLAMATION
       };
   
       let answer = unsafe{ MessageBoxW(parent, text.as_ptr(), title.as_ptr(), buttons | icons) };
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
           _ => MessageChoice::Cancel
       }
}

/**
    Create an application wide message box. 
    It is recommended to use `modal_message` because it locks the window that creates the message box.
    This method may be deprecated in the future

    Parameters:  
    * params: A `MessageParams` structure that defines how the message box should look

    ```rust
    use native_windows_gui as nwg;
    fn test_message() {
        let p = nwg::MessageParams {
            title: "Hey",
            content: "Cats are cute",
            buttons: nwg::MessageButtons::Ok,
            icons: nwg::MessageIcons::Warning
        };

        assert!(nwg::message(&p) == nwg::MessageChoice::Ok)
    }
    ```
*/
pub fn message<'a>(params: &MessageParams) -> MessageChoice {
    inner_message(ptr::null_mut(), params)
}


/**
    Create a message box for a selected window. The window will be locked until the user close the message box.

    This functions panics if a non window control is used as parent (ex: a menu)

    Parameters:  
    * parent: The reference to a window-like control
    * params: A `MessageParams` structure that defines how the message box should look

    ```rust
    use native_windows_gui as nwg;
    fn test_message(parent: &nwg::Window) {
        let p = nwg::MessageParams {
            title: "Hey",
            content: "Cats are cute",
            buttons: nwg::MessageButtons::Ok,
            icons: nwg::MessageIcons::Warning
        };

        assert!(nwg::modal_message(parent, &p) == nwg::MessageChoice::Ok)
    }
    ```
*/
pub fn modal_message<'a, P: Into<ControlHandle>>(parent: P, params: &MessageParams) -> MessageChoice {
    let control_handle = parent.into();
    let hwnd = control_handle.hwnd().expect("expected window like control");
    inner_message(hwnd, params)
}

/**
    Display a message box and then panic. The message box has for style `MessageButtons::Ok` and `MessageIcons::Error` .
    It is recommended to use `modal_fatal_message` because it locks the window that creates the message box.
    This method may be deprecated in the future

    Parameters:
    * title: The message box title
    * content: The message box message
*/
pub fn fatal_message<'a>(title: &'a str, content: &'a str) -> ! {
    error_message(title, content);
    panic!("{} - {}", title, content);
}


/**
    Display a message box and then panic. The message box has for style `MessageButtons::Ok` and `MessageIcons::Error` .

    This functions panics if a non window control is used as parent (ex: a menu)

    Parameters:
    * parent: Parent window to lock for the duration of the message box
    * title: The message box title
    * content: The message box message
*/
pub fn modal_fatal_message<'a, P: Into<ControlHandle>>(parent: P, title: &'a str, content: &'a str) -> ! {
    modal_error_message(parent, title, content);
    panic!("{} - {}", title, content);
}


/**
    Display a simple error message box. The message box has for style `MessageButtons::Ok` and `MessageIcons::Error`.
    It is recommended to use `modal_error_message` because it locks the window that creates the message box.
    This method may be deprecated in the future

    Parameters:
    * title: The message box title
    * content: The message box message
*/
pub fn error_message<'a>(title: &'a str, content: &'a str) -> MessageChoice {
    let params = MessageParams {
        title,
        content,
        buttons: MessageButtons::Ok,
        icons: MessageIcons::Error
    };

    message(&params)
}


/**
    Display a simple error message box. The message box has for style `MessageButtons::Ok` and `MessageIcons::Error`.

    This functions panics if a non window control is used as parent (ex: a menu)

    Parameters:
    * parent: Parent window to lock for the duration of the message box
    * title: The message box title
    * content: The message box message
*/
pub fn modal_error_message<'a, P: Into<ControlHandle>>(parent: P, title: &'a str, content: &'a str) -> MessageChoice {
    let params = MessageParams {
        title,
        content,
        buttons: MessageButtons::Ok,
        icons: MessageIcons::Error
    };

    modal_message(parent, &params)
}


/**
    Display a simple message box. The message box has for style `MessageButtons::Ok` and `MessageIcons::Info`.
    It is recommended to use `modal_info_message` because it locks the window that creates the message box.
    This method may be deprecated in the future

    Parameters:
    * title: The message box title
    * content: The message box message
*/
pub fn simple_message<'a>(title: &'a str, content: &'a str) -> MessageChoice {
    let params = MessageParams {
        title,
        content,
        buttons: MessageButtons::Ok,
        icons: MessageIcons::Info
    };

    message(&params)
}


/**
    Display a simple message box. The message box has for style `MessageButtons::Ok` and `MessageIcons::Info`.

    This functions panics if a non window control is used as parent (ex: a menu)

    Parameters:
    * parent: Parent window to lock for the duration of the message box
    * title: The message box title
    * content: The message box message
*/
pub fn modal_info_message<'a, P: Into<ControlHandle>>(parent: P, title: &'a str, content: &'a str) -> MessageChoice {
    let params = MessageParams {
        title,
        content,
        buttons: MessageButtons::Ok,
        icons: MessageIcons::Info
    };

    modal_message(parent, &params)
}
