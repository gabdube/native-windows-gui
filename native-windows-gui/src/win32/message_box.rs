use super::base_helper::to_utf16;
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

/**
    Create an application wide message box

    Parameters:  
    * params: A `MessageParams` structure that defines how the message box should look
*/
pub fn message<'a>(params: &MessageParams) -> MessageChoice {
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

    let answer = unsafe{ MessageBoxW(ptr::null_mut(), text.as_ptr(), title.as_ptr(), buttons | icons) };
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
        icons: MessageIcons::Error
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
        icons: MessageIcons::Info
    };

    message(&params)
}

