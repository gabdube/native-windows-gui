#![allow(unused_variables)]

extern crate native_windows_gui as nwg;
use nwg::constants::Error;
use nwg::actions::{Action, ActionReturn};
use nwg::actions::helper;
use nwg::constants::{HTextAlign, VTextAlign, WindowDisplay, MessageButtons, MessageIcons};

macro_rules! test_action {
    ($ui:expr, $a:expr, $b:pat, $c: block) => (
        match $ui.exec("TEST1", $a).unwrap() {
            $b => $c,
            _ => panic!("Bad actionreturn type returned")
        }
    );

    ($ui:expr, $a:expr, $b:pat, $c: block, $d: expr) => (
        match $ui.exec($d, $a).unwrap() {
            $b => $c,
            _ => panic!("Bad actionreturn type returned")
        }
    );
}

fn setup_window(ui: &mut nwg::Ui<&'static str>) {
    let main_window = nwg::controls::Window {
        caption: "Test".to_string(),
        size: (200, 200),
        position: (100, 100),
        visible: false,
        resizable: false,
        exit_on_close: true
    };

    let sub_window = nwg::controls::Window {
        caption: "Test".to_string(),
        size: (200, 200),
        position: (100, 100),
        visible: false,
        resizable: false,
        exit_on_close: false
    };

    ui.new_control("MainWindow", main_window).unwrap();
    ui.new_control("SubWindow", sub_window).unwrap();
}

#[test]
fn buttons() {
    let mut ui: nwg::Ui<&'static str> = nwg::Ui::new();
    setup_window(&mut ui);

    let grp = nwg::controls::GroupBox { text: "group".to_string(), size: (200, 200), position: (10, 10), parent: "MainWindow", text_align: HTextAlign::Center };
    let b1 = nwg::controls::Button { text: "test".to_string(), size: (102, 102), position: (100, 100), parent: "GROUP", text_align: (HTextAlign::Center, VTextAlign::Center) };
    let b2 = nwg::controls::Button { text: "test".to_string(), size: (100, 100), position: (100, 100), parent: "Bob", text_align: (HTextAlign::Center, VTextAlign::Center) };

    assert!(ui.new_control("GROUP", grp).is_ok());
    assert!(ui.new_control("TEST1", b1).is_ok());

    // Bad parent
    let r = ui.new_control("TEST2", b2);
    assert!(r.is_err());
    assert!(r.err().unwrap() == Error::TEMPLATE_CREATION);

    // Actions
    test_action!(ui, Action::None, ActionReturn::NotSupported, {});
    test_action!(ui, helper::message("A", "A", MessageButtons::Ok, MessageIcons::Error), ActionReturn::NotSupported, {});

    test_action!(ui, Action::GetParent, ActionReturn::Parent(p), { assert!(*p == "GROUP"); } );
    test_action!(ui, helper::set_parent("hya!"), ActionReturn::Error(e), { assert!(e == Error::CONTROL_NOT_FOUND); } );
    test_action!(ui, helper::remove_parent(), ActionReturn::Error(e), { assert!(e == Error::MUST_HAVE_PARENT); } );
    test_action!(ui, helper::set_parent("MainWindow"), ActionReturn::None, {} );
    test_action!(ui, Action::GetParent, ActionReturn::Parent(p), { assert!(*p == "MainWindow"); } );
    test_action!(ui, helper::set_parent("GROUP"), ActionReturn::None, {} );

    test_action!(ui, Action::GetChildren, ActionReturn::Children(c), {assert!(*c == ["GROUP"]);}, "MainWindow");
    test_action!(ui, Action::GetDescendants, ActionReturn::Children(c), {assert!(*c == ["GROUP", "TEST1"]);}, "MainWindow");

    test_action!(ui, Action::GetParent, ActionReturn::None, { }, "SubWindow" );
    test_action!(ui, helper::set_parent("MainWindow"), ActionReturn::None, {}, "SubWindow");
    test_action!(ui, Action::GetParent, ActionReturn::Parent(p), { assert!(*p == "MainWindow"); }, "SubWindow" );

    test_action!(ui, Action::GetPosition, ActionReturn::Position(x,y), { assert!((x,y) == (100, 100)); } );
    test_action!(ui, Action::SetPosition(150, 150), ActionReturn::None, {} );
    test_action!(ui, Action::GetPosition, ActionReturn::Position(x,y), { assert!((x,y) == (150, 150)); } );
    
    test_action!(ui, Action::GetSize, ActionReturn::Size(w,h), { assert!((w,h) == (102, 102)); } );
    test_action!(ui, Action::SetSize(151, 151), ActionReturn::None, {} );
    test_action!(ui, Action::GetSize, ActionReturn::Size(w,h), { assert!((w,h) == (151, 151)); } );

    test_action!(ui, Action::GetText, ActionReturn::Text(t), { assert!(*t == "test"); } );
    test_action!(ui, helper::set_text("Haha"), ActionReturn::None, {} );
    test_action!(ui, Action::GetText, ActionReturn::Text(t), { assert!(*t == "Haha"); } );

    test_action!(ui, Action::GetEnabled, ActionReturn::Enabled(e), { assert!(e); } );
    test_action!(ui, Action::SetEnabled(false), ActionReturn::None, {} );
    test_action!(ui, Action::GetEnabled, ActionReturn::Enabled(e), { assert!(!e); } );
    test_action!(ui, Action::SetEnabled(true), ActionReturn::None, {} );

    test_action!(ui, Action::GetVisibility, ActionReturn::Visibility(v), { assert!(!v); }, "MainWindow" );
    test_action!(ui, Action::SetVisibility(true), ActionReturn::None, {}, "MainWindow" );
    test_action!(ui, Action::GetVisibility, ActionReturn::Visibility(v), { assert!(v); }, "MainWindow" );
    test_action!(ui, Action::SetVisibility(false), ActionReturn::None, {}, "MainWindow" );
    
    test_action!(ui, Action::GetWindowDisplay, ActionReturn::WindowDisplay(d), {d == WindowDisplay::Normal}, "MainWindow" );
    test_action!(ui, Action::SetWindowDisplay(WindowDisplay::Maximised), ActionReturn::None, {}, "MainWindow");
    test_action!(ui, Action::GetWindowDisplay, ActionReturn::WindowDisplay(d), {d == WindowDisplay::Maximised}, "MainWindow" );
    test_action!(ui, Action::SetWindowDisplay(WindowDisplay::Minimized), ActionReturn::None, {}, "MainWindow");
    test_action!(ui, Action::GetWindowDisplay, ActionReturn::WindowDisplay(d), {d == WindowDisplay::Minimized}, "MainWindow" );
}