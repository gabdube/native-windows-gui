#![allow(unused_must_use)]
#![allow(unused_variables)]

extern crate native_windows_gui as nwg;

use nwg::*;
use nwg::constants::*;
use nwg::events::*;
use nwg::events as nwge;

fn setup_ui() -> Ui<u64> { Ui::new().unwrap() }
fn window() -> WindowT<&'static str> {  WindowT{title: "", position:(-600,-600), size:(100, 100), resizable:true, visible:true, disabled:false, exit_on_close:true} }
fn default_font() -> FontT<&'static str> { FontT{ family: "Arial", size: 10, weight: FONT_WEIGHT_BOLD, decoration: FONT_DECO_ITALIC|FONT_DECO_STRIKEOUT } }

macro_rules! test_visibility {
    ($ui:expr, $id:expr, $t:ty) => (
        {
            let x = $ui.get::<$t>($id).expect("Failed to get the control");

            x.set_visibility(true);
            assert!(x.get_visibility() == true, "Window is not visible");

            x.set_visibility(false);
            assert!(x.get_visibility() == false, "Window is visible");

            x.set_visibility(true);
        }
    )
}

macro_rules! test_position {
    ($ui:expr, $id:expr, $t:ty) => (
        {
            let x = $ui.get::<$t>($id).expect("Failed to get the control");
            x.set_position(-600, -600);
            assert!(x.get_position() == (-600, -600), "Window position do not match");
        }
    )
}

macro_rules! test_size {
    ($ui:expr, $id:expr, $t:ty) => (
        {
            let x = $ui.get::<$t>($id).expect("Failed to get the control");
            x.set_size(200, 200);
            assert!(x.get_size() == (200, 200), "Window size do not match");
        }
    );
    ($ui:expr, $id:expr, $t:ty, $d: expr) => (
        {
            let x = $ui.get::<$t>($id).expect("Failed to get the control");
            x.set_size(200, 200);
            assert!(x.get_size() == $d, "Window size do not match");
        }
    )
}

macro_rules! test_enabled {
    ($ui:expr, $id:expr, $t:ty) => (
        {
            let x = $ui.get::<$t>($id).expect("Failed to get the control");
            x.set_enabled(false);
            assert!(x.get_enabled() == false, "Window enabled state do not match");

            x.set_enabled(true);
            assert!(x.get_enabled() == true, "Window enabled state do not match");
        }
    )
}


#[test]
fn test_ui_new() {
    match Ui::<u64>::new() {
        Ok(ui) => ui,
        Err(e) => panic!("Ui creation failed: {:?}", e)
    };
}

#[test]
fn test_ui_pack_user_value() {
    let ui = setup_ui();

    // Test simple pack
    ui.pack_value(&1000, "Test");
    ui.pack_value(&1001, vec![5u32, 6, 7]);

    // Value shouldn't be packed until commit is called
    assert!(!ui.has_id(&1000), "ID 1000 was found in ui before commit");
    assert!(!ui.has_id(&1001), "ID 1001 was found in ui before commit");

    ui.commit().expect("Commit was not successful");
    
    // Both id should have been added
    assert!(ui.has_id(&1000), "ID 1000 was not found in ui after commit");
    assert!(ui.has_id(&1001), "ID 1001 was not found in ui after commit");

    // Test partially good pack (the second entry has a key that is already present)
    ui.pack_value(&1002, "Test");
    ui.pack_value(&1001, 5u32);

    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyExists, "Commit was successful");

    // The first entry should have been executed successfully
    assert!(ui.has_id(&1002), "ID 1002 was not found in ui after commit");

    // Test good get (ids exists and type is correct)
    {
        let w = ui.get::<Vec<u32>>(&1001);
        let x = ui.get::<&'static str>(&1000);
        assert!(w.is_ok() && (**w.unwrap()) == [5u32, 6, 7]);
        assert!(x.is_ok() && (**x.unwrap()) == "Test");
    }

    // Test bad get (ids do not exists and type is not correct)
    {
        let y = ui.get::<&'static str>(&1003);
        let z = ui.get::<bool>(&1000);
    
        assert!(y.is_err() && y.err().unwrap() == Error::KeyNotFound);
        assert!(z.is_err() && z.err().unwrap() == Error::BadType);
    }

    // Test mutable borrow
    {
        { ui.get_mut::<Vec<u32>>(&1001).unwrap().push(1000); }
        let w = ui.get::<Vec<u32>>(&1001);
        assert!(w.is_ok() && (**w.unwrap()) == [5u32, 6, 7, 1000]);
    }

    // Test mutable borrow twice
    {
        let x = ui.get_mut::<&'static str>(&1000);
        let x2 = ui.get_mut::<&'static str>(&1000);
        let x3 = ui.get::<&'static str>(&1000);
        assert!(x.is_ok() && (**x.unwrap()) == "Test");
        assert!(x2.is_err() && x2.err().unwrap() == Error::BorrowError);
        assert!(x3.is_err() && x3.err().unwrap() == Error::BorrowError);
    }
    

}

