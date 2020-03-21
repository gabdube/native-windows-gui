use winapi::um::memoryapi::{MapViewOfFile, UnmapViewOfFile, FILE_MAP_ALL_ACCESS};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{HANDLE, WCHAR};
use winapi::shared::minwindef::DWORD;
use winapi::shared::basetsd::SIZE_T;
use std::{ptr, mem};
use crate::opengl_canvas::Texel;

const NAME: &'static str = "SyncDraw_Shared_Memory";
const HEADER_SIZE: SIZE_T = mem::size_of::<SharedHeader>() as SIZE_T;
const DATA_SIZE: SIZE_T = mem::size_of::<SharedData>() as SIZE_T;
const MAX_TEXTURE_PIXELS: usize = 7000*4320;
type Size = (u32, u32);


#[repr(C)]
#[derive(Copy, Clone)]
struct SharedHeader {
    next_instance_id: u32,
    window_width: u32,
    window_height: u32,
    texture_width: u32,
    texture_height: u32,

    /// Array of active instances ids, 32 should be enough
    instances: [u32; 32],
}


#[repr(C)]
struct SharedData {
    /// Shared data between instances
    header: SharedHeader,

    /// Shared texture data.
    texture_data: [Texel; MAX_TEXTURE_PIXELS]
}

/**
    A wrapper over a named shared memory.
*/
pub struct SharedMemory {
    handle: HANDLE
}

impl SharedMemory {

    /// Try to create a new shared memory region
    pub fn new() -> SharedMemory {
        use winapi::um::memoryapi::{CreateFileMappingW};
        use winapi::um::handleapi::INVALID_HANDLE_VALUE;
        use winapi::um::winnt::PAGE_READWRITE;

        let handle = unsafe {
            let name = SharedMemory::name();
            let buffer_size = mem::size_of::<SharedData>() as DWORD;
            let handle = CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                ptr::null_mut(),
                PAGE_READWRITE,
                0,
                buffer_size,
                name.as_ptr()
            );

            if handle.is_null() {
                panic!("Could not create shared memory region.");
            }

            handle
        };

