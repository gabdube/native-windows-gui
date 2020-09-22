pub(crate) mod base_helper;
pub(crate) mod window_helper;
pub(crate) mod resources_helper;
pub(crate) mod window;
pub(crate) mod message_box;
pub(crate) mod high_dpi;

#[cfg(feature = "menu")]
pub(crate) mod menu;

#[cfg(feature = "cursor")]
pub(crate) mod cursor;

#[cfg(feature = "clipboard")]
pub(crate) mod clipboard;

#[cfg(feature = "tabs")]
pub(crate) mod tabs;

#[cfg(feature = "extern-canvas")]
pub(crate) mod extern_canvas;

#[cfg(feature = "image-decoder")]
pub(crate) mod image_decoder;

#[cfg(feature = "rich-textbox")]
pub(crate) mod richedit;

use std::{mem, ptr};
use crate::errors::NwgError;


use winapi::um::winuser::{IsDialogMessageW, GetAncestor, TranslateMessage, DispatchMessageW, GA_ROOT};

/**
    Dispatch system events in the current thread. This method will pause the thread until there are events to process.
*/
pub fn dispatch_thread_events() {
    use winapi::um::winuser::MSG;
    use winapi::um::winuser::GetMessageW;

    unsafe {
        let mut msg: MSG = mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
            if IsDialogMessageW(GetAncestor(msg.hwnd, GA_ROOT), &mut msg) == 0 {
                TranslateMessage(&msg); 
                DispatchMessageW(&msg); 
            }
        }
    }
}


/**
    Dispatch system evetns in the current thread AND execute a callback after each peeking attempt.
    Unlike `dispath_thread_events`, this method will not pause the thread while waiting for events.
*/
pub fn dispatch_thread_events_with_callback<F>(mut cb: F) 
    where F: FnMut() -> () + 'static
{
    use winapi::um::winuser::MSG;
    use winapi::um::winuser::{PeekMessageW, PM_REMOVE, WM_QUIT};

    unsafe {
        let mut msg: MSG = mem::zeroed();
        while msg.message != WM_QUIT {
            let has_message = PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) != 0;
            if has_message {
                if IsDialogMessageW(GetAncestor(msg.hwnd, GA_ROOT), &mut msg) == 0 {
                    TranslateMessage(&msg); 
                    DispatchMessageW(&msg); 
                }
            }

            cb();
        }
    }
}

/**
    Break the events loop running on the current thread
*/
pub fn stop_thread_dispatch() {
  use winapi::um::winuser::PostMessageW;
  use winapi::um::winuser::WM_QUIT;

  unsafe { PostMessageW(ptr::null_mut(), WM_QUIT, 0, 0) };
}


/**
  Enable the Windows visual style in the application without having to use a manifest
*/
pub fn enable_visual_styles() {
    use winapi::shared::minwindef::{ULONG, DWORD, MAX_PATH};
    use winapi::shared::basetsd::ULONG_PTR;
    use winapi::um::winbase::{ACTCTXW, CreateActCtxW, ActivateActCtx};
    use winapi::um::sysinfoapi::GetSystemDirectoryW;

    const ACTCTX_FLAG_RESOURCE_NAME_VALID: DWORD = 0x008;
    const ACTCTX_FLAG_SET_PROCESS_DEFAULT: DWORD = 0x010;
    const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: DWORD = 0x004;

    let mut sys_dir: Vec<u16> = Vec::with_capacity(MAX_PATH);
    unsafe {
        sys_dir.set_len(MAX_PATH);
        GetSystemDirectoryW(sys_dir.as_mut_ptr(), MAX_PATH as u32);
    }
   
    let mut source = base_helper::to_utf16("shell32.dll");

    let mut activation_cookie: ULONG_PTR = 0;
    let mut act_ctx = ACTCTXW {
        cbSize: mem::size_of::<ACTCTXW> as ULONG,
        dwFlags: ACTCTX_FLAG_RESOURCE_NAME_VALID | ACTCTX_FLAG_SET_PROCESS_DEFAULT | ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID,
        lpSource: source.as_mut_ptr(),
        wProcessorArchitecture: 0,
        wLangId: 0,
        lpAssemblyDirectory: sys_dir.as_mut_ptr(),
        lpResourceName: unsafe { mem::transmute(124usize) }, // ID_MANIFEST
        lpApplicationName: ptr::null_mut(),
        hModule: ptr::null_mut()
    };

    unsafe {
        let handle = CreateActCtxW(&mut act_ctx);
        ActivateActCtx(handle, &mut activation_cookie);
    }
}

/**
    Ensure that the dll containing the winapi controls is loaded.
    Also register the custom classes used by NWG
*/
pub fn init_common_controls() -> Result<(), NwgError> {
    use winapi::um::objbase::CoInitialize;
    use winapi::um::libloaderapi::LoadLibraryW;
    use winapi::um::commctrl::{InitCommonControlsEx, INITCOMMONCONTROLSEX};
    use winapi::um::commctrl::{ICC_BAR_CLASSES, ICC_STANDARD_CLASSES, ICC_DATE_CLASSES, ICC_PROGRESS_CLASS,
     ICC_TAB_CLASSES, ICC_TREEVIEW_CLASSES, ICC_LISTVIEW_CLASSES};
    use winapi::shared::winerror::{S_OK, S_FALSE};

    unsafe {
        let mut classes = ICC_BAR_CLASSES | ICC_STANDARD_CLASSES;

        if cfg!(feature = "datetime-picker") {
            classes |= ICC_DATE_CLASSES;
        }

        if cfg!(feature = "progress-bar") {
            classes |= ICC_PROGRESS_CLASS;
        }

        if cfg!(feature = "tabs") {
            classes |= ICC_TAB_CLASSES;
        }

        if cfg!(feature = "tree-view") {
            classes |= ICC_TREEVIEW_CLASSES;
        }

        if cfg!(feature = "list-view") {
            classes |= ICC_LISTVIEW_CLASSES;
        }

        if cfg!(feature = "rich-textbox") {
            let lib = base_helper::to_utf16("Msftedit.dll");
            LoadLibraryW(lib.as_ptr());
        }

        let data = INITCOMMONCONTROLSEX {
            dwSize: mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
            dwICC: classes
        };

        InitCommonControlsEx(&data);
    }

    window::init_window_class()?;
    tabs_init()?;
    extern_canvas_init()?;
    frame_init()?;
    
    match unsafe { CoInitialize(ptr::null_mut()) } {
        S_OK | S_FALSE => Ok(()),
        _ => Err(NwgError::initialization("CoInitialize failed"))
    }
}

#[cfg(feature = "tabs")]
fn tabs_init() -> Result<(), NwgError> { tabs::create_tab_classes() }

#[cfg(not(feature = "tabs"))]
fn tabs_init() -> Result<(), NwgError> { Ok(()) }

#[cfg(feature = "extern-canvas")]
fn extern_canvas_init() -> Result<(), NwgError> { extern_canvas::create_extern_canvas_classes() }

#[cfg(not(feature = "extern-canvas"))]
fn extern_canvas_init() -> Result<(), NwgError> { Ok(()) }

#[cfg(feature = "frame")]
fn frame_init() -> Result<(), NwgError> { window::create_frame_classes() }

#[cfg(not(feature = "frame"))]
fn frame_init() -> Result<(), NwgError> { Ok(()) }

