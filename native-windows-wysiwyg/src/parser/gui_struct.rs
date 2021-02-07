use super::{ParserError, GuiStructFinder};
use std::{fs, path::{Path, PathBuf}, time::SystemTime};


pub struct GuiMember {
    name: String
}

impl GuiMember {

    pub fn name(&self) -> &str {
        &self.name
    }

}

/**
    Represents a native-windows-gui GUI struct in a file
*/
pub struct GuiStruct {
    /// Path to the rust source file containing the gui struct
    path: PathBuf,

    /// Last modified time of the source file
    accessed: SystemTime,

    /// Name of the struct
    name: String,

    /// Gui members of the struct
    members: Vec<GuiMember>,
}

impl GuiStruct {

    /// Returns an iterator over the rust source file at `path` that return instances of GuiStruct
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

    /// Creates a new GuiStruct
    pub fn new(path: PathBuf, accessed: SystemTime, data: syn::ItemStruct) -> Result<GuiStruct, ParserError> {
        let name = data.ident.to_string();

        let mut members = Vec::new();
        Self::collect_fields(&mut members, &data)?;

        let gui = GuiStruct {
            path,
            accessed,
            name,
            members
        };

        Ok(gui)
    }

    /// Returns the name of the gui struct
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns a unique string identifier for the gui struct "name (file_name)"
    pub fn full_name(&self) -> String {
        let file_name = self.path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.to_string())
            .unwrap_or("".to_string());

        format!("{} ({})", self.name, file_name)
    }

    /// Returns the gui members of the struct
    pub fn members(&self) -> &[GuiMember] {
        &self.members
    }

    /// Parse the gui fields of a gui struct and store them in `members`
    fn collect_fields(members: &mut Vec<GuiMember>, data: &syn::ItemStruct) -> Result<(), ParserError> {
        use syn::Fields;

        let named = match &data.fields {
            Fields::Named(f) => f.named.iter(),
            _ => { return Err(ParserError::StructParsing("Failed to parse gui struct fields: Fields are not named.".to_owned())) }
        };

        for field in named {
            let field_name = match field.ident.as_ref() {
                Some(n) => n.to_string(),
                None => { continue; }
            };

            let member = GuiMember {
                name: field_name,
            };

            members.push(member);
        }

        Ok(())
    }

}
