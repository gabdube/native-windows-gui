/*!
    Public type definitions
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use winapi::{c_int, DWORD, CW_USEDEFAULT};

/**
    Checkbox checkstate
*/
#[derive(Clone, Debug, PartialEq)]
pub enum CheckState {
    Checked,
    Unchecked,
    Indeterminate // Tristate only
}

/**
    Mouse buttons
*/
pub enum MouseButton {
    Left,
    Right,
    Middle
}

/**
    Text align constant on the horizontal axis
*/
#[derive(PartialEq, Debug, Clone)]
pub enum HTextAlign {
    Left,
    Center,
    Right
}

/**
    Text align constant on the vertical axis
*/
#[derive(PartialEq, Debug, Clone)]
pub enum VTextAlign {
    Top,
    Center,
    Bottom
}

/**
    Enum of message box buttons (to use with `MessageParams` )
*/
#[derive(Clone, PartialEq, Debug)]
pub enum MessageButtons {
    AbortTryIgnore,
    CancelTryContinue,
    Ok,
    OkCancel,
    RetryCancel,
    YesNo,
    YesNoCancel,
}

/**
    Enum of message box icons (to use with `MessageParams` )
*/
#[derive(Clone, PartialEq, Debug)]
pub enum MessageIcons {
    Warning,
    Info,
    Question,
    Error,
    None
}

/**
    Return value of `message`. Define the button that the user clicked. If the user 
    cancelled the message box by clicking on X button of the window, `MessageChoice::Cancel` is returned.
*/
#[derive(Clone, PartialEq, Debug)]
pub enum MessageChoice {
    Abort,
    Cancel,
    Continue,
    Ignore,
    No,
    Ok,
    Retry,
    TryAgain,
    Yes
}

/**
    A structure that defines how a messagebox should look and behave. 

    Members:  
    * `title`: The title of the message box  
    * `content`: The message of the message box  
    * `buttons`: The button of the message box  
    * `icons`: The message box icon  
*/
#[derive(Clone, PartialEq, Debug)]
pub struct MessageParams<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub buttons: MessageButtons,
    pub icons: MessageIcons
}

/**
    A enum that dictates how a file dialog should behave

    Members:  
    * `Open`: User can select a file that is not a directory  
    * `OpenDirectory`: User can select a directory  
    * `Save`: User select the name of a file. If it already exists, a confirmation message will be raised  
*/
#[derive(Clone, PartialEq, Debug)]
pub enum FileDialogAction {
    Open,
    OpenDirectory,
    Save,
}

/**
    Define the state of a progress bar

    Members:  
    * `Normal`: Default state of a progress bar  (in progress)
    * `Paused`: Paused  
    * `Error`: Error   
*/
#[derive(Clone, PartialEq, Debug)]
pub enum ProgressBarState {
    Normal,
    Paused,
    Error,
}

/**
    Define a rectangle shape that can be used with canvases
*/
#[derive(Clone)]
pub struct Rectangle {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32
}

/**
    Define a ellipse shape that can be used with canvases
*/
#[derive(Clone)]
pub struct Ellipse {
    pub center: (f32, f32),
    pub radius: (f32, f32),
}

/**
    A brush using a single solid color. Used when painting in a canvas

    Members:  
    • `color`: The brush color (red, green, blue, alpha)
*/
#[derive(Clone)]
pub struct SolidBrush {
    pub color: (f32, f32, f32, f32)
}

/**
    Cap style used when creating a Pen
*/
#[derive(Clone, Debug)]
pub enum CapStyle {
    Flat = 0,
    Square = 1,
    Round = 2,
    Triangle = 3
}

/**
    Line join type used when creating a Pen
*/
#[derive(Clone, Debug)]
pub enum LineJoin {
    D2d1LineJoinMiter = 0,
    D2d1LineJoinBevel = 1,
    D2d1LineJoinRound = 2,
    D2d1LineJoinMiterOrBevel = 3
}

/**
    Dash style used when creating a Pen
*/
#[derive(Clone, Debug)]
pub enum DashStyle {
    D2d1DashStyleSolid = 0,
    D2d1DashStyleDash = 1,
    D2d1DashStyleDot = 2,
    D2d1DashStyleDashDot = 3,
    D2d1DashStyleDashDotDot = 4,
}

