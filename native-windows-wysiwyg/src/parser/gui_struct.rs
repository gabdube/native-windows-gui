use super::{ParserError, GuiStructFinder};
use std::{fs, path::{Path, PathBuf}, time::SystemTime};


/**
    Represents a native-windows-gui GUI struct in a file
*/
pub struct GuiStruct {
    /// Path to the rust source file containing the gui struct
    pub(super) path: PathBuf,

    /// Last modified time of the source file
    pub(super) accessed: SystemTime,

    /// Data in the gui struct
    pub(super) data: syn::ItemStruct,
}

impl GuiStruct {

    pub fn find(path: &Path) -> Result<GuiStructFinder, ParserError> {
        use std::str::FromStr;

        let meta = fs::metadata(path)?;

        let content = fs::read_to_string(path)?;
        let src = proc_macro2::TokenStream::from_str(&content)?;
        let iter = src.into_iter().peekable();
        
        let finder = GuiStructFinder {
            path: path.to_owned(),
            accessed: meta.accessed().unwrap_or(SystemTime::now()),
            src_iter: iter
        };

        Ok(finder)
    }

    /// Return and string identifier for the gui struct
    pub fn full_name(&self) -> String {
        "".into()
    }

}
