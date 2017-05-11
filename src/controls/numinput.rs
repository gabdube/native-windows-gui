/*!
    Numeric input control definition
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use std::hash::Hash;
use std::any::TypeId;
use std::ptr;

use winapi::{HWND, HFONT, UINT, WPARAM, LPARAM, LRESULT, UINT_PTR, DWORD_PTR};

use ui::Ui;
use controls::{Control, ControlT, ControlType, AnyHandle};
use error::Error;
use events::Event;

/**
    An NumericInput control is a pair of arrow buttons next to a edit control.
    The numeric input edit only accepts numbers.

    Members:  
    • `value`: The default value of the control  
    • `position`: The start position of the numeric input  
    • `size`: The start size of the numeric input  
    • `visible`: If the numeric input should be visible to the user  
    • `disabled`: If the user can or can't execute any actions on the numeric input  
    • `readonly`: If the user can or can't edit the numeric input value 
    • `range`: The range of values accepted by the control
    • `parent`: The numeric input parent  
    • `font`: The numeric input font. If None, use the system default  
*/
pub struct NumericInputT<ID: Hash+Clone> {
    pub value: i64,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub visible: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub range: (i64, i64),
    pub parent: ID,
    pub font: Option<ID>,
}

impl<ID: Hash+Clone> ControlT<ID> for NumericInputT<ID> {
    fn type_id(&self) -> TypeId { TypeId::of::<NumericInput>() }

    fn events(&self) -> Vec<Event> {
        vec![Event::Destroyed, Event::Moved, Event::Resized]
    }

    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error> {
        use low::window_helper::{set_window_font, handle_of_window, handle_of_font};

        return Err(Error::Unimplemented);

        // A NumericInput is composed of three controls: a custom window, a textinput and a "up down".

        // Get the parent handle
        let parent = match handle_of_window(ui, &self.parent, "The parent of a numeric input must be a window-like control.") {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        // Get the font handle (if any)
        let font_handle: Option<HFONT> = match self.font.as_ref() {
            Some(font_id) => 
                match handle_of_font(ui, &font_id, "The font of a button must be a font resource.") {
                    Ok(h) => Some(h),
                    Err(e) => { return Err(e); }
                },
            None => None
        };

        let base_handle = match unsafe{ build_base(self, parent) } {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        let edit_handle =  match unsafe{ build_edit(self, base_handle) } {
            Ok(h) => h,
            Err(e) => { return Err(e); }
        };

        unsafe{ 
            hook(edit_handle);
            set_window_font(edit_handle, font_handle, true); 
        }

        Ok(Box::new(
            NumericInput {
                handle: base_handle,
                edit_handle: edit_handle,
            }
        ))
    }
}

pub struct NumericInput {
    handle: HWND,
    edit_handle: HWND,
}

impl Control for NumericInput {

    fn handle(&self) -> AnyHandle {
        AnyHandle::HWND(self.handle)
    }

    fn control_type(&self) -> ControlType { 
        ControlType::NumericInput 
    }

    fn free(&mut self) {
        use user32::{DestroyWindow, UnregisterClassW};
        use kernel32::GetModuleHandleW;
        use low::other_helper::to_utf16;
        
        unsafe{
            unhook(self.edit_handle);

            DestroyWindow(self.handle); 

            let cls = to_utf16(NUMERICINPUT_CLASS_NAME);
            let hmod = GetModuleHandleW(ptr::null_mut());
            UnregisterClassW(cls.as_ptr(), hmod);
        }
    }

}


// Private methods

const NUMERICINPUT_CLASS_NAME: &'static str = "NWG_BUILTIN_NUMERICINPUT";
const CUSTOM_EVENTS_DISPATCH_ID: UINT_PTR = 5674;

#[allow(unused_variables)]
unsafe extern "system" fn numeric_sysproc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::{WM_CREATE, WM_CLOSE};
    use user32::{DefWindowProcW, ShowWindow};

    let handled = match msg {
        WM_CREATE => true,
        WM_CLOSE => {
            ShowWindow(hwnd, 0);
            true
        }
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}

#[allow(unused_variables)]
unsafe extern "system" fn edit_hook(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT {
    use comctl32::DefSubclassProc;
    DefSubclassProc(hwnd, msg, w, l)
}

#[inline(always)]
unsafe fn build_base<ID: Hash+Clone>(t: &NumericInputT<ID>, parent: HWND) -> Result<HWND, Error> {
    use low::window_helper::{WindowParams, SysclassParams, build_sysclass, build_window};
    use winapi::{CS_HREDRAW, CS_VREDRAW, WS_VISIBLE, WS_CHILD, WS_BORDER, WS_DISABLED, WS_CLIPCHILDREN};

    let params = SysclassParams { 
        class_name: NUMERICINPUT_CLASS_NAME,
        sysproc: Some(numeric_sysproc),
        background: None,
        style: Some(CS_HREDRAW | CS_VREDRAW)
    };
    
    if let Err(e) = build_sysclass(params) {
        return Err(Error::System(e));
    }

    let flags = WS_CHILD | WS_BORDER | WS_CLIPCHILDREN |
    if t.visible    { WS_VISIBLE }   else { 0 } |
    if t.disabled   { WS_DISABLED }  else { 0 };

    let params = WindowParams {
        title: "",
        class_name: NUMERICINPUT_CLASS_NAME,
        position: t.position.clone(),
        size: t.size.clone(),
        flags: flags,
        ex_flags: Some(0),
        parent: parent
    };

    match build_window(params) {
        Ok(h) => {
            //set_window_long(h, GWL_USERDATA, t.exit_on_close as usize);
            Ok(h)
        },
        Err(e) => Err(Error::System(e))
    }
}

#[inline(always)]
unsafe fn build_edit<ID: Hash+Clone>(t: &NumericInputT<ID>, parent: HWND) -> Result<HWND, Error> {
    use low::window_helper::{WindowParams, build_window};
    use low::defs::{ES_AUTOHSCROLL, ES_READONLY};
    use winapi::{DWORD, WS_VISIBLE, WS_CHILD};

    let flags: DWORD = WS_CHILD | ES_AUTOHSCROLL | WS_VISIBLE | 
        if t.readonly { ES_READONLY } else { 0 };

    let mut size = t.size.clone();
    size.0 -= 15;

    let params = WindowParams {
        title: format!("{}", t.value),
        class_name: "EDIT",
        position: (0,0),
        size: size,
        flags: flags,
        ex_flags: Some(0),
        parent: parent
    };

    match build_window(params) {
        Ok(h) => Ok(h),
        Err(e) => Err(Error::System(e))
    }
}

#[allow(unused_variables, dead_code)]
#[inline(always)]
unsafe fn build_updown<ID: Hash+Clone>(t: &NumericInputT<ID>, parent: HWND) -> Result<HWND, Error> {
    Err(Error::Unimplemented)
}

#[inline(always)]
unsafe fn hook(edit: HWND) {
    use comctl32::SetWindowSubclass;
    SetWindowSubclass(edit, Some(edit_hook), CUSTOM_EVENTS_DISPATCH_ID, 0);
}

#[inline(always)]
unsafe fn unhook(edit: HWND) {
    use comctl32::RemoveWindowSubclass;
    RemoveWindowSubclass(edit, Some(edit_hook), CUSTOM_EVENTS_DISPATCH_ID);
}