#[test]
fn test_ui_pack_control() {
    let ui = setup_ui();

    ui.pack_control(&1000, window());
    ui.pack_control(&1001, MenuItemT{text: "", parent: 1000, disabled: false});

    assert!(!ui.has_id(&1000), "ID 1000 was found in ui before commit");
    ui.commit().expect("Commit was not successful");

    // Check if the added control are accessible
    assert!(ui.has_id(&1000), "ID 1000 was not found in ui after commit");
    { let w = ui.get::<Window>(&1000); w.expect("Failed to get control"); }

    // Id already exists
    ui.pack_control(&1000, MenuItemT{text: "", parent: 1000, disabled: false});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyExists, "Commit was successful");

}

#[test]
fn test_ui_pack_resource() {
    let ui = setup_ui();

    ui.pack_resource(&1000, default_font());
    ui.commit().expect("Commit was not successful");

    // Check if the added control are accessible
    assert!(ui.has_id(&1000), "ID 1000 was not found in ui after commit");
    { let f = ui.get::<Font>(&1000); f.expect("Failed to get control"); }

    // Id already exists
    ui.pack_resource(&1000, default_font());
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyExists, "Commit was successful");
}

#[test]
fn test_ui_unpack() {
    let ui = setup_ui();
    let mut free_count: u8 = 0;
    let x = &mut free_count as *mut u8;

    ui.pack_value(&1000, 5u32);
    ui.pack_control(&1001, window());
    ui.pack_value(&1002, true);
    ui.pack_resource(&1003, default_font());
    ui.pack_resource(&1004, default_font());
    ui.pack_control(&1005, ButtonT{text: "TEST", position:(10, 10), size: (100, 30), visible: true, disabled: false, parent: 1001, font: None});
    
    ui.bind(&1001, &5000, Destroyed, move |_, _, _, _|{ unsafe{ *(&mut *x) += 1; } } );
    ui.bind(&1005, &5000, Destroyed, move |_, _, _, _|{ unsafe{ *(&mut *x) += 1; } } );

    ui.commit().expect("Commit was not successful");

    ui.unpack(&1000);
    ui.unpack(&1001);
    ui.unpack(&1003);

    ui.commit().expect("Commit was not successful");

    // Unpacked ids shoudn't be present anymore
    assert!(!ui.has_id(&1000), "ID 1000 was found in ui after commit");
    assert!(!ui.has_id(&1001), "ID 1001 was found in ui after commit");
    assert!(!ui.has_id(&1003), "ID 1003 was found in ui after commit");
    assert!(!ui.has_id(&1005), "ID 1005 was found in ui after commit");

    // Destroy callback should have been executed when unpacking
    assert!(free_count==2, "Destroy callback was not executed.");

    // It should be impossible to unpack a borrowed control/resource
    {
        let x = ui.get::<bool>(&1002);

        ui.unpack(&1002);
        let r = ui.commit();
        assert!(r.is_err() && r.err().unwrap() == Error::ControlInUse, "Commit was successful");

        let y = ui.get::<Font>(&1004);

        ui.unpack(&1004);
        let r = ui.commit();
        assert!(r.is_err() && r.err().unwrap() == Error::ResourceInUse, "Commit was successful");
    }
    
}

