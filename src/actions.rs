/*!
    Actions is an enumeration of any actions that is possible to do 
    on the controls. 

    No controls implement all Actions.
*/

pub struct ActMessageParams {
    title: String,
    content: String,
    type_: u32
}

/**
    Possible message to send to an Ui
*/
pub enum Action {
    Message(Box<ActMessageParams>)
}

/**
    Possible values return by message sent to an Ui
*/
pub enum ActionReturn {
    None
}


pub mod helper {
    use actions::{Action, ActMessageParams};

    /**
        Action helper for the message action.
    */
    pub fn message<S: Into<String>>(title: S, content: S, type_: u32) -> Action {
        Action::Message(Box::new(ActMessageParams{
            title: title.into(),
            content: content.into(),
            type_: type_
        }))
    }

}