use winapi::um::winuser::{ES_AUTOVSCROLL, ES_AUTOHSCROLL, WS_VISIBLE, WS_DISABLED};
use crate::win32::window_helper as wh;
use crate::Font;
use super::ControlHandle;
use std::ops::Range;

const NOT_BOUND: &'static str = "TextBox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TextBox handle is not HWND!";


bitflags! {
    pub struct TextBoxFlags: u32 {
        const VSCROLL = ES_AUTOVSCROLL;
        const HSCROLL = ES_AUTOHSCROLL;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
    }
}


/// An edit control is a rectangular control window to permit the user to enter and edit text by typing on the keyboard
/// This control allow multi line input. For a single line of text, use `TextInput`.
///
/// Note: Use `\r\n` to input a new line not just `\n`.
#[derive(Default, Debug)]
pub struct TextBox {
    pub handle: ControlHandle
}

impl TextBox {

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return the number of maximum character allowed in this text input
    pub fn limit(&self) -> u32 {
        use winapi::um::winuser::EM_GETLIMITTEXT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_GETLIMITTEXT as u32, 0, 0) as u32
    }

    /// Set the number of maximum character allowed in this text input
    pub fn set_limit(&self, limit: usize) {
        use winapi::um::winuser::EM_SETLIMITTEXT;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_SETLIMITTEXT as u32, limit, 0);
    }

    /// Check if the content of the text input was modified after it's creation
    pub fn modified(&self) -> bool {
        use winapi::um::winuser::EM_GETMODIFY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_GETMODIFY as u32, 0, 0) != 0
    }

    /// Manually set modified flag of the text input
    pub fn set_modified(&self, e: bool) {
        use winapi::um::winuser::EM_SETMODIFY;
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_SETMODIFY as u32, e as usize, 0);
    }

    /// Undo the last action by the user in the control
    pub fn undo(&self) {
        use winapi::um::winuser::EM_UNDO;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_UNDO as u32, 0, 0);
    }

    /// Return the selected range of characters by the user in the text input
    pub fn selection(&self) -> Range<u32> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let (w, l) = unsafe { get_selection(handle) };

        Range { start: w as u32, end: l as u32 }
    }

    /// Return the selected range of characters by the user in the text input
    pub fn set_selection(&self, r: Range<u32>) {
        use winapi::um::winuser::EM_SETSEL;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::send_message(handle, EM_SETSEL as u32, r.start as usize, r.end as isize);
    }

    /// Return the length of the user input in the control. This is better than test.len() as it
    /// does not allocate a string in memory
    pub fn len(&self) -> u32 {
        use winapi::um::winuser::EM_LINELENGTH;
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        wh::send_message(handle, EM_LINELENGTH as u32, 0, 0) as u32
    }

    /// Return true if the TextInput value cannot be edited. Retrurn false otherwise.
    /// A user can still copy text from a readonly TextEdit (unlike disabled)
    pub fn readonly(&self) -> bool {
        use winapi::um::winuser::ES_READONLY;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::get_style(handle) & ES_READONLY == ES_READONLY
    }

    /// Set the readonly flag of the text input
    /// A user can still copy text from a readonly TextEdit (unlike disabled)
    pub fn set_readonly(&self, r: bool) {
        use winapi::um::winuser::EM_SETREADONLY;
        use winapi::shared::minwindef::WPARAM;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        wh::send_message(handle, EM_SETREADONLY as u32, r as WPARAM, 0);
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the text displayed in the TextInput
    pub fn text(&self) -> String { 
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the text displayed in the TextInput
    pub fn set_text<'a>(&self, v: &'a str) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        Some("EDIT")
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {;
        ::winapi::um::winuser::WS_VISIBLE
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_BORDER, WS_CHILD, ES_MULTILINE};
        
        WS_BORDER | WS_CHILD | ES_MULTILINE
    }

}

use winapi::shared::windef::HWND;

unsafe fn get_selection(hwnd: HWND) -> (u32, u32) {
    use winapi::um::winuser::EM_GETSEL;
    use winapi::um::winuser::SendMessageW;
    use std::mem;

    let (mut out1, mut out2) = (0u32, 0u32);
    SendMessageW(hwnd, EM_GETSEL as u32, mem::transmute(&mut out1), mem::transmute(&mut out2));

    (out1, out2) 
}
