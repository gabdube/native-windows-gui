/*!
    A control that handle system tray notification.
    A TrayNotification wraps a single icon in the Windows system tray.
    An application can have many TrayNotification.

    Winapi docs: https://docs.microsoft.com/en-us/windows/win32/shell/notification-area
*/
use winapi::um::shellapi::{NIIF_NONE, NIIF_INFO, NIIF_WARNING, NIIF_ERROR, NIIF_USER, NIIF_NOSOUND, NIIF_LARGE_ICON, NIIF_RESPECT_QUIET_TIME};
use super::{ControlBase, ControlHandle};
use crate::win32::base_helper::to_utf16;
use crate::{Image, SystemError};
use std::{mem, ptr};


bitflags! {
    pub struct TrayNotificationFlags: u32 {
        const NO_ICON = NIIF_NONE;
        const INFO_ICON = NIIF_INFO;
        const WARNING_ICON = NIIF_WARNING;
        const ERROR_ICON = NIIF_ERROR;
        const USER_ICON = NIIF_USER;
        const SILENT = NIIF_NOSOUND;
        const LARGE_ICON = NIIF_LARGE_ICON;
        const QUIET = NIIF_RESPECT_QUIET_TIME;
    }
}


/// TrayNotification manager. See module level documentation
#[derive(Default)]
pub struct TrayNotification {
    pub handle: ControlHandle
}

impl TrayNotification {

    pub fn builder<'a>() -> TrayNotificationBuilder<'a> {
        TrayNotificationBuilder {
            parent: None,
            icon: None,
            balloon_icon: None,
            tip: None,
            info: None,
            info_title: None,
            flags: TrayNotificationFlags::NO_ICON
        }
    }

}


pub struct TrayNotificationBuilder<'a> {
    parent: Option<ControlHandle>,
    icon: Option<&'a Image>,

    tip: Option<&'a str>,

    info: Option<&'a str>,
    info_title: Option<&'a str>,
    flags: TrayNotificationFlags,
    balloon_icon: Option<&'a Image>,
}

impl<'a> TrayNotificationBuilder<'a> {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TrayNotificationBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn icon(mut self, ico: Option<&'a Image>) -> TrayNotificationBuilder<'a> {
        self.icon = ico;
        self
    }

    /// Note: balloon_icon is only used if `info` is set AND flags uses `USER_ICON`
    pub fn balloon_icon(mut self, ico: Option<&'a Image>) -> TrayNotificationBuilder<'a> {
        self.balloon_icon = ico;
        self
    }

    /// Note: flags are only used if `info` is set
    pub fn flags(mut self, flags: TrayNotificationFlags) -> TrayNotificationBuilder<'a> {
        self.flags = flags;
        self
    }

    /// Note: tip will be truncated to 127 characters
    pub fn tip(mut self, tip: Option<&'a str>) -> TrayNotificationBuilder<'a> {
        self.tip = tip;
        self
    }

    /// Note: info will be truncated to 255 characters
    pub fn info(mut self, info: Option<&'a str>) -> TrayNotificationBuilder<'a> {
        self.info = info;
        self
    }

    /// Note: info will be truncated to 63 characters
    /// Note 2: This value is only used if info is also specified
    pub fn info_title(mut self, title: Option<&'a str>) -> TrayNotificationBuilder<'a> {
        self.info_title = title;
        self
    }

    pub fn build(self, out: &mut TrayNotification) -> Result<(), SystemError> {
        use winapi::um::shellapi::Shell_NotifyIconW;
        use winapi::um::shellapi::{NIM_ADD, NIF_ICON, NIF_TIP, NIF_SHOWTIP, NIF_INFO, NOTIFYICONDATAW, NOTIFYICONDATAW_u, NOTIFYICON_VERSION_4};
        use winapi::shared::windef::HICON;
        use winapi::um::winnt::WCHAR;

        // Flags

        let mut flags = NIF_ICON;
        let mut info_flags = 0;
        let mut version = 0;
        if self.info.is_some() {
            flags |= NIF_INFO;
            info_flags |= self.flags.bits();
            version = NOTIFYICON_VERSION_4;
        } 
        
        if self.tip.is_some() {
            flags |= NIF_TIP | NIF_SHOWTIP;
        }

        // Resource handles

        let parent = match self.parent {
            Some(p) => match p.hwnd() {
                Some(handle) => Ok(handle),
                None => Err(SystemError::WrongParentType)
            },
            None => Err(SystemError::ControlWithoutParent)
        }?;

        let icon = match self.icon {
            Some(i) => i.handle as HICON,
            None => panic!("Icon required. TODO ERR")
        };

        let balloon_icon = match (self.info.is_some(), self.balloon_icon) {
            (false, _) | (true, None) => ptr::null_mut(),
            (true, Some(i)) => i.handle as HICON
        };

        // UID
        let handle = ControlBase::build_tray_notification()
            .parent(parent)
            .build()?;

        let (_, id) = handle.other().unwrap();
        
        // Tips or infos
        let mut tip: [WCHAR; 128] = [0; 128];
        if self.tip.is_some() {
            let tip_v = to_utf16(self.tip.unwrap());
            let length = if tip_v.len() >= 128 { 127 } else { tip_v.len() };
            for i in 0..length {
                tip[i] = tip_v[i];
            }
        }

        let mut info: [WCHAR; 256] = [0; 256];
        if self.info.is_some() {
            let info_v = to_utf16(self.info.unwrap());
            let length = if info_v.len() >= 256 { 255 } else { info_v.len() };
            for i in 0..length {
                info[i] = info_v[i];
            }
        }
        
        let mut title: [WCHAR; 64] = [0; 64];
        if self.info.is_some() && self.info_title.is_some() {
            let info_title_v = to_utf16(self.info_title.unwrap());
            let length = if info_title_v.len() >= 256 { 255 } else { info_title_v.len() };
            for i in 0..length {
                title[i] = info_title_v[i];
            }
        }

        unsafe {
            let mut u: NOTIFYICONDATAW_u = mem::zeroed();
            *u.uVersion_mut() = version;

            let mut data = winapi::um::shellapi::NOTIFYICONDATAW {
                cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: parent,
                uID: id,
                uFlags: flags,
                uCallbackMessage: 0,
                hIcon: icon,
                szTip: tip,
                dwState: 0,
                dwStateMask: 0,
                szInfo: info,
                u,
                szInfoTitle: title,
                dwInfoFlags: info_flags,
                guidItem: mem::zeroed(),
                hBalloonIcon: balloon_icon
            };

            Shell_NotifyIconW(NIM_ADD, &mut data);
        }

        out.handle = handle;

        Ok(())
    }

}
