use winapi::shared::windef::{HFONT, HBITMAP};
use winapi::ctypes::c_int;
use winapi::um::winnt::HANDLE;

use crate::resources::OemImage;
use super::base_helper::{get_system_error, to_utf16};

#[allow(unused_imports)] use std::{ptr, mem};
#[allow(unused_imports)] use crate::NwgError;

#[cfg(feature = "file-dialog")] use winapi::Interface;
#[cfg(feature = "file-dialog")] use winapi::um::shobjidl_core::IShellItem;
#[cfg(feature = "file-dialog")] use crate::resources::FileDialogAction;
#[cfg(feature = "file-dialog")] use winapi::um::shobjidl::{IFileDialog, IFileOpenDialog};
#[cfg(feature = "file-dialog")] use std::ffi::OsString;


pub fn is_bitmap(handle: HBITMAP) -> bool {
    use winapi::um::wingdi::GetBitmapBits;
    use winapi::shared::minwindef::LPVOID;

    let mut bits: [u8; 1] = [0; 1];
    unsafe { GetBitmapBits(handle, 1, &mut bits as *mut [u8; 1] as LPVOID) != 0 }
}

pub fn destroy_icon(icon: HANDLE) {
    unsafe { winapi::um::winuser::DestroyIcon(icon as _); }
} 

pub fn destroy_cursor(cursor: HANDLE) {
    unsafe { winapi::um::winuser::DestroyCursor(cursor as _); }
} 

pub fn destroy_obj(obj: HANDLE) {
    unsafe { winapi::um::wingdi::DeleteObject(obj as _); }
} 

pub unsafe fn build_font(
    size: i32,
    weight: u32,
    style: [bool; 3],
    family_name: Option<&str>,
) -> Result<HFONT, NwgError> 
{  
    use winapi::um::wingdi::{DEFAULT_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, CLEARTYPE_QUALITY, VARIABLE_PITCH};
    use winapi::um::wingdi::CreateFontW;
    let [use_italic, use_underline, use_strikeout] = style;

    let fam;
    let family_name_ptr;
    if family_name.is_some() {
        fam = to_utf16(&family_name.unwrap());
        family_name_ptr = fam.as_ptr();
    } else {
        fam = Vec::new();
        family_name_ptr = ptr::null();
    }

    let (size, _) = super::high_dpi::logical_to_physical(size as i32, 0);

    let handle = CreateFontW(
        size as c_int,            // nHeight
        0, 0, 0,                  // nWidth, nEscapement, nOrientation
        weight as c_int,          // fnWeight
        use_italic as u32,         // fdwItalic
        use_underline as u32,     // fdwUnderline
        use_strikeout as u32,     // fdwStrikeOut
        DEFAULT_CHARSET,          // fdwCharSet
        OUT_DEFAULT_PRECIS,       // fdwOutputPrecision
        CLIP_DEFAULT_PRECIS,      // fdwClipPrecision
        CLEARTYPE_QUALITY,        // fdwQuality
        VARIABLE_PITCH,           // fdwPitchAndFamily
        family_name_ptr,     // lpszFace
    );

    drop(fam);

    if handle.is_null() {
        Err( NwgError::resource_create("Failed to create font") )
    } else {
        Ok( handle )
    }
}

pub unsafe fn build_image<'a>(
    source: &'a str,
    size: Option<(u32, u32)>,
    strict: bool,
    image_type: u32
) -> Result<HANDLE, NwgError>
{

    use winapi::um::winuser::{LR_LOADFROMFILE, LR_CREATEDIBSECTION, LR_DEFAULTSIZE, LR_SHARED, IMAGE_ICON, IDC_ARROW, IDI_ERROR, IMAGE_CURSOR, IMAGE_BITMAP};
    use winapi::um::winuser::LoadImageW;

    let filepath = to_utf16(source);
    let (width, height) = size.unwrap_or((0,0));

    let mut handle = LoadImageW(ptr::null_mut(), filepath.as_ptr(), image_type, width as i32, height as i32, LR_LOADFROMFILE);
    if handle.is_null() {
        let (code, _) = get_system_error();
        if code == 2 && !strict {
            // If the file was not found (err code: 2) and the loading is not strict, replace the image by the system error icon
            handle = match image_type {
                IMAGE_ICON => {
                    let dr = (IDI_ERROR as usize) as *const u16;
                    LoadImageW(ptr::null_mut(), dr, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE|LR_SHARED)
                },
                IMAGE_CURSOR => {
                    let dr = (IDC_ARROW as usize) as *const u16;
                    LoadImageW(ptr::null_mut(), dr, IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE|LR_SHARED)
                },
                IMAGE_BITMAP => {
                    let dr = (32754 as usize) as *const u16;
                    LoadImageW(ptr::null_mut(), dr, IMAGE_BITMAP, 0, 0, LR_CREATEDIBSECTION|LR_DEFAULTSIZE|LR_SHARED)
                },
                _ => { unreachable!() }
            };
            
        }
    }

    if handle.is_null() {
        Err( NwgError::resource_create(format!("Failed to create image from source '{}' ", source)))
    } else {
        Ok(handle)
    }
}

