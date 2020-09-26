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
use crate::controls::{CharFormat, ParaFormat, CharEffects, UnderlineType, ParaNumbering,
ParaNumberingStyle, ParaAlignment, ParaLineSpacing};
use std::{mem, ptr};
use std::convert::TryFrom;

pub const EM_SETBKGNDCOLOR: u32 = WM_USER + 67;

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

const PFM_STARTINDENT: u32 = 0x00000001;
const PFM_RIGHTINDENT: u32 = 0x00000002;
const PFM_OFFSET: u32 = 0x00000004;
const PFM_ALIGNMENT: u32 = 0x00000008;
const PFM_NUMBERING: u32 = 0x00000020;
const PFM_SPACEBEFORE: u32 = 0x00000040;
const PFM_SPACEAFTER: u32 = 0x00000080;
const PFM_LINESPACING: u32 = 0x00000100;
const PFM_NUMBERINGSTYLE: u32 = 0x00002000;
const PFM_NUMBERINGTAB: u32 = 0x00004000;
const PFM_NUMBERINGSTART: u32 = 0x00008000;
const PFM_RTLPARA: u32 = 0x00010000;

const PFN_BULLET: u16 = 1;
const PFN_ARABIC: u16 = 2;
const PFN_LCLETTER: u16 = 3;
const PFN_LCROMAN: u16 = 4;
const PFN_UCLETTER: u16 = 5;
const PFN_UCROMAN: u16 = 6;
const PFN_CUSTOM: u16 = 7;

const PFNS_PAREN: u16 = 0x000;
const PFNS_PARENS: u16 = 0x100;
const PFNS_PERIOD: u16 = 0x200;
const PFNS_PLAIN: u16 = 0x300;
const PFNS_NONUMBER: u16 = 0x400;
const PFNS_NEWNUMBER: u16 = 0x8000;

const PFA_LEFT: u16 = 1;
const PFA_RIGHT: u16 = 2;
const PFA_CENTER: u16 = 3;
const PFA_JUSTIFY: u16 = 4;
const PFA_FULL_INTERWORD: u16 = 4;

