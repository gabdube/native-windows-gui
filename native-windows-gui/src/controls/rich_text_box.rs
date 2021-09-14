use winapi::shared::minwindef::{WPARAM, LPARAM};
use winapi::um::winuser::{ES_AUTOVSCROLL, ES_AUTOHSCROLL, WS_VISIBLE, WS_DISABLED, WS_TABSTOP, WS_VSCROLL, WS_HSCROLL};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::check_hwnd;
use crate::win32::richedit as rich;
use crate::{Font, NwgError};
use super::{ControlBase, ControlHandle};
use std::ops::Range;
use newline_converter::{unix2dos, dos2unix};

const NOT_BOUND: &'static str = "RichTextBox is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: RichTextBox handle is not HWND!";


const ES_SAVESEL: u32 = 32768;

bitflags! {
    /**
        The rich text box flags

        * VSCROLL:  The text box has a vertical scrollbar
        * HSCROLL:  The text box has a horizontal scrollbar
        * AUTOVSCROLL:  The rich text box has a vertical scrollbar
        * AUTOHSCROLL:  The rich text box has a horizontal scrollbar
        * VISIBLE:  The rich text box is immediatly visible after creation
        * DISABLED: The rich text box cannot be interacted with by the user. It also has a grayed out look.
        * TAB_STOP: The rich text box can be selected using tab navigation
        * SAVE_SELECTION: Keep the selected text when the control lose focus
    */
    pub struct RichTextBoxFlags: u32 {
        const VSCROLL = WS_VSCROLL;
        const HSCROLL = WS_HSCROLL;
        const AUTOVSCROLL = ES_AUTOVSCROLL;
        const AUTOHSCROLL = ES_AUTOHSCROLL;
        const VISIBLE = WS_VISIBLE;
        const DISABLED = WS_DISABLED;
        const TAB_STOP = WS_TABSTOP;
        const SAVE_SELECTION = ES_SAVESEL;
    }
}

