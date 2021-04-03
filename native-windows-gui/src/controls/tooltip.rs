use winapi::shared::minwindef::{UINT, LPARAM, WPARAM};
use winapi::um::winnt::WCHAR;
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{check_hwnd, to_utf16, from_utf16};
use crate::{Icon, NwgError};
use super::{ControlBase, ControlHandle};
use std::{mem, ptr};

const NOT_BOUND: &'static str = "Tooltip is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Tooltip handle is not HWND!";


/// A select of default icon to show in a tooltip
#[derive(Copy, Clone, Debug)]
pub enum TooltipIcon {
    None,
    Info,
    Warning,
    Error,
    InfoLarge,
    WarningLarge,
    ErrorLarge
}

/**
Tooltips appear automatically, or pop up, when the user pauses the mouse pointer over a tool or
some other UI element. The tooltip appears near the pointer and disappears when the user
clicks a mouse button, moves the pointer away from the tool, or simply waits for a few seconds.

A tooltip can be applied to multiple controls, each with their own custom text.
This is done/undone using the `register`/`unregister` functions. So do not think
as Tooltip as a standalone toolip, but more like a manager.

A tooltip can support static text using `register` and dynamic text using `register_callback`.

Tooltip requires the `tooltip` features

Example:

```rust
use native_windows_gui as nwg;

/// Building a tooltip and add tooltips at the same time
fn build_tooltip(tt: &mut nwg::Tooltip, btn1: &nwg::Button, btn2: &nwg::Button) {
    nwg::Tooltip::builder()
        .register(btn1, "A test button")
        .register_callback(btn2)
        .build(tt);
}

/// Adding/Updating a tooltip after the initial tooltip creation
fn add_tooltip(btn: &nwg::Button, tt: &nwg::Tooltip) {
    tt.register(btn, "This is a button!");
}

/// Dynamic tooltip callback setup
fn add_dynamic_tooltip(tt: &nwg::Tooltip, btn: &nwg::Button) {
    tt.register_callback(btn);
}


struct GuiStruct {
    // Skipping other members
    tt: nwg::Tooltip,
    button: nwg::Button
}

impl GuiStruct {
    /// The dynamic tooltip callback, triggered by the event loop
    fn events_callback(&self, evt: nwg::Event, evt_data: &nwg::EventData, handle: nwg::ControlHandle) {
        match evt {
            nwg::Event::OnTooltipText => {
                // Compare the handle to check which control will display the tooltip
                if &handle == &self.button {
                    let tooltip_data = evt_data.on_tooltip_text();
                    tooltip_data.set_text(&format!("Button text: \"{}\"", self.button.text()));
                }
            },
            _ => {}
        }
    }
}



```

*/
#[derive(Default, PartialEq, Eq)]
pub struct Tooltip {
    pub handle: ControlHandle
}

impl Tooltip {

    pub fn builder<'a>() -> TooltipBuilder<'a> {
        TooltipBuilder {
            title: None,
            ico: None,
            default_ico: None,
            register: Vec::new(),
            register_cb: Vec::new()
        }
    }

    /*
    Work with Comclt32.dll version 6.0. Should be implemented eventually
    Return the icon if it is a icon defined in TooltipIcon. If not, returns `None`.
    pub fn default_icon(&self) -> Option<TooltipIcon> {
        use winapi::um::commctrl::{TTGETTITLE, TTM_GETTITLE};
        use winapi::um::commctrl::{TTI_NONE, TTI_INFO, TTI_WARNING, TTI_ERROR, TTI_INFO_LARGE, TTI_WARNING_LARGE, TTI_ERROR_LARGE};

        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let mut tt = TTGETTITLE {
            dwSize: mem::size_of::<TTGETTITLE>() as DWORD,
            uTitleBitmap: 0,
            cch: 0,
            pszTitle: ptr::null_mut()
        };

        let tt_ptr = &mut tt as *mut TTGETTITLE;
        wh::send_message(handle, TTM_GETTITLE, 0, tt_ptr as LPARAM);

        println!("{:?}", tt.uTitleBitmap);

        match tt.uTitleBitmap as usize {
            TTI_NONE => Some(TooltipIcon::None),
            TTI_INFO => Some(TooltipIcon::Info),
            TTI_WARNING => Some(TooltipIcon::Warning),
            TTI_ERROR => Some(TooltipIcon::Error),
            TTI_INFO_LARGE => Some(TooltipIcon::InfoLarge),
            TTI_WARNING_LARGE => Some(TooltipIcon::WarningLarge),
            TTI_ERROR_LARGE => Some(TooltipIcon::ErrorLarge),
            _ => None
        }
    }
    */

