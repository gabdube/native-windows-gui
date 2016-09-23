/*!
    Actions is an enumeration of any actions that is possible to do 
    on the controls. 

    No controls implement all Actions.
*/

pub struct ActMessageParams {
    pub title: String,
    pub content: String,
    pub type_: u32
}

/**
    Possible message to send to an Ui
*/
pub enum Action {
    GetPosition,
    SetPosition(i32, i32),
    GetText,
    SetText(Box<String>),
    Message(Box<ActMessageParams>)
}

/**
    Possible values return by message sent to an Ui
*/
pub enum ActionReturn {
    None,
    Position(i32, i32),
    Text(Box<String>),
    NotSupported
}


pub mod helper {
    use actions::{Action, ActMessageParams};

    /**
        Action helper for the Message action.
    */
    #[inline(always)]
    pub fn message<S: Into<String>>(title: S, content: S, type_: u32) -> Action {
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
    pub fn set_text<S: Into<String>>(text: S) -> Action {
        Action::SetText(Box::new(text.into()))
    }

}