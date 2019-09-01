use std::{mem, ptr};

pub(crate) mod base_helper;
pub(crate) mod window_helper;
pub(crate) mod window;
pub(crate) mod menu;
pub(crate) mod message_box;

/**
    Dispatch system events in the current thread
*/
pub fn dispatch_thread_events() {
    use winapi::um::winuser::MSG;
    use winapi::um::winuser::{GetMessageW, TranslateMessage, DispatchMessageW};

    unsafe {
        let mut msg: MSG = mem::uninitialized();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
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
