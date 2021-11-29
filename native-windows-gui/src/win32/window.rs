/*!
Native Windows GUI windowing base. Includes events dispatching and window creation.

Warning. Not for the faint of heart.
*/
use winapi::shared::minwindef::{BOOL, UINT, DWORD, HMODULE, WPARAM, LPARAM, LRESULT};
use winapi::shared::windef::{HWND, HMENU, HBRUSH};
use winapi::shared::basetsd::{DWORD_PTR, UINT_PTR};
use winapi::um::winuser::{WNDPROC, NMHDR, IDCANCEL, IDOK};
use winapi::um::commctrl::{NMTTDISPINFOW, SUBCLASSPROC};
use super::base_helper::{CUSTOM_ID_BEGIN, to_utf16};
use super::window_helper::{NOTICE_MESSAGE, NWG_INIT, NWG_TRAY, NWG_TIMER_TICK, NWG_TIMER_STOP};
use super::high_dpi;
use crate::controls::ControlHandle;
use crate::{Event, EventData, NwgError};
use std::{ptr, mem};
use std::rc::Rc;
use std::ffi::OsString;
use std::os::windows::prelude::OsStringExt;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};


static TIMER_ID: AtomicU32 = AtomicU32::new(1); 
static NOTICE_ID: AtomicU32 = AtomicU32::new(1); 
static EVENT_HANDLER_ID: AtomicUsize = AtomicUsize::new(1);

const NO_DATA: EventData = EventData::NoData;

type RawCallback = dyn Fn(HWND, UINT, WPARAM, LPARAM) -> Option<LRESULT>;
type Callback = dyn Fn(Event, EventData, ControlHandle) -> ();

/**
    An opaque structure that represent a window subclass hook. 
*/
pub struct EventHandler {
    handles: Vec<HWND>,
    id: SUBCLASSPROC,
    subclass_id: UINT_PTR
}

/**
    An opaque structure that represent a window subclass hook. 
*/
pub struct RawEventHandler {
    handle: HWND,
    subclass_proc: SUBCLASSPROC,
    handler_id: UINT_PTR
}


/**
    Note. While there might be a race condition here, it does not matter because
    All controls are thread local and the true id is (HANDLE + NOTICE_ID)
    The same apply to timers
*/
pub fn build_notice(parent: HWND) -> ControlHandle {
    let id = NOTICE_ID.fetch_add(1, Ordering::SeqCst);
    ControlHandle::Notice(parent, id)
}

pub unsafe fn build_timer(parent: HWND, interval: u32, stopped: bool) -> ControlHandle {
    use winapi::um::winuser::SetTimer;
    
    let id = TIMER_ID.fetch_add(1, Ordering::SeqCst);

    if !stopped {
        SetTimer(parent, id as UINT_PTR, interval as UINT, None);
    }
    
    ControlHandle::Timer(parent, id)
}

/**
    Hook the window subclass with the default event dispatcher.
    The hook is applied to the window and all it's children (recursively).

    Returns a `EventHandler` that can be passed to `unbind_event_handler` to remove the callbacks.

    This function will panic if `handle` is not a window handle.
*/
pub fn full_bind_event_handler<F>(handle: &ControlHandle, f: F) -> EventHandler
    where F: Fn(Event, EventData, ControlHandle) -> () + 'static
{
    use winapi::um::winuser::EnumChildWindows;

    struct SetSubclassParam {
        callback_ptr: *mut *const Callback,
        subclass_id: UINT_PTR,
    }

    /**
        Function that iters over a top level window and bind the events dispatch callback
    */
    unsafe extern "system" fn set_children_subclass(h: HWND, p: LPARAM) -> i32 {
        let params_ptr = p as *mut SetSubclassParam;
        let params = &*params_ptr;
        
        let cb: Rc<Callback> = Rc::from_raw(*params.callback_ptr);

        // Simply increase the rc count because the callback
        // will also be stored into the current children window. 
        mem::forget(cb.clone());
        SetWindowSubclass(h, Some(process_events), params.subclass_id, params.callback_ptr as UINT_PTR);

        // Do not decrease the refcount
        mem::forget(cb);

        1
    }

    /**
        Push the children window handle into the EventHandler
    */
    unsafe extern "system" fn handler_children(h: HWND, p: LPARAM) -> i32 {
        let handles_ptr: *mut Vec<HWND> = p as *mut Vec<HWND>;
        let handles = &mut *handles_ptr;
        handles.push(h);
        1
    }

    let hwnd = handle.hwnd().expect("Cannot bind control with an handle of type");

    // The callback function must be passed to each children of the control
    // To do so, we must RC the callback
    let callback: Rc<Callback> = Rc::new(f);
    let callback_box: Box<*const Callback> = Box::new(Rc::into_raw(callback));
    let callback_ptr: *mut *const Callback = Box::into_raw(callback_box);
    
    let callback_fn: SUBCLASSPROC = Some(process_events);
    let subclass_id = EVENT_HANDLER_ID.fetch_add(1, Ordering::SeqCst);
    let mut handler = EventHandler {
        handles: vec![hwnd],
        id: callback_fn,
        subclass_id,
    };


    let params = Box::new(SetSubclassParam { callback_ptr, subclass_id });
    let params_ptr: *mut SetSubclassParam = Box::into_raw(params);

    unsafe {
        EnumChildWindows(hwnd, Some(handler_children), (&mut handler.handles as *mut Vec<HWND>) as LPARAM);
        EnumChildWindows(hwnd, Some(set_children_subclass), params_ptr as LPARAM);
        SetWindowSubclass(hwnd, callback_fn, subclass_id, callback_ptr as UINT_PTR);
        Box::from_raw(params_ptr);
    }

    handler
}


