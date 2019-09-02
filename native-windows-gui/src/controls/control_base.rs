use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::{HWND, HMENU};
use super::ControlHandle;
use crate::win32::window::{build_hwnd_control, build_timer, build_notice};
use crate::win32::menu::build_hmenu_control;
use crate::SystemError;


#[derive(Debug, Clone)]
pub struct ControlBase {
    pub handle: ControlHandle
}

impl ControlBase {

    pub fn build_hwnd() -> HwndBuilder {
        HwndBuilder::default()
    }

    pub fn build_hmenu() -> HmenuBuilder {
        HmenuBuilder::default()
    }

    pub fn build_timer() -> TimerBuilder {
        TimerBuilder::default()
    }

    pub fn build_notice() -> NoticeBuilder {
        NoticeBuilder::default()
    }
}

#[derive(Default)]
pub struct HwndBuilder {
    class_name: Option<String>,
    text: Option<String>,
    size: Option<(i32, i32)>,
    pos: Option<(i32, i32)>,
    forced_flags: DWORD,
    flags: Option<DWORD>,
    ex_flags: Option<DWORD>,
    parent: Option<HWND>
}

impl HwndBuilder {

    pub fn class_name<'a>(mut self, name: Option<&'a str>) -> HwndBuilder {
        self.class_name = name.map(|v| v.to_string());
        self
    }

    pub fn text<'a>(mut self, text: &'a str) -> HwndBuilder {
        self.text = Some(text.to_string());
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> HwndBuilder {
        self.size = Some(size);
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> HwndBuilder {
        self.pos = Some(pos);
        self
    }

    pub fn flags(mut self, flags: (u32, u32)) -> HwndBuilder {
        let (flags, ex_flags) = flags;
        self.flags = Some(flags as DWORD);
        self.ex_flags = Some(ex_flags as DWORD);
        self
    }

    pub fn forced_flags(mut self, flags: u32) -> HwndBuilder {
        self.forced_flags = flags as DWORD;
        self
    }

    pub fn parent(mut self, parent: &ControlBase) -> HwndBuilder {
        self.parent = parent.handle.hwnd();
        self
    }

    pub fn build(self) -> Result<ControlBase, SystemError> {
        let handle = unsafe { build_hwnd_control(
            self.class_name.as_ref().map(|v| v as &str),
            self.text.as_ref().map(|v| v as &str),
            self.size,
            self.pos,
            self.flags,
            self.ex_flags,
            self.forced_flags,
            self.parent
        )? };

        Ok(ControlBase { handle })
    }
}


#[derive(Default)]
pub struct HmenuBuilder {
    text: Option<String>,
    item: bool,
    separator: bool,
    parent_menu: Option<HMENU>,
    parent_window: Option<HWND>,
}

impl HmenuBuilder {

    /// Set the text of the Menu
    pub fn text<'a>(mut self, text: &'a str) -> HmenuBuilder {
        self.text = Some(text.to_string());
        self
    }

    /// Set if the menu should be an item or a menu
    pub fn item(mut self, i: bool) -> HmenuBuilder {
        self.item = i;
        self
    }

    /// Set if the menu item should be a separator
    pub fn separator(mut self, i: bool) -> HmenuBuilder {
        self.separator = i;
        self
    }

    /// Set the parent of the menu. Can be a window or another menu.
    pub fn parent(mut self, parent: &ControlBase) -> HmenuBuilder {
        self.parent_window = parent.handle.hwnd();
        self.parent_menu = parent.handle.hmenu().as_ref().map(|(_, m)| *m);
        self
    }

    pub fn build(self) -> Result<ControlBase, SystemError> {
        let handle = unsafe { build_hmenu_control(
            self.text,
            self.item,
            self.separator,
            self.parent_menu,
            self.parent_window
        )? };

        Ok(ControlBase { handle })
    }

}


#[derive(Default)]
pub struct TimerBuilder {
    parent: Option<HWND>,
    interval: u32,
    stopped: bool
}

impl TimerBuilder {

    pub fn stopped(mut self, v: bool) -> TimerBuilder {
        self.stopped = v;
        self
    }

    pub fn interval(mut self, i: u32) -> TimerBuilder {
        self.interval = i;
        self
    }

    pub fn parent(mut self, parent: &ControlBase) -> TimerBuilder {
        self.parent = parent.handle.hwnd();
        self
    }

    pub fn build(self) -> Result<ControlBase, SystemError> {
        let handle = unsafe { build_timer(
            self.parent.expect("Internal error. Timer without window parent"),
            self.interval,
            self.stopped
        ) };
        Ok(ControlBase { handle })
    }

}


#[derive(Default)]
pub struct NoticeBuilder {
    parent: Option<HWND>
}

impl NoticeBuilder {

    pub fn parent(mut self, parent: &ControlBase) -> NoticeBuilder {
        self.parent = parent.handle.hwnd();
        self
    }

    pub fn build(self) -> Result<ControlBase, SystemError> {
        let handle = self.parent.expect("Internal error. Notice without window parent");
        Ok(ControlBase { handle: build_notice(handle) } )
    }

}
