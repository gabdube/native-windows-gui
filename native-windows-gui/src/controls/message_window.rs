/*!
    A message-only window enables you to send and receive messages. It is not visible, has no z-order, cannot be enumerated, and does not
    receive broadcast messages. The window simply dispatches messages.

    A MessageWindow do not have any builder parameter, but still provides the API for the derive macro.

    Requires the `message-window` feature. 

    ## Example
    ```
    use native_windows_gui as nwg;

    let mut window = Default::default();
    nwg::MessageWindow::builder().build(&mut window);
    ```

    When making a system-tray application (with TrayNotification), this is the recommended top level window type.
*/
use super::ControlHandle;
use crate::win32::window::create_message_window;
use crate::NwgError;

/**
    A message only top level window. At least one top level window is required to make a NWG application.
    See the module documentation
*/
#[derive(Default, PartialEq, Eq)]
pub struct MessageWindow {
    pub handle: ControlHandle
}

impl MessageWindow {

    pub fn builder() -> MessageWindowBuilder {
        MessageWindowBuilder {}
    }

}

impl Drop for MessageWindow {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}
pub struct MessageWindowBuilder {
}

impl MessageWindowBuilder {

    pub fn build(self, out: &mut MessageWindow) -> Result<(), NwgError> {
        *out = Default::default();
        out.handle = create_message_window()?;
        Ok(())
    }

}
