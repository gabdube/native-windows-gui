use winapi::shared::windef::{HWND, HMENU};


#[derive(Debug, Clone, Copy)]
pub enum ControlHandle {
    NoHandle,
    Hwnd(HWND),

    /// (Parent menu / Menu). 
    /// Parent menu must be there as WINAPI does not have any function to fetch the parent
    Menu(HMENU, HMENU),

    /// (Parent window / Menu). 
    PopMenu(HWND, HMENU),

    /// (Parent menu / Unique ID). 
    MenuItem(HMENU, u32),

    /// Timer / Notice handle / Tray notification handle
    Other(HWND, u32)
}

impl ControlHandle {

    pub fn blank(&self) -> bool {
        match self {
            &ControlHandle::NoHandle => true,
            _ => false
        }
    }

    pub fn hwnd(&self) -> Option<HWND> {
        match self {
            &ControlHandle::Hwnd(h) => Some(h),
            _ => None,
        }
    }

    pub fn hmenu(&self) -> Option<(HMENU, HMENU)> {
        match self {
            &ControlHandle::Menu(h1, h2) => Some((h1, h2)),
            _ => None,
        }
    }

    pub fn pop_hmenu(&self) -> Option<(HWND, HMENU)> {
        match self {
            &ControlHandle::PopMenu(h1, h2) => Some((h1, h2)),
            _ => None,
        }
    }

    pub fn hmenu_item(&self) -> Option<(HMENU, u32)> {
        match self {
            &ControlHandle::MenuItem(h, i) => Some((h, i)),
            _ => None,
        }
    }

    pub fn other(&self) -> Option<(HWND, u32)> {
        match self {
            &ControlHandle::Other(h, i) => Some((h, i)),
            _ => None,
        }
    }
}


impl Default for ControlHandle {

    fn default() -> ControlHandle {
        ControlHandle::NoHandle
    }

}

impl PartialEq for ControlHandle {
    fn eq(&self, other: &Self) -> bool {
        match self {
            // NoHandle
            &ControlHandle::NoHandle => match other {
                &ControlHandle::NoHandle => true,
                _ => false
            },
            // HWND
            &ControlHandle::Hwnd(hwnd1) => match other {
                &ControlHandle::Hwnd(hwnd2) => hwnd1 == hwnd2,
                _ => false
            },
            // HMENU
            &ControlHandle::Menu(h1, h2) => match other {
                &ControlHandle::Menu(h3, h4) => h1 == h3 && h2 == h4,
                _ => false
            },
            // HMENU
            &ControlHandle::PopMenu(_, h1) => match other {
                &ControlHandle::Menu(_, h2) => h1 == h2,
                _ => false
            },
            // HMENU / ITEM
            &ControlHandle::MenuItem(value1, id1) => match other {
                &ControlHandle::MenuItem(value2, id2) => value1 == value2 && id1 == id2,
                _ => false
            },
            // TIMER / Notice / Tray notification
            &ControlHandle::Other(hwnd1, id1) => match other {
                &ControlHandle::Other(hwnd2, id2) => hwnd1 == hwnd2 && id1 == id2,
                _ => false
            }
        }
    }
}

impl Eq for ControlHandle {}

impl From<&ControlHandle> for ControlHandle {
    fn from(control: &ControlHandle) -> Self { *control }
}
