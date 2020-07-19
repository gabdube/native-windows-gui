use winapi::shared::minwindef::DWORD;
use std::sync::Arc;
use std::thread;

use crate::opengl_canvas::Texel;
use crate::{nwg, Win32Event, Win32EventWaitResult, SharedMemory};

type Size = (u32, u32);

/**
    Different mode in the application.
    
    - Draw: Paint pixels
    - Erase: Remove painted pixel
*/
#[derive(Debug, Copy, Clone)]
pub enum AppMode {
    Draw,
    Erase
}

/**
    Application state.
    Includes the current drawing state and relation with the other running instances of SyncDraw.
*/
#[derive(Default)]
pub struct AppData {
    shared: SharedMemory,

    event: Arc<Win32Event>,
    thread_handle: Option<thread::JoinHandle<()>>,

    /// The ID associated with the process
    pub instance_id: u32,

    /// If the user is currently drawing on the control
    pub drawing: bool,

    /// The current drawing mode in the app
    pub mode: AppMode,
}


impl AppData {

    pub fn new() -> AppData {

        // Create or load the shared memory region depending if the current instance is the only one on the host
        let instances = AppData::collect_instances();
        let shared = match instances.len() == 0 {
            true => SharedMemory::new(),
            false => SharedMemory::load(),
        };

        // Fetch the next instance ID
        let instance_id = shared.next_instance_id();

        // Build an event for the instance
        let event_name = format!("SyncDrawEvent{}", instance_id);
        let event = match Win32Event::create(&event_name) {
            Ok(evt) => Arc::new(evt), 
            Err(_) => panic!("Failed to create an event")
        };

        // Save the instance in the shared memory
        shared.save_instance_id(instance_id);

        let data = AppData {
            shared,
            event,
            thread_handle: None,
            instance_id,
            drawing: false,
            mode: AppMode::Draw,
        };

        data
    }

    /// Free allocated resources
    /// Wait until the event threads closes
    pub fn close(&mut self) {
        self.shared.close(self.instance_id);
        self.event.close();

        self.thread_handle.take()
            .map(|h| h.join().expect("The thread being joined has panicked") );
    }

    /// Returns true if this instance created the shared memory. Returns false otherwise.
    pub fn first_instance(&self) -> bool {
        self.instance_id == 1
    }

    pub fn instances_count(&self) -> usize {
        self.shared.instances().len()
    }

    /// Spawns a new thread that notice the GUI thread when other instances edited the shared memory.
    pub fn listen_events(&mut self, sender: nwg::NoticeSender) {
        let thread_event = self.event.clone();
        let thread_result = thread::Builder::new().name("event_thread".to_string()).spawn(move || {
            loop {
                // Once the event is destroyed on the main thread, this will return `Failed`
                match thread_event.wait(1000) {
                    Win32EventWaitResult::Signaled => { sender.notice() },
                    Win32EventWaitResult::Failed => { break; },
                    _ => {}
                }
            }
        });

        self.thread_handle = match thread_result {
            Ok(h) => Some(h),
            Err(_) => panic!("Failed to spawn event_thread.")
        };
    }

    /// Update the window size in the shared memory and propagate the change to all running instances
    pub fn set_window_size(&self, size: Size) {
        self.shared.set_window_size(size);
    }

    /// Fetch the window size in the shared memory
    pub fn window_size(&self) -> Size {
        self.shared.window_size()
    }

    /// Return the texture size saved in the shared memory
    pub fn texture_size(&self) -> Size {
        self.shared.texture_size()
    }

    /// Sets the texture data in the shared memory. Also updates the texture size.
    pub fn set_texture_data(&self, width: u32, height: u32, texture_data: &[Texel]) {
        self.shared.set_texture_data(width, height, texture_data);
    }

    /// Return a copy of the texture data stored in the shared memory
    pub fn texture_data(&self) -> Vec<Texel> {
        self.shared.texture_data()
    }

    /// Send a message to every other instances to tell them to apply changes to the shared memory
    pub fn sync(&self) {
        for instance_id in self.shared.instances() {
            if instance_id == self.instance_id {
                continue;
            }

            let event_name = format!("SyncDrawEvent{}", instance_id);
            match Win32Event::open(&event_name) {
                Ok(evt) => {
                    evt.set();
                    evt.close();
                },
                Err(_) => println!("Failed to open event {:?}", event_name)
            }
        }
    }

    /// Collect the instances of SyncDraw running on the host
    fn collect_instances() -> Vec<DWORD> {
        use winapi::um::psapi::{EnumProcesses, GetModuleFileNameExW};
        use winapi::um::processthreadsapi::{OpenProcess, GetCurrentProcessId};
        use winapi::um::winnt::{WCHAR, PROCESS_VM_READ, PROCESS_QUERY_INFORMATION};
        use winapi::um::handleapi::CloseHandle;
        use std::os::windows::ffi::OsStringExt;
        use std::ffi::OsString;
        use std::path::Path;
        use std::{ptr, mem};

        const DWORD_SIZE: usize = mem::size_of::<DWORD>();
        const PROCESS_BUFFER_SIZE: usize = 1024;
        const NAME_BUFFER_SIZE: usize = 200;

        let mut instances_pid = Vec::new();

        unsafe {
            let current_pid = GetCurrentProcessId();

            let max_process_ids_size = (DWORD_SIZE * 1024) as DWORD;
            let mut process_ids_size = 0;
            let mut process_ids: Vec<DWORD> = Vec::with_capacity(PROCESS_BUFFER_SIZE);
            process_ids.set_len(PROCESS_BUFFER_SIZE);

            if EnumProcesses(process_ids.as_mut_ptr(), max_process_ids_size, &mut process_ids_size) == 0 {
                panic!("TODO handle lookup failure");
            }

            let processes_count = process_ids_size as usize / DWORD_SIZE;
            for &process_id in &process_ids[0..processes_count] {
                if process_id == current_pid {
                    continue;
                }
                
                let handle = OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, process_id);
                if handle.is_null() {
                    continue;
                }

                let mut process_name: Vec<WCHAR> = Vec::with_capacity(NAME_BUFFER_SIZE);
                process_name.set_len(100);
                if GetModuleFileNameExW(handle, ptr::null_mut(), process_name.as_mut_ptr(), NAME_BUFFER_SIZE as DWORD) == 0 {
                    CloseHandle(handle);
                    continue;
                }

                let length: usize = process_name.iter().position(|&n| n==0).unwrap_or(0);
                let name = OsString::from_wide(&process_name[..length]);
                match Path::new(&name).file_name() {
                    Some(name) => if name == "syncdraw.exe" {
                        instances_pid.push(process_id);
                    },
                    None => {}
                }

                CloseHandle(handle);
            }

            instances_pid
        }
    }

}


impl Default for AppMode {
    fn default() -> AppMode {
        AppMode::Draw
    }
}
