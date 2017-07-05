/*!
    Control trait definition. The base control definitions are located in the submodules.
*/

pub mod window;
pub mod menu;
pub mod button;
pub mod textinput;
pub mod textbox;
pub mod checkbox;
pub mod radiobutton;
pub mod label;
pub mod listbox;
pub mod combobox;
pub mod groupbox;
pub mod progress_bar;
pub mod datepicker;
pub mod image_frame;
pub mod file_dialog;
pub mod timer;
pub mod canvas;
pub mod treeview;
pub mod frame;

use std::any::TypeId;
use std::hash::Hash;

use winapi::{HWND, HANDLE, HCURSOR, HICON, HMENU, UINT, HFONT, HTREEITEM};

pub use controls::window::{WindowT, Window};
pub use controls::menu::{MenuT, Menu, MenuItemT, MenuItem, SeparatorT, Separator, ContextMenuT, ContextMenu};
pub use controls::button::{ButtonT, Button};
pub use controls::textinput::{TextInputT, TextInput};
pub use controls::textbox::{TextBoxT, TextBox};
pub use controls::checkbox::{CheckBoxT, CheckBox};
pub use controls::radiobutton::{RadioButtonT, RadioButton};
pub use controls::label::{LabelT, Label};
pub use controls::listbox::{ListBoxT, ListBox};
pub use controls::combobox::{ComboBoxT, ComboBox};
pub use controls::groupbox::{GroupBoxT, GroupBox};
pub use controls::progress_bar::{ProgressBarT, ProgressBar};
pub use controls::file_dialog::{FileDialogT, FileDialog};
pub use controls::timer::{TimerT, Timer};
pub use controls::canvas::{CanvasT, Canvas, CanvasRenderer};
pub use controls::datepicker::{DatePickerT, DatePicker};
pub use controls::image_frame::{ImageFrameT, ImageFrame};
pub use controls::treeview::{TreeViewT, TreeView, TreeViewItemT, TreeViewItem};
pub use controls::frame::{FrameT, Frame};
use ui::Ui;
use error::Error;

/**
    A type that expose the different underlying handle into one type
*/
#[derive(Clone, Hash, Debug)]
#[allow(non_camel_case_types)]
pub enum AnyHandle {
    HWND(HWND),
    HMENU(HMENU),
    HMENU_ITEM(HMENU, UINT),
    HTREE_ITEM(HTREEITEM, HWND),
    HFONT(HFONT),
    HCURSOR(HCURSOR),
    HICON(HICON),
    HANDLE(HANDLE, HandleSpec),
    Custom(TypeId, usize)
}

impl AnyHandle {
    pub fn human_name(&self) -> String {
        match self {
            &AnyHandle::HWND(_) => "Window-like",
            &AnyHandle::HMENU(_) => "Menu",
            &AnyHandle::HMENU_ITEM(_,_) => "Menu item",
            &AnyHandle::HTREE_ITEM(_,_) => "TreeView item",
            &AnyHandle::HFONT(_) => "Font",
            &AnyHandle::HCURSOR(_) => "Cursor",
            &AnyHandle::HICON(_) => "Icon",
            &AnyHandle::HANDLE(_, ref s) => match s {
                &HandleSpec::Bitmap => "Bitmap"
            },
            &AnyHandle::Custom(_, _) => "Custom"
        }.to_string()
    }
}

/**
    Because the `HANDLE` type is too generic and can be used by alot of different resources,
    NWG store additinal information to remember the type of resource it holds
*/
#[derive(Clone, Hash, Debug)]
pub enum HandleSpec {
    Bitmap
}

/**
    An enum that list type names for the common controls.
*/
#[derive(Clone, Debug, PartialEq)]
pub enum ControlType {
    Window,
    Button,
    TextInput,
    TextBox,
    CheckBox,
    RadioButton,
    ListBox,
    Label,
    ProgressBar,
    Menu,
    MenuItem,
    Timer,
    ComboBox,
    GroupBox,
    NumericInput,
    DatePicker,
    FileDialog,
    Canvas,
    ImageFrame,
    TreeView,
    TreeViewItem,
    Frame,
    Undefined  // Control is not a common control
}

/**
    Structures implementing this trait can be used by a Ui to build a Control
*/
pub trait ControlT<ID: Clone+Hash> {

    /**
        Should return the TypeId of the generated control. For example a `WindowT` struct returns the TypeId of a `Window` struct.
    */
    fn type_id(&self) -> TypeId;

    /**
        Should instance the control and return it as a Box<Control>. If an error is raised, it will be returned by `ui.commit`.
    */
    fn build(&self, ui: &Ui<ID>) -> Result<Box<Control>, Error>;

}

/**
    Structures implementing this trait are controls that can be stored in a Ui
*/
pub trait Control {

    /**
        Should return the underlying handle to the object
    */
    fn handle(&self) -> AnyHandle;

    /**
        Should return the type of the control. For custom controls, the return value should be `Undefined` (the default).
    */
    fn control_type(&self) -> ControlType { ControlType::Undefined }

    /**
        Should return a list of any children control.  
        Called when unpacking a control to get its children. Handle that do not belong to NWG will be ignored.
    */
    fn children(&self) -> Vec<AnyHandle> { Vec::new() }


    /**
        If specified, should free any ressource allocated in the template `build` function. This includes functions like `DestroyWindow`.
    */
    fn free(&mut self) {}

}