/**
Hook the window subclass with the default event dispatcher.
The hook is applied to the control and its parent. All common controls send their events to their parent.

Arguments:
    - handle: Handle to the main control to hook
    - parent_handle: Parent to the main control.
    - f: User event callback

Returns a `EventHandler` that can be passed to `unbind_event_handler` to remove the callbacks.

*/
pub fn bind_event_handler<F>(handle: &ControlHandle, parent_handle: &ControlHandle, f: F) -> EventHandler
    where F: Fn(Event, EventData, ControlHandle) -> () + 'static
{
    let hwnd = handle.hwnd().expect("Cannot bind control with an handle of type");
    let parent_hwnd = parent_handle.hwnd().expect("Cannot bind control with an handle of type");
    
    let callback: Rc<Callback> = Rc::new(f);
    let parent_callback = callback.clone();

    let callback_box: Box<*const Callback> = Box::new(Rc::into_raw(callback));
    let callback_box_parent: Box<*const Callback> = Box::new(Rc::into_raw(parent_callback));

    let callback_ptr: *mut *const Callback = Box::into_raw(callback_box);
    let callback_ptr_parent: *mut *const Callback = Box::into_raw(callback_box_parent);

    let callback_fn: SUBCLASSPROC = Some(process_events);
    let subclass_id = EVENT_HANDLER_ID.fetch_add(1, Ordering::SeqCst);
    let handler = EventHandler {
        handles: vec![hwnd, parent_hwnd],
        id: callback_fn,
        subclass_id,
    };

    unsafe {
        SetWindowSubclass(hwnd, callback_fn, subclass_id, callback_ptr as UINT_PTR);
        SetWindowSubclass(parent_hwnd, callback_fn, subclass_id, callback_ptr_parent as UINT_PTR);
    }

    handler
}


/**
    Free all associated callbacks with the event handler.

    This function will panic if the handler was already freed.
*/
pub fn unbind_event_handler(handler: &EventHandler)
{
    let id = handler.id;
    let subclass_id = handler.subclass_id;
    let mut callback_ptr: *mut *const Callback = ptr::null_mut();

    for &handle in handler.handles.iter() {
        unsafe { 
            let mut callback_value: UINT_PTR = 0;
            let result = GetWindowSubclass(handle, id, subclass_id, &mut callback_value);
            if result == 0 {
                panic!("Parent of hander was either freed or is already unbound");
            }

            callback_ptr = callback_value as *mut *const Callback;
            let callback: Rc<Callback> = Rc::from_raw(*callback_ptr);
            mem::drop(callback);

            RemoveWindowSubclass(handle, id, subclass_id);
        };
    }

    // Finally free the pointer to the pointer to the callback
    unsafe {
        Box::from_raw(callback_ptr);
    }
}

pub(crate) fn bind_raw_event_handler_inner<F>(handle: &ControlHandle, handler_id: UINT_PTR, f: F) -> Result<RawEventHandler, NwgError>
    where F: Fn(HWND, UINT, WPARAM, LPARAM) -> Option<LRESULT> + 'static
{
    let handler_id = handler_id;
    let subclass_proc: SUBCLASSPROC = Some(process_raw_events);
    
    let handle = match handle {
        &ControlHandle::Hwnd(h) => unsafe {
            // Check if the handler is already bound to the control
            let mut tmp_value = 0;
            let result = GetWindowSubclass(h, subclass_proc, handler_id, &mut tmp_value);
            if result != 0 {
                return Err(NwgError::events_binding(format!("Events id {} is already present on this", handler_id)))
            }

            // Bind the callback
            let boxed_proc: Box<RawCallback> = Box::new(f);
            let boxed_proc_wrapper: Box<*mut RawCallback> = Box::new(Box::into_raw(boxed_proc));
            let proc_data: *mut *mut RawCallback = Box::into_raw(boxed_proc_wrapper);
            SetWindowSubclass(h, subclass_proc, handler_id, proc_data as UINT_PTR);

            h
        },
        htype => panic!("Cannot bind control with an handle of type {:?}.", htype)
    };

    Ok(RawEventHandler {
        handle,
        subclass_proc,
        handler_id
    })
}

/**

Set a window subclass the uses the `process_raw_events` function of NWG.
The subclass is only applied to the control itself and NOT the children.

When assigning multiple callback to the same control, a different `id` must be specified for each call
or otherwise, the old callback will be replaced by the new one. See `Label::hook_background_color` for example.

Error:
- If the event handler with the same ID is already bound, this function will return an Error. The `has_raw_handler` method can be used to check this.

Panic:
- If the `handle` parameter is not a window-like control
- If the `handler_id` parameter is <= 0xFFFF


```rust
use native_windows_gui as nwg;

fn bind_raw_handler(window: &nwg::Window) -> nwg::RawEventHandler {
    const WM_MOVE: u32 = 3287542; // Not the actual value, but who cares?
    let handler_id = 0x10000;     // handler ids equal or smaller than 0xFFFF are reserved by NWG

    nwg::bind_raw_event_handler(&window.handle, handler_id, move |_hwnd, msg, _w, _l| {
        if msg == WM_MOVE {
            println!("MOVING!");
        }
        None
    }).unwrap()
}

```
*/
pub fn bind_raw_event_handler<F>(handle: &ControlHandle, handler_id: UINT_PTR, f: F) -> Result<RawEventHandler, NwgError>
where F: Fn(HWND, UINT, WPARAM, LPARAM) -> Option<LRESULT> + 'static
{
    if handler_id <= 0xFFFF {
        panic!("handler_id <= 0xFFFF are reserved by NWG");
    }

    bind_raw_event_handler_inner(handle, handler_id, f)
}


/** 
    Check if a raw handler with the specified handler_id is currently bound on the control.
    This function will panic if the handle parameter is not a window control.
*/
pub fn has_raw_handler(handle: &ControlHandle, handler_id: UINT_PTR) -> bool {
    let handle = handle.hwnd().expect("This type of control cannot have a raw handler.");
    let subclass_proc: SUBCLASSPROC = Some(process_raw_events);
    let mut tmp_value = 0;
    unsafe { GetWindowSubclass(handle, subclass_proc, handler_id, &mut tmp_value) != 0 }
}

