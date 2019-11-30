/*!
    A control that handle system tray notification.
    A TrayNotification wraps a single icon in the Windows system tray.
    An application can have many TrayNotification.

    Winapi docs: https://docs.microsoft.com/en-us/windows/win32/shell/notification-area
*/
use super::{ControlBase, ControlHandle};
use crate::{Image, SystemError};
use std::mem;


/// TrayNotification manager. See module level documentation
pub struct TrayNotification {
    pub handle: ControlHandle
}

impl TrayNotification {

    pub fn builder<'a>() -> TrayNotificationBuilder<'a> {
        TrayNotificationBuilder {
            parent: None,
            icon: None
        }
    }

}


pub struct TrayNotificationBuilder<'a> {
    parent: Option<ControlHandle>,
    icon: Option<&'a Image>
}

impl<'a> TrayNotificationBuilder<'a> {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TrayNotificationBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut TrayNotification) -> Result<(), SystemError> {
        use winapi::um::shellapi::Shell_NotifyIconW;
        use winapi::um::shellapi::{NIM_ADD, NOTIFYICONDATAW};

        let parent = match self.parent {
            Some(p) => match p.hwnd() {
                Some(handle) => Ok(handle),
                None => Err(SystemError::WrongParentType)
            },
            None => Err(SystemError::ControlWithoutParent)
        }?;

        let handle = ControlBase::build_tray_notification()
            .parent(parent)
            .build()?;

        /*unsafe {
            let mut data = winapi::um::shellapi::NOTIFYICONDATAW {
                cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: parent,
                uFlags: 0
            };

            Shell_NotifyIconW(NIM_ADD, &mut data);
        }*/

        out.handle = handle;

        Ok(())
    }

}
