use std::hash::{Hash, Hasher};
use winapi::shared::windef::{HWND};


#[derive(Debug, Clone)]
pub enum ControlHandle {
    NoHandle,
    Hwnd(HWND)
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
            &ControlHandle::NoHandle => None,
            &ControlHandle::Hwnd(h) => Some(h)
        }
    }

}


impl Default for ControlHandle {

    fn default() -> ControlHandle {
        ControlHandle::NoHandle
    }

}

impl Hash for ControlHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &ControlHandle::NoHandle => 0.hash(state),
            &ControlHandle::Hwnd(value) => {
                (1, (value as usize)).hash(state);
            },
        }
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
            &ControlHandle::Hwnd(value1) => match other {
                &ControlHandle::Hwnd(value2) => value1 == value2,
                _ => false
            }
        }
    }
}

impl Eq for ControlHandle {}