/**
    Remove the raw event handler from the associated window.
    Calling unbind twice or trying to unbind an handler after destroying its parent will cause the function to panic.
*/
pub fn unbind_raw_event_handler(handler: &RawEventHandler) -> Result<(), NwgError>
{
    let subclass_proc = handler.subclass_proc;
    let handler_id = handler.handler_id;
    let handle = handler.handle;

    unsafe {
        let mut callback_value: UINT_PTR = 0;
        let result = GetWindowSubclass(handle, subclass_proc, handler_id, &mut callback_value);
        if result == 0 {
            let err = format!(concat!(
                "Could not fetch raw event handler #{:?}.",
                "This can happen if the control ({:?}) was freed or",
                "if this raw event handler was already unbound"
            ), handler_id, handle);
            return Err(NwgError::EventsBinding(err));
        }

        let callback_wrapper_ptr = callback_value as *mut *mut RawCallback;
        let callback_wrapper: Box<*mut RawCallback> = Box::from_raw(callback_wrapper_ptr);
        let callback: Box<RawCallback> = Box::from_raw(*callback_wrapper);
        mem::drop(callback);

        RemoveWindowSubclass(handle, subclass_proc, handler_id);
        Ok(())
    }
}

/**
    High level function that handle the creation of custom window control or built in window control
*/
pub(crate) unsafe fn build_hwnd_control<'a>(
    class_name: &'a str,
    window_title: Option<&'a str>,
    size: Option<(i32, i32)>,
    pos: Option<(i32, i32)>,
    flags: Option<DWORD>,
    ex_flags: Option<DWORD>,
    forced_flags: DWORD,
    parent: Option<HWND>
) -> Result<ControlHandle, NwgError> 
{
    use winapi::um::winuser::{WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_CLIPCHILDREN, /*WS_EX_LAYERED*/};
    use winapi::um::winuser::{CreateWindowExW, AdjustWindowRectEx};
    use winapi::shared::windef::RECT;
    use winapi::um::libloaderapi::GetModuleHandleW;

    let hmod = GetModuleHandleW(ptr::null_mut());
    if hmod.is_null() { return Err(NwgError::initialization("GetModuleHandleW failed")); }

    let class_name = to_utf16(class_name);
    let window_title = to_utf16(window_title.unwrap_or("New Window"));
    let ex_flags = ex_flags.unwrap_or(0);
    let flags = flags.unwrap_or(WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_VISIBLE) | forced_flags;

    let pos = pos.unwrap_or((0, 0));
    let size = size.unwrap_or((500, 500));
    let (px, py) = high_dpi::logical_to_physical(pos.0, pos.1);
    let (mut sx, mut sy) = high_dpi::logical_to_physical(size.0, size.1);
    let parent_handle = parent.unwrap_or(ptr::null_mut());
    let menu = ptr::null_mut();
    let lp_params = ptr::null_mut();

    if parent.is_none() {
        let mut rect = RECT {left: 0, top: 0, right: sx, bottom: sy};
        AdjustWindowRectEx(&mut rect, flags, 0, ex_flags);

        sx = rect.right - rect.left;
        sy = rect.bottom  - rect.top;
    }

    let handle = CreateWindowExW (
        ex_flags,
        class_name.as_ptr(), window_title.as_ptr(),
        flags,
        px, py,
        sx, sy,
        parent_handle,
        menu,
        hmod,
        lp_params
    );

    
    if handle.is_null() {
        Err(NwgError::initialization("Window creation failed"))
    } else {
        Ok(ControlHandle::Hwnd(handle))
    }
}

pub(crate) unsafe fn build_sysclass<'a>(
    hmod: HMODULE,
    class_name: &'a str,
    clsproc: WNDPROC,
    background: Option<HBRUSH>,
    style: Option<UINT>
) -> Result<(), NwgError> 
{
    use winapi::um::winuser::{LoadCursorW, RegisterClassExW};
    use winapi::um::winuser::{CS_HREDRAW, CS_VREDRAW, COLOR_WINDOW, IDC_ARROW, WNDCLASSEXW};
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::shared::winerror::ERROR_CLASS_ALREADY_EXISTS;

    let class_name = to_utf16(class_name);
    let background: HBRUSH = background.unwrap_or(COLOR_WINDOW as usize as HBRUSH);
    let style: UINT = style.unwrap_or(CS_HREDRAW | CS_VREDRAW);

    let class =
    WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
        style,
        lpfnWndProc: clsproc, 
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hmod,
        hIcon: ptr::null_mut(),
        hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
        hbrBackground: background,
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: ptr::null_mut()
    };

    let class_token = RegisterClassExW(&class);
    if class_token == 0 && GetLastError() != ERROR_CLASS_ALREADY_EXISTS { 
        Err(NwgError::initialization("System class creation failed"))
    } else {
        Ok(())
    }
}

/// Create the window class for the base nwg window
pub(crate) fn init_window_class() -> Result<(), NwgError> {
    use winapi::um::libloaderapi::GetModuleHandleW;
    
    unsafe {
        let hmod = GetModuleHandleW(ptr::null_mut());
        if hmod.is_null() { return Err(NwgError::initialization("GetModuleHandleW failed")); }

        build_sysclass(hmod, "NativeWindowsGuiWindow", Some(blank_window_proc), None, None)?;
    }
    
    Ok(())
}


#[cfg(feature = "frame")]
/// Create the window class for the frame control
pub(crate) fn create_frame_classes() -> Result<(), NwgError> {
    use winapi::um::libloaderapi::GetModuleHandleW;
    
    unsafe {
        let hmod = GetModuleHandleW(ptr::null_mut());
        if hmod.is_null() { return Err(NwgError::initialization("GetModuleHandleW failed")); }

        build_sysclass(hmod, "NWG_FRAME", Some(blank_window_proc), None, None)?;
    }
    
    Ok(())
}