#[test]
fn test_ui_bind() {
    let ui = setup_ui();

    ui.pack_value(&1000, 5u32);
    ui.pack_control(&1001, window());
    ui.pack_control(&1002, window());
    ui.pack_control(&1003, MenuItemT{text: "", parent: 1001, disabled: false});

    // Binding successful
    ui.bind(&1001, &5000, Destroyed, |ui, id, _, _|{
        // When event callbacks are being dipatched, it is impossible to bind a new callback on the same control with the same event
        ui.bind(&1001, &5001, Destroyed, |_, _, _, _|{});
        let r = ui.commit();
        assert!(r.is_err() && r.err().unwrap() == Error::ControlInUse, "Commit was successful");

        // Deleting the control while its executing its callback is also prohibited...
        ui.unpack(id);
        let r = ui.commit();
        assert!(r.is_err() && r.err().unwrap() == Error::ControlInUse, "Commit was successful");

        // On other control or on other events, binding new callback is permitted
        // Still binding new events in a destroy callback is a horrible idea, because, unless specified, the NWG destroy order is random.
        ui.bind(&1002, &5001, KeyDown, |_, _, _, _|{ } );
        ui.commit().expect("Commit was not successful");
        exit();
    });

    ui.commit().expect("Commit was not successful");

    // Cannot bind events to user values
    ui.bind(&1000, &5000, Destroyed, |_, _, _, _|{});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::ControlRequired, "Commit was successful");

    // Key not in Ui
    ui.bind(&1005, &5000, Destroyed, |_, _, _, _|{});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyNotFound, "Commit was successful");

    // Event not supported
    ui.bind(&1003, &5000, MouseUp, |_, _, _, _|{});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::EventNotSupported(MouseUp), "Commit was successful");

    // Callback id already exists
    ui.bind(&1001, &5000, Destroyed, |_, _, _, _|{});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyExists, "Commit was successful");

    ui.unpack(&1001);

    dispatch_events();
}

#[test]
fn test_ui_unbind() {
    let ui = setup_ui();
    
    ui.pack_control(&1000, window());
    ui.pack_control(&1002, MenuItemT{text: "", parent: 1000, disabled: false});
    ui.pack_value(&1001, 5u32);

    ui.bind(&1000, &5000, Destroyed, |_, _, _, _|{});
    ui.commit().expect("Commit was not successful");

    ui.unbind(&1000, &5000, Destroyed);
    ui.commit().expect("Commit was not successful");

    // Should be able to rebind destroyed callbacks
    ui.bind(&1000, &5000, Destroyed, |_, _, _, _|{});
    ui.commit().expect("Commit was not successful");

    // Cannot unbind events to user values
    ui.unbind(&1001, &5000, Destroyed);
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::ControlRequired, "Commit was successful");

    // Key not in Ui
    ui.unbind(&1005, &5000, Destroyed);
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyNotFound, "Commit was successful");

    // Event not supported
    ui.unbind(&1002, &5000, MouseUp);
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::EventNotSupported(MouseUp), "Commit was successful");

    // Callback do not exists
    ui.unbind(&1000, &5001, Destroyed);
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyNotFound, "Commit was successful");

    ui.bind(&1000, &5001, Destroyed, |ui, id, _, _|{
        // When event callbacks are being dipatched, it is impossible to unbind a callbacks on the same control with the same event
        ui.unbind(&1000, &5000, Destroyed);
        let r = ui.commit();
        assert!(r.is_err() && r.err().unwrap() == Error::ControlInUse, "Commit was successful");
    });
    ui.commit().expect("Commit was not successful");

    { ui.get::<Window>(&1000).unwrap().close(); }
    dispatch_events();
}