    /// Return the current text of the tooltip. There is no way to know the size of the text so you have
    /// to pass the buffer size. The default buffer size is 200 characters.
    pub fn text(&self, owner: &ControlHandle, buffer_size: Option<usize>) -> String {
        use winapi::um::commctrl::{TTM_GETTEXTW, TTTOOLINFOW, TTF_IDISHWND, TTF_SUBCLASS};
        use winapi::shared::{basetsd::UINT_PTR, windef::RECT};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let owner_handle = {
            if owner.blank() { panic!("{}", NOT_BOUND); }
            owner.hwnd().expect(BAD_HANDLE)
        };

        let buffer_size = buffer_size.unwrap_or(200);
        let mut text: Vec<WCHAR> = Vec::with_capacity(buffer_size);
        unsafe { text.set_len(buffer_size); }

        let mut tool = TTTOOLINFOW {
            cbSize: mem::size_of::<TTTOOLINFOW>() as UINT,
            uFlags: TTF_IDISHWND | TTF_SUBCLASS,
            hwnd: owner_handle,
            uId: owner_handle as UINT_PTR,
            rect: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            hinst: ptr::null_mut(),
            lpszText: text.as_mut_ptr(),
            lParam: 0,
            lpReserved: ptr::null_mut()
        };

        let tool_ptr = &mut tool as *mut TTTOOLINFOW;
        wh::send_message(handle, TTM_GETTEXTW, 0, tool_ptr as LPARAM);

        from_utf16(&text)
    }

