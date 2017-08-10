/**
    Simple example on how to use the nwg template system.
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Ui, simple_message, fatal_message, dispatch_events};
use nwg::events as nwge;
use nwg::constants as nwgc;
use nwg::EventArgs;

#[derive(Debug, Clone, Hash)]
pub enum AppID {
    MainWindow,
    Tree,
    Item(u8),

    HResize,

    DefaultCursor,
    HResizeCursor
}

use AppID::*;

nwg_template!(
    head: setup_ui<AppID>,
    controls: [
        (MainWindow, nwg_window!( title="Flicker Test"; size=(500, 300); resizable=true )),
        (Tree, nwg_treeview!( parent=MainWindow; position=(0,0); size=(180, 300) )),
        (Item(0), nwg_treeview_item!( parent=Tree; text="Item 0" )),
        (Item(1), nwg_treeview_item!( parent=Item(0); text="Item 1" )),
        (Item(2), nwg_treeview_item!( parent=Item(0); text="Item 2" )),
        (Item(3), nwg_treeview_item!( parent=Item(2); text="Item 3" )),
        (Item(4), nwg_treeview_item!( parent=Item(2); text="Item 4" ))
    ];
    events: [
        (Tree, HResize, nwge::MouseMove, |ui,_,evt,args| { resize_tree(ui, evt, args); }),
        (Tree, HResize, nwge::MouseDown, |ui,_,evt,args| { resize_tree(ui, evt, args); }),
        (Tree, HResize, nwge::MouseUp, |ui,_,evt,args| { resize_tree(ui, evt, args); })
    ];
    resources: [
        (HResizeCursor, nwg_oem_image!( source=nwgc::OemImage::Cursor(nwgc::OemCursor::SizeWE); ) ),
        (DefaultCursor, nwg_oem_image!( source=nwgc::OemImage::Cursor(nwgc::OemCursor::Normal); ) )
    ];
    values: []
);

/// Resize the component tree
fn resize_tree(ui: &Ui<AppID>, evt: &nwge::Event, args: &nwge::EventArgs) {
    let (main, tree) = nwg_get!(ui; [(MainWindow, nwg::Window), (Tree, nwg::TreeView)]);
    let (width, height) = tree.get_size();
    let (max_width, _) = main.get_size();
    let (width_i, maxwidth_i) = (width as i32, max_width as i32);
    let captured = nwg::Cursor::get_capture(ui).is_some();

    let delta_x = match args {
        &EventArgs::Position(x, _) | &EventArgs::MouseClick{btn: _, pos: (x, _)} => width_i-x,
        _ => { unreachable!(); }
    };

    if delta_x < 5 || captured { 
        nwg::Cursor::set(ui, &HResizeCursor).is_ok(); 

        if *evt == nwge::MouseDown {
            if let Ok(true) = nwg::Cursor::dragging(ui, &Tree, None) {
                nwg::Cursor::set_capture(ui, &Tree).is_ok();
            }
        } else if *evt == nwge::MouseUp {
             nwg::Cursor::release();
        } else {
            if captured {
                let new_width = width_i - delta_x;
                if new_width > 10 && new_width < (maxwidth_i-10) && new_width != width_i {
                    tree.set_size(new_width as u32, height);
                }
            }
        }

    } else { 
        nwg::Cursor::set(ui, &DefaultCursor).is_ok(); 
    }
}

fn main() {
    let app: Ui<AppID>;

    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    if let Err(e) = setup_ui(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    dispatch_events();
}
