/*!
    Actions is an enumeration of any actions that is possible to do 
    on the controls. 

    No controls implement all Actions.
*/
use std::hash::Hash;

pub struct ActMessageParams {
    pub title: String,
    pub content: String,
    pub type_: u32
}

/**
    Possible message to send to an Ui
*/
pub enum Action<ID: Eq+Clone+Hash> {
    None,
    GetParent,
    SetParent(Box<Option<ID>>),
    GetPosition,
    SetPosition(i32, i32),
    GetSize,
    SetSize(u32, u32),
    GetText,
    SetText(Box<String>),
    Message(Box<ActMessageParams>)
}

/**
    Possible values returned by message sent to an Ui
*/
pub enum ActionReturn<ID: Eq+Clone+Hash> {
    None,
    Parent(Box<Option<ID>>),
    Position(i32, i32),
    Size(u32, u32),
    Text(Box<String>),
    Error(u32),
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

}