    /// Change the text of a previously registered control
    /// Use the `register` function is associate a control with this tooltip
    pub fn set_text<'a>(&self, owner: &ControlHandle, text: &'a str) {
        use winapi::um::commctrl::{TTM_UPDATETIPTEXTW, TTTOOLINFOW, TTF_IDISHWND, TTF_SUBCLASS};
        use winapi::shared::{basetsd::UINT_PTR, windef::RECT};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let mut text = to_utf16(text);
        let owner_handle = {
            if owner.blank() { panic!("{}", NOT_BOUND); }
            owner.hwnd().expect(BAD_HANDLE)
        };

        let tool = TTTOOLINFOW {
            cbSize: mem::size_of::<TTTOOLINFOW>() as UINT,
            uFlags: TTF_IDISHWND | TTF_SUBCLASS,
            hwnd: owner_handle,
            uId: owner_handle as UINT_PTR,
            rect: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            hinst: ptr::null_mut(),
            lpszText: text.as_mut_ptr(),
            lParam: 0,
            lpReserved: ptr::null_mut()
        };

        let tool_ptr = &tool as *const TTTOOLINFOW;
        wh::send_message(handle, TTM_UPDATETIPTEXTW, 0, tool_ptr as LPARAM);
    }

    /// Set the icon and the title of a tooltip. This method use custom icon defined by user resources
    pub fn set_decoration<'a>(&self, title: &'a str, ico: &Icon) {
        use winapi::um::commctrl::{TTM_SETTITLEW};
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let title = to_utf16(title);
        wh::send_message(handle, TTM_SETTITLEW, ico.handle as WPARAM, title.as_ptr() as LPARAM);
    }

    /// Set the icon and the title of a tooltip. This method use built-in icon defined by TooltipIcon
    pub fn set_default_decoration<'a>(&self, title: &'a str, icon: TooltipIcon) {
        use winapi::um::commctrl::{TTM_SETTITLEW};
        use winapi::um::commctrl::{TTI_NONE, TTI_INFO, TTI_WARNING, TTI_ERROR, TTI_INFO_LARGE, TTI_WARNING_LARGE, TTI_ERROR_LARGE};
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let bitmap_handle = match icon {
            TooltipIcon::None => TTI_NONE,
            TooltipIcon::Info => TTI_INFO,
            TooltipIcon::Warning => TTI_WARNING,
            TooltipIcon::Error => TTI_ERROR,
            TooltipIcon::InfoLarge => TTI_INFO_LARGE,
            TooltipIcon::WarningLarge => TTI_WARNING_LARGE,
            TooltipIcon::ErrorLarge => TTI_ERROR_LARGE
        };

        let title = to_utf16(title);

        wh::send_message(handle, TTM_SETTITLEW, bitmap_handle as WPARAM, title.as_ptr() as LPARAM);
    }

    /// Hide the tooltip popup
    pub fn hide(&self) {
        use winapi::um::commctrl::{TTM_POP};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TTM_POP, 0, 0);
    }

    /// Return the number of controls registered by the tooltip
    pub fn count(&self) -> usize {
        use winapi::um::commctrl::{TTM_GETTOOLCOUNT};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TTM_GETTOOLCOUNT, 0, 0) as usize
    }

    /// Set the delay time for the tooltip to spawn in milliseconds
    /// Set the value to `None` to reset the value to default
    pub fn set_delay_time(&self, delay: Option<u16>) {
        use winapi::um::commctrl::{TTDT_INITIAL, TTM_SETDELAYTIME};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let value = match delay {
            Some(d) => d & 0xFFFF,
            None => u16::max_value() & 0xFFFF,
        };

        wh::send_message(handle, TTM_SETDELAYTIME, TTDT_INITIAL as WPARAM, value as LPARAM);
    }

    /// Return the delay time of the tooltip in milliseconds
    pub fn delay_time(&self) -> u16 {
        use winapi::um::commctrl::{TTDT_INITIAL, TTM_GETDELAYTIME};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TTM_GETDELAYTIME, TTDT_INITIAL as WPARAM, 0) as u16
    }

    /// Enable or disable the control
    /// Windows does not support reading the enabled state of a tooltip btw.
    pub fn set_enabled(&self, v: bool) {
        use winapi::um::commctrl::TTM_ACTIVATE;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, TTM_ACTIVATE, v as WPARAM, 0);
    }

    /// Register the tooltip under a control.
    /// `owner` must be a window control.
    pub fn register<'a, W: Into<ControlHandle>>(&self, owner: W, text: &'a str) {
        use winapi::um::commctrl::{TTM_ADDTOOLW, TTTOOLINFOW, TTF_IDISHWND, TTF_SUBCLASS};
        use winapi::shared::{basetsd::UINT_PTR, windef::RECT};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let owner = owner.into();

        let mut text = to_utf16(text);
        let owner_handle = {
            if owner.blank() { panic!("{}", NOT_BOUND); }
            owner.hwnd().expect(BAD_HANDLE)
        };

        let tool = TTTOOLINFOW {
            cbSize: mem::size_of::<TTTOOLINFOW>() as UINT,
            uFlags: TTF_IDISHWND | TTF_SUBCLASS,
            hwnd: owner_handle,
            uId: owner_handle as UINT_PTR,
            rect: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            hinst: ptr::null_mut(),
            lpszText: text.as_mut_ptr(),
            lParam: 0,
            lpReserved: ptr::null_mut()
        };

        let tool_ptr = &tool as *const TTTOOLINFOW;
        wh::send_message(handle, TTM_ADDTOOLW, 0, tool_ptr as LPARAM);
    }

    /// Register the tooltip under a control.
    /// `owner` must be a window control.
    /// When the user trigger the tooltip, the application receives a `OnTooltipText` event
    pub fn register_callback<W: Into<ControlHandle>>(&self, owner: W) {
        use winapi::um::commctrl::{TTM_ADDTOOLW, TTTOOLINFOW, TTF_IDISHWND, TTF_SUBCLASS, LPSTR_TEXTCALLBACKW};
        use winapi::shared::{basetsd::UINT_PTR, windef::RECT};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let owner = owner.into();
        let owner_handle = {
            if owner.blank() { panic!("{}", NOT_BOUND); }
            owner.hwnd().expect(BAD_HANDLE)
        };

        let tool = TTTOOLINFOW {
            cbSize: mem::size_of::<TTTOOLINFOW>() as UINT,
            uFlags: TTF_IDISHWND | TTF_SUBCLASS,
            hwnd: owner_handle,
            uId: owner_handle as UINT_PTR,
            rect: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            hinst: ptr::null_mut(),
            lpszText: LPSTR_TEXTCALLBACKW,
            lParam: 0,
            lpReserved: ptr::null_mut()
        };

        let tool_ptr = &tool as *const TTTOOLINFOW;
        wh::send_message(handle, TTM_ADDTOOLW, 0, tool_ptr as LPARAM);
    }

    /// Remove the tooltip from a control
    pub fn unregister<W: Into<ControlHandle>>(&self, owner: W) {
        use winapi::um::commctrl::{TTM_DELTOOLW, TTTOOLINFOW, TTF_IDISHWND, TTF_SUBCLASS};
        use winapi::shared::{basetsd::UINT_PTR, windef::RECT};

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let owner = owner.into();

        let owner_handle = {
            if owner.blank() { panic!("{}", NOT_BOUND); }
            owner.hwnd().expect(BAD_HANDLE)
        };

        let tool = TTTOOLINFOW {
            cbSize: mem::size_of::<TTTOOLINFOW>() as UINT,
            uFlags: TTF_IDISHWND | TTF_SUBCLASS,
            hwnd: owner_handle,
            uId: owner_handle as UINT_PTR,
            rect: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            hinst: ptr::null_mut(),
            lpszText: ptr::null_mut(),
            lParam: 0,
            lpReserved: ptr::null_mut()
        };

        let tool_ptr = &tool as *const TTTOOLINFOW;
        wh::send_message(handle, TTM_DELTOOLW, 0, tool_ptr as LPARAM);   
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        winapi::um::commctrl::TOOLTIPS_CLASS
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        0
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_POPUP};
        use winapi::um::commctrl::{TTS_ALWAYSTIP, TTS_NOPREFIX};

        WS_POPUP | TTS_ALWAYSTIP | TTS_NOPREFIX
    }

}

