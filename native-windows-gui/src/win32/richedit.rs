//! winapi-rs does not implements richedit.h, so here's the low level stuff
//! implemented here instead of in rich_text_box because it's kind of messy
use winapi::um::winuser::WM_USER;
use winapi::um::wingdi::{LF_FACESIZE, RGB};
use winapi::shared::{
    minwindef::{UINT, DWORD, WORD, BYTE},
    ntdef::{LONG, SHORT, LCID},
    windef::{HWND, COLORREF}
};
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16, from_utf16};
use crate::controls::{CharFormat, ParaFormat, CharEffects, UnderlineType, ParaNumbering};
use std::{mem, ptr};
use std::convert::TryFrom;


const EM_GETCHARFORMAT: u32 = WM_USER + 58;
const EM_GETPARAFORMAT: u32 = WM_USER + 61;
const EM_SETCHARFORMAT: u32 = WM_USER + 68;
const EM_SETPARAFORMAT: u32 = WM_USER + 71;
const SCF_SELECTION: u32 = 1;

const MAX_TAB_STOPS: usize = 32;

const CFM_EFFECTS: u32 = 0x001 | 0x002 | 0x004 | 0x008 | 0x010 | 0x020 | 0x40000000;
const CFM_SIZE: u32 = 0x80000000;
const CFM_OFFSET: u32 = 0x10000000;
const CFM_COLOR: u32 = 0x40000000;
const CFM_FACE: u32 = 0x20000000;
const CFM_UNDERLINETYPE: u32 = 0x00800000;

const PFM_NUMBERING: u32 = 32;