/**
    Describe how lines should be painted. Used when painting in a canvas
    
    Members:  
    • `start_cap`: The cap applied to the start of all the open figures in a stroked geometry. 
    • `end_cap`: The cap applied to the end of all the open figures in a stroked geometry.
    • `dash_cap`: The shape at either end of each dash segment.
    • `line_join`: A value that describes how segments are joined. This value is ignored for a vertex if the segment flags specify that the segment should have a smooth join. 
    • `miter_limit`: The limit of the thickness of the join on a mitered corner. This value is always treated as though it is greater than or equal to 1.0f. 
    • `dash_style`: A value that specifies whether the stroke has a dash pattern and, if so, the dash style. 
    • `dash_offset`: A value that specifies an offset in the dash sequence. A positive dash offset value shifts the dash pattern, in units of stroke width,  
       toward the start of the stroked geometry. A negative dash offset value shifts the dash pattern, in units of stroke width, toward the end of the stroked geometry.
*/
#[derive(Clone, Debug)]
pub struct Pen {
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub dash_cap: CapStyle,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_style: DashStyle,
    pub dash_offset: f32,
}


// Special window position constants
pub const DEFAULT_POSITION: c_int = CW_USEDEFAULT;
pub const CENTER_POSITION: c_int = CW_USEDEFAULT + 1;

// Special window size contants
pub const DEFAULT_SIZE: c_int = CW_USEDEFAULT;

// Font weight enum
pub const FONT_WEIGHT_DONTCARE: c_int = 0;
pub const FONT_WEIGHT_THIN: c_int = 100;
pub const FONT_WEIGHT_EXTRALIGHT: c_int = 200;
pub const FONT_WEIGHT_LIGHT: c_int = 300;
pub const FONT_WEIGHT_NORMAL: c_int = 400;
pub const FONT_WEIGHT_MEDIUM: c_int = 500;
pub const FONT_WEIGHT_SEMIBOLD: c_int = 600;
pub const FONT_WEIGHT_BOLD: c_int = 700;
pub const FONT_WEIGHT_EXTRABOLD: c_int = 800;
pub const FONT_WEIGHT_BLACK: c_int = 900;

// Font decoration constants
pub const FONT_DECO_NORMAL: DWORD = 0x00;
pub const FONT_DECO_ITALIC: DWORD = 0x01;
pub const FONT_DECO_UNDERLINE: DWORD = 0x02;
pub const FONT_DECO_STRIKEOUT: DWORD = 0x04;

pub mod keys {
    //! Windows virtual key code
    
