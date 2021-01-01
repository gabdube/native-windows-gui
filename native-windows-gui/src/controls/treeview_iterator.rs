use winapi::shared::windef::HWND;
use winapi::um::commctrl::{HTREEITEM, TVGN_ROOT, TVGN_CHILD, TVGN_NEXT, TVGN_PARENT};
use crate::{TreeView, TreeItem};
use crate::win32::window_helper as wh;


#[derive(Copy, Clone)]
#[repr(usize)]
enum NextAction {
    Root = TVGN_ROOT,
    Child = TVGN_CHILD,
    Sibling = TVGN_NEXT,
    Parent = TVGN_PARENT,
}

/** 
A structure to iterate over the items of a `TreeView`
Requires the feature `tree-view-iterator` and `tree-view`

```rust
use native_windows_gui as nwg;
fn iter_tree_view(tree: &mut nwg::TreeView) {
    for item in tree.iter() {
        println!("{:?}", tree.item_text(&item));
    }
}
```
*/
#[allow(unused)]
pub struct TreeViewIterator<'a> {
    tree_view: &'a TreeView,
    tree_view_handle: HWND,
    base_item: HTREEITEM,
    current_item: HTREEITEM,
    action: NextAction,
}

impl<'a> TreeViewIterator<'a> {

    /// Use `TreeView.iter` to create a `TreeViewIterator`
    pub(crate) fn new(tree_view: &'a TreeView, current_item: HTREEITEM) -> TreeViewIterator {
        let tree_view_handle = tree_view.handle.hwnd().unwrap();

        let action = match current_item.is_null() {
            true => NextAction::Root,
            false => NextAction::Child
        };

        TreeViewIterator {
            tree_view,
            tree_view_handle,
            base_item: current_item,
            current_item,
            action,
        }
    }

}

impl<'a> Iterator for TreeViewIterator<'a> {
    type Item = TreeItem;

    fn next(&mut self) -> Option<TreeItem> {
        use NextAction::*;

        let mut item: Option<TreeItem>;

        loop {
            item = next_item(self.tree_view_handle, self.action, self.current_item);
            self.action = match (self.action, item.is_some()) {
                (Root, _) => Child,
                (Child, true) => Child,
                (Child, false) => Sibling,
                (Sibling, true) => Child,
                (Sibling, false) => Parent,
                (Parent, true) => {
                    // Use the parent as current item for the next loop run
                    self.current_item = item.as_ref().map(|i| i.handle).unwrap();

                    // If we are iterating over an item, and we are back to it, finish the iteration.
                    if self.base_item == self.current_item {
                        return None;
                    }

                    // Do not return parents has they have already been iterated upon
                    item = None;  

                    Sibling
                }
                (Parent, false) => { return None; }
            };

            if item.is_some() {
                self.current_item = item.as_ref().map(|i| i.handle).unwrap();
                break;
            }
        }

        item
    }
}


fn next_item(tree: HWND, action: NextAction, handle: HTREEITEM) -> Option<TreeItem> {
    use winapi::shared::minwindef::{WPARAM, LPARAM};
    use winapi::um::commctrl::TVM_GETNEXTITEM;

    let handle = wh::send_message(tree, TVM_GETNEXTITEM, action as WPARAM, handle as LPARAM) as HTREEITEM;
    if handle.is_null() {
        None
    } else {
        Some(TreeItem { handle })
    }
}