const PFN_BULLET: u16 = 1;
const PFN_ARABIC: u16 = 2;
const PFN_LCLETTER: u16 = 3;
const PFN_LCROMAN: u16 = 4;
const PFN_UCLETTER: u16 = 5;
const PFN_UCROMAN: u16 = 6;
const PFN_CUSTOM: u16 = 7;

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
struct CHARFORMATW {
    cbSize: UINT,
    dwMask: DWORD,
    dwEffects: DWORD,
    yHeight: LONG,
    yOffset: LONG,
    crTextColor: COLORREF,
    bCharSet: BYTE,
    bPitchAndFamily: BYTE,
    szFaceName: [u16; LF_FACESIZE],
    wWeight: WORD,
    sSpacing: SHORT,
    crBackColor: COLORREF,
    lcid: LCID,
    reserved: DWORD,
    sStyle: SHORT,
    wKerning: WORD,
    bUnderlineType: BYTE,
    bAnimation: BYTE,
    bRevAuthor: BYTE,
    bUnderlineColor: BYTE
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
struct PARAFORMAT {
    cbSize: UINT,
    dwMask: DWORD,
    wNumbering: WORD,
    wEffects: WORD,
    dxStartIndent: LONG,
    dxRightIndent: LONG,
    dxOffset: LONG,
    wAlignment: WORD,
    cTabCount: SHORT,
    rgxTabs: [LONG; MAX_TAB_STOPS],
    dySpaceBefore: LONG,
    dySpaceAfter: LONG,
    dyLineSpacing: LONG,
    sStyle: SHORT,
    bLineSpacingRule: BYTE,
    bOutlineLevel: BYTE,
    wShadingWeight: WORD,
    wShadingStyle: WORD,
    wNumberingStart: WORD,
    wNumberingStyle: WORD,
    wNumberingTab: WORD,
    wBorderSpace: WORD,
    wBorderWidth: WORD,
    wBorders: WORD
}


pub(crate) fn set_char_format(handle: HWND, fmt: &CharFormat) {

    let mut mask = 0;
    if fmt.effects.is_some() { mask |= CFM_EFFECTS; }
    if fmt.height.is_some() { mask |= CFM_SIZE; }
    if fmt.y_offset.is_some() { mask |= CFM_OFFSET; }
    if fmt.text_color.is_some() { mask |= CFM_COLOR; }
    if fmt.font_face_name.is_some() { mask |= CFM_FACE; }
    if fmt.underline_type.is_some() { mask |= CFM_UNDERLINETYPE }
    let mut color = 0;
    if let Some([r, g, b]) = fmt.text_color {
        color = RGB(r, g, b);
    }

    let mut face: [u16; LF_FACESIZE] = [0; LF_FACESIZE];
    if let Some(face_name) = fmt.font_face_name.as_ref() {
        let face_name = to_utf16(&face_name);
        if face_name.len() >= LF_FACESIZE {
            panic!("Font face name cannot be longer than {:?} characters", LF_FACESIZE);
        }

        unsafe {
            ptr::copy_nonoverlapping(face_name.as_ptr(), face.as_mut_ptr(), face_name.len());
        }
    }

    let mut underline_type = 0;
    if let Some(under) = fmt.underline_type {
        underline_type = match under {
            UnderlineType::None => 0,
            UnderlineType::Solid => 1,
            UnderlineType::Dash => 5,
            UnderlineType::DashDot => 6,
            UnderlineType::DashDotDot => 7,
            UnderlineType::Dotted => 4,
            UnderlineType::DoubleSolid => 3,
            UnderlineType::Wave => 8,
        };
    }

    let mut fmt = CHARFORMATW {
        cbSize: mem::size_of::<CHARFORMATW>() as _,
        dwMask: mask,
        dwEffects: fmt.effects.map(|e| e.bits()).unwrap_or(0),
        yHeight: fmt.height.unwrap_or(0),
        yOffset: fmt.y_offset.unwrap_or(0),
        crTextColor: color,
        bUnderlineType: underline_type,
        .. Default::default()
    };

    wh::send_message(handle, EM_SETCHARFORMAT, SCF_SELECTION as _, &mut fmt as *mut CHARFORMATW as _);
}

pub(crate) fn char_format(handle: HWND) -> CharFormat {
    use winapi::um::wingdi::{GetRValue, GetGValue, GetBValue};

    let mut fmt: CHARFORMATW = CHARFORMATW {
        cbSize: mem::size_of::<CHARFORMATW>() as _,
        ..Default::default()
    };

    wh::send_message(handle, EM_GETCHARFORMAT, SCF_SELECTION as _, &mut fmt as *mut CHARFORMATW as _);

    let effects = Some(CharEffects::from_bits_truncate(fmt.dwEffects));

    let mut height = None;
    if fmt.yHeight != 0 {
        height = Some(fmt.yHeight);
    }

    let mut y_offset = None;
    if fmt.yOffset != 0 {
        y_offset = Some(fmt.yOffset);
    }

    let mut text_color = None;
    if fmt.crTextColor != 0 {
        text_color = Some([
            GetRValue(fmt.crTextColor),
            GetGValue(fmt.crTextColor),
            GetBValue(fmt.crTextColor),
        ]);
    }

    let underline_type = match fmt.bUnderlineType {
        1 => Some(UnderlineType::Solid),
        5 => Some(UnderlineType::Dash),
        6 => Some(UnderlineType::DashDot),
        7 => Some(UnderlineType::DashDotDot),
        4 => Some(UnderlineType::Dotted),
        3 => Some(UnderlineType::DoubleSolid),
        8 => Some(UnderlineType::Wave),
        _ => None,
    };

    let mut font_face_name = None;
    if fmt.szFaceName[0] != 0 {
        font_face_name = Some(from_utf16(&fmt.szFaceName));
    }

    CharFormat {
        effects,
        height,
        y_offset,
        text_color,
        font_face_name,
        underline_type,
    }
}

pub(crate) fn set_para_format(handle: HWND, fmt: &ParaFormat) {

    let mut mask = 0;
    if fmt.numbering.is_some() { mask |= PFM_NUMBERING; }

    let mut numbering_start = 0;
    let mut numbering = 0;
    if let Some(num) = fmt.numbering {
        numbering = match num {
            ParaNumbering::None => 0,
            ParaNumbering::Bullet => PFN_BULLET,
            ParaNumbering::Arabic => PFN_ARABIC,
            ParaNumbering::LcLetter => PFN_LCLETTER,
            ParaNumbering::LcRoman => PFN_LCROMAN,
            ParaNumbering::UcLetter => PFN_UCLETTER,
            ParaNumbering::UcRoman => PFN_UCROMAN,
            ParaNumbering::Seq(c) => {
                numbering_start = c as u16;
                PFN_CUSTOM
            }
        }
    }

    let mut para = PARAFORMAT {
        cbSize: mem::size_of::<PARAFORMAT>() as _,
        dwMask: mask,
        wNumbering: numbering,
        wNumberingStart: numbering_start,
        ..Default::default()
    };

    wh::send_message(handle, EM_SETPARAFORMAT, 0, &mut para as *mut PARAFORMAT as _);
}

pub(crate) fn para_format(handle: HWND) -> ParaFormat {
    let mut para = PARAFORMAT {
        cbSize: mem::size_of::<PARAFORMAT>() as _,
        ..Default::default()
    };

    wh::send_message(handle, EM_GETPARAFORMAT, 0, &mut para as *mut PARAFORMAT as _);
    

    let mut numbering = None;
    if para.dwMask & PFM_NUMBERING == PFM_NUMBERING && para.wNumbering != 0 {
        numbering = Some(match para.wNumbering {
            PFN_BULLET => ParaNumbering::Bullet,
            PFN_ARABIC => ParaNumbering::Arabic,
            PFN_LCLETTER => ParaNumbering::LcLetter,
            PFN_LCROMAN => ParaNumbering::LcRoman,
            PFN_UCLETTER => ParaNumbering::UcLetter,
            PFN_UCROMAN => ParaNumbering::UcRoman,
            PFN_CUSTOM => ParaNumbering::Seq(char::try_from(para.wNumberingStart as u32).unwrap_or('?')), 
            _ => ParaNumbering::None
        });
    }

    ParaFormat {
        numbering,
    }
}

