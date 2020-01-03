use winapi::shared::{minwindef::DWORD, windef::COLORREF};
use winapi::um::commdlg::{CHOOSECOLORW, CC_RGBINIT, ChooseColorW};
use winapi::um::wingdi::{GetBValue, GetRValue, GetGValue, RGB};
use crate::controls::ControlHandle;
use crate::NwgError;
use std::cell::{RefCell};
use std::{ptr, mem};
use std::pin::Pin;


struct InnerColorDialog {
    custom_colors: Pin<Box<[COLORREF; 16]>>,
    dialog: CHOOSECOLORW,
}

/**
 Displays a modal dialog box that allows the user to choose a specific color value.
*/
pub struct ColorDialog {
    data: RefCell<InnerColorDialog>,
}

impl ColorDialog {

    pub fn builder() -> ColorDialogBuilder {
        ColorDialogBuilder {
            default_colors: Default::default()
        }
    }

    /**
    Execute the color dialog.
    This function will return `true` if the user select a color or `false` if the dialog is cancelled
    */
    pub fn run<C: Into<ControlHandle>>(&self, owner: Option<C>) -> bool {
        if owner.is_some() {
            let ownder_handle = owner.unwrap().into();
            self.data.borrow_mut().dialog.hwndOwner = ownder_handle.hwnd().expect("Color dialog must be a window control");
        }

        unsafe {
            let mut data = self.data.borrow_mut();
            let data_ref = &mut data.dialog;
            ChooseColorW(data_ref as *mut CHOOSECOLORW) > 0
        }
    }

    /**
    Return the color choosen by the user. The returned color is a [r, g, b] array.
    If the dialog was never executed, this returns `[0, 0, 0]` (black);
    */
    pub fn color(&self) -> [u8; 3] {
        let v = self.data.borrow().dialog.rgbResult;
        [GetRValue(v), GetGValue(v), GetBValue(v)]
    }

    /**
        Sets one of the saved color in the dialog. A dialog supports up to 16 colors (index: 0 to 15).

        Panics:
            - If the index is out of bound
    */
    pub fn set_saved_color(&self, index: usize, color: &[u8; 3]) {
        if index > 15 { panic!("{:?} is outside the dialog saved color bounds", index); }
        self.data.borrow_mut().custom_colors[index] = RGB(color[0], color[1], color[2]);
    }

    /**
        Returns one of the saved color in the dialog. A dialog supports up to 16 colors (index: 0 to 15).

        Panics:
            - If the index is out of bound
    */
    pub fn saved_color(&self, index: usize) -> [u8; 3] {
        if index > 15 { panic!("{:?} is outside the dialog saved color bounds", index); }
        let v = self.data.borrow().custom_colors[index];
        [GetRValue(v), GetGValue(v), GetBValue(v)]
    }

}

/// The builder for a `ColorDialog` object. Use `ColorDialog::builder` to create one.
pub struct ColorDialogBuilder {
    default_colors: [COLORREF; 16]
}

impl ColorDialogBuilder {

    pub fn saved_color(mut self, index: usize, color: &[u8; 3]) -> ColorDialogBuilder {
        self.default_colors[index] = RGB(color[0], color[1], color[2]);
        self
    }

    pub fn build(self, out: &mut ColorDialog) -> Result<(), NwgError> {
        *out.data.borrow_mut().custom_colors.as_mut() = self.default_colors;
        Ok(())
    }

}

impl Default for ColorDialog {

    fn default() -> ColorDialog {
        let dialog = CHOOSECOLORW {
            lStructSize: mem::size_of::<CHOOSECOLORW>() as DWORD,
            hwndOwner: ptr::null_mut(),
            hInstance: ptr::null_mut(),
            rgbResult: 0,
            lpCustColors: ptr::null_mut(),
            Flags: CC_RGBINIT,
            lCustData: 0,
            lpfnHook: None,
            lpTemplateName: ptr::null()
        };

        let mut inner = InnerColorDialog {
            custom_colors: Box::pin(Default::default()),
            dialog
        };

        let mut cols = inner.custom_colors.as_mut();
        let cols_ref: &mut [COLORREF; 16] = &mut cols;
        inner.dialog.lpCustColors = cols_ref as *mut [COLORREF; 16] as *mut COLORREF;

        ColorDialog {
            data: RefCell::new(inner)
        }
    }

}