bitflags! {
    /**
        The effets that can be applied to the text of a rich edit control

        * BOLD:      Characters are bold.
        * ITALIC:    Characters are italic. 
        * STRIKEOUT: Characters are struck. 
        * UNDERLINE: Characters are underlined. 
        * AUTOCOLOR: Characters use the default system color
    */
    pub struct CharEffects: u32 {
        const BOLD = 0x0001;
        const ITALIC = 0x0002;
        const UNDERLINE = 0x0004;
        const STRIKEOUT = 0x0008;
        const AUTOCOLOR = 0x40000000;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum UnderlineType {
    None,
    Solid,
    Dash,
    DashDot,
    DashDotDot,
    Dotted,
    DoubleSolid,
    Wave,
}

/// Contains information about character formatting in a rich edit control
#[derive(Clone, Debug, Default)]
pub struct CharFormat {
    /// Character effects (bold, italics, strikeout, etc)
    ///
    /// When returned by `char_format`, specifies which attributes are consistent throughout the entire selection. 
    /// For example, if the entire selection is either in italics or not in italics.
    pub effects: Option<CharEffects>,

    /// Character height, in twips (1/1440 of an inch or 1/20 of a printer's point).
    pub height: Option<i32>,

    /// Character offset, in twips, from the baseline. If the value of this member is positive, the character is a superscript; if it is negative, the character is a subscript.
    pub y_offset: Option<i32>,

    /// Text color. This member is ignored if the AUTOCOLOR character effect is specified. 
    pub text_color: Option<[u8; 3]>,

    /// The font family name
    pub font_face_name: Option<String>,

    /// Text underline type. Does not work with effects
    pub underline_type: Option<UnderlineType>,
}


#[derive(Copy, Clone, Debug)]
/// Options used for bulleted or numbered paragraphs. 
pub enum ParaNumbering {
    /// No paragraph numbering or bullets. 
    None,

    /// Insert a bullet at the beginning of each selected paragraph. 
    Bullet,

    /// Use Arabic numbers (0, 1, 2, and so on). 
    Arabic,

    /// Use lowercase letters (a, b, c, and so on). 
    LcLetter,

    /// Use lowercase Roman letters (i, ii, iii, and so on). 
    LcRoman,

    /// Use uppercase letters (A, B, C, and so on). 
    UcLetter,

    /// Use uppercase Roman letters (I, II, III, and so on). 
    UcRoman,

    /// Uses a sequence of characters beginning with the Unicode character specified
    Seq(char)
}


#[derive(Copy, Clone, Debug)]
/// Numbering style used with the numbering paragraphs. Used with `ParamNumbering`
pub enum ParaNumberingStyle {
    /// Follows the number with a right parenthesis
    Paren,
    /// Encloses the number in parentheses
    Parens,
    /// Follows the number with a period
    Period,
    /// Display only the number
    Plain,
    /// Continues a numbered lsit without applying the next number of bullet
    NoNumber,
    /// Starts a new number using the value of `ParaNumbering::Seq(char)`
    NewNumber
}

#[derive(Copy, Clone, Debug)]
/// Paragraph alignment
pub enum ParaAlignment {
    /// Paragraphs are aligned with the left margin. 
    Left,
    /// Paragraphs are aligned with the right margin. 
    Right,
    /// Paragraphs are centered. 
    Center,
    /// Paragraphs are justified.
    Justify,
    /// Paragraphs are justified by expanding the blanks alone. 
    FullInterword
}

/// Type of line spacing
#[derive(Copy, Clone, Debug)]
pub enum ParaLineSpacing {
    /// Single spacing. 
    Single,
    
    /// One-and-a-half spacing.
    OneAndHalf,

    /// Double spacing. 
    Double,

    /// Value in twips (twentieth of a point). If the value specifies a value that is less than single spacing, the control displays single-spaced text
    SingleOr(i32),

    /// Value in twips (twentieth of a point). The control uses the exact spacing specified, even if dyLineSpacing specifies a value that is less than single spacing. 
    Exact(i32),

    /// The value of `value` / 20 is the spacing, in lines, from one line to the next. 20 produces single-spaced text, 40 is double spaced, 60 is triple spaced, and so on. 
    Exact20(i32)
}

/// Contains information about paragraph formatting in a rich edit control
#[derive(Clone, Debug, Default)]
pub struct ParaFormat {
    /// Options used for bulleted or numbered paragraphs.
    pub numbering: Option<ParaNumbering>,

    /// Numbering style used with numbered paragraphs. 
    pub numbering_style: Option<ParaNumberingStyle>,

    /// Minimum space between a paragraph number and the paragraph text, in twips (twentieth of a point). 
    pub numbering_tab: Option<u16>,

    /// Paragraph alignment
    pub alignment: Option<ParaAlignment>,

    /// Size of the spacing above the paragraph, in twips (twentieth of a point).
    pub space_before: Option<i32>,

    /// Specifies the size of the spacing below the paragraph, in twips (twentieth of a point).
    pub space_after: Option<i32>,

    /// Indentation of the paragraph's first line, in twips (twentieth of a point). The indentation of subsequent lines depends on the `offset` member
    pub start_indent: Option<i32>,

    /// Indentation of the right side of the paragraph, relative to the right margin, in twips (twentieth of a point).
    pub right_indent: Option<i32>,

    /// Indentation of the second and subsequent lines, **relative** to the indentation of the first line, in twips. The first line is indented if this member is negative or outdented if this member is positive.
    pub offset: Option<i32>,

    /// Line spacing. For a description of how this value is interpreted, see `ParaLineSpacing`
    pub line_spacing: Option<ParaLineSpacing>,

    /// Displays text using right-to-left (or left-to-right if set to false)
    pub rtl: Option<bool>,
}

/**
An edit control is a rectangular control window to permit the user to enter and edit text by typing on the keyboard
This control allow multi line input. For a single line of text, use `TextInput`.

A rich text box is almost a superset of the normal textbox. Unlike text box, rich text box has more features and can support Component Object Model (COM) objects.

The rich text box control supports the following rich text features:

* Colored text
* Multiple fonts
* Styled text such as bold, underscore, strikeout, etc
* Bullet point list
* Paragraph with custom indent/offset
* Custom line spacing


See: https://docs.microsoft.com/en-us/windows/win32/controls/about-rich-edit-controls#rich-edit-version-41

Note: Use `\r\n` to input a new line not just `\n`.

**Builder parameters:**
  * `parent`:   **Required.** The text box parent container.
  * `text`:     The text box text.
  * `size`:     The text box size.
  * `position`: The text box position.
  * `flags`:    A combination of the TextBoxFlags values.
  * `ex_flags`: A combination of win32 window extended flags. Unlike `flags`, ex_flags must be used straight from winapi
  * `font`:     The font used for the text box text
  * `limit`:    The maximum number of character that can be inserted in the control
  * `readonly`: If the textbox should allow user input or not
  * `focus`:    The control receive focus after being created

**Control events:**
  * `OnMouseMove`:   Generic mouse mouse event
  * `OnMouseWheel`:  Generic mouse wheel event
  * `MousePress(_)`: Generic mouse press events on the button
  * `OnKeyPress`:    Generic key press event
  * `OnKeyRelease`:  Generic key release event
  * `OnChar`:        Generic key event. Returns a `char` instead of a virtual key code
*/
#[derive(Default, PartialEq, Eq)]
pub struct RichTextBox {
    pub handle: ControlHandle
}

impl RichTextBox {

    pub fn builder<'a>() -> RichTextBoxBuilder<'a> {
        RichTextBoxBuilder {
            text: "",
            size: (100, 25),
            position: (0, 0),
            flags: None,
            ex_flags: 0,
            limit: 0,
            readonly: false,
            focus: false,
            font: None,
            parent: None
        }
    }

    /// Sets the background color for a rich edit control.
    /// You cannot get the background color of a rich text box
    pub fn set_background_color(&self, color: [u8; 3]) {
        use winapi::um::wingdi::RGB;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        let color = RGB(color[0], color[1], color[2]);
        wh::send_message(handle, rich::EM_SETBKGNDCOLOR, 0, color as _);
    }

    /// Sets the character format of the currently selected text
    pub fn set_char_format(&self, fmt: &CharFormat) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        rich::set_char_format(handle, fmt);
    }

    /// Returns the character format of the current selected text
    pub fn char_format(&self) -> CharFormat {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        rich::char_format(handle)
    }

    /// Sets the paragraph formatting for the current selection in a rich edit control
    pub fn set_para_format(&self, fmt: &ParaFormat) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        rich::set_para_format(handle, fmt)
    }

    /// Returns the paragraph formatting for the current selection in a rich edit control
    /// If more than one paragraph is selected, receive the attributes of the first paragraph
    pub fn para_format(&self) -> ParaFormat {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        rich::para_format(handle)
    }

    /// Set the font of the control
    /// It is not possible to get the base font handle of a rich label. Use `char_format` instead.
    pub fn set_font(&self, font: Option<&Font>) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return the number of maximum character allowed in this text input
    pub fn limit(&self) -> u32 {
        use winapi::um::winuser::EM_GETLIMITTEXT;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_GETLIMITTEXT as u32, 0, 0) as u32
    }

    /// Set the number of maximum character allowed in this text input
    pub fn set_limit(&self, limit: usize) {
        use winapi::um::winuser::EM_SETLIMITTEXT;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_SETLIMITTEXT as u32, limit, 0);
    }

    /// Check if the content of the text input was modified after it's creation
    pub fn modified(&self) -> bool {
        use winapi::um::winuser::EM_GETMODIFY;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_GETMODIFY as u32, 0, 0) != 0
    }

    /// Manually set modified flag of the text input
    pub fn set_modified(&self, e: bool) {
        use winapi::um::winuser::EM_SETMODIFY;
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_SETMODIFY as u32, e as usize, 0);
    }

    /// Undo the last action by the user in the control
    pub fn undo(&self) {
        use winapi::um::winuser::EM_UNDO;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_UNDO as u32, 0, 0);
    }

    /// Return the selected range of characters by the user in the text input
    pub fn selection(&self) -> Range<u32> {
        use winapi::um::winuser::EM_GETSEL;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);

        let (mut out1, mut out2) = (0u32, 0u32);
        let (ptr1, ptr2) = (&mut out1 as *mut u32, &mut out2 as *mut u32);
        wh::send_message(handle, EM_GETSEL as u32, ptr1 as WPARAM, ptr2 as LPARAM);

        Range { start: out1 as u32, end: out2 as u32 }
    }

    /// Return the selected range of characters by the user in the text input
    pub fn set_selection(&self, r: Range<u32>) {
        use winapi::um::winuser::EM_SETSEL;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_SETSEL as u32, r.start as usize, r.end as isize);
    }

    /// Return the length of the user input in the control. Performs a newline conversion first since
    /// Windows treats "\r\n" as a single character
    pub fn len(&self) -> u32 {
        use std::convert::TryInto;

        dos2unix(&self.text()).chars().count().try_into().unwrap_or_default()
    }
    
    /// Return the number of lines in the multiline edit control.
    /// If the control has no text, the return value is 1.
    pub fn linecount(&self) -> i32 {
        use winapi::um::winuser::EM_GETLINECOUNT;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_GETLINECOUNT as u32, 0, 0) as i32
    }  
    
    /// Scroll `v` lines in the multiline edit control.
    pub fn scroll(&self, v: i32) {
        use winapi::um::winuser::EM_LINESCROLL;
        
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_LINESCROLL as u32, 0, v as LPARAM);
    }
    
    /// Get the linecount and then scroll the text to the last line
    pub fn scroll_lastline(&self) {
        let lines = self.linecount();
        self.scroll(lines * -1);
        self.scroll(lines - 2);
    }

    /// Return true if the TextInput value cannot be edited. Retrurn false otherwise.
    /// A user can still copy text from a readonly TextEdit (unlike disabled)
    pub fn readonly(&self) -> bool {
        use winapi::um::winuser::ES_READONLY;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::get_style(handle) & ES_READONLY == ES_READONLY
    }

    /// Set the readonly flag of the text input
    /// A user can still copy text from a readonly TextEdit (unlike disabled)
    pub fn set_readonly(&self, r: bool) {
        use winapi::um::winuser::EM_SETREADONLY;

        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        wh::send_message(handle, EM_SETREADONLY as u32, r as WPARAM, 0);
    }

    /// Remove all text from the textbox
    pub fn clear(&self) {
        self.set_text("");
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button
    pub fn set_focus(&self) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Return the text displayed in the TextInput
    pub fn text(&self) -> String { 
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::get_window_text(handle) }
    }

    /// Set the text displayed in the TextInput
    pub fn set_text<'a>(&self, v: &'a str) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_text(handle, v) }
    }

    /// Set the text in the current control, converting unix-style newlines in the input to "\r\n"
    pub fn set_text_unix2dos<'a>(&self, v: &'a str) {
        let handle = check_hwnd(&self.handle, NOT_BOUND, BAD_HANDLE);
        unsafe { wh::set_window_text(handle,  &unix2dos(&v).to_string()) }
    }

    /// Append text to the current control
    pub fn append<'a>(&self, v: &'a str) {
        let text = self.text() + &unix2dos(&v).to_string();
        self.set_text(&text);
        self.scroll_lastline();
    }

    /// Append text to the current control followed by "\r\n"
    pub fn appendln<'a>(&self, v: &'a str) {
        let text = self.text() + &unix2dos(&v).to_string() + "\r\n";
        self.set_text(&text);
        self.scroll_lastline();
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> &'static str {
        "RICHEDIT50W"
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        WS_VISIBLE | ES_AUTOVSCROLL | ES_AUTOHSCROLL | WS_TABSTOP | WS_VSCROLL | WS_HSCROLL | ES_SAVESEL
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_BORDER, WS_CHILD, ES_MULTILINE, ES_WANTRETURN};
        
        WS_BORDER | WS_CHILD | ES_MULTILINE | ES_WANTRETURN
    }

}