#[cfg(feature="image-decoder")]
pub unsafe fn build_image_decoder<'a>(
    source: &'a str,
    size: Option<(u32, u32)>,
    _strict: bool,
    _image_type: u32
) -> Result<HANDLE, NwgError>
{
    use crate::ImageDecoder;

    let decoder = ImageDecoder::new()?;
    
    let mut image_frame = decoder
        .from_filename(source)?
        .frame(0)?;

    if let Some((width, height)) = size {
        image_frame = decoder.resize_image(&image_frame, [width, height])?;
    }
    
    let mut bitmap = image_frame.as_bitmap()?;

    bitmap.owned = false;

    Ok(bitmap.handle)
}

#[cfg(feature="image-decoder")]
pub unsafe fn build_image_decoder_from_memory<'a>(
    src: &'a [u8],
    size: Option<(u32, u32)>,
) -> Result<HANDLE, NwgError>
{
    use crate::ImageDecoder;

    let decoder = ImageDecoder::new()?;
    
    let mut image_frame = decoder
        .from_stream(src)?
        .frame(0)?;

    if let Some((width, height)) = size {
        image_frame = decoder.resize_image(&image_frame, [width, height])?;
    }
    
    let mut bitmap = image_frame.as_bitmap()?;

    bitmap.owned = false;

    Ok(bitmap.handle)
}

pub unsafe fn build_oem_image(
    source: OemImage,
    size: Option<(u32, u32)>,
) -> Result<HANDLE, NwgError> 
{
    use winapi::um::winuser::{LR_DEFAULTSIZE, LR_SHARED, IMAGE_ICON, IMAGE_CURSOR, IMAGE_BITMAP};
    use winapi::um::winuser::LoadImageW;
    use winapi::shared::ntdef::LPCWSTR;

    let (width, height) = size.unwrap_or((0,0));

    let (c_res_type, res_identifier) = match source {
        OemImage::Bitmap(b) => {
            (IMAGE_BITMAP, (b as usize) as LPCWSTR)
        },
        OemImage::Cursor(c) => {
            (IMAGE_CURSOR, (c as usize) as LPCWSTR)
        },
        OemImage::Icon(i) => {
            (IMAGE_ICON, (i as usize) as LPCWSTR)
        }
    };

    let flags = if (width, height) == (0, 0) {
        LR_DEFAULTSIZE|LR_SHARED
    } else {
        LR_SHARED
    };

    let handle = LoadImageW(ptr::null_mut(), res_identifier, c_res_type, width as i32, height as i32, flags);

    if handle.is_null() {
        Err( NwgError::resource_create("Failed to create image from system resource") )
    } else {
        Ok(handle)
    }
}