impl Drop for Tooltip {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}
pub struct TooltipBuilder<'a> {
    title: Option<&'a str>,
    ico: Option<&'a Icon>,
    default_ico: Option<TooltipIcon>,
    register: Vec<(ControlHandle, &'a str)>,
    register_cb: Vec<ControlHandle>,
}

impl<'a> TooltipBuilder<'a> {

    pub fn register<W: Into<ControlHandle>>(mut self, widget: W, text: &'a str) -> TooltipBuilder<'a> {
        self.register.push((widget.into(), text));
        self
    }

    pub fn register_callback<W: Into<ControlHandle>>(mut self, widget: W) -> TooltipBuilder<'a> {
        self.register_cb.push(widget.into());
        self
    }

    pub fn decoration(mut self, title: Option<&'a str>, ico: Option<&'a Icon>) -> TooltipBuilder<'a> {
        self.title = title;
        self.ico = ico;
        self
    }

    pub fn default_decoration(mut self, title: Option<&'a str>, ico: Option<TooltipIcon>) -> TooltipBuilder<'a> {
        self.title = title;
        self.default_ico = ico;
        self
    }

    pub fn build(self, tooltip: &mut Tooltip) -> Result<(), NwgError> {
        *tooltip = Default::default();

        tooltip.handle = ControlBase::build_hwnd()
            .class_name(tooltip.class_name())
            .forced_flags(tooltip.forced_flags())
            .flags(tooltip.flags())
            .build()?;

        if self.title.is_some() || self.ico.is_some() || self.default_ico.is_some() {
            let title = self.title.unwrap_or("");
            match (self.ico, self.default_ico) {
                (Some(ico), None) | (Some(ico), _) => tooltip.set_decoration(title, ico),
                (None, Some(ico)) => tooltip.set_default_decoration(title, ico),
                (None, None) => tooltip.set_default_decoration(title, TooltipIcon::None),
            }
        }
        
        for (handle, text) in self.register {
            tooltip.register(&handle, text);
        }

        for handle in self.register_cb {
            tooltip.register_callback(&handle);
        }

        Ok(())
    }

}