#[test]
fn test_user_trigger() {
    let ui = setup_ui();
    let mut flag_set: bool = false;
    let x = &mut flag_set as *mut bool;
    
    ui.pack_value(&1001, 5u32);
    ui.pack_control(&1000, window());
    ui.bind(&1000, &5000, MouseDown, move |_, _, _, _|{ unsafe{ *(&mut *x) = true; } });
    ui.bind(&1000, &5000, MouseUp, move |ui, _, _, _|{ 
        ui.trigger(&1000, MouseDown, EventArgs::None);
        let r = ui.commit().expect("Commit was not successful");;
    });
   
    ui.trigger(&1000, MouseDown, EventArgs::None);
    ui.commit().expect("Commit was not successful");

    ui.trigger(&1030, MouseDown, EventArgs::None);
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyNotFound, "Commit was successful");

    ui.trigger(&1001, MouseDown, EventArgs::None);
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::ControlRequired, "Commit was successful");

    ui.trigger(&1000, nwge::listbox::SelectionChanged, EventArgs::None);
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::EventNotSupported(nwge::listbox::SelectionChanged), "Commit was successful");

    ui.trigger(&1000, MouseUp, EventArgs::None);
    let r = ui.commit();

    assert!(flag_set, "Flag was not set");
}


#[test]
fn test_window_control_user_close() {
    let ui = setup_ui();
    let mut callback_executed: bool = false;
    let x = &mut callback_executed as *mut bool;

    ui.pack_control(&1000, window());
    ui.bind(&1000, &5000, Destroyed, move |_, _, _, _|{ unsafe{ *(&mut *x) = true; } });
    ui.commit().expect("Commit was not successful");

    assert!(ui.has_id(&1000), "ID 1000 was not found in id after commit");
    
    // Try to close the window
    { ui.get::<Window>(&1000).unwrap().close(); }

    // Dispatch the waiting close event
    dispatch_events();
}

#[test]
fn test_drop_callback() {
    let mut callback_executed: bool = false;
    
    {
        let x = &mut callback_executed as *mut bool;
        let ui = setup_ui();
        ui.pack_control(&1000, window());
        ui.bind(&1000, &5000, Destroyed, move |_, _, _, _|{ unsafe{ *(&mut *x) = true; } });
        ui.commit().expect("Commit was not successful");
    }

    assert!(callback_executed, "Destroy callback was not executed.")
}