/** 
    Create a bitmap from memory. Only supports bitmap. Enable the `image-decoder` to load more image type from memory
    The memory must contain the whole file (including the bitmap header).
*/
#[cfg(not(feature="image-decoder"))]
pub unsafe fn bitmap_from_memory(source: &[u8]) -> Result<HANDLE, NwgError> {
    use winapi::um::wingdi::{CreateCompatibleBitmap, CreateCompatibleDC, SetDIBits, BITMAPFILEHEADER, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, BI_RGB, RGBQUAD};
    use winapi::shared::{ntdef::LONG, minwindef::DWORD};
    use winapi::um::winuser::{GetDC, ReleaseDC};
    use winapi::ctypes::c_void;

    // Check the header size requirement
    let fheader_size = mem::size_of::<BITMAPFILEHEADER>();
    let iheader_size = mem::size_of::<BITMAPINFOHEADER>();
    let header_size = fheader_size + iheader_size;
    if source.len() < header_size {
        let msg = format!("Invalid source. The source size ({} bytes) is smaller than the required headers size ({} bytes).", source.len(), header_size);
        return Err(NwgError::ResourceCreationError(msg));
    }

    // Read the bitmap file header
    let src: *const u8 = source.as_ptr();
    let fheader_ptr = src as *const BITMAPFILEHEADER;
    let fheader: BITMAPFILEHEADER = ptr::read( fheader_ptr );

    // Read the bitmap info header
    let iheader_ptr = src.offset(fheader_size as isize) as *const BITMAPINFOHEADER;
    let iheader: BITMAPINFOHEADER = ptr::read( iheader_ptr );

    let (w, h) = (iheader.biWidth, iheader.biHeight);

    let screen_dc = GetDC(ptr::null_mut());
    let hdc = CreateCompatibleDC(screen_dc);
    let bitmap = CreateCompatibleBitmap(screen_dc, w, h);
    ReleaseDC(ptr::null_mut(), screen_dc);

    let header = BITMAPINFOHEADER {
        biSize: mem::size_of::<BITMAPINFOHEADER>() as DWORD,
        biWidth: w as LONG, biHeight: h as LONG, 
        biPlanes: 1, biBitCount: 24, biCompression: BI_RGB,
        biSizeImage: (w * h * 3) as u32,
        biXPelsPerMeter: 0, biYPelsPerMeter: 0,
        biClrUsed: 0, biClrImportant: 0
    };

    let quad = RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 };
    let info = BITMAPINFO {
        bmiHeader: header,
        bmiColors: [quad],
    };

    let data_ptr = source.as_ptr().offset(fheader.bfOffBits as isize) as *const c_void;
    if 0 == SetDIBits(hdc, bitmap, 0, h as u32, data_ptr, &info, DIB_RGB_COLORS) {
        let msg = "SetDIBits failed.".to_string();
        return Err(NwgError::ResourceCreationError(msg));
    }

    return Ok(bitmap as HANDLE);
}

/** 
    Create a bitmap from memory. The source can be any image type supported by the windows imaging component.
    The memory must contain the whole file (including the file header).
*/
#[cfg(feature="image-decoder")]
pub unsafe fn bitmap_from_memory(src: &[u8]) -> Result<HANDLE, NwgError> {
    build_image_decoder_from_memory(src, None)
}

#[cfg(feature="image-decoder")]
pub unsafe fn icon_from_memory(src: &[u8], strict: bool, size: Option<(u32, u32)>) -> Result<HANDLE, NwgError> {
    use winapi::um::wingdi::DeleteObject;
    use winapi::um::winuser::{LoadImageW, CreateIconIndirect};
    use winapi::um::winuser::{ICONINFO, IDI_ERROR, IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED};

    let color_bmp = build_image_decoder_from_memory(src, size);
    if color_bmp.is_err() {
        if strict {
            return color_bmp;
        } else {
            let dr = (IDI_ERROR as usize) as *const u16;
            return Ok(LoadImageW(ptr::null_mut(), dr, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE|LR_SHARED));
        }
    }

    let color_bmp = color_bmp?;
    let mut icon_info = ICONINFO {
        fIcon: 1,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: color_bmp as _,
        hbmColor: color_bmp as _
    };

    let icon = CreateIconIndirect(&mut icon_info);
    match icon.is_null() {
        true => match strict {
            true => Err(NwgError::resource_create("Failed to create icon from source")),
            false => {
                let dr = (IDI_ERROR as usize) as *const u16;
                Ok(LoadImageW(ptr::null_mut(), dr, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE|LR_SHARED))
            }
        },
        false => {
            DeleteObject(color_bmp);
            Ok(icon as _)
        }
    }
}

#[cfg(not(feature="image-decoder"))]
pub unsafe fn icon_from_memory(_src: &[u8], _strict: bool, _size: Option<(u32, u32)>) -> Result<HANDLE, NwgError> {
    unimplemented!("Loading icons from memory require the \"image-decoder\" feature");
}

//
// File dialog low level methods
//

