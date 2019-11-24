pub(crate) mod base_helper;
pub(crate) mod window_helper;
pub(crate) mod resources_helper;
pub(crate) mod window;
pub(crate) mod menu;
pub(crate) mod message_box;
pub(crate) mod cursor;

#[cfg(feature = "tabs")]
pub(crate) mod tabs;

#[cfg(feature = "canvas")]
pub(crate) mod canvas;

#[cfg(feature = "file-dialog")] use winapi::shared::guiddef::GUID;
use std::{mem, ptr};
use crate::errors::SystemError;


/**
    Dispatch system events in the current thread
*/
pub fn dispatch_thread_events() {
    use winapi::um::winuser::MSG;
    use winapi::um::winuser::{SendMessageW, GetMessageW, TranslateMessage, DispatchMessageW};
    use winapi::shared::windef::HWND;

    unsafe {
        let mut msg: MSG = mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {

            // Dispatch notice message sent from other threads
            if msg.message == window_helper::NOTICE_MESSAGE {
                let hwnd = msg.lParam as HWND;
                SendMessageW(hwnd, window_helper::NOTICE_MESSAGE, msg.wParam, 0);
                continue;
            }

            TranslateMessage(&msg); 
            DispatchMessageW(&msg); 
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
pub fn init_common_controls() -> Result<(), SystemError> {
    use winapi::um::commctrl::{InitCommonControlsEx, INITCOMMONCONTROLSEX};
    use winapi::um::commctrl::{ICC_BAR_CLASSES, ICC_STANDARD_CLASSES, ICC_DATE_CLASSES, ICC_PROGRESS_CLASS,
     ICC_TAB_CLASSES, ICC_TREEVIEW_CLASSES};

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

        let data = INITCOMMONCONTROLSEX {
            dwSize: mem::size_of::<INITCOMMONCONTROLSEX>() as u32,
            dwICC: classes
        };

        InitCommonControlsEx(&data);
    }

    window::init_window_class()?;
    tabs_init()?;
    canvas_init()?;

    Ok(())
}

#[cfg(feature = "tabs")]
fn tabs_init() -> Result<(), SystemError> { tabs::create_tab_classes() }

#[cfg(not(feature = "tabs"))]
fn tabs_init() -> Result<(), SystemError> { Ok(()) }

#[cfg(feature = "canvas")]
fn canvas_init() -> Result<(), SystemError> { canvas::create_canvas_classes() }

#[cfg(not(feature = "canvas"))]
fn canvas_init() -> Result<(), SystemError> { Ok(()) }


#[allow(unused_macros)]
macro_rules! define_guid {
    ($n1:ident, $d1:expr, $d2:expr, $d3:expr, $d4:expr) => (

        #[inline(always)]
        #[allow(non_snake_case)]
        pub(crate) fn $n1() ->  GUID {
            GUID {
                Data1: $d1,
                Data2: $d2,
                Data3: $d3,
                Data4: $d4
            }
        }
    
    )
}

#[cfg(feature = "file-dialog")] define_guid!(UUIDOF_IFileDialog, 1123569974, 56190, 17308, [133, 241, 228, 7, 93, 19, 95, 200]);
#[cfg(feature = "file-dialog")] define_guid!(UUIDOF_IFileOpenDialog, 3581702792, 54445, 18280, [190, 2, 157, 150, 149, 50, 217, 96]);
#[cfg(feature = "file-dialog")] define_guid!(IID_IShellItem, 1132621086, 59160, 17134, [188, 85, 161, 226, 97, 195, 123, 254]);
