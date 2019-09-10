use winapi::shared::windef::{HFONT};
use winapi::ctypes::c_int;
use winapi::um::winnt::HANDLE;
use winapi::um::shobjidl_core::{IShellItem};

use super::base_helper::{get_system_error, to_utf16};
use crate::{SystemError, UserError};
use std::{ptr, mem};

#[cfg(feature = "file-dialog")] use crate::resources::FileDialogAction;
#[cfg(feature = "file-dialog")] use winapi::um::shobjidl::{IFileDialog, IFileOpenDialog};

pub unsafe fn build_font(
    size: u32,
    weight: u32,
    style: [bool; 3],
    family_name: Option<String>,
) -> Result<HFONT, SystemError> 
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
        println!("{:?}", get_system_error());
        Err( SystemError::FontCreationFailed )
    } else {
        Ok( handle )
    }
}


pub unsafe fn build_image<'a>(
    source: &'a str,
    size: Option<(u32, u32)>,
    strict: bool,
    image_type: u32
) -> Result<HANDLE, SystemError>
{
    use winapi::um::winuser::{LR_LOADFROMFILE, LR_DEFAULTSIZE, LR_SHARED, IMAGE_ICON, IDC_HAND};
    use winapi::um::winuser::LoadImageW;

    let filepath = to_utf16(source);
    let (width, height) = size.unwrap_or((0,0));

    let mut handle = LoadImageW(ptr::null_mut(), filepath.as_ptr(), image_type, width as i32, height as i32, LR_LOADFROMFILE);
    if handle.is_null() {
        let (code, _) = get_system_error();
        if code == 2 && !strict {
            // If the file was not found (err code: 2) and the loading is not strict, replace the image by the system error icon
            let hand_resource = (IDC_HAND as usize) as *const u16;
            handle = LoadImageW(ptr::null_mut(), hand_resource, IMAGE_ICON, 0, 0, LR_DEFAULTSIZE|LR_SHARED);
        }
    }

    if handle.is_null() {
        Err(SystemError::ImageCreationFailed)
    } else {
        Ok(handle)
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
) -> Result<*mut IFileDialog, SystemError> 
{
    use winapi::um::shobjidl_core::{CLSID_FileSaveDialog, CLSID_FileOpenDialog};
    use winapi::um::shobjidl::{FOS_PICKFOLDERS, FOS_ALLOWMULTISELECT, FOS_FORCEFILESYSTEM};
    use winapi::um::combaseapi::CoCreateInstance;
    use winapi::shared::{wtypesbase::CLSCTX_INPROC_SERVER, winerror::S_OK};
    use super::{UUIDOF_IFileDialog, UUIDOF_IFileOpenDialog};

    let (clsid, uuid) = match action {
        FileDialogAction::Save => (CLSID_FileSaveDialog, UUIDOF_IFileDialog()),
        _ => (CLSID_FileOpenDialog, UUIDOF_IFileOpenDialog())
    };

    let mut handle: *mut IFileDialog = ptr::null_mut();
    let r = CoCreateInstance(&clsid, ptr::null_mut(), CLSCTX_INPROC_SERVER, &uuid, mem::transmute(&mut handle) );
    if r != S_OK {
        return Err(SystemError::FileDialogCreationFailed);
    }

    let file_dialog = &mut *handle;
    let mut flags = 0;

    // Set dialog options
    if file_dialog.GetOptions(&mut flags) != S_OK {
        println!("get options");
        file_dialog.Release(); 
        return Err(SystemError::FileDialogCreationFailed);
    }
 
    let use_dir = if action == FileDialogAction::OpenDirectory { FOS_PICKFOLDERS } else { 0 };
    let multiselect = if multiselect { FOS_ALLOWMULTISELECT } else { 0 };
    if file_dialog.SetOptions(flags | FOS_FORCEFILESYSTEM | use_dir | multiselect) != S_OK {
        file_dialog.Release();
        return Err(SystemError::FileDialogCreationFailed);
    }

    
    // Set the default folder
    match &default_folder {
        &Some(ref f) => match file_dialog_set_default_folder(file_dialog, f) {
            Ok(_) => (),
            Err(e) => { println!("set default folder"); file_dialog.Release(); return Err(e); }
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
pub unsafe fn file_dialog_set_default_folder<'a>(dialog: &mut IFileDialog, folder_name: &'a str) -> Result<(), SystemError> {
    use winapi::um::shobjidl_core::{SFGAOF};
    use winapi::um::objidl::IBindCtx;
    use winapi::shared::{winerror::{S_OK, S_FALSE}, guiddef::REFIID, ntdef::{HRESULT, PCWSTR}};
    use winapi::ctypes::c_void;
    use super::IID_IShellItem;

    const SFGAO_FOLDER: u32 = 0x20000000;

    extern "system" {
        pub fn SHCreateItemFromParsingName(pszPath: PCWSTR, pbc: *mut IBindCtx, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT;
    }

    // Code starts here :)

    let mut shellitem: *mut IShellItem = ptr::null_mut();
    let path = to_utf16(&folder_name);

    if SHCreateItemFromParsingName(path.as_ptr(), ptr::null_mut(), &IID_IShellItem(), mem::transmute(&mut shellitem) ) != S_OK {
        return Err(SystemError::FileDialogCreationFailed);
    }

    let shellitem = &mut *shellitem;
    let mut file_properties: SFGAOF = 0;
    
    let results = shellitem.GetAttributes(SFGAO_FOLDER, &mut file_properties);

    if results != S_OK && results != S_FALSE {
        shellitem.Release();
        return Err(SystemError::FileDialogCreationFailed);
    }

    if file_properties & SFGAO_FOLDER != SFGAO_FOLDER {
        shellitem.Release();
        return Err(SystemError::FileDialogCreationFailed);
    }

    if dialog.SetDefaultFolder(shellitem) != S_OK {
        shellitem.Release();
        return Err(SystemError::FileDialogCreationFailed);
    }

    shellitem.Release();

    Ok(())
}


#[cfg(feature = "file-dialog")]
pub unsafe fn file_dialog_set_filters<'a>(dialog: &mut IFileDialog, filters: &'a str) -> Result<(), SystemError> {
    use winapi::shared::minwindef::UINT;
    use winapi::um::shtypes::COMDLG_FILTERSPEC;
    use winapi::shared::winerror::S_OK;

    let mut raw_filters: Vec<COMDLG_FILTERSPEC> = Vec::with_capacity(3);
    let mut keep_alive: Vec<(Vec<u16>, Vec<u16>)> = Vec::with_capacity(3);

    for f in filters.split('|') {
        let end = f.rfind('(');
        if end.is_none() {
            println!("Bad extension filter format: {:?}", filters);
            return Err(SystemError::FileDialogCreationFailed);
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
        println!("Failed to set the filters using {:?}", filters);
        Err(SystemError::FileDialogCreationFailed)
    }
}

#[cfg(feature = "file-dialog")]
pub unsafe fn filedialog_get_item(dialog: &mut IFileDialog) -> Result<String, UserError> {
    use winapi::shared::winerror::S_OK;
    
    let mut _item: *mut IShellItem = ptr::null_mut();

    if dialog.GetResult(&mut _item) != S_OK {
        return Err(UserError::FileDialog("Failed to get result".to_string()));
    }

    let text = get_ishellitem_path(&mut *_item);
    (&mut *_item).Release();

    text
}

#[cfg(feature = "file-dialog")]
pub unsafe fn filedialog_get_items(dialog: &mut IFileOpenDialog) -> Result<Vec<String>, UserError> {
    use winapi::um::shobjidl::IShellItemArray;
    use winapi::shared::{winerror::S_OK, minwindef::DWORD};
    
    let mut _item: *mut IShellItem = ptr::null_mut();
    let mut _items: *mut IShellItemArray = ptr::null_mut();

    if dialog.GetResults( mem::transmute(&mut _items) ) != S_OK {
        return Err(UserError::FileDialog("Failed to get results".into()));
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
unsafe fn get_ishellitem_path(item: &mut IShellItem) -> Result<String, UserError> {
    use winapi::um::shobjidl_core::SIGDN_FILESYSPATH;
    use winapi::shared::{ntdef::PWSTR, winerror::S_OK};
    use winapi::um::combaseapi::CoTaskMemFree;
    use super::base_helper::from_wide_ptr;

    let mut item_path: PWSTR = ptr::null_mut();
    if item.GetDisplayName(SIGDN_FILESYSPATH, &mut item_path) != S_OK {
        return Err(UserError::FileDialog("Failed to get file name".into()));
    }

    let text = from_wide_ptr(item_path);

    CoTaskMemFree(mem::transmute(item_path));

    Ok(text)
}

#[cfg(feature = "file-dialog")]
pub unsafe fn file_dialog_options(dialog: &mut IFileDialog) -> Result<u32, UserError> {
    use winapi::shared::winerror::S_OK;

    let mut flags = 0;
    if dialog.GetOptions(&mut flags) != S_OK {
        return Err(UserError::FileDialog("Failed to get the file dialog options".into()));
    }

    Ok(flags)
}

#[cfg(feature = "file-dialog")]
pub unsafe fn toggle_dialog_flags(dialog: &mut IFileDialog, flag: u32, enabled: bool) -> Result<(), UserError> {
    use winapi::shared::winerror::S_OK;
    
    let mut flags = file_dialog_options(dialog)?;
    flags = match enabled {
        true => flags | flag,
        false => flags & (!flag)
    };

    if dialog.SetOptions(flags) != S_OK {
        return Err(UserError::FileDialog("Failed to set the file dialog options".into()));
    } else {
        Ok(())
    }
}