#[cfg(feature = "file-dialog")]
pub unsafe fn create_file_dialog<'a, 'b>(
    action: FileDialogAction,
    multiselect: bool,
    default_folder: Option<String>,
    filters: Option<String>
) -> Result<*mut IFileDialog, NwgError> 
{
    use winapi::um::shobjidl_core::{CLSID_FileSaveDialog, CLSID_FileOpenDialog};
    use winapi::um::shobjidl::{FOS_PICKFOLDERS, FOS_ALLOWMULTISELECT, FOS_FORCEFILESYSTEM};
    use winapi::um::combaseapi::CoCreateInstance;
    use winapi::shared::{wtypesbase::CLSCTX_INPROC_SERVER, winerror::S_OK};

    let (clsid, uuid) = match action {
        FileDialogAction::Save => (CLSID_FileSaveDialog, IFileDialog::uuidof()),
        _ => (CLSID_FileOpenDialog, IFileOpenDialog::uuidof())
    };

    let mut handle: *mut IFileDialog = ptr::null_mut();
    let r = CoCreateInstance(&clsid, ptr::null_mut(), CLSCTX_INPROC_SERVER, &uuid, mem::transmute(&mut handle) );
    if r != S_OK {
        return Err(NwgError::file_dialog("Filedialog creation failed"));
    }

    let file_dialog = &mut *handle;
    let mut flags = 0;

    // Set dialog options
    if file_dialog.GetOptions(&mut flags) != S_OK {
        file_dialog.Release(); 
        return Err(NwgError::file_dialog("Filedialog creation failed"));
    }
 
    let use_dir = if action == FileDialogAction::OpenDirectory { FOS_PICKFOLDERS } else { 0 };
    let multiselect = if multiselect { FOS_ALLOWMULTISELECT } else { 0 };
    if file_dialog.SetOptions(flags | FOS_FORCEFILESYSTEM | use_dir | multiselect) != S_OK {
        file_dialog.Release();
        return Err(NwgError::file_dialog("Filedialog creation failed"));
    }

    
    // Set the default folder
    match &default_folder {
        &Some(ref f) => match file_dialog_set_default_folder(file_dialog, f) {
            Ok(_) => (),
            Err(e) => { file_dialog.Release(); return Err(e); }
        },
        &None => ()
    }

    // Set the default filters
    match &filters {
        &Some(ref f) => match file_dialog_set_filters(file_dialog, f) {
            Ok(_) => (),
            Err(e) => { println!("set filters"); file_dialog.Release(); return Err(e); }
        },
        &None => ()
    }

    Ok(handle)
}


#[cfg(feature = "file-dialog")]
pub unsafe fn file_dialog_set_default_folder<'a>(dialog: &mut IFileDialog, folder_name: &'a str) -> Result<(), NwgError> {
    use winapi::um::shobjidl_core::{SFGAOF};
    use winapi::um::objidl::IBindCtx;
    use winapi::shared::{winerror::{S_OK, S_FALSE}, guiddef::REFIID, ntdef::{HRESULT, PCWSTR}};
    use winapi::ctypes::c_void;

    const SFGAO_FOLDER: u32 = 0x20000000;

    extern "system" {
        pub fn SHCreateItemFromParsingName(pszPath: PCWSTR, pbc: *mut IBindCtx, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT;
    }

    // Code starts here :)

    let mut shellitem: *mut IShellItem = ptr::null_mut();
    let path = to_utf16(&folder_name);

    if SHCreateItemFromParsingName(path.as_ptr(), ptr::null_mut(), &IShellItem::uuidof(), mem::transmute(&mut shellitem) ) != S_OK {
        return Err(NwgError::file_dialog("Failed to set default folder"));
    }

    let shellitem = &mut *shellitem;
    let mut file_properties: SFGAOF = 0;
    
    let results = shellitem.GetAttributes(SFGAO_FOLDER, &mut file_properties);

    if results != S_OK && results != S_FALSE {
        shellitem.Release();
        return Err(NwgError::file_dialog("Failed to set default folder"));
    }

    if file_properties & SFGAO_FOLDER != SFGAO_FOLDER {
        shellitem.Release();
        return Err(NwgError::file_dialog("Failed to set default folder"));
    }

    if dialog.SetDefaultFolder(shellitem) != S_OK {
        shellitem.Release();
        return Err(NwgError::file_dialog("Failed to set default folder"));
    }

    shellitem.Release();

    Ok(())
}


#[cfg(feature = "file-dialog")]
pub unsafe fn file_dialog_set_filters<'a>(dialog: &mut IFileDialog, filters: &'a str) -> Result<(), NwgError> {
    use winapi::shared::minwindef::UINT;
    use winapi::um::shtypes::COMDLG_FILTERSPEC;
    use winapi::shared::winerror::S_OK;

    let mut raw_filters: Vec<COMDLG_FILTERSPEC> = Vec::with_capacity(3);
    let mut keep_alive: Vec<(Vec<u16>, Vec<u16>)> = Vec::with_capacity(3);

    for f in filters.split('|') {
        let end = f.rfind('(');
        if end.is_none() {
            let err = format!("Bad extension filter format: {:?}", filters);
            return Err(NwgError::file_dialog(&err));
        }

        let (_name, _filter) = f.split_at(end.unwrap());
        let (name, filter) = (to_utf16(_name), to_utf16(&_filter[1.._filter.len()-1]));
        
        raw_filters.push(COMDLG_FILTERSPEC{ pszName: name.as_ptr(), pszSpec: filter.as_ptr() });
        keep_alive.push( (name, filter) );
    }

    let filters_count = raw_filters.len() as UINT;
    if dialog.SetFileTypes(filters_count, raw_filters.as_ptr()) == S_OK {
        Ok(())
    } else {
        let err = format!("Failed to set the filters using {:?}", filters);
        return Err(NwgError::file_dialog(&err));
    }
}

