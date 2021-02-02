use super::ParserError;
use std::path::{Path, PathBuf};


/**
    Represents a native-windows-gui GUI struct in a file
*/
pub struct GuiStruct {
    /// Path to the rust source file containing the gui struct
    path: PathBuf
}

impl GuiStruct {

    pub fn find(path: &Path) -> Result<GuiStructFinder, ParserError> {
        let finder = GuiStructFinder {

        };

        Ok(finder)
    }

}


pub struct GuiStructFinder {

}

impl Iterator for GuiStructFinder {
    type Item = Result<GuiStruct, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }

}