    pub const BACK: u32 = 0x08;
    pub const TAB: u32 = 0x09;
    pub const CLEAR: u32 = 0x0C;
    pub const RETURN: u32 = 0x0D;
    pub const SHIFT: u32 = 0x10;
    pub const CONTROL: u32 = 0x11;
    pub const ALT: u32 = 0x12;
    pub const PAUSE: u32 = 0x13;
    pub const CAPITAL: u32 = 0x14;
    pub const KANA: u32 = 0x15;
    pub const HANGUEL: u32 = 0x15;
    pub const HANGUL: u32 = 0x15;
    pub const JUNJA: u32 = 0x17;
    pub const FINAL: u32 = 0x18;
    pub const HANJA: u32 = 0x19;
    pub const KANJI: u32 = 0x19;
    pub const ESCAPE: u32 = 0x1B;
    pub const CONVERT: u32 = 0x1C;
    pub const NONCONVERT: u32 = 0x1D;
    pub const ACCEPT: u32 = 0x1E;
    pub const MODECHANGE: u32 = 0x1F;
    pub const SPACE: u32 = 0x20;
    pub const PRIOR: u32 = 0x21;
    pub const NEXT: u32 = 0x22;
    pub const END: u32 = 0x23;
    pub const HOME: u32 = 0x24;
    pub const LEFT: u32 = 0x25;
    pub const UP: u32 = 0x26;
    pub const RIGHT: u32 = 0x27;
    pub const DOWN: u32 = 0x28;
    pub const SELECT: u32 = 0x29;
    pub const PRINT: u32 = 0x2A;
    pub const EXECUTE: u32 = 0x2B;
    pub const SNAPSHOT: u32 = 0x2C;
    pub const INSERT: u32 = 0x2D;
    pub const DELETE: u32 = 0x2E;
    pub const HELP: u32 = 0x2F;
    pub const _0: u32 = 0x30;
    pub const _1: u32 = 0x31;
    pub const _2: u32 = 0x32;
    pub const _3: u32 = 0x33;
    pub const _4: u32 = 0x34;
    pub const _5: u32 = 0x35;
    pub const _6: u32 = 0x36;
    pub const _7: u32 = 0x37;
    pub const _8: u32 = 0x38;
    pub const _9: u32 = 0x39;
    pub const _A: u32 = 0x41;
    pub const _B: u32 = 0x42;
    pub const _C: u32 = 0x43;
    pub const _D: u32 = 0x44;
    pub const _E: u32 = 0x45;
    pub const _F: u32 = 0x46;
    pub const _G: u32 = 0x47;
    pub const _H: u32 = 0x48;
    pub const _I: u32 = 0x49;
    pub const _J: u32 = 0x4A;
    pub const _K: u32 = 0x4B;
    pub const _L: u32 = 0x4C;
    pub const _M: u32 = 0x4D;
    pub const _N: u32 = 0x4E;
    pub const _O: u32 = 0x4F;
    pub const _P: u32 = 0x50;
    pub const _Q: u32 = 0x51;
    pub const _R: u32 = 0x52;
    pub const _S: u32 = 0x53;
    pub const _T: u32 = 0x54;
    pub const _U: u32 = 0x55;
    pub const _V: u32 = 0x56;
    pub const _W: u32 = 0x57;
    pub const _X: u32 = 0x58;
    pub const _Y: u32 = 0x59;
    pub const _Z: u32 = 0x5A;
    pub const LWIN: u32 = 0x5B;
    pub const RWIN: u32 = 0x5C;
    pub const APPS: u32 = 0x5D;
    pub const SLEEP: u32 = 0x5F;
    pub const NUMPAD0: u32 = 0x60;
    pub const NUMPAD1: u32 = 0x61;
    pub const NUMPAD2: u32 = 0x62;
    pub const NUMPAD3: u32 = 0x63;
    pub const NUMPAD4: u32 = 0x64;
    pub const NUMPAD5: u32 = 0x65;
    pub const NUMPAD6: u32 = 0x66;
    pub const NUMPAD7: u32 = 0x67;
    pub const NUMPAD8: u32 = 0x68;
    pub const NUMPAD9: u32 = 0x69;
    pub const MULTIPLY: u32 = 0x6A;
    pub const ADD: u32 = 0x6B;
    pub const SEPARATOR: u32 = 0x6C;
    pub const SUBTRACT: u32 = 0x6D;
    pub const DECIMAL: u32 = 0x6E;
    pub const DIVIDE: u32 = 0x6F;
    pub const F1: u32 = 0x70;
    pub const F2: u32 = 0x71;
    pub const F3: u32 = 0x72;
    pub const F4: u32 = 0x73;
    pub const F5: u32 = 0x74;
    pub const F6: u32 = 0x75;
    pub const F7: u32 = 0x76;
    pub const F8: u32 = 0x77;
    pub const F9: u32 = 0x78;
    pub const F10: u32 = 0x79;
    pub const F11: u32 = 0x7A;
    pub const F12: u32 = 0x7B;
    pub const F13: u32 = 0x7C;
    pub const F14: u32 = 0x7D;
    pub const F15: u32 = 0x7E;
    pub const F16: u32 = 0x7F;
    pub const F17: u32 = 0x80;
    pub const F18: u32 = 0x81;
    pub const F19: u32 = 0x82;
    pub const F20: u32 = 0x83;
    pub const F21: u32 = 0x84;
    pub const F22: u32 = 0x85;
    pub const F23: u32 = 0x86;
    pub const F24: u32 = 0x87;
    pub const NUMLOCK: u32 = 0x90;
    pub const SCROLL: u32 = 0x91;
    pub const OEM_25: u32 = 0x92;
    pub const OEM_26: u32 = 0x93;
    pub const OEM_27: u32 = 0x94;
    pub const OEM_28: u32 = 0x95;
    pub const OEM_29: u32 = 0x96;
    pub const LSHIFT: u32 = 0xA0;
    pub const RSHIFT: u32 = 0xA1;
    pub const LCONTROL: u32 = 0xA2;
    pub const RCONTROL: u32 = 0xA3;
    pub const LMENU: u32 = 0xA4;
    pub const RMENU: u32 = 0xA5;
    pub const BROWSER_BACK: u32 = 0xA6;
    pub const BROWSER_FORWARD: u32 = 0xA7;
    pub const BROWSER_REFRESH: u32 = 0xA8;
    pub const BROWSER_STOP: u32 = 0xA9;
    pub const BROWSER_SEARCH: u32 = 0xAA;
    pub const BROWSER_FAVORITES: u32 = 0xAB;
    pub const BROWSER_HOME: u32 = 0xAC;
    pub const VOLUME_MUTE: u32 = 0xAD;
    pub const VOLUME_DOWN: u32 = 0xAE;
    pub const VOLUME_UP: u32 = 0xAF;
    pub const MEDIA_NEXT_TRACK: u32 = 0xB0;
    pub const MEDIA_PREV_TRACK: u32 = 0xB1;
    pub const MEDIA_STOP: u32 = 0xB2;
    pub const MEDIA_PLAY_PAUSE: u32 = 0xB3;
    pub const LAUNCH_MAIL: u32 = 0xB4;
    pub const LAUNCH_MEDIA_SELECT: u32 = 0xB5;
    pub const LAUNCH_APP1: u32 = 0xB6;
    pub const LAUNCH_APP2: u32 = 0xB7;
    pub const OEM_1: u32 = 0xBA;
    pub const OEM_PLUS: u32 = 0xBB;
    pub const OEM_COMMA: u32 = 0xBC;
    pub const OEM_MINUS: u32 = 0xBD;
    pub const OEM_PERIOD: u32 = 0xBE;
    pub const OEM_2: u32 = 0xBF;
    pub const OEM_3: u32 = 0xC0;
    pub const OEM_4: u32 = 0xDB;
    pub const OEM_5: u32 = 0xDC;
    pub const OEM_6: u32 = 0xDD;
    pub const OEM_7: u32 = 0xDE;
    pub const OEM_8: u32 = 0xDF;
    pub const OEM_9: u32 = 0xE1;
    pub const OEM_102: u32 = 0xE2;
    pub const OEM_10: u32 = 0xE3;
    pub const OEM_11: u32 = 0xE4;
    pub const PROCESSKEY: u32 = 0xE5;
    pub const OEM_X: u32 = 0xE6;
    pub const PACKET: u32 = 0xE7;
    pub const OEM_12: u32 = 0xE9;
    pub const OEM_13: u32 = 0xEA;
    pub const OEM_14: u32 = 0xEB;
    pub const OEM_15: u32 = 0xEC;
    pub const OEM_16: u32 = 0xED;
    pub const OEM_17: u32 = 0xEE;
    pub const OEM_18: u32 = 0xEF;
    pub const OEM_19: u32 = 0xF0;
    pub const OEM_20: u32 = 0xF1;
    pub const OEM_21: u32 = 0xF2;
    pub const OEM_22: u32 = 0xF3;
    pub const OEM_23: u32 = 0xF4;
    pub const OEM_24: u32 = 0xF5;
    pub const ATTN: u32 = 0xF6;
    pub const CRSEL: u32 = 0xF7;
    pub const EXSEL: u32 = 0xF8;
    pub const EREOF: u32 = 0xF9;
    pub const PLAY: u32 = 0xFA;
    pub const ZOOM: u32 = 0xFB;
    pub const NONAME: u32 = 0xFC;
    pub const PA1: u32 = 0xFD;
    pub const OEM_CLEAR: u32 = 0xFE;
}