#[cfg(feature = "message-window")]
/// Create a message only window. Used with the `MessageWindow` control
pub(crate) fn create_message_window() -> Result<ControlHandle, NwgError> {
    use winapi::um::winuser::HWND_MESSAGE;
    use winapi::um::winuser::CreateWindowExW;
    use winapi::um::libloaderapi::GetModuleHandleW;


    let class_name = to_utf16("NativeWindowsGuiWindow");
    let window_title = vec![0];

    unsafe {
        let hmod = GetModuleHandleW(ptr::null_mut());
        if hmod.is_null() { return Err(NwgError::initialization("GetModuleHandleW failed")); }
        
        let handle = CreateWindowExW (
            0,
            class_name.as_ptr(),
            window_title.as_ptr(),
            0,
            0, 0,
            0, 0,
            HWND_MESSAGE,
            ptr::null_mut(),
            hmod,
            ptr::null_mut()
        );

        if handle.is_null() {
            Err(NwgError::initialization("Message only window creation failed"))
        } else {
            Ok(ControlHandle::Hwnd(handle))
        }
    }
}


/**
    A blank system procedure used when creating new window class. Actual system event handling is done in the subclass procedure `process_events`.
*/
unsafe extern "system" fn blank_window_proc(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM) -> LRESULT {
    use winapi::um::winuser::{WM_CREATE, WM_CLOSE, SW_HIDE};
    use winapi::um::winuser::{DefWindowProcW, PostMessageW, ShowWindow};

    let handled = match msg {
        WM_CREATE => {
            PostMessageW(hwnd, NWG_INIT, 0, 0);
            true
        },
        WM_CLOSE => {
            ShowWindow(hwnd, SW_HIDE);
            true
        },
        _ => false
    };

    if handled {
        0
    } else {
        DefWindowProcW(hwnd, msg, w, l)
    }
}