impl Drop for RichTextBox {
    fn drop(&mut self) {
        self.handle.destroy();
    }
}
pub struct RichTextBoxBuilder<'a> {
    text: &'a str,
    size: (i32, i32),
    position: (i32, i32),
    flags: Option<RichTextBoxFlags>,
    ex_flags: u32,
    limit: usize,
    readonly: bool,
    focus: bool,
    font: Option<&'a Font>,
    parent: Option<ControlHandle>
}

impl<'a> RichTextBoxBuilder<'a> {

    pub fn flags(mut self, flags: RichTextBoxFlags) -> RichTextBoxBuilder<'a> {
        self.flags = Some(flags);
        self
    }

    pub fn ex_flags(mut self, flags: u32) -> RichTextBoxBuilder<'a> {
        self.ex_flags = flags;
        self
    }

    pub fn text(mut self, text: &'a str) -> RichTextBoxBuilder<'a> {
        self.text = text;
        self
    }

    pub fn size(mut self, size: (i32, i32)) -> RichTextBoxBuilder<'a> {
        self.size = size;
        self
    }

    pub fn position(mut self, pos: (i32, i32)) -> RichTextBoxBuilder<'a> {
        self.position = pos;
        self
    }

    pub fn limit(mut self, limit: usize) -> RichTextBoxBuilder<'a> {
        self.limit = limit;
        self
    }

    pub fn readonly(mut self, read: bool) -> RichTextBoxBuilder<'a> {
        self.readonly = read;
        self
    }

    pub fn font(mut self, font: Option<&'a Font>) -> RichTextBoxBuilder<'a> {
        self.font = font;
        self
    }

    pub fn focus(mut self, focus: bool) -> RichTextBoxBuilder<'a> {
        self.focus = focus;
        self
    }

    pub fn parent<C: Into<ControlHandle>>(mut self, p: C) -> RichTextBoxBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut RichTextBox) -> Result<(), NwgError> {
        let flags = self.flags.map(|f| f.bits()).unwrap_or(out.flags());

        let parent = match self.parent {
            Some(p) => Ok(p),
            None => Err(NwgError::no_parent("RichTextBox"))
        }?;

        *out = Default::default();

        out.handle = ControlBase::build_hwnd()
            .class_name(out.class_name())
            .forced_flags(out.forced_flags())
            .flags(flags)
            .ex_flags(self.ex_flags)
            .size(self.size)
            .position(self.position)
            .text(self.text)
            .parent(Some(parent))
            .build()?;

        if self.limit > 0 {
            out.set_limit(self.limit);
        }

        if self.readonly {
            out.set_readonly(self.readonly);
        }

        if self.font.is_some() {
            out.set_font(self.font);
        } else {
            out.set_font(Font::global_default().as_ref());
        }

        if self.focus {
            out.set_focus();
        }

        Ok(())
    }

}
