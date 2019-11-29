/*!
    Collections of errors that can be returned by the direct draw wrappers
*/
use winapi::shared::ntdef::HRESULT;


#[derive(Copy, Clone, Debug)]
pub enum WriteError {
    MissingParameter(&'static str),
    Unknown(HRESULT)
}