/**
    A window subclass procedure that dispatch the windows control events to the associated application control
*/
#[allow(unused_variables)]
unsafe extern "system" fn process_events(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT {
    use std::char;
    use crate::events::*;

    use winapi::um::commctrl::{DefSubclassProc, TTN_GETDISPINFOW};
    use winapi::um::winuser::{GetClassNameW, GetMenuItemID, GetSubMenu};
    use winapi::um::winuser::{WM_CLOSE, WM_COMMAND, WM_MENUCOMMAND, WM_TIMER, WM_NOTIFY, WM_HSCROLL, WM_VSCROLL, WM_LBUTTONDOWN, WM_LBUTTONUP,
      WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SIZE, WM_MOVE, WM_PAINT, WM_MOUSEMOVE, WM_CONTEXTMENU, WM_INITMENUPOPUP, WM_MENUSELECT, WM_EXITSIZEMOVE,
      WM_ENTERSIZEMOVE, SIZE_MAXIMIZED, SIZE_MINIMIZED, WM_KEYDOWN, WM_KEYUP, WM_CHAR, WM_MOUSEWHEEL, WM_DROPFILES, GET_WHEEL_DELTA_WPARAM,
      WM_GETMINMAXINFO, WM_ENTERMENULOOP, WM_EXITMENULOOP, WM_SYSKEYDOWN, WM_SYSKEYUP};
    use winapi::um::shellapi::{NIN_BALLOONSHOW, NIN_BALLOONHIDE, NIN_BALLOONTIMEOUT, NIN_BALLOONUSERCLICK};
    use winapi::um::winnt::WCHAR;
    use winapi::shared::minwindef::{HIWORD, LOWORD};

    let callback_ptr = data as *mut *const Callback;
    let callback: &Callback = &**callback_ptr;
    let base_handle = ControlHandle::Hwnd(hwnd);

    match msg {
        WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => {
            let evt = match msg {
                WM_SYSKEYDOWN => Event::OnSysKeyPress,
                WM_SYSKEYUP=> Event::OnSysKeyRelease,
                WM_KEYDOWN => Event::OnKeyPress,
                _ /* WM_KEYUP */ => Event::OnKeyRelease,
            };

            // Block the textbox ESC key from closing the whole application
            if w == 27 {
                if is_textbox_control(hwnd) {
                    return 0;
                }
            }

            let keycode = w as u32;
            let data = EventData::OnKey(keycode);
            callback(evt, data, base_handle);
        },
        WM_NOTIFY => {
            let code = {
                let notif_ptr: *mut NMHDR = mem::transmute(l);
                (&*notif_ptr).code
            };
        
            match code {
                TTN_GETDISPINFOW => handle_tooltip_callback(mem::transmute::<_, *mut NMTTDISPINFOW>(l), callback),
                _ => handle_default_notify_callback(mem::transmute::<_, *const NMHDR>(l), callback)
            }
        },
        WM_MENUCOMMAND => {
            let parent_handle: HMENU = mem::transmute(l);
            let item_id = GetMenuItemID(parent_handle, w as i32);
            let handle = ControlHandle::MenuItem(parent_handle, item_id);
            callback(Event::OnMenuItemSelected, NO_DATA, handle);
        },
        WM_INITMENUPOPUP => {
            callback(Event::OnMenuOpen, NO_DATA, ControlHandle::Menu(ptr::null_mut(), w as HMENU));
        },
        WM_ENTERMENULOOP => {
            callback(Event::OnMenuEnter, NO_DATA, ControlHandle::Menu(ptr::null_mut(), w as HMENU));
        },
        WM_EXITMENULOOP  => {
            callback(Event::OnMenuExit, NO_DATA, ControlHandle::Menu(ptr::null_mut(), w as HMENU));
        },
        WM_MOUSEWHEEL => {
            callback(Event::OnMouseWheel, EventData::OnMouseWheel(GET_WHEEL_DELTA_WPARAM(w) as i32), base_handle);
        },
        WM_MENUSELECT => {
            let index = LOWORD(w as u32) as u32;
            let parent = l as HMENU;
            if index < CUSTOM_ID_BEGIN {
                // Item is a sub menu
                callback(Event::OnMenuHover, NO_DATA, ControlHandle::Menu(parent, GetSubMenu(parent, index as i32)));
            } else {
                // Item is a menu item
                callback(Event::OnMenuHover, NO_DATA, ControlHandle::MenuItem(parent, index));
            }
        },
        WM_COMMAND => {
            let child_handle: HWND = l as HWND;
            let message = HIWORD(w as u32) as u16;
            let handle = ControlHandle::Hwnd(child_handle);
            
            // Converting the class name into rust string might not be the most efficient way to do this
            // It might be a good idea to just compare the class_name_raw
            let mut class_name_raw: [WCHAR; 100] = [0; 100];
            let count = GetClassNameW(child_handle, class_name_raw.as_mut_ptr(), 100) as usize;
            let class_name = OsString::from_wide(&class_name_raw[..count]).into_string().unwrap_or("".to_string());

            match &class_name as &str {
                "Button" => callback(button_commands(message), NO_DATA, handle),
                "Edit" => callback(edit_commands(message), NO_DATA, handle),
                "ComboBox" => callback(combo_commands(message), NO_DATA, handle),
                "Static" => callback(static_commands(child_handle, message), NO_DATA, handle),
                "ListBox" => callback(listbox_commands(message), NO_DATA, handle),
                _ => match w as i32 {
                    IDOK | IDCANCEL => callback(no_class_name_commands(w), NO_DATA, base_handle),
                    _ => {}
                },
            }
        },
        WM_CONTEXTMENU => {
            let target_handle = w as HWND;
            let handle = ControlHandle::Hwnd(target_handle);
            callback(Event::OnContextMenu, NO_DATA, handle);
        },
        NWG_TRAY => {
            let msg = LOWORD(l as u32) as u32;
            let handle = ControlHandle::SystemTray(hwnd);

            match msg {
                NIN_BALLOONSHOW => callback(Event::OnTrayNotificationShow, NO_DATA, handle),
                NIN_BALLOONHIDE => callback(Event::OnTrayNotificationHide, NO_DATA, handle),
                NIN_BALLOONTIMEOUT => callback(Event::OnTrayNotificationTimeout, NO_DATA, handle),
                NIN_BALLOONUSERCLICK => callback(Event::OnTrayNotificationUserClose, NO_DATA, handle),
                WM_LBUTTONUP => callback(Event::OnMousePress(MousePressEvent::MousePressLeftUp), NO_DATA,  handle), 
                WM_LBUTTONDOWN => callback(Event::OnMousePress(MousePressEvent::MousePressLeftDown), NO_DATA, handle), 
                WM_RBUTTONUP => {
                    callback(Event::OnMousePress(MousePressEvent::MousePressRightUp), NO_DATA, handle);
                    callback(Event::OnContextMenu, NO_DATA, handle);
                }, 
                WM_RBUTTONDOWN => callback(Event::OnMousePress(MousePressEvent::MousePressRightDown), NO_DATA, handle),
                WM_MOUSEMOVE => callback(Event::OnMouseMove, NO_DATA, handle),
                _ => {}
            }
        },
        WM_SIZE => {
            match w {
                SIZE_MAXIMIZED => callback(Event::OnWindowMaximize, NO_DATA, base_handle),
                SIZE_MINIMIZED => callback(Event::OnWindowMinimize, NO_DATA, base_handle),
                _ => callback(Event::OnResize, NO_DATA, base_handle)
            }
        },
        WM_PAINT => {
            let data = EventData::OnPaint(PaintData { hwnd } );
            callback(Event::OnPaint, data, base_handle)
        },
        WM_DROPFILES => {
            let data = EventData::OnFileDrop(DropFiles { drop: w as _ });
            callback(Event::OnFileDrop, data, base_handle)
        },
        WM_GETMINMAXINFO => {
            let data = EventData::OnMinMaxInfo(MinMaxInfo { inner: l as _ });
            callback(Event::OnMinMaxInfo, data, base_handle)
        },
        WM_CHAR => callback(Event::OnChar, EventData::OnChar(char::from_u32(w as u32).unwrap_or('?')), base_handle),
        WM_EXITSIZEMOVE => callback(Event::OnResizeEnd, NO_DATA, base_handle),
        WM_ENTERSIZEMOVE => callback(Event::OnResizeBegin, NO_DATA, base_handle),
        WM_TIMER => callback(Event::OnTimerTick, NO_DATA, ControlHandle::Timer(hwnd, w as u32)),
        WM_MOVE => callback(Event::OnMove, NO_DATA, base_handle),
        WM_HSCROLL => callback(Event::OnHorizontalScroll, NO_DATA, ControlHandle::Hwnd(l as HWND)),
        WM_VSCROLL => callback(Event::OnVerticalScroll, NO_DATA, ControlHandle::Hwnd(l as HWND)),
        WM_MOUSEMOVE => callback(Event::OnMouseMove, NO_DATA, base_handle), 
        WM_LBUTTONUP => callback(Event::OnMousePress(MousePressEvent::MousePressLeftUp), NO_DATA,  base_handle), 
        WM_LBUTTONDOWN => callback(Event::OnMousePress(MousePressEvent::MousePressLeftDown), NO_DATA, base_handle), 
        WM_RBUTTONUP => callback(Event::OnMousePress(MousePressEvent::MousePressRightUp), NO_DATA, base_handle), 
        WM_RBUTTONDOWN => callback(Event::OnMousePress(MousePressEvent::MousePressRightDown), NO_DATA, base_handle),
        NOTICE_MESSAGE => callback(Event::OnNotice, NO_DATA, ControlHandle::Notice(hwnd, w as u32)),
        NWG_TIMER_STOP => callback(Event::OnTimerStop, NO_DATA, ControlHandle::Timer(hwnd, w as u32)),
        NWG_TIMER_TICK => callback(Event::OnTimerTick, NO_DATA, ControlHandle::Timer(hwnd, w as u32)),
        NWG_INIT => callback(Event::OnInit, NO_DATA, base_handle),
        WM_CLOSE => {
            let mut should_exit = true;
            let data = EventData::OnWindowClose(WindowCloseData { data: &mut should_exit as *mut bool });
            callback(Event::OnWindowClose, data, base_handle);

            if !should_exit {
                return 0;
            }
        },
        _ => {}
    }

    DefSubclassProc(hwnd, msg, w, l)
}

/**
    A window subclass procedure that dispatch the windows control events to the associated application control
*/
#[allow(unused_variables)]
unsafe extern "system" fn process_raw_events(hwnd: HWND, msg: UINT, w: WPARAM, l: LPARAM, id: UINT_PTR, data: DWORD_PTR) -> LRESULT {
    let callback_wrapper_ptr = data as *mut *mut RawCallback;
    let callback: Box<RawCallback> = Box::from_raw(*callback_wrapper_ptr);

    let result = callback(hwnd, msg, w, l);
    Box::into_raw(callback);

    match result {
        Some(r) => r,
        None => ::winapi::um::commctrl::DefSubclassProc(hwnd, msg, w, l)
    }
}

fn button_commands(m: u16) -> Event {
    use winapi::um::winuser::{BN_CLICKED, BN_DBLCLK};
    match m {
        BN_CLICKED => Event::OnButtonClick,
        BN_DBLCLK => Event::OnButtonDoubleClick,
        _ => Event::Unknown
    }
}

fn edit_commands(m: u16) -> Event {
    use winapi::um::winuser::{EN_CHANGE};

    match m {
        EN_CHANGE => Event::OnTextInput,
        _ => Event::Unknown
    }
}

fn combo_commands(m: u16) -> Event {
    use winapi::um::winuser::{CBN_CLOSEUP, CBN_DROPDOWN, CBN_SELCHANGE};
    match m {
        CBN_CLOSEUP => Event::OnComboBoxClosed,
        CBN_DROPDOWN => Event::OnComboBoxDropdown,
        CBN_SELCHANGE => Event::OnComboxBoxSelection,
        _ => Event::Unknown
    }
}

fn datetimepick_commands(m: u32) -> Event {
    use winapi::um::commctrl::{DTN_CLOSEUP, DTN_DROPDOWN, DTN_DATETIMECHANGE};
    match m {
        DTN_CLOSEUP => Event::OnDatePickerClosed,
        DTN_DROPDOWN => Event::OnDatePickerDropdown,
        DTN_DATETIMECHANGE => Event::OnDatePickerChanged,
        _ => Event::Unknown
    }
}

fn tabs_commands(m: u32) -> Event {
    use winapi::um::commctrl::{TCN_SELCHANGE, TCN_SELCHANGING};
    match m {
        TCN_SELCHANGE => Event::TabsContainerChanged,
        TCN_SELCHANGING => Event::TabsContainerChanging,
        _ => Event::Unknown
    }
}

fn track_commands(m: u32) -> Event {
    use winapi::um::commctrl::NM_RELEASEDCAPTURE;

    match m {
        NM_RELEASEDCAPTURE => Event::TrackBarUpdated,
        _ => Event::Unknown
    }
}

fn tree_commands(m: u32) -> Event {
    use winapi::um::commctrl::{
        NM_CLICK, NM_DBLCLK, NM_KILLFOCUS, NM_RCLICK, NM_SETFOCUS, TVN_BEGINLABELEDITW,
        TVN_DELETEITEMW, TVN_ENDLABELEDITW, TVN_ITEMCHANGEDW, TVN_ITEMEXPANDEDW, TVN_SELCHANGEDW,
    };

    match m {
        NM_CLICK => Event::OnTreeViewClick,
        NM_DBLCLK  => Event::OnTreeViewDoubleClick,
        NM_KILLFOCUS => Event::OnTreeFocusLost,
        NM_SETFOCUS => Event::OnTreeFocus,
        NM_RCLICK => Event::OnTreeViewRightClick,
        TVN_DELETEITEMW => Event::OnTreeItemDelete,
        TVN_ITEMEXPANDEDW => Event::OnTreeItemExpanded,
        TVN_SELCHANGEDW => Event::OnTreeItemSelectionChanged,
        TVN_ITEMCHANGEDW => Event::OnTreeItemChanged,
        TVN_BEGINLABELEDITW => Event::OnTreeViewBeginItemEdit,
        TVN_ENDLABELEDITW => Event::OnTreeViewEndItemEdit,
        _ => Event::Unknown,
    }
}

fn list_view_commands(m: u32) -> Event {
    use winapi::um::commctrl::{NM_KILLFOCUS, NM_SETFOCUS, LVN_DELETEALLITEMS,
        LVN_DELETEITEM, LVN_INSERTITEM, LVN_ITEMACTIVATE, LVN_ITEMCHANGED,
        NM_CLICK, NM_DBLCLK, NM_RCLICK, LVN_COLUMNCLICK};

    match m {
        NM_CLICK => Event::OnListViewClick,
        NM_DBLCLK  => Event::OnListViewDoubleClick,
        NM_RCLICK => Event::OnListViewRightClick,
        LVN_COLUMNCLICK => Event::OnListViewColumnClick,
        LVN_DELETEALLITEMS => Event::OnListViewClear,
        LVN_DELETEITEM => Event::OnListViewItemRemoved,
        LVN_INSERTITEM => Event::OnListViewItemInsert,
        LVN_ITEMACTIVATE => Event::OnListViewItemActivated,
        LVN_ITEMCHANGED => Event::OnListViewItemChanged,
        NM_KILLFOCUS => Event::OnListViewFocusLost,
        NM_SETFOCUS => Event::OnListViewFocus,
        _ => Event::Unknown
    }
}

fn no_class_name_commands(m: usize) -> Event {
    match m as i32 {
        IDOK => Event::OnKeyEnter,
        IDCANCEL => Event::OnKeyEsc,
        _ => Event::Unknown,
    }
}

#[cfg(feature = "tree-view")]
fn tree_data(m: u32, notif_raw: *const NMHDR) -> EventData {
    use crate::{ExpandState, TreeItem, TreeItemAction, TreeItemState};
    use winapi::um::commctrl::{
        NMTREEVIEWW, NMTVDISPINFOW, NMTVITEMCHANGE, TVE_COLLAPSE, TVE_EXPAND, TVN_DELETEITEMW,
        TVN_ENDLABELEDITW, TVN_ITEMCHANGEDW, TVN_ITEMEXPANDEDW, TVN_SELCHANGEDW,
    };

    match m {
        TVN_DELETEITEMW => {
            let data = unsafe { &*(notif_raw as *const NMTREEVIEWW) };
            let item = TreeItem { handle: data.itemOld.hItem };
            EventData::OnTreeItemDelete(item)
        },
        TVN_ITEMEXPANDEDW => {
            let data = unsafe { &*(notif_raw as *const NMTREEVIEWW) };
            let item = TreeItem { handle: data.itemNew.hItem };

            let action = match data.action as usize {
                TVE_COLLAPSE => TreeItemAction::Expand(ExpandState::Collapse),
                TVE_EXPAND => TreeItemAction::Expand(ExpandState::Expand),
                _ => TreeItemAction::Unknown // Other values shoudn't be raised by this event
            };

            EventData::OnTreeItemUpdate { item, action }
        },
        TVN_SELCHANGEDW => {
            let data = unsafe { &*(notif_raw as *const NMTREEVIEWW) };
            let new = TreeItem { handle: data.itemNew.hItem };
            let old = TreeItem { handle: data.itemOld.hItem };
            EventData::OnTreeItemSelectionChanged { old, new }
        },
        TVN_ITEMCHANGEDW => {
            let data = unsafe { &*(notif_raw as *const NMTVITEMCHANGE) };
            let item = TreeItem { handle: data.hItem };
            let action = TreeItemAction::State { 
                new: TreeItemState::from_bits_truncate(data.uStateNew),
                old: TreeItemState::from_bits_truncate(data.uStateOld)
            };
            EventData::OnTreeItemUpdate { item, action }
        }
        TVN_ENDLABELEDITW => {
            let data = unsafe { &*(notif_raw as *const NMTVDISPINFOW) };
            let new_psztext = data.item.pszText;
            if !new_psztext.is_null() {
                let new_text_osstr = unsafe { u16_ptr_to_string(new_psztext) };
                if let Ok(new_text) = new_text_osstr.into_string() {
                    EventData::OnTreeViewEndItemEdit {
                        f_cancel: false,
                        new_text,
                    }
                } else {
                    EventData::OnTreeViewEndItemEdit {
                        f_cancel: false,
                        new_text: String::from(""),
                    }
                }
            } else {
                EventData::OnTreeViewEndItemEdit {
                    f_cancel: true,
                    new_text: String::from(""),
                }
            }
        }
        _ => NO_DATA,
    }
}

unsafe fn u16_ptr_to_string(ptr: *const u16) -> OsString {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice)
}