#[cfg(feature = "file-dialog")]
pub unsafe fn filedialog_get_item(dialog: &mut IFileDialog) -> Result<OsString, NwgError> {
    use winapi::shared::winerror::S_OK;
    
    let mut _item: *mut IShellItem = ptr::null_mut();

    if dialog.GetResult(&mut _item) != S_OK {
        return Err(NwgError::file_dialog("Failed to get dialog item"));
    }

    let text = get_ishellitem_path(&mut *_item);
    (&mut *_item).Release();

    text
}

#[cfg(feature = "file-dialog")]
pub unsafe fn filedialog_get_items(dialog: &mut IFileOpenDialog) -> Result<Vec<OsString>, NwgError> {
    use winapi::um::shobjidl::IShellItemArray;
    use winapi::shared::{winerror::S_OK, minwindef::DWORD};
    
    let mut _item: *mut IShellItem = ptr::null_mut();
    let mut _items: *mut IShellItemArray = ptr::null_mut();

    if dialog.GetResults( mem::transmute(&mut _items) ) != S_OK {
        return Err(NwgError::file_dialog("Failed to get dialog items"));
    }

    let items = &mut *_items;
    let mut count: DWORD = 0;
    items.GetCount(&mut count);
    
    let mut item_names: Vec<OsString> = Vec::with_capacity(count as usize);
    for i in 0..count {
        items.GetItemAt(i, &mut _item);
        match get_ishellitem_path(&mut *_item) {
            Ok(s) => item_names.push(s),
            Err(_) => {}
        }
    }

    items.Release();

    Ok(item_names)
}

#[cfg(feature = "file-dialog")]
unsafe fn get_ishellitem_path(item: &mut IShellItem) -> Result<OsString, NwgError> {
    use winapi::um::shobjidl_core::SIGDN_FILESYSPATH;
    use winapi::shared::{ntdef::PWSTR, winerror::S_OK};
    use winapi::um::combaseapi::CoTaskMemFree;
    use super::base_helper::os_string_from_wide_ptr;

    let mut item_path: PWSTR = ptr::null_mut();
    if item.GetDisplayName(SIGDN_FILESYSPATH, &mut item_path) != S_OK {
        return Err(NwgError::file_dialog("Failed to get file name"));
    }

    let text = os_string_from_wide_ptr(item_path, None);

    CoTaskMemFree(mem::transmute(item_path));

    Ok(text)
}

#[cfg(feature = "file-dialog")]
pub unsafe fn file_dialog_options(dialog: &mut IFileDialog) -> Result<u32, NwgError> {
    use winapi::shared::winerror::S_OK;

    let mut flags = 0;
    if dialog.GetOptions(&mut flags) != S_OK {
        return Err(NwgError::file_dialog("Failed to get the file dialog options"));
    }

    Ok(flags)
}

#[cfg(feature = "file-dialog")]
pub unsafe fn toggle_dialog_flags(dialog: &mut IFileDialog, flag: u32, enabled: bool) -> Result<(), NwgError> {
    use winapi::shared::winerror::S_OK;
    
    let mut flags = file_dialog_options(dialog)?;
    flags = match enabled {
        true => flags | flag,
        false => flags & (!flag)
    };

    if dialog.SetOptions(flags) != S_OK {
        return Err(NwgError::file_dialog("Failed to set the file dialog options"));
    } else {
        Ok(())
    }
}
