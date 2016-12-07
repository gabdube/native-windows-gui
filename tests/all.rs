#![allow(unused_must_use)]
#![allow(unused_variables)]

extern crate native_windows_gui as nwg;

use nwg::*;

fn setup_ui() -> Ui<u64> { Ui::new().unwrap() }
fn window() -> WindowT<&'static str> {  WindowT{title: "", position:(0,0), size:(0,0), resizable:true, visible:false, disabled:false, exit_on_close:true} }

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
    ui.pack_value(1000, "Test");
    ui.pack_value(1001, vec![5u32, 6, 7]);

    // Value shouldn't be packed until commit is called
    assert!(!ui.has_id(&1000), "ID 1000 was found in ui before commit");
    assert!(!ui.has_id(&1001), "ID 1001 was found in ui before commit");

    ui.commit().expect("Commit was not successful");
    
    // Both id should have been added
    assert!(ui.has_id(&1000), "ID 1000 was not found in ui after commit");
    assert!(ui.has_id(&1001), "ID 1001 was not found in ui after commit");

    // Test partially good pack (the second entry has a key that is already present)
    ui.pack_value(1002, "Test");
    ui.pack_value(1001, 5u32);

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

    ui.pack_control(1000, window());

    assert!(!ui.has_id(&1000), "ID 1000 was found in ui before commit");
    ui.commit().expect("Commit was not successful");
    assert!(ui.has_id(&1000), "ID 1000 was not found in ui after commit");

    {
        let w = ui.get::<Window>(&1000);
        w.expect("Failed to get control");
    }
}

#[test]
fn test_ui_unpack() {
    let ui = setup_ui();
    let mut callback_executed: bool = false;
    let x = &mut callback_executed as *mut bool;

    ui.pack_value(1000, 5u32);
    ui.pack_control(1001, window());
    ui.pack_value(1002, true);
    ui.bind(&1001, &5000, Event::Destroyed, move |_, _, _, _|{ unsafe{ *(&mut *x) = true; } } );

    ui.commit().expect("Commit was not successful");

    ui.unpack(&1000);
    ui.unpack(&1001);

    ui.commit().expect("Commit was not successful");

    // Unpacked ids shoudn't be present anymore
    assert!(!ui.has_id(&1000), "ID 1000 was found in ui after commit");
    assert!(!ui.has_id(&1001), "ID 1001 was found in ui after commit");

    // Destroy callback should have been executed when unpacking
    assert!(callback_executed, "Destroy callback was not executed.");

    // It should be impossible to unpack a borrowed control
    {
        let x = ui.get::<bool>(&1002);

        ui.unpack(&1002);
        let r = ui.commit();
        assert!(r.is_err() && r.err().unwrap() == Error::ControlInUse, "Commit was successful");
    }
    
}

#[test]
fn test_ui_bind() {
    let ui = setup_ui();

    ui.pack_value(1000, 5u32);
    ui.pack_control(1001, window());
    ui.pack_control(1002, window());

    // Binding successful
    ui.bind(&1001, &5000, Event::Destroyed, |ui, id, _, _|{
        // When event callbacks are being dipatched, it is impossible to bind a new callback on the same control with the same event
        ui.bind(&1001, &5001, Event::Destroyed, |_, _, _, _|{});
        let r = ui.commit();
        assert!(r.is_err() && r.err().unwrap() == Error::ControlInUse, "Commit was successful");

        // Deleting the control while its executing its callback is also prohibited...
        ui.unpack(id);
        let r = ui.commit();
        assert!(r.is_err() && r.err().unwrap() == Error::ControlInUse, "Commit was successful");

        // On other control or on other events, binding new callback is permitted
        // Still binding new events in a destroy callback is a horrible idea, because, unless specified, the NWG destroy order is random.
        // For this test, if there wasn't the `close` bit at the end, there would be no way to ensure that the commit would work 100% of the time.
        ui.bind(&1002, &5001, Event::Destroyed, |_, _, _, _|{ } );
        ui.commit().expect("Commit was not successful");
    });

    ui.commit().expect("Commit was not successful");

    // Cannot bind events to user values
    ui.bind(&1000, &5000, Event::Destroyed, |_, _, _, _|{});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::ControlRequired, "Commit was successful");

    // Key not in Ui
    ui.bind(&1005, &5000, Event::Destroyed, |_, _, _, _|{});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyNotFound, "Commit was successful");

    // Event not supported
    ui.bind(&1001, &5000, Event::Clicked, |_, _, _, _|{});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::EventNotSupported(Event::Clicked), "Commit was successful");

    // Callback id already exists
    ui.bind(&1001, &5000, Event::Destroyed, |_, _, _, _|{});
    let r = ui.commit();
    assert!(r.is_err() && r.err().unwrap() == Error::KeyExists, "Commit was successful");

    { ui.get::<Window>(&1001).unwrap().close(); }
    dispatch_events();
}

#[test]
fn test_window_control_user_close() {
    let ui = setup_ui();
    let mut callback_executed: bool = false;
    let x = &mut callback_executed as *mut bool;

    ui.pack_control(1000, window());
    ui.bind(&1000, &5000, Event::Destroyed, move |_, _, _, _|{ unsafe{ *(&mut *x) = true; } });
    ui.commit().expect("Commit was not successful");

    assert!(ui.has_id(&1000), "ID 1000 was not found in id after commit");
    
    // Try to close the window
    { ui.get::<Window>(&1000).unwrap().close(); }

    // Dispatch the waiting close event
    dispatch_events();

    assert!(!ui.has_id(&1000), "ID 1000 was found in after window close");
    assert!(callback_executed, "Destroy callback was not executed.")
}

#[test]
fn test_drop_callback() {
    let mut callback_executed: bool = false;
    
    {
        let x = &mut callback_executed as *mut bool;
        let ui = setup_ui();
        ui.pack_control(1000, window());
        ui.bind(&1000, &5000, Event::Destroyed, move |_, _, _, _|{ unsafe{ *(&mut *x) = true; } });
        ui.commit().expect("Commit was not successful");
    }

    assert!(callback_executed, "Destroy callback was not executed.")
}