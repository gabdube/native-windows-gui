use winapi::um::shellapi::{NIIF_NONE, NIIF_INFO, NIIF_WARNING, NIIF_ERROR, NIIF_USER, NIIF_NOSOUND, NIIF_LARGE_ICON, NIIF_RESPECT_QUIET_TIME};
use winapi::um::shellapi::{Shell_NotifyIconW, NOTIFYICONDATAW};
use super::{ControlBase, ControlHandle};
use crate::win32::base_helper::to_utf16;
use crate::win32::window_helper as wh;
use crate::{Icon, NwgError};
use std::{mem, ptr};

const NOT_BOUND: &'static str = "TrayNotification is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TrayNotification handle is not HWND!";


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


/**
    A control that handle system tray notification.
    A TrayNotification wraps a single icon in the Windows system tray.
    
    An application can have many TrayNotification, but each window (aka parent) can only have a single traynotification.
    It is possible to create system tray only application with the `MessageOnlyWindow` control.

    A system tray will receive events if `callback` is set to true in the builder (the default behaviour).
    The control will generate mouse events such as `OnMouseMove` when the user interact with the tray icon or the message popup.
    A system tray will also receive a `OnContextMenu` when the user right click the icon. It is highly recommended handle this message and display a popup menu

    You can't get information on the state of a tray notification (such as visibility) because Windows don't want you to.

    **Builder parameters:**
        * `parent`:       **Required.** The tray notification parent container.
        * `icon`:         **Required.** The icon to display in the system tray
        * `tips`:         Display a simple tooltip when hovering the icon in the system tray
        * `flags`:        A combination of the TrayNotificationFlags values.
        * `visible`:      If the icon should be visible in the system tray
        * `realtime`:     If the balloon notification cannot be displayed immediately, discard it.
        * `info`:         Display a fancy tooltip when the system tray icon is hovered (replaces tip) 
        * `balloon_icon`: The icon to display in the fancy tooltip  
        * `info_title`:   The title of the fancy tooltip  

    **Control events:**
        * `OnContextMenu`: When the user right clicks on the system tray icon
        * `MousePressLeftUp`: When the user left click the system tray icon
        * `OnTrayNotificationShow`: When a TrayNotification info popup (not the tooltip) is shown 
        * `OnTrayNotificationHide`: When a TrayNotification info popup (not the tooltip) is hidden 
        * `OnTrayNotificationTimeout`: When a TrayNotification is closed due to a timeout
        * `OnTrayNotificationUserClose`: When a TrayNotification is closed due to a user click

    ## Example

    ```rust
    use native_windows_gui as nwg;

    fn notice_user(tray: &nwg::TrayNotification, image: &nwg::Icon) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        tray.show("Hello World", Some("Welcome to my application"), Some(flags), Some(image));
    }
    ```

    ```rust
    use native_windows_gui as nwg;
    fn build_tray(tray: &mut nwg::TrayNotification, window: &nwg::Window, icon: &nwg::Icon) {
        nwg::TrayNotification::builder()
            .parent(window)
            .icon(Some(icon))
            .tip(Some("Hello"))
            .build(tray);
    }
    ```

    Winapi docs: https://docs.microsoft.com/en-us/windows/win32/shell/notification-area
*/
#[derive(Default, PartialEq, Eq)]
pub struct TrayNotification {
    pub handle: ControlHandle,
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
            flags: TrayNotificationFlags::NO_ICON,
            realtime: false,
            callback: true,
            visible: true,
        }
    }

    /// Set the visibility of the icon in the system tray
    pub fn set_visibility(&self, v: bool) {
        use winapi::um::shellapi::{NIF_STATE, NIM_MODIFY, NIS_HIDDEN};  

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        self.handle.tray().expect(BAD_HANDLE);

        unsafe {
            let mut data = self.notify_default();
            data.uFlags |= NIF_STATE;
            data.dwState = if v { 0 } else { NIS_HIDDEN };
            data.dwStateMask = NIS_HIDDEN;
            Shell_NotifyIconW(NIM_MODIFY, &mut data);
        }
    }

    /// Set the tooltip for the tray notification.
    /// Note: tip will be truncated to 127 characters
    pub fn set_tip<'a>(&self, tip: &'a str) {
        use winapi::um::shellapi::{NIM_MODIFY, NIF_TIP, NIF_SHOWTIP};  

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        self.handle.tray().expect(BAD_HANDLE);

        unsafe {
            let mut data = self.notify_default();
            
            data.uFlags = NIF_TIP | NIF_SHOWTIP;
            
            let tip_v = to_utf16(tip);
            let length = if tip_v.len() >= 128 { 127 } else { tip_v.len() };
            for i in 0..length {
                data.szTip[i] = tip_v[i];
            }

            Shell_NotifyIconW(NIM_MODIFY, &mut data);
        }
    }

    /// Set the focus to the tray icon
    pub fn set_focus(&self) {
        use winapi::um::shellapi::{NIM_SETFOCUS};

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        self.handle.tray().expect(BAD_HANDLE);

        unsafe {
            let mut data = self.notify_default();
            Shell_NotifyIconW(NIM_SETFOCUS, &mut data);
        }
    }

    /// Update the icon in the system tray
    pub fn set_icon(&self, icon: &Icon) {
        use winapi::um::shellapi::{NIF_ICON, NIM_MODIFY};
        use winapi::shared::windef::HICON;

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        self.handle.tray().expect(BAD_HANDLE);

        unsafe {
            let mut data = self.notify_default();
            
            data.uFlags = NIF_ICON;
            data.hIcon = icon.handle as HICON;
            Shell_NotifyIconW(NIM_MODIFY, &mut data);
        }
    }

    /// Shows a popup message on top of the system tray
    ///
    /// Parameters:
    ///   - text: The text in the popup
    ///   - title: The title of the popup
    ///   - flags: Flags that specify how the popup is shown. Default is NO_ICON | QUIET.
    ///   - icon: Icon to display in the popup. Only used if `USER_ICON` is set in flags.
    ///
    /// Note 1: text will be truncated to 255 characters
    /// Note 2: title will be truncated to 63 characters
    pub fn show<'a>(&self, text: &'a str, title: Option<&'a str>, flags: Option<TrayNotificationFlags>, icon: Option<&'a Icon>) {
        use winapi::um::shellapi::{NIF_INFO, NIM_MODIFY};
        use winapi::shared::windef::HICON;

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        self.handle.tray().expect(BAD_HANDLE);

        let default_flags = TrayNotificationFlags::NO_ICON | TrayNotificationFlags::SILENT;

        unsafe {
            let mut data = self.notify_default();
            data.uFlags = NIF_INFO;
            data.dwInfoFlags = flags.unwrap_or(default_flags).bits();
            data.hBalloonIcon = icon.map(|i| i.handle as HICON).unwrap_or(ptr::null_mut());
            
            let info_v = to_utf16(text);
            let length = if info_v.len() >= 256 { 255 } else { info_v.len() };
            for i in 0..length {
                data.szInfo[i] = info_v[i];
            }
        
            let info_title_v = match title {
                Some(t) => to_utf16(t),
                None => vec![]
            };

            let length = if info_title_v.len() >= 256 { 255 } else { info_title_v.len() };
            for i in 0..length {
                data.szInfoTitle[i] = info_title_v[i];
            }

            Shell_NotifyIconW(NIM_MODIFY, &mut data);
        }
    }

    fn notify_default(&self) -> NOTIFYICONDATAW {
        unsafe {
            let parent = self.handle.tray().unwrap();
            NOTIFYICONDATAW {
                cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: parent,
                uID: 0,
                uFlags: 0,
                uCallbackMessage: 0,
                hIcon: ptr::null_mut(),
                szTip: mem::zeroed(),
                dwState: 0,
                dwStateMask: 0,
                szInfo: mem::zeroed(),
                u: mem::zeroed(),
                szInfoTitle: mem::zeroed(),
                dwInfoFlags: 0,
                guidItem: mem::zeroed(),
                hBalloonIcon: ptr::null_mut()
            }
        }
    }

}

impl Drop for TrayNotification {
    fn drop(&mut self) {
        use winapi::um::shellapi::NIM_DELETE;

        if self.handle.tray().is_some() {
            let mut data = self.notify_default();
            unsafe {
                Shell_NotifyIconW(NIM_DELETE, &mut data);
            }
        }

        self.handle.destroy();
    }
}

pub struct TrayNotificationBuilder<'a> {
    parent: Option<ControlHandle>,
    icon: Option<&'a Icon>,

    tip: Option<&'a str>,

    info: Option<&'a str>,
    info_title: Option<&'a str>,
    flags: TrayNotificationFlags,
    balloon_icon: Option<&'a Icon>,

    realtime: bool,
    callback: bool,
    visible: bool,
}

impl<'a> TrayNotificationBuilder<'a> {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> TrayNotificationBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn icon(mut self, ico: Option<&'a Icon>) -> TrayNotificationBuilder<'a> {
        self.icon = ico;
        self
    }

    pub fn realtime(mut self, r: bool) -> TrayNotificationBuilder<'a> {
        self.realtime = r;
        self
    }

    pub fn callback(mut self, cb: bool) -> TrayNotificationBuilder<'a> {
        self.callback = cb;
        self
    }

    pub fn visible(mut self, v: bool) -> TrayNotificationBuilder<'a> {
        self.visible = v;
        self
    }

    /// Note: balloon_icon is only used if `info` is set AND flags uses `USER_ICON`
    pub fn balloon_icon(mut self, ico: Option<&'a Icon>) -> TrayNotificationBuilder<'a> {
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

    pub fn build(self, out: &mut TrayNotification) -> Result<(), NwgError> {
        use winapi::um::shellapi::{NIM_ADD, NIF_ICON, NIF_TIP, NIF_SHOWTIP, NIF_INFO, NOTIFYICONDATAW_u, NOTIFYICON_VERSION_4,
         NIF_REALTIME, NIF_MESSAGE, NIS_HIDDEN, NIF_STATE};
        use winapi::shared::windef::HICON;
        use winapi::um::winnt::WCHAR;

        // Flags
        let version = NOTIFYICON_VERSION_4;
        let mut flags = NIF_ICON;
        let mut info_flags = 0;
        let mut state = 0;
        
        if self.info.is_some() {
            flags |= NIF_INFO;
            info_flags |= self.flags.bits();
        } 
        
        if self.tip.is_some() {
            flags |= NIF_TIP | NIF_SHOWTIP;
        }

        if self.realtime { flags |= NIF_REALTIME; }
        if self.callback { flags |= NIF_MESSAGE; }
        if !self.visible { state |= NIS_HIDDEN; flags |= NIF_STATE; }

        // Resource handles

        let parent = match self.parent {
            Some(p) => match p.hwnd() {
                Some(handle) => Ok(handle),
                None => Err(NwgError::control_create("TrayNotification must be window-like control."))
            },
            None => Err(NwgError::no_parent("Button"))
        }?;

        let icon = match self.icon {
            Some(i) => i.handle as HICON,
            None => panic!("Tray notification requires an Icon at creation")
        };

        let balloon_icon = match (self.info.is_some(), self.balloon_icon) {
            (false, _) | (true, None) => ptr::null_mut(),
            (true, Some(i)) => i.handle as HICON
        };

        // UID
        let handle = ControlBase::build_tray_notification()
            .parent(parent)
            .build()?;
        
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

        // Creation
        unsafe {
            let mut u: NOTIFYICONDATAW_u = mem::zeroed();
            *u.uVersion_mut() = version;

            let mut data = NOTIFYICONDATAW {
                cbSize: mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: parent,
                uID: 0,
                uFlags: flags,
                uCallbackMessage: wh::NWG_TRAY,
                hIcon: icon,
                szTip: tip,
                dwState: state,
                dwStateMask: state,
                szInfo: info,
                u,
                szInfoTitle: title,
                dwInfoFlags: info_flags,
                guidItem: mem::zeroed(),
                hBalloonIcon: balloon_icon
            };

            Shell_NotifyIconW(NIM_ADD, &mut data);
        }


        // Finish
        *out = Default::default();
        out.handle = handle;

        Ok(())
    }

}