const PFE_RTLPARA: u16 = (PFM_RTLPARA >> 16) as u16;

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
        szFaceName: face,
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
    if fmt.numbering_style.is_some() { mask |= PFM_NUMBERINGSTYLE; }
    if fmt.numbering_tab.is_some() { mask |= PFM_NUMBERINGTAB; }
    if fmt.alignment.is_some() { mask |= PFM_ALIGNMENT; }
    if fmt.space_before.is_some() { mask |= PFM_SPACEBEFORE; }
    if fmt.space_after.is_some() { mask |= PFM_SPACEAFTER; }
    if fmt.start_indent.is_some() { mask |= PFM_STARTINDENT; }
    if fmt.right_indent.is_some() { mask |= PFM_RIGHTINDENT; }
    if fmt.offset.is_some() { mask |= PFM_OFFSET; }
    if fmt.line_spacing.is_some() { mask |= PFM_LINESPACING; }
    if fmt.rtl.is_some() { mask |= PFM_RTLPARA; }

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
                mask |= PFM_NUMBERINGSTART;
                PFN_CUSTOM
            }
        }
    }

    let mut numbering_style = 0;
    if let Some(style) = fmt.numbering_style {
        numbering_style = match style {
            ParaNumberingStyle::Paren => PFNS_PAREN,
            ParaNumberingStyle::Parens => PFNS_PARENS,
            ParaNumberingStyle::Period => PFNS_PERIOD,
            ParaNumberingStyle::Plain => PFNS_PLAIN,
            ParaNumberingStyle::NoNumber => PFNS_NONUMBER,
            ParaNumberingStyle::NewNumber => PFNS_NEWNUMBER,
        };
    }

    let mut aligment = 0;
    if let Some(align) = fmt.alignment {
        aligment = match align {
            ParaAlignment::Left => PFA_LEFT,
            ParaAlignment::Right => PFA_RIGHT,
            ParaAlignment::Center => PFA_CENTER,
            ParaAlignment::Justify => PFA_JUSTIFY,
            ParaAlignment::FullInterword => PFA_FULL_INTERWORD,
        };
    }

    let mut line_spacing = 0;
    let mut line_spacing_rule = 0;
    if let Some(spacing) = fmt.line_spacing {
        line_spacing_rule = match spacing {
            ParaLineSpacing::Single => 0,
            ParaLineSpacing::OneAndHalf => 1,
            ParaLineSpacing::Double => 2,
            ParaLineSpacing::SingleOr(v) => {
                line_spacing = v;
                3
            },
            ParaLineSpacing::Exact(v) => {
                line_spacing = v;
                4
            },
            ParaLineSpacing::Exact20(v) => {
                line_spacing = v;
                5
            },
        };
    }

    let mut effects = 0;
    if let Some(true) = fmt.rtl {
        effects |= PFE_RTLPARA;
    }

    let numbering_tab = fmt.numbering_tab.unwrap_or(0);
    let space_before = fmt.space_before.unwrap_or(0);
    let space_after = fmt.space_after.unwrap_or(0);
    let start_indent = fmt.start_indent.unwrap_or(0);
    let right_indent = fmt.right_indent.unwrap_or(0);
    let offset = fmt.offset.unwrap_or(0);

    let mut para = PARAFORMAT {
        cbSize: mem::size_of::<PARAFORMAT>() as _,
        dwMask: mask,
        wEffects: effects,
        wNumbering: numbering,
        wNumberingStart: numbering_start,
        wNumberingStyle: numbering_style,
        wNumberingTab: numbering_tab,
        dxStartIndent: start_indent,
        dxRightIndent: right_indent,
        dxOffset: offset,
        dyLineSpacing: line_spacing,
        bLineSpacingRule: line_spacing_rule,
        wAlignment: aligment,
        dySpaceBefore: space_before,
        dySpaceAfter: space_after,
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
    let mut numbering_style = None;
    let mut numbering_tab = None;

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

        numbering_style = Some(match para.wNumberingStyle {
            PFNS_PARENS => ParaNumberingStyle::Parens,
            PFNS_PERIOD => ParaNumberingStyle::Period,
            PFNS_PLAIN => ParaNumberingStyle::Plain,
            PFNS_NONUMBER => ParaNumberingStyle::NoNumber,
            PFNS_NEWNUMBER => ParaNumberingStyle::NewNumber,
            _ => ParaNumberingStyle::Paren
        });

        numbering_tab = Some(para.wNumberingTab);
    }

    let mut alignment = None;
    if para.dwMask & PFM_ALIGNMENT == PFM_ALIGNMENT {
        alignment = Some(match para.wAlignment {
            PFA_RIGHT => ParaAlignment::Right,
            PFA_CENTER => ParaAlignment::Center,
            PFA_JUSTIFY => ParaAlignment::Justify,
            _ => ParaAlignment::Left
        });
    }

    let mut space_before = None;
    if para.dwMask & PFM_SPACEBEFORE == PFM_SPACEBEFORE {
        space_before = Some(para.dySpaceBefore);
    }

    let mut space_after = None;
    if para.dwMask & PFM_SPACEAFTER == PFM_SPACEAFTER {
        space_after = Some(para.dySpaceAfter);
    }

    let mut start_indent = None;
    if para.dwMask & PFM_STARTINDENT == PFM_STARTINDENT {
        start_indent = Some(para.dxStartIndent);
    }

    let mut right_indent = None;
    if para.dwMask & PFM_RIGHTINDENT == PFM_RIGHTINDENT {
        right_indent = Some(para.dxRightIndent);
    }

    let mut offset = None;
    if para.dwMask & PFM_OFFSET == PFM_OFFSET {
        offset = Some(para.dxOffset);
    }

    let mut line_spacing = None;
    if para.dwMask & PFM_LINESPACING == PFM_LINESPACING {
        line_spacing = Some(match para.bLineSpacingRule {
            1 => ParaLineSpacing::OneAndHalf,
            2 => ParaLineSpacing::Double,
            3 => ParaLineSpacing::SingleOr(para.dyLineSpacing),
            4 => ParaLineSpacing::Exact(para.dyLineSpacing),
            5 => ParaLineSpacing::Exact20(para.dyLineSpacing),
            _ => ParaLineSpacing::Single,
        });
    }

    let mut rtl = None;
    if para.dwMask & PFM_RTLPARA == PFM_RTLPARA {
        rtl = Some(para.wEffects & PFE_RTLPARA == PFE_RTLPARA);
    }

    ParaFormat {
        numbering,
        numbering_style,
        numbering_tab,
        alignment,
        space_before,
        space_after,
        start_indent,
        right_indent,
        offset,
        line_spacing,
        rtl,
    }
}

