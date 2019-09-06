use winapi::shared::windef::HFONT;
use winapi::ctypes::c_int;
use winapi::um::winnt::HANDLE;
use super::base_helper::{get_system_error, to_utf16};
use crate::{SystemError};
use std::ptr;


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
