use super::{GuiStruct, ParserError};
use proc_macro2::{TokenTree, TokenStream, token_stream::IntoIter};
use std::{
    path::PathBuf,
    time::SystemTime,
    iter::{Peekable, FromIterator}
};

pub struct GuiStructFinder {
    pub(super) path: PathBuf,
    pub(super) accessed: SystemTime,
    pub(super) src_iter: Peekable<IntoIter>
}

impl GuiStructFinder {

    /// Find a derive attribute that implements NwgUi
    /// Returns false if it exhaust all tokens
    fn find_derive(&mut self) -> bool {
        let mut ok = false;

        loop {
            match self.src_iter.peek() {
                Some(token) => if Self::is_derive_nwg_ui(token) {
                    ok = true;
                    break;
                },
                None => { break; }
            };

            self.src_iter.next();
        }

        ok
    }

    /// Find the `struct` token in the token list
    /// Returns false if it exhaust all tokens
    fn find_struct(&mut self) -> bool {
        let mut ok = false;

        loop {
            match self.src_iter.peek() {
                Some(TokenTree::Ident(i)) => {
                    if i.to_string() == "struct" {
                        ok = true;
                        break;
                    }
                },
                Some(_) => { }
                None => { break; }
            }

            self.src_iter.next();
        }

        ok
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


}

impl Iterator for GuiStructFinder {
    type Item = Result<GuiStruct, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Advance to derive 
            if !self.find_derive() {
                break;
            }

            // Advance to struct identifier
            if !self.find_struct() {
                break;
            }

            // Parse the struct
            // TODO: take(3) should be replace by code that finds the end of the struct
            let stream = TokenStream::from_iter(self.src_iter.clone().take(3));
            let data: syn::ItemStruct = match syn::parse2(stream) {
                Ok(s) => s,
                Err(_e) => {
                    self.src_iter.next();
                    continue;
                }
            };

            return Some(GuiStruct::new(
                self.path.clone(),
                self.accessed.clone(),
                data
            ));
        }

        None
    }

}
