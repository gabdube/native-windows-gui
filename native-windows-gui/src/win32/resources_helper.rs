use winapi::shared::windef::{HFONT, HBITMAP};
use winapi::ctypes::c_int;
use winapi::um::winnt::HANDLE;

use crate::resources::OemImage;
use super::base_helper::{get_system_error, to_utf16};

#[allow(unused_imports)] use std::{ptr, mem};
#[allow(unused_imports)] use crate::{NwgError};

#[cfg(feature = "file-dialog")] use winapi::Interface;
#[cfg(feature = "file-dialog")] use winapi::um::shobjidl_core::{IShellItem};
#[cfg(feature = "file-dialog")] use crate::resources::FileDialogAction;
#[cfg(feature = "file-dialog")] use winapi::um::shobjidl::{IFileDialog, IFileOpenDialog};


pub fn is_bitmap(handle: HBITMAP) -> bool {
    use winapi::um::wingdi::GetBitmapBits;
    use winapi::shared::minwindef::LPVOID;

    let mut bits: [u8; 1] = [0; 1];
    unsafe { GetBitmapBits(handle, 1, &mut bits as *mut [u8; 1] as LPVOID) != 0 }
}

pub unsafe fn build_font(
    size: u32,
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
        // println!("{:?}", get_system_error());
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
                    LoadImageW(ptr::null_mut(), dr, IMAGE_ICON, 0, 0, LR_CREATEDIBSECTION|LR_DEFAULTSIZE|LR_SHARED)
                },
                IMAGE_CURSOR => {
                    let dr = (IDC_ARROW as usize) as *const u16;
                    LoadImageW(ptr::null_mut(), dr, IMAGE_CURSOR, 0, 0, LR_CREATEDIBSECTION|LR_DEFAULTSIZE|LR_SHARED)
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
        Err( NwgError::resource_create("Failed to create image") )
    } else {
        Ok(handle)
    }
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


pub unsafe fn make_bitmap_transparent(handle: HANDLE, size: (i32, i32), key: [u8; 3]) -> Result<HANDLE, NwgError> {
    use winapi::um::wingdi::{RGB, TransparentBlt, CreateCompatibleDC, CreateCompatibleBitmap, DeleteDC, DeleteObject, SelectObject};
    use winapi::shared::windef::{HGDIOBJ};

    let (w, h) = size;
    let color_key = RGB(key[0], key[1], key[2]);

    let src_hdc = CreateCompatibleDC(ptr::null_mut());
    SelectObject(src_hdc, handle as HGDIOBJ);

    let dst_hdc = CreateCompatibleDC(ptr::null_mut());
    let dst_hbitmap = CreateCompatibleBitmap(src_hdc, w, h);
    SelectObject(dst_hdc, dst_hbitmap as HGDIOBJ);

    let ok = TransparentBlt (
        dst_hdc,
        0, 0,
        w, h,
        src_hdc,
        0, 0,
        w, h,
        color_key
    );

    DeleteDC(src_hdc);
    DeleteDC(dst_hdc);
    DeleteObject(handle as HANDLE);

    match ok == 1 {
        true => Ok(dst_hbitmap as HANDLE),
        false => Err( NwgError::resource_create("Failed to remove bitmap background") )
    }
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
pub unsafe fn filedialog_get_item(dialog: &mut IFileDialog) -> Result<String, NwgError> {
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
pub unsafe fn filedialog_get_items(dialog: &mut IFileOpenDialog) -> Result<Vec<String>, NwgError> {
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
    
    let mut item_names: Vec<String> = Vec::with_capacity(count as usize);
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
unsafe fn get_ishellitem_path(item: &mut IShellItem) -> Result<String, NwgError> {
    use winapi::um::shobjidl_core::SIGDN_FILESYSPATH;
    use winapi::shared::{ntdef::PWSTR, winerror::S_OK};
    use winapi::um::combaseapi::CoTaskMemFree;
    use super::base_helper::from_wide_ptr;

    let mut item_path: PWSTR = ptr::null_mut();
    if item.GetDisplayName(SIGDN_FILESYSPATH, &mut item_path) != S_OK {
        return Err(NwgError::file_dialog("Failed to get file name"));
    }

    let text = from_wide_ptr(item_path);

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
