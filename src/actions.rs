/*!
    Actions is an enumeration of any actions that is possible to do 
    on the controls. 

    No controls implement all Actions.
*/
use std::hash::Hash;
use constants::{CheckState, WindowDisplay};

#[derive(PartialEq)]
pub struct ActMessageParams {
    pub title: String,
    pub content: String,
    pub type_: u32
}

/**
    Possible message to send to an Ui
*/
#[derive(PartialEq)]
pub enum Action<ID: Eq+Clone+Hash> {
    None,
    
    GetParent,
    SetParent(Box<Option<ID>>),
    GetChildren,
    GetDescendants,
    
    GetPosition,
    SetPosition(i32, i32),
    
    GetSize,
    SetSize(u32, u32),
    
    GetText,
    SetText(Box<String>),
    
    GetCheckState,
    SetCheckState(CheckState),
    
    GetEnabled,
    SetEnabled(bool),
    
    GetVisibility,
    SetVisibility(bool),
    
    GetWindowDisplay,
    SetWindowDisplay(WindowDisplay),
    
    GetTextLimit,
    SetTextLimit(u32),
    
    GetSelectedBounds,
    SetSelectedBounds((u32, u32)),
    
    GetReadonly,
    SetReadonly(bool),
    
    GetSelectedIndex,
    SetSelectedIndex(Option<u32>),

    GetPlaceholder,
    SetPlaceholder(Box<Option<String>>),
    
    Reset,

    GetDropdownVisibility,
    SetDropdownVisibility(bool),
    
    AddString(Box<String>),
    RemoveString(Box<String>),
    FindString(Box<String>),
    
    GetStringCollection,
    SetStringCollection(Box<Vec<String>>),

    CountItems,
    GetIndexedItem(u32),
    RemoveIndexedItem(u32),

    Undo,
    Message(Box<ActMessageParams>)
}

/**
    Possible values returned by message sent to an Ui
*/
#[derive(PartialEq)]
pub enum ActionReturn<ID: Eq+Clone+Hash> {
    None,
    Parent(Box<Option<ID>>),
    Children(Box<Vec<ID>>),
    Position(i32, i32),
    Size(u32, u32),
    Text(Box<String>),
    Error(::constants::Error),
    CheckState(CheckState),
    Enabled(bool),
    Visibility(bool),
    WindowDisplay(WindowDisplay),
    TextLimit(u32),
    SelectBounds((u32, u32)),
    Readonly(bool),
    ItemIndex(u32),
    ItemCount(u32),
    StringCollection(Box<Vec<String>>),
    NotSupported
}


pub mod helper {
    use actions::{Action, ActMessageParams};
    use std::hash::Hash;

    /**
        Action helper for the Message action.
    */
    #[inline(always)]
    pub fn message<ID: Eq+Clone+Hash, S: Into<String>>(title: S, content: S, type_: u32) -> Action<ID> {
        Action::Message(Box::new(ActMessageParams{
            title: title.into(),
            content: content.into(),
            type_: type_
        }))
    }

    /**
        Action helper for the SetText action.
    */
    #[inline(always)]
    pub fn set_text<ID: Eq+Clone+Hash, S: Into<String>>(text: S) -> Action<ID> {
        Action::SetText(Box::new(text.into()))
    }

    /**
        Action helper for the SetParent action.
    */
    #[inline(always)]
    pub fn set_parent<ID: Eq+Clone+Hash>(p: ID) -> Action<ID> {
        Action::SetParent(Box::new(Some(p)))
    }

    /**
        Action helper for the SetParent action.
    */
    #[inline(always)]
    pub fn remove_parent<ID: Eq+Clone+Hash>() -> Action<ID> {
        Action::SetParent(Box::new(None))
    }

    /**
        Action helper for AddString
    */
    #[inline(always)]
    pub fn add_string<ID: Eq+Clone+Hash, S: Into<String>>(s: S) -> Action<ID> {
        Action::AddString(Box::new(s.into()))
    }

    /**
        Action helper for FindString
    */
    #[inline(always)]
    pub fn find_string<ID: Eq+Clone+Hash, S: Into<String>>(s: S) -> Action<ID> {
        Action::FindString(Box::new(s.into()))
    }

    /**
        Action helper for RemoveString
    */
    #[inline(always)]
    pub fn remove_string<ID: Eq+Clone+Hash, S: Into<String>>(s: S) -> Action<ID> {
        Action::RemoveString(Box::new(s.into()))
    }

    /**
        Action helper for SetSelectedIndex
    */
    #[inline(always)]
    pub fn set_selected_index<ID: Eq+Clone+Hash>(i: u32) -> Action<ID> {
        Action::SetSelectedIndex(Some(i))
    }

    /**
        Action helper for SetSelectedIndex
    */
    #[inline(always)]
    pub fn remove_selected<ID: Eq+Clone+Hash>() -> Action<ID> {
        Action::SetSelectedIndex(None)
    }

    /**
        Action helper to set a placeholder on a control
    */
    #[inline(always)]
    pub fn set_placeholder<ID: Eq+Clone+Hash, S: Into<String>>(s: S) -> Action<ID> {
        Action::SetPlaceholder(Box::new(Some(s.into())))
    }

    /**
        Action helper to set a placeholder on a control
    */
    #[inline(always)]
    pub fn remove_placeholder<ID: Eq+Clone+Hash>() -> Action<ID> {
        Action::SetPlaceholder(Box::new(None))
    }

}