#[cfg(not(feature="tree-view"))]
fn tree_data(_m: u32, _notif_raw: *const NMHDR) -> EventData {
    // If tree-view is not enabled, the data type won't be available so we return NO_DATA
    NO_DATA
}

#[cfg(feature="list-view")]
fn list_view_data(m: u32, notif_raw: *const NMHDR) -> EventData {
    use winapi::um::commctrl::{NMLISTVIEW, NMITEMACTIVATE, LVN_DELETEITEM, LVN_ITEMACTIVATE,
        LVN_INSERTITEM, LVN_ITEMCHANGED, LVIS_SELECTED, LVN_COLUMNCLICK,
        NM_CLICK, NM_RCLICK, NM_DBLCLK};

    match m {
        LVN_DELETEITEM | LVN_INSERTITEM | LVN_COLUMNCLICK => {
            let data: &NMLISTVIEW = unsafe { &*(notif_raw as *const NMLISTVIEW) };
            EventData::OnListViewItemIndex { 
                row_index: data.iItem as _,
                column_index: data.iSubItem as _
            }
        },
        LVN_ITEMACTIVATE | NM_CLICK | NM_DBLCLK | NM_RCLICK => {
            let data: &NMITEMACTIVATE = unsafe { &*(notif_raw as *const NMITEMACTIVATE) };
            EventData::OnListViewItemIndex { 
                row_index: data.iItem as _,
                column_index: data.iSubItem as _
            }
        },
        LVN_ITEMCHANGED => {
            let data: &NMLISTVIEW = unsafe { &*(notif_raw as *const NMLISTVIEW) };
            EventData::OnListViewItemChanged { 
                row_index: data.iItem as _,
                column_index: data.iSubItem as _,
                selected: data.uNewState & LVIS_SELECTED == LVIS_SELECTED
            }
        },
        _ => NO_DATA
    }
}

