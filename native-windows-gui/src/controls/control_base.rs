use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::HWND;
use super::ControlHandle;
use crate::win32::window::build_hwnd_control;
use crate::SystemError;


#[derive(Debug, Clone)]
pub struct ControlBase {
    pub handle: ControlHandle
}

impl ControlBase {

    pub fn build_hwnd() -> HwndBuilder {
        HwndBuilder::new()
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
    pub fn new() -> HwndBuilder {
        HwndBuilder::default()
    }

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