#[test]
fn test_menus() {
    let ui = setup_ui();
    
    let mut free_count: u8 = 0;
    let x = &mut free_count as *mut u8;

    ui.pack_control(&1000, window());
    ui.bind(&1000, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    
    ui.pack_control(&1001, MenuT{ text: "Test1", parent: 1000, disabled: false  });
    ui.bind(&1001, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });

    ui.pack_control(&2003, MenuItemT{ text: "TestItem4", parent: 1000, disabled: false  });
    ui.bind(&2003, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    
    ui.pack_control(&1002, MenuT{ text: "Test2", parent: 1000, disabled: false  });
    ui.pack_control(&1003, MenuT{ text: "Test3", parent: 1002, disabled: false  });
    ui.pack_control(&1004, MenuT{ text: "Test4", parent: 1002, disabled: false  });
    ui.pack_control(&2000, MenuItemT{ text: "TestItem1", parent: 1002, disabled: false  });
    ui.bind(&1002, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    ui.bind(&1003, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    ui.bind(&1004, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    ui.bind(&2000, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });

    ui.pack_control(&1005, MenuT{ text: "Test5", parent: 1000, disabled: false });
    ui.pack_control(&1006, MenuT{ text: "Test6", parent: 1005, disabled: false });
    ui.pack_control(&1007, MenuT{ text: "Test7", parent: 1006, disabled: false });
    ui.pack_control(&2001, MenuItemT{ text: "TestItem2", parent: 1007, disabled: false  });
    ui.bind(&1005, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    ui.bind(&1006, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    ui.bind(&1007, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    ui.bind(&2001, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });

    ui.pack_control(&1008, MenuT{ text: "Test8", parent: 1000, disabled: false });
    ui.pack_control(&1009, MenuT{ text: "Test9", parent: 1008, disabled: false });
    ui.pack_control(&2002, MenuItemT{ text: "TestItem3", parent: 1000, disabled: false  });
    ui.bind(&1008, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    ui.bind(&1009, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });
    ui.bind(&2002, &10_000, Destroyed, move |_,_,_,_|{ unsafe{  *(&mut *x) += 1 } });

    ui.commit().expect("Commit was not successful");

    {
        // Menu should be enabled
        let (menu, menuitem) = (ui.get::<Menu>(&1001).unwrap(), ui.get::<MenuItem>(&2002).unwrap());
        assert!(menu.get_enabled() == true, "Menu is not enabled");
        assert!(menuitem.get_enabled() == true, "Menu is not enabled");

        menu.set_enabled(false);
        menuitem.set_enabled(false);

        assert!(menu.get_enabled() == false, "Menu is enabled");
        assert!(menuitem.get_enabled() == false, "Menu is enabled");
    }

    // Removing a menu without items
    ui.unpack(&1001);
    ui.commit().expect("Commit was not successful");
    assert!(ui.has_id(&1001) == false, "Destroyed menu key '1001' was found in ui");
    assert!(free_count == 1, "Freecount was not increased!");

    // Removing a menu item
    ui.unpack(&2003);
    ui.commit().expect("Commit was not successful");
    assert!(ui.has_id(&2003) == false, "Destroyed menu key '1001' was found in ui");
    assert!(free_count == 2, "Freecount was not increased!");

    // Removing a menu with subitems
    ui.unpack(&1002);
    ui.commit().expect("Commit was not successful");
    assert!(ui.has_id(&1002) == false, "Destroyed menu key '1002' was found in ui");
    assert!(ui.has_id(&1003) == false, "Destroyed menu key '1003' was found in ui");
    assert!(ui.has_id(&1004) == false, "Destroyed menu key '1004' was found in ui");
    assert!(ui.has_id(&2000) == false, "Destroyed menu item key '2000' was found in ui");
    assert!(free_count == 6, "Freecount was not increased by 4!");

    // Removing a menu with subitems that have subitems
    ui.unpack(&1005);
    ui.commit().expect("Commit was not successful");
    assert!(ui.has_id(&1005) == false, "Destroyed menu key '1005' was found in ui");
    assert!(ui.has_id(&1006) == false, "Destroyed menu key '1006' was found in ui");
    assert!(ui.has_id(&1007) == false, "Destroyed menu key '1007' was found in ui");
    assert!(ui.has_id(&2001) == false, "Destroyed menu item key '2001' was found in ui");
    assert!(free_count == 10, "Freecount was not increased by 4!");

    // Removing a window should also free its menus
    ui.unpack(&1000);
    ui.commit().expect("Commit was not successful");
    assert!(ui.has_id(&1000) == false, "Destroyed menu key '1000' was found in ui");
    assert!(ui.has_id(&1008) == false, "Destroyed menu key '1008' was found in ui");
    assert!(ui.has_id(&1009) == false, "Destroyed menu key '1009' was found in ui");
    assert!(ui.has_id(&2002) == false, "Destroyed menu item key '2002' was found in ui");
    assert!(free_count == 14, "Freecount was not increased by 3!");
}

#[test]
fn test_window() {
    let ui = setup_ui();

    // pack test
    ui.pack_control(&1000, window());
    ui.commit().expect("Commit was not successful");

    // methods test
    test_visibility!(ui, &1000, Window);
    test_position!(ui, &1000, Window);
    test_size!(ui, &1000, Window);
    test_enabled!(ui, &1000, Window);
}

#[test]
fn test_buttons() {
    let ui = setup_ui();

    let mut btn_t = ButtonT{text: "TEST", position:(10, 10), size: (100, 30), visible: true, disabled: false, parent: 1000, font: None};
    let btn_t2 = CheckBoxT{text: "TEST", position:(10, 10), size: (100, 30), visible: true, disabled: false, checkstate: CheckState::Checked, tristate: false, parent: 1000, font: None};

    ui.pack_resource(&10_000, default_font());
    ui.pack_control(&1000, window());
    ui.pack_control(&1001, MenuItemT{text: "", parent: 1000, disabled: false});

    // pack test
    ui.pack_control(&1002, btn_t.clone());
    ui.pack_control(&1010, btn_t2.clone());

    btn_t.font = Some(10_000);
    ui.pack_control(&1003, btn_t.clone() );
    ui.commit().expect("Commit was not successful");

    btn_t.parent = 9999;
    ui.pack_control(&1004, btn_t.clone() );
    match ui.commit() { Err(Error::KeyNotFound) => {}, r => panic!("Should have returned Error::KeyNotFound, got {:?}", r) }

    btn_t.parent = 1001;
    ui.pack_control(&1004, btn_t.clone() );
    match ui.commit() { Err(Error::BadParent(_)) => {}, r => panic!("Should have returned Error::BadParent, got {:?}", r) }

    btn_t.parent = 1000;
    btn_t.font = Some(1000);
    ui.pack_control(&1004, btn_t.clone() );
    match ui.commit() { Err(Error::BadResource(_)) => {}, r => panic!("Should have returned Error::BadResource, got {:?}", r) }

    // methods test
    test_visibility!(ui, &1002, Button);
    test_position!(ui, &1002, Button);
    test_size!(ui, &1002, Button);
    test_enabled!(ui, &1002, Button);

    {
        let checkbox = ui.get::<CheckBox>(&1010).expect("Control not found");

        assert!(checkbox.get_checkstate() == CheckState::Checked);
        checkbox.set_checkstate(CheckState::Unchecked);
        assert!(checkbox.get_checkstate() == CheckState::Unchecked);

        assert!(checkbox.get_text().as_str() == "TEST");
        checkbox.set_text("Waloo");
        assert!(checkbox.get_text().as_str() == "Waloo");
    }
}

#[test]
fn test_listbox() {
    let ui = setup_ui();

    let col = vec!["Foo", "FooBar", "Excelsior", "wOOsh"];
    let mut lb_t = ListBoxT {
        collection: col.clone(),
        position:(10, 50), size: (100, 90),
        visible: true, disabled: false,  readonly: false, multi_select: false,
        parent: 1000,
        font: None 
    };

    ui.pack_control(&1000, window());

    // pack test
    ui.pack_control(&1002, lb_t.clone());
    lb_t.multi_select = true;
    ui.pack_control(&1003, lb_t);
    ui.commit().expect("Commit was not successful");

    // methods test
    test_visibility!(ui, &1002, ListBox<&'static str>);
    test_position!(ui, &1002, ListBox<&'static str>);
    test_size!(ui, &1002, ListBox<&'static str>, (200, 192)); // Listbox height is forced to be rounded to match the items height.
    test_enabled!(ui, &1002, ListBox<&'static str>);

    {
        let mut lb = ui.get_mut::<ListBox<&'static str>>(&1002).expect("Control not found!");

        assert!(lb.get_readonly() == false, "Listbox should not be readonly");
        assert!(lb.get_multi_select() == false, "Listbox should not be multi select");

        lb.set_readonly(true);
        assert!(lb.get_readonly() == true, "Listbox should be readonly");
        lb.set_readonly(false);

        lb.set_multi_select(true);
        assert!(lb.get_multi_select() == true, "Listbox should be multi-select");
        lb.set_multi_select(false);

        assert!(lb.collection() == &col, "Collection do not match");
        assert!(lb.collection_mut() == &col, "Collection do not match");
        assert!(lb.len() == 4, "Collection length should be 4");

        lb.push("Foohoy!");
        assert!(lb.collection() == &["Foo", "FooBar", "Excelsior", "wOOsh", "Foohoy!"], "Collection do not match");
        assert!(lb.get_string(4).unwrap().as_str() == "Foohoy!", "Item text do not match");
        assert!(lb.len() == 5, "Collection length should be 5");

        lb.remove(0);
        assert!(lb.collection() == &["FooBar", "Excelsior", "wOOsh", "Foohoy!"], "Collection do not match");
        assert!(lb.get_string(0).unwrap().as_str() == "FooBar", "Item text do not match");

        assert!(lb.get_selected_index().is_none(), "No index should be selected");
        assert!(lb.get_selected_indexes().len() == 0, "Indexes vector length should be 0");

        lb.set_selected_index(1);

        assert!(lb.get_selected_index() == Some(1), "Current index is not 1");
        assert!(lb.get_selected_indexes().len() == 0, "Indexes vector length should be 0");
        assert!(lb.index_selected(1), "Index 1 is not selected");
        assert!(lb.index_selected(2) == false, "Index 2 is selected");
        assert!(lb.index_selected(665) == false, "Index 665 is selected");
        assert!(lb.len_selected() == 1, "Selected length is not 1");

        lb.set_selected_index(usize::max_value());
        assert!(lb.get_selected_index().is_none(), "No index should be selected");

        assert!(lb.find_string("foo", false) == Some(0), "find_string shoud have returned 0");
        assert!(lb.find_string("foo", true) == None, "find_string shoud have returned None");
        assert!(lb.find_string("Foohoy!", true) == Some(3), "find_string shoud have returned None");

        assert!(lb.get_string(100) == None, "Item text should be None");

        lb.collection_mut().remove(0);
        assert!(lb.get_string(0).unwrap().as_str() == "FooBar", "Item text do not match"); // Ui and inner collection not synced
        lb.sync();
        assert!(lb.get_string(0).unwrap().as_str() == "Excelsior", "Item text do not match"); // Ui and inner collection synced

        lb.clear();
        assert!(lb.len() == 0, "Length is not 0");
    }

    {
        let lb = ui.get::<ListBox<&'static str>>(&1003).expect("Control not found!");

        assert!(lb.get_multi_select() == true, "Listbox should not be multi select");

        assert!(lb.get_selected_indexes().len() == 0, "Indexes vector length should be 0");

        lb.set_index_selected(0, true);
        lb.set_index_selected(2, true);
        assert!(lb.len_selected() == 2, "Selected length is not 2");

        assert!(lb.get_selected_indexes() == [0, 2], "Selected indexes do not match");

        lb.set_index_selected(0, false);
        assert!(lb.len_selected() == 1, "Selected length is not 1");
        assert!(lb.get_selected_indexes() == [2], "Selected indexes do not match");

        lb.set_index_selected(usize::max_value(), true);
        assert!(lb.len_selected() == 4, "Selected length is not 4");
        assert!(lb.get_selected_indexes() == [0,1,2,3], "Selected indexes do not match");

        lb.set_range_selected(0, 2, false);
        assert!(lb.len_selected() == 1, "Selected length is not 1");
        
        lb.set_range_selected(0, 6, true);
        assert!(lb.len_selected() == 4, "Selected length is not 4");
        assert!(lb.get_selected_indexes() == [0,1,2,3], "Selected indexes do not match");
    }
    
}

#[test]
fn test_timer() {
    // TODO
}

#[test]
fn test_combobox() {
    let ui = setup_ui();

    let col = vec!["Foo", "FooBar", "Excelsior", "wOOsh"];
    let cb_t = ComboBoxT {
        collection: col.clone(),
        position:(10, 50), size: (100, 90),
        visible: true, disabled: false,
        placeholder: Some("TEST"),
        parent: 1000,
        font: None 
    };

    ui.pack_control(&1000, window());

    // pack test
    ui.pack_control(&1002, cb_t);
    ui.commit().expect("Commit was not successful");

    // methods test
    test_visibility!(ui, &1002, ComboBox<&'static str>);
    test_position!(ui, &1002, ComboBox<&'static str>);
    test_enabled!(ui, &1002, ComboBox<&'static str>);

    {
        let mut cb = ui.get_mut::<ComboBox<&'static str>>(&1002).expect("Control not found!");

        assert!(cb.collection() == &col, "Collection do not match");
        assert!(cb.collection_mut() == &col, "Collection do not match");
        assert!(cb.len() == 4, "Collection length should be 4");

        cb.push("Foohoy!");
        assert!(cb.collection() == &["Foo", "FooBar", "Excelsior", "wOOsh", "Foohoy!"], "Collection do not match");
        assert!(cb.get_string(4).unwrap().as_str() == "Foohoy!", "Item text do not match");
        assert!(cb.len() == 5, "Collection length should be 5");

        cb.remove(0);
        assert!(cb.collection() == &["FooBar", "Excelsior", "wOOsh", "Foohoy!"], "Collection do not match");
        assert!(cb.get_string(0).unwrap().as_str() == "FooBar", "Item text do not match");

        assert!(cb.get_selected_index().is_none(), "No index should be selected");
        assert!(cb.get_selected_text().as_str() == "");

        cb.set_selected_index(1);
        assert!(cb.get_selected_text().as_str() == "Excelsior");
        assert!(cb.get_selected_index() == Some(1), "Current index is not 1");

        cb.set_selected_index(usize::max_value());
        assert!(cb.get_selected_index().is_none(), "No index should be selected");

        assert!(cb.find_string("foo", false) == Some(0), "find_string shoud have returned 0");
        assert!(cb.find_string("foo", true) == None, "find_string shoud have returned None");
        assert!(cb.find_string("Foohoy!", true) == Some(3), "find_string shoud have returned None");

        assert!(cb.get_string(100) == None, "Item text should be None");

        cb.collection_mut().remove(0);
        assert!(cb.get_string(0).unwrap().as_str() == "FooBar", "Item text do not match"); // Ui and inner collection not synced
        cb.sync();
        assert!(cb.get_string(0).unwrap().as_str() == "Excelsior", "Item text do not match"); // Ui and inner collection synced

        cb.clear();
        assert!(cb.len() == 0, "Length is not 0");
    }
}

#[test]
fn text_textinput() {
    let ui = setup_ui();

    let ti_t = TextInputT::<_, &'static str, _> {
        text: "TEST",
        position: (0, 0), size: (100, 30), 
        visible: true, disabled: false, readonly: false, password: false,
        limit: 10,
        placeholder: None,
        parent: 1000,
        font: None
    };

    ui.pack_control(&1000, window());

    // pack test
    ui.pack_control(&1001, ti_t);
    ui.commit().expect("Commit was not successful");

    // methods test
    test_visibility!(ui, &1001, TextInput);
    test_position!(ui, &1001, TextInput);
    test_size!(ui, &1001, TextInput);
    test_enabled!(ui, &1001, TextInput);

    {
        let tinput = ui.get::<TextInput>(&1001).expect("Control not found");

        assert!(tinput.get_text().as_str() == "TEST");
        tinput.set_text("Waloo");
        assert!(tinput.get_text().as_str() == "Waloo");

        assert!(tinput.get_readonly() == false);
        tinput.set_readonly(true);
        assert!(tinput.get_readonly() == true);

        assert!(tinput.get_password() == false);
        tinput.set_password(true);
        assert!(tinput.get_password() == true);

        assert!(tinput.get_limit() == 10);
        tinput.set_limit(10_000);
        assert!(tinput.get_limit() == 10_000);
    }
}

#[test]
fn sizeof_events_unpack_function() {
    use std::mem::{size_of_val, size_of};

    // When function pointers are hashed, they are interpreted as [usize;2]
    // This test makes sure that its always the case
    if let &nwge::Event::Single(_, ref fnptr1, ref fnptr2) = &nwge::KeyDown {
        assert!(size_of_val(fnptr1) == size_of::<[usize; 2]>());
        assert!(size_of_val(fnptr2) == size_of::<[usize; 2]>());
    } else {
        panic!("What?")
    }

    if let &nwge::Event::Group(_, ref fnptr1, ref fnptr2) = &nwge::MouseDown {
        assert!(size_of_val(fnptr1) == size_of::<[usize; 2]>());
        assert!(size_of_val(fnptr2) == size_of::<[usize; 2]>());
    } else {
        panic!("What?")
    }

   
}