#[cfg(not(feature="list-view"))]
fn list_view_data(_m: u32, _notif_raw: *const NMHDR) -> EventData {
    // If list-view is not enabled, the data type won't be available so we return NO_DATA
    NO_DATA
}


unsafe fn static_commands(handle: HWND, m: u16) -> Event {
    use winapi::um::winuser::{STN_CLICKED, STN_DBLCLK, STM_GETIMAGE, IMAGE_BITMAP, IMAGE_ICON, IMAGE_CURSOR};
    use winapi::um::winuser::SendMessageW;

    let has_image = SendMessageW(handle, STM_GETIMAGE, IMAGE_BITMAP as usize, 0) != 0;
    let has_icon = SendMessageW(handle, STM_GETIMAGE, IMAGE_ICON as usize, 0) != 0;
    let has_cursor = SendMessageW(handle, STM_GETIMAGE, IMAGE_CURSOR as usize, 0) != 0;

    if has_image | has_icon | has_cursor {
        match m {
            STN_CLICKED => Event::OnImageFrameClick,
            STN_DBLCLK => Event::OnImageFrameDoubleClick,
            _ => Event::Unknown
        }
    } else {
        match m {
            STN_CLICKED => Event::OnLabelClick,
            STN_DBLCLK => Event::OnLabelDoubleClick,
            _ => Event::Unknown
        }
    }   
}

unsafe fn listbox_commands(m: u16) -> Event {
    use winapi::um::winuser::{LBN_SELCHANGE, LBN_DBLCLK};

    match m {
        LBN_SELCHANGE => Event::OnListBoxSelect,
        LBN_DBLCLK => Event::OnListBoxDoubleClick,
        _ => Event::Unknown
    }
}

unsafe fn handle_tooltip_callback<'a>(notif: *mut NMTTDISPINFOW, callback: &Callback) {
    use crate::events::ToolTipTextData;

    let notif = &mut *notif;
    let handle = ControlHandle::Hwnd(notif.hdr.idFrom as HWND);
    let data = EventData::OnTooltipText(ToolTipTextData { data: notif });
    callback(Event::OnTooltipText, data, handle);
}

unsafe fn handle_default_notify_callback<'a>(notif_raw: *const NMHDR, callback: &Callback){
    use winapi::um::winnt::WCHAR;
    use winapi::um::winuser::GetClassNameW;

    let notif = &*notif_raw;
    let handle = ControlHandle::Hwnd(notif.hwndFrom);

    let mut class_name_raw: [WCHAR; 100] = mem::zeroed();
    let count = GetClassNameW(notif.hwndFrom, class_name_raw.as_mut_ptr(), 100) as usize;
    let class_name = OsString::from_wide(&class_name_raw[..count]).into_string().unwrap_or("".to_string());

    let code = notif.code;

    match &class_name as &str {
        "SysDateTimePick32" => callback(datetimepick_commands(code), NO_DATA, handle),
        "SysTabControl32" => callback(tabs_commands(code), NO_DATA, handle),
        "msctls_trackbar32" => callback(track_commands(code), NO_DATA, handle),
        winapi::um::commctrl::WC_TREEVIEW => callback(tree_commands(code), tree_data(code, notif_raw), handle),
        winapi::um::commctrl::WC_LISTVIEW => callback(list_view_commands(code), list_view_data(code, notif_raw), handle),
        _ => {}
    }
}

unsafe fn is_textbox_control(hwnd: HWND) -> bool {
    use winapi::um::winnt::WCHAR;
    use winapi::um::winuser::GetClassNameW;

    let mut class_name_raw: [WCHAR; 100] = [0; 100];
    let count = GetClassNameW(hwnd, class_name_raw.as_mut_ptr(), 100) as usize;
    let class_name = OsString::from_wide(&class_name_raw[..count]).into_string().unwrap_or("".to_string());
    
    class_name == "Edit" || class_name == "RICHEDIT50W"
}

//
// Hack to make `GetWindowSubclass` work on GNU
//

#[cfg(target_env="gnu")] use std::{sync::Mutex, collections::HashMap};

#[cfg(target_env="gnu")]
type SubclassId = (usize, usize, UINT_PTR);

#[cfg(target_env="gnu")]
static mut SUBCLASS_COLLECTION: Option<Mutex<HashMap<SubclassId, DWORD_PTR>>> = None;


#[cfg(target_env="gnu")]
#[allow(non_snake_case)]
unsafe fn GetWindowSubclass(hwnd: HWND, proc: SUBCLASSPROC, uid: UINT_PTR, data: *mut DWORD_PTR) -> BOOL {
    if SUBCLASS_COLLECTION.is_none() {
        SUBCLASS_COLLECTION = Some(Mutex::new(HashMap::new()));
    }

    let id = (hwnd as usize, mem::transmute(proc), uid);
    match SUBCLASS_COLLECTION.as_ref() {
        Some(collection_mutex) => {
            let collection = collection_mutex.lock().unwrap();
            match collection.get(&id) {
                Some(v) => { *data = *v; 1 },
                None => { 0 }
            }
        },
        None => unreachable!()
    }
}

#[cfg(target_env="gnu")]
#[allow(non_snake_case)]
unsafe fn SetWindowSubclass(hwnd: HWND, proc: SUBCLASSPROC, uid: UINT_PTR, data: DWORD_PTR) -> BOOL {
    use winapi::um::commctrl::SetWindowSubclass;

    if SUBCLASS_COLLECTION.is_none() {
        SUBCLASS_COLLECTION = Some(Mutex::new(HashMap::new()));
    }

    let id = (hwnd as usize, mem::transmute(proc), uid);
    match SUBCLASS_COLLECTION.as_ref() {
        Some(collection_mutex) => {
            let mut collection = collection_mutex.lock().unwrap();
            collection.insert(id, data);
        },
        None => unreachable!()
    }


    SetWindowSubclass(hwnd, proc, uid, data)
}


#[cfg(target_env="gnu")]
#[allow(non_snake_case)]
unsafe fn RemoveWindowSubclass(hwnd: HWND, proc: SUBCLASSPROC, uid: UINT_PTR) -> BOOL {
    use winapi::um::commctrl::RemoveWindowSubclass;

    if SUBCLASS_COLLECTION.is_none() {
        SUBCLASS_COLLECTION = Some(Mutex::new(HashMap::new()));
    }

    let id = (hwnd as usize, mem::transmute(proc), uid);
    match SUBCLASS_COLLECTION.as_ref() {
        Some(collection_mutex) => {
            let mut collection = collection_mutex.lock().unwrap();
            collection.remove(&id);
        },
        None => unreachable!()
    }

    RemoveWindowSubclass(hwnd, proc, uid)
}

#[cfg(not(target_env="gnu"))]
#[allow(non_snake_case)]
unsafe fn GetWindowSubclass(hwnd: HWND, proc: SUBCLASSPROC, uid: UINT_PTR, data: *mut DWORD_PTR) -> BOOL {
    use winapi::um::commctrl::GetWindowSubclass;
    GetWindowSubclass(hwnd, proc, uid, data)
}

#[cfg(not(target_env="gnu"))]
#[allow(non_snake_case)]
unsafe fn SetWindowSubclass(hwnd: HWND, proc: SUBCLASSPROC, uid: UINT_PTR, data: DWORD_PTR) -> BOOL {
    use winapi::um::commctrl::SetWindowSubclass;
    SetWindowSubclass(hwnd, proc, uid, data)
}

#[cfg(not(target_env="gnu"))]
#[allow(non_snake_case)]
unsafe fn RemoveWindowSubclass(hwnd: HWND, proc: SUBCLASSPROC, uid: UINT_PTR) -> BOOL {
    use winapi::um::commctrl::RemoveWindowSubclass;
    RemoveWindowSubclass(hwnd, proc, uid)
}