        SharedMemory {
            handle
        }
    }

    /// Try to load an existing shared memory region
    pub fn load() -> SharedMemory {
        use winapi::um::memoryapi::OpenFileMappingW;

        let handle = unsafe {
            let name = SharedMemory::name();
            let handle = OpenFileMappingW(FILE_MAP_ALL_ACCESS, 0, name.as_ptr());
            if handle.is_null() {
                panic!("Could not load shared memory region.");
            }

            handle
        };
        
        SharedMemory {
            handle
        }
    }

    /// Close the handle to the shared memory
    /// Also removes the current instance from the instance list
    pub fn close(&self, id: u32) {
        unsafe {
            let header_ptr = SharedMemory::map_view(self.handle, 0, HEADER_SIZE) as *mut SharedHeader;
            let header = &mut *header_ptr;

            match header.instances.iter_mut().find(|h| **h == id ) {
                Some(handle) => { *handle = 0 },
                None => {}
            }

            drop(header);

            CloseHandle(self.handle);
        }
    }

    /// Returns the instances registered in the shared memory
    pub fn instances(&self) -> Vec<u32> {
        let header_ptr = SharedMemory::map_view(self.handle, 0, HEADER_SIZE) as *mut SharedHeader;
        let instances: Vec<DWORD> = unsafe {
            let header = &mut *header_ptr;
            header.instances.iter().filter(|&&i| i != 0 ).map(|i| *i).collect()
        };

        SharedMemory::unmap_view(header_ptr);

        instances
    }

    /// Fetch the next instance id in the shared memory.
    pub fn next_instance_id(&self) -> u32 {
        let header_ptr = SharedMemory::map_view(self.handle, 0, HEADER_SIZE) as *mut SharedHeader;
        let next_id = unsafe {
            let header = &mut *header_ptr;
            header.next_instance_id += 1;
            header.next_instance_id
        };

        SharedMemory::unmap_view(header_ptr);

        next_id
    }

    /// Saves the selected id in the instance list
    pub fn save_instance_id(&self, instance_id: u32)  {
        let header_ptr = SharedMemory::map_view(self.handle, 0, HEADER_SIZE) as *mut SharedHeader;
        let header = unsafe { &mut *header_ptr };

        match header.instances.iter_mut().find(|h| **h == 0 ) {
            Some(handle) => { *handle = instance_id; },
            None => panic!("No more space left in the instance list")
        }

        SharedMemory::unmap_view(header_ptr);
    }

    /// Sets the global window size
    pub fn set_window_size(&self, size: Size) {
        let header_ptr = SharedMemory::map_view(self.handle, 0, HEADER_SIZE) as *mut SharedHeader;
        let header = unsafe { &mut *header_ptr };
        let (width, height) = size;
        header.window_width = width;
        header.window_height = height;
        SharedMemory::unmap_view(header_ptr);
    }

    /// Returns the shared window size
    pub fn window_size(&self) -> Size {
        let header_ptr = SharedMemory::map_view(self.handle, 0, HEADER_SIZE) as *mut SharedHeader;
        let header = unsafe { &mut *header_ptr };
        let size = (header.window_width, header.window_height);
        SharedMemory::unmap_view(header_ptr);
        size
    }

    /// Returns the texture size in the shared memory
    pub fn texture_size(&self) -> Size {
        let header_ptr = SharedMemory::map_view(self.handle, 0, HEADER_SIZE) as *mut SharedHeader;
        let header = unsafe { &mut *header_ptr };
        let size = (header.texture_width, header.texture_height);
        SharedMemory::unmap_view(header_ptr);
        size
    }

    /// Sets the texture data in the shared memory
    /// Will panic if the size is bigger than "MAX_TEXTURE_PIXELS"
    pub fn set_texture_data(&self, width: u32, height: u32, texture_data: &[Texel]) {
        let data_ptr = SharedMemory::map_view(self.handle, 0, DATA_SIZE) as *mut SharedData;
        let data = unsafe { &mut *data_ptr };

        let pixel_count = (width * height) as usize;
        if pixel_count > MAX_TEXTURE_PIXELS {
            panic!("Texture is bigger than shared buffer: texture {} VS buffer {}", pixel_count, MAX_TEXTURE_PIXELS);
        }

        data.header.texture_width = width;
        data.header.texture_height = height;
        unsafe {
            
            ptr::copy_nonoverlapping(texture_data.as_ptr(), data.texture_data.as_mut_ptr(), pixel_count);
        }

        SharedMemory::unmap_view(data_ptr);
    }

    /// Returns a copy of the texture data shared in memory
    pub fn texture_data(&self) -> Vec<Texel> {
        let data_ptr = SharedMemory::map_view(self.handle, 0, DATA_SIZE) as *mut SharedData;
        let data = unsafe { &*data_ptr };

        // Pixel count will never be bigger than the capacity of the shared texture. 
        // `set_texture_data` ensures that.
        let pixel_count = (data.header.texture_width * data.header.texture_height) as usize;
        let texture_data: &[Texel] = &data.texture_data[0..pixel_count];

        let mut out_texture_data = Vec::with_capacity(pixel_count);
        unsafe {
            out_texture_data.set_len(pixel_count);
            ptr::copy_nonoverlapping(texture_data.as_ptr(), out_texture_data.as_mut_ptr(), pixel_count);
        }

        SharedMemory::unmap_view(data_ptr);
        out_texture_data
    }

    /// Returns the name of the shared memory encoded in utf16
    fn name() -> Vec<WCHAR> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        OsStr::new(NAME)
            .encode_wide()
            .chain(Some(0u16).into_iter())
            .collect()
    }

    /// Safe wrapper over MapViewOfFile
    fn map_view(handle: HANDLE, offset: DWORD, size: SIZE_T) -> *mut u8 {
        unsafe {
            let handle = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, offset, size) as *mut u8;
            if handle.is_null() {
                panic!("Could not map memory region");
            }

            handle
        }
    }

    /// Safe wrapper over UnmapViewOfFile
    fn unmap_view<T>(handle: *mut T) {
        unsafe {
            UnmapViewOfFile(handle as _);
        }
    }

}

impl Default for SharedMemory {

    fn default() -> SharedMemory {
        SharedMemory {
            handle: ptr::null_mut()
        }
    }

}

