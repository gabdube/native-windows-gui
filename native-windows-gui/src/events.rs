//! All the events that can be dispatched by the built-in controls of native-windows-gui


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MousePressEvent {
    MousePressLeftUp,
    MousePressLeftDown,
    MousePressRightUp,
    MousePressRightDown
}

/// Events are identifier that are sent by controls on user interaction
/// Some events also have data that can be further processed by the event loop. See `EventData`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Event {
    /// Undefined / not implemented event
    Unknown,

    /// Generic mouse press events that can be sent to most window controls
    MousePress(MousePressEvent),

    /// Generic mouse move event that can be sent to most window controls
    OnMouseMove,

    /// Generic window event when the user right click a window
    OnContextMenu,

    /// When a top level window control is created.
    OnInit,

    /// When a control needs to be redrawn
    OnPaint,
    
    /// When a control is resized by the user. 
    /// This is typically applied to top level windows but it also applies to children when layouts are used.
    OnResize,

    /// When a control is about to be resized by the user. 
    OnResizeBegin,

    /// When a control stops being resized
    OnResizeEnd,

    /// When a control is moved by the user. This is typically applied to top level windows.
    /// This is typically applied to top level windows but it also applies to children when layouts are used.
    OnMove,

    /// When a bar like control value is changed.
    OnVerticalScroll,

    /// When a bar like control value is changed.
    OnHorizontalScroll,

    /// When a button is clicked. Similar to a MouseUp event, but only for button control
    OnButtonClick,

    /// When a button is clicked twice rapidly
    OnButtonDoubleClick,

    /// When a label is clicked
    OnLabelClick,

    /// When a label is clicked twice rapidly
    OnLabelDoubleClick,

    /// When a ImageFrame is clicked
    OnImageFrameClick,

    /// When a ImageFrame is clicked twice rapidly
    OnImageFrameDoubleClick,

    /// When a TextInput value is changed
    OnTextInput,

    /// When the list of a combobox is closed
    OnComboBoxClosed,

    /// When the list of a combobox is about to be visible
    OnComboBoxDropdown,

    /// When the current selection of the combobox was changed
    OnComboxBoxSelection,

    /// When the date select dropdown is expanded
    OnDatePickerDropdown,

    /// When the date select dropdown is closed
    OnDatePickerClosed,

    /// When the value of the date select is changed
    OnDatePickerChanged,

    /// When an item on a list box is clicked twice
    OnListBoxDoubleClick,

    /// When an item on a list box is selected
    OnListBoxSelect,

    /// The select tab of a TabsContainer changed
    TabsContainerChanged,

    /// The select tab of a TabsContainer is about to be changed
    TabsContainerChanging,

    /// When the trackbar thumb is released by the user
    TrackBarUpdated,

    /// When a menu control is opened
    OnMenuOpen,

    /// When a menu is hovered (either through mouse or keyboard)
    OnMenuHover,

    /// When the user selects on a menu item
    OnMenuItemSelected,

    /// When the user hovers over a callback tooltip
    /// The callback will also receive a `EventData::OnTooltipText`
    OnTooltipText,

    /// When a TrayNotification info popup (not the tooltip) is shown 
    OnTrayNotificationShow,

    /// When a TrayNotification info popup (not the tooltip) is hidden 
    OnTrayNotificationHide,

    /// When a TrayNotification is closed due to a timeout
    OnTrayNotificationTimeout,

    /// When a TrayNotification is closed due to a user click
    OnTrayNotificationUserClose,

    /// When a timer delay is elapsed
    OnTimerTick,

    /// When a notice is... noticed
    OnNotice,

    /// When a user click on the X button of a window
    OnWindowClose,

    /// When most control receive keyboard focus
    OnFocus,

    /// When most control lose keyboard focus
    OnFocusLost
}


/// Events data sent by the controls. 
#[derive(Debug)]
pub enum EventData {
    /// The event has no data
    NoData,

    /// Sets the text of a tooltip.
    /// The method `on_tooltip_text` should be used to access the inner data
    OnTooltipText(ToolTipTextData)
}

impl EventData {

    pub fn on_tooltip_text(&self) -> &ToolTipTextData {
        match self {
            EventData::OnTooltipText(d) => d,
            d => panic!("Wrong data type: {:?}", d)
        }
    }

}

//
// Events data structures
//

use winapi::um::commctrl::NMTTDISPINFOW;
use std::fmt;

/// A wrapper structure that set the tooltip text on a `OnTooltipText` callback
pub struct ToolTipTextData {
    pub(crate) data: *mut NMTTDISPINFOW
}

impl ToolTipTextData {

    /// Tells the application to save the text value of the callback
    /// The `OnTooltipText` will not be called a second time for the associated control
    pub fn keep(&self, keep: bool) {
        use ::winapi::um::commctrl::TTF_DI_SETITEM;
        
        let data = unsafe { &mut *self.data };
        match keep {
            true => { data.uFlags |= TTF_DI_SETITEM; },
            false => { data.uFlags &= !TTF_DI_SETITEM; }
        }
    }

    /// Sets the text of the callback. This function will copy the text.
    /// WINAPI do not easily allow tooltip with more than 79 characters (80 with NULL)
    /// With a text > 79 characters, this method will do nothing.
    pub fn set_text<'b>(&self, text: &'b str) {
        use crate::win32::base_helper::to_utf16;
        use std::ptr;
        
        let text_len = text.len();
        if text_len > 79 {
            return;
        }

        self.clear();
        unsafe {
            let data = &mut *self.data;
            let local_text = to_utf16(text);
            ptr::copy_nonoverlapping(local_text.as_ptr(), data.szText.as_mut_ptr(), text_len);
        }
    }

    fn clear(&self) {
        use winapi::um::winnt::WCHAR;
        use std::{ptr, mem};
        
        unsafe {
            let data = &mut *self.data;
            ptr::write(&mut data.szText as *mut [WCHAR; 80], mem::zeroed());
        }
    }

}

impl fmt::Debug for ToolTipTextData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ToolTipTextData")
    }
}

