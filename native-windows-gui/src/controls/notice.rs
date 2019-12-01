use super::control_handle::ControlHandle;
use crate::win32::window_helper as wh;

/// An invisible component that can be triggered by other thread
#[derive(Default)]
pub struct Notice {
    pub handle: ControlHandle
}

/// NoticeSender sends message to it's parent `Notice` from another thread
#[derive(Clone, Copy)]
pub struct NoticeSender {
    hwnd: usize,
    id: u32,
    thread_id: u32,
}


impl Notice {

    /// Create a new `NoticeSender` bound to this Notice
    pub fn sender(&self) -> NoticeSender {
        use winapi::um::processthreadsapi::GetCurrentThreadId;

        if self.handle.blank() { panic!("Notice is not yet bound to a winapi object"); }
        let (hwnd, id) = self.handle.notice().expect("INTERNAL ERROR: Notice handle has the wrong type!");

        NoticeSender { 
            hwnd: hwnd as usize,
            id,
            thread_id: unsafe { GetCurrentThreadId() },
        }
    }

}

impl NoticeSender {

    /// Send a message to the thread of the parent `Notice` 
    pub fn notice(&self) {
        unsafe { wh::send_notice(self.thread_id, self.hwnd, self.id) }
    }

}
