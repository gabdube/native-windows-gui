/*!
An invisible component that can be triggered by other thread.

A notice object do not send data between threads. Rust has already plenty of way to do this.
The notice object only serve to "wake up" the GUI thread.

A notice must have a parent window.

## Example
```
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

use super::control_handle::ControlHandle;
use crate::win32::{window_helper as wh, window::build_notice};
use crate::NwgError;


/// An invisible component that can be triggered by other thread
#[derive(Default, PartialEq, Eq)]
pub struct Notice {
    pub handle: ControlHandle
}

/// NoticeSender sends message to its parent `Notice` from another thread
#[derive(Clone, Copy)]
pub struct NoticeSender {
    hwnd: usize,
    id: u32,
}


impl Notice {

    pub fn builder() -> NoticeBuilder {
        NoticeBuilder {
            parent: None
        }
    }

    /// Create a new `NoticeSender` bound to this Notice
    pub fn sender(&self) -> NoticeSender {
        if self.handle.blank() { panic!("Notice is not yet bound to a winapi object"); }
        let (hwnd, id) = self.handle.notice().expect("INTERNAL ERROR: Notice handle has the wrong type!");

        NoticeSender { 
            hwnd: hwnd as usize,
            id,
        }
    }

}

impl NoticeSender {

    /// Send a message to the thread of the parent `Notice` 
    pub fn notice(&self) {
        use winapi::um::winuser::SendNotifyMessageW;
        use winapi::shared::minwindef::{WPARAM, LPARAM};
        use winapi::shared::windef::HWND;

        unsafe {
            SendNotifyMessageW(self.hwnd as HWND, wh::NOTICE_MESSAGE, self.id as WPARAM, self.hwnd as LPARAM);
            /*let res = PostThreadMessageW(self.thread_id, wh::NOTICE_MESSAGE, self.id as WPARAM, self.hwnd as LPARAM);
            println!("{:?}", res);*/
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
