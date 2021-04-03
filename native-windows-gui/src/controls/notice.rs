use super::control_handle::ControlHandle;
use crate::win32::{window_helper as wh, window::build_notice};
use crate::NwgError;


const NOT_BOUND: &'static str = "Notice is not yet bound to a winapi object";
const UNUSABLE_NOTICE: &'static str = "Notice parent window was freed";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: Notice handle is not Notice!";

/**
An invisible component that can be triggered by other thread.

A notice object does not send data between threads. Rust has already plenty of way to do this.
The notice object only serve to "wake up" the GUI thread.

A notice must have a parent window. If the parent is destroyed before the notice, the notice becomes invalid.

Requires the `notice` feature. 

## Example

```rust
use native_windows_gui as nwg;
fn build_notice(notice: &mut nwg::Notice, window: &nwg::Window) {
    nwg::Notice::builder()
        .parent(window)
        .build(notice);
}
```

```rust
use native_windows_gui as nwg;
use std::thread;
use std::time;

fn notice(noticer: &nwg::Notice) {
    let sender = noticer.sender();
    
    thread::spawn(move || {
        thread::sleep(time::Duration::new(5, 0));
        sender.notice();
    });
}

```

*/
#[derive(Default, PartialEq, Eq)]
pub struct Notice {
    pub handle: ControlHandle
}

impl Notice {

    pub fn builder() -> NoticeBuilder {
        NoticeBuilder {
            parent: None
        }
    }

    /// A shortcut over the builder API for the notice object
    pub fn create<C: Into<ControlHandle>>(parent: C) -> Result<Notice, NwgError> {
        let mut notice = Self::default();
        Self::builder()
            .parent(parent)
            .build(&mut notice)?;

        Ok(notice)
    }

    /// Checks if the notice is still usable. A notice becomes unusable when the parent window is destroyed.
    /// This will also return false if the notice is not initialized.
    pub fn valid(&self) -> bool {
        if self.handle.blank() { return false; }
        let (hwnd, _) = self.handle.notice().expect(BAD_HANDLE);
        wh::window_valid(hwnd)
    } 

    /// Return an handle to the notice window or `None` if the window was destroyed.
    pub fn window_handle(&self) -> Option<ControlHandle> {
        match self.valid() {
            true => Some(ControlHandle::Hwnd(self.handle.notice().unwrap().0)),
            false => None
        }
    }

    /// Change the parent window of the notice. This won't update the NoticeSender already created.
    /// Panics if the control is not a window-like control or if the notice was not initialized
    pub fn set_window_handle<C: Into<ControlHandle>>(&mut self, window: C) {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }

        let hwnd = window.into().hwnd().expect("New notice parent is not a window control");
        let (_, id) = self.handle.notice().expect(BAD_HANDLE);

        self.handle = ControlHandle::Notice(hwnd, id);
    }

    /// Create a new `NoticeSender` bound to this Notice
    pub fn sender(&self) -> NoticeSender {
        if self.handle.blank() { panic!("{}", NOT_BOUND); }
        if !self.valid() { panic!("{}", UNUSABLE_NOTICE); }
        let (hwnd, id) = self.handle.notice().expect(BAD_HANDLE);

        NoticeSender { 
            hwnd: hwnd as usize,
            id,
        }
    }

}

impl Drop for Notice {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

/// NoticeSender sends message to its parent `Notice` from another thread
#[derive(Clone, Copy)]
pub struct NoticeSender {
    hwnd: usize,
    id: u32,
}


impl NoticeSender {

    /// Send a message to the thread of the parent `Notice` 
    pub fn notice(&self) {
        use winapi::um::winuser::SendNotifyMessageW;
        use winapi::shared::minwindef::{WPARAM, LPARAM};
        use winapi::shared::windef::HWND;

        unsafe {
            SendNotifyMessageW(self.hwnd as HWND, wh::NOTICE_MESSAGE, self.id as WPARAM, self.hwnd as LPARAM);
        }
    }

}


pub struct NoticeBuilder {
    parent: Option<ControlHandle>
}

impl NoticeBuilder {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> NoticeBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut Notice) -> Result<(), NwgError> {
        let parent = match self.parent {
            Some(p) => match p.hwnd() {
                Some(handle) => Ok(handle),
                None => Err(NwgError::control_create("Wrong parent type"))
            },
            None => Err(NwgError::no_parent("Notice"))
        }?;

        out.handle = build_notice(parent);
        
        Ok(())
    }

}
