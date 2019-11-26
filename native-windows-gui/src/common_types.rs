/**
    Text align constant on the horizontal axis
*/
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum HTextAlign {
    Left,
    Center,
    Right
}

/**
    Text align constant on the vertical axis
*/
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum VTextAlign {
    Top,
    Center,
    Bottom
}

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
