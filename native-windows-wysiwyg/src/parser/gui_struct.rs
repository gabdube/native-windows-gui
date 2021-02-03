use proc_macro2::{TokenTree, TokenStream, token_stream::IntoIter};
use super::ParserError;
use std::{fs, path::{Path, PathBuf}, time::SystemTime, iter::{Peekable, FromIterator}};


/**
    Represents a native-windows-gui GUI struct in a file
*/
pub struct GuiStruct {
    /// Path to the rust source file containing the gui struct
    path: PathBuf,

    /// Last modified time of the source file
    accessed: SystemTime,

    /// Data in the gui struct
    data: syn::ItemStruct,
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

}


pub struct GuiStructFinder {
    path: PathBuf,
    accessed: SystemTime,
    src_iter: Peekable<IntoIter>
}

impl Iterator for GuiStructFinder {
    type Item = GuiStruct;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Find an instance of `#[derive(NwgUi)]`
            match self.src_iter.peek() {
                Some(token) => match is_derive_nwg_ui(token) {
                    true => {},
                    false => {
                        self.src_iter.next();
                        continue;
                    }
                },
                None => { break; }
            };

            // Skip to the struct token
            loop {
                match self.src_iter.peek() {
                    Some(TokenTree::Ident(i)) => {
                        if i.to_string() == "struct" {
                            break;
                        }
                    },
                    Some(_) => { }
                    None => { break; }
                }

                self.src_iter.next();
            }

            // Parse the struct
            let stream = TokenStream::from_iter(self.src_iter.clone().take(3));
            let data: syn::ItemStruct = match syn::parse2(stream) {
                Ok(s) => s,
                Err(_e) => {
                    self.src_iter.next();
                    continue;
                }
            };
            
            let gui_struct = GuiStruct {
                path: self.path.clone(),
                accessed: self.accessed.clone(),
                data,
            };

            return Some(gui_struct);
        }

        None
    }

}

/// Checks if a pm2::Group implements `#[derive(NwgUi)]`
fn is_derive_nwg_ui(tree: &TokenTree) -> bool {
    let group = match tree {
        TokenTree::Group(g) => g,
        _ => { return false; }
    };

    // Check if the group is a derive attribute
    let mut group_iter = group.stream().into_iter();
    let group_ident = group_iter.next();
    match group_ident {
        Some(TokenTree::Ident(i)) => {
            if i.to_string() != "derive" {
                return false;
            }
        },
        _ => { return false; }
    }

    // Check if the derive attribute implement NwgUi
    let derive_content = match group_iter.next() {
        Some(TokenTree::Group(g)) => g.stream(),
        _ => { return false; }
    };

    derive_content.into_iter()
        .filter_map(|tk| match tk { TokenTree::Ident(i) => Some(i), _=> None } )
        .any(|id| id.to_string() == "NwgUi")
}
