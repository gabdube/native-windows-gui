/*!
    Functions to parse rust files and extract nwg gui struct.
    There is currently a limit of 1 gui struct per file.
*/
mod parser_error;
pub use parser_error::ParserError;

mod gui_struct;
pub use gui_struct::GuiStruct;

mod gui_struct_finder;
pub use gui_struct_finder::GuiStructFinder;

use std::path::Path;

/**
    Parse a gui struct from a file.

    Returns `None` if no struct was found in the file.
    May return an error if the file cannot be read or if the parsing fails
*/
pub fn parse<P: AsRef<Path>>(path: P) -> Result<Option<GuiStruct>, ParserError> {
    let mut finder = GuiStruct::find(path.as_ref())?;
    match finder.next() {
        None => Ok(None),
        Some(gui) => Ok(Some(gui)),
    }
}

/// Helpers to check if a file already has a GUI struct defined
pub fn has_gui_struct<P: AsRef<Path>>(path: P) -> bool {
    parse(path)
        .map(|s| s.is_some())
        .unwrap_or(false)
}
