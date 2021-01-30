use super::control_handle::ControlHandle;
use crate::win32::{window_helper as wh, window::build_custom_event};
use crate::NwgError;

const NOT_BOUND: &'static str = "CustomEvent is not yet bound to a winapi object";
const UNUSABLE_EVENT: &'static str = "CustomEvent parent window was freed";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: CustomEvent handle is not CustomEvent!";

/**
A custom event control is a mechanism to create and trigger custom events in an application.

Internally, a CustomEvent simply post events to the parent window event queue.

A CustomEvent must have a parent window. That window will receive the custom event.
If the parent is destroyed before the event, the custom event becomes invalid.

If you need to send events between threads, see the [Notice] control.

If you need to send events at a regular interval, see the [Timer] control.

Requires the `custom_event` feature. 

**Builder parameters:**
  * `parent`:   **Required.** The event parent.

**Control events:**
  * `OnCustomEvent`: When the event was triggered

*/
#[derive(Default, PartialEq, Eq, Clone)]
pub struct CustomEvent {
    pub handle: ControlHandle
}

impl CustomEvent {

    pub fn builder() -> CustomEventBuilder {
        CustomEventBuilder {
            parent: None
        }
    }

    /// A shortcut over the builder API for the custom event object
    pub fn create<C: Into<ControlHandle>>(parent: C) -> Result<CustomEvent, NwgError> {
        let mut evt = Self::default();
        Self::builder()
            .parent(parent)
            .build(&mut evt)?;

        Ok(evt)
    }

    /// Checks if the custom event is still usable. A custom event becomes unusable when the parent window is destroyed.
    /// This will also return false if the custom event is not initialized.
    pub fn valid(&self) -> bool {
        if self.handle.blank() { return false; }
        let (hwnd, _) = self.handle.custom_event().expect(BAD_HANDLE);
        wh::window_valid(hwnd)
    } 

    /// Trigger the custom event
    /// Panics if the custom event window was destroyed
    pub fn trigger(&self) {
        use winapi::um::winuser::PostMessageW;

        if self.handle.blank() { panic!(NOT_BOUND); }
        if !self.valid() { panic!(UNUSABLE_EVENT); }
        let (hwnd, id) = self.handle.custom_event().expect(BAD_HANDLE);

        unsafe {    
            PostMessageW(hwnd, wh::NWG_CUSTOM_EVENT, id as _, 0);
        }
    }

}

impl Drop for CustomEvent {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}

pub struct CustomEventBuilder {
    parent: Option<ControlHandle>
}

impl CustomEventBuilder {

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> CustomEventBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut CustomEvent) -> Result<(), NwgError> {
        let parent = match self.parent {
            Some(p) => match p.hwnd() {
                Some(handle) => Ok(handle),
                None => Err(NwgError::control_create("Wrong parent type"))
            },
            None => Err(NwgError::no_parent("Notice"))
        }?;

        out.handle = build_custom_event(parent);
        
        Ok(())
    }

}

