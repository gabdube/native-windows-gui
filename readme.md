# Native Windows GUI

Native Window GUI (nwg for short) is a GUI library for Windows. Its current state is a "Proof of concept"

Because current GUI libraries uses OOP in order to work and because these concepts do not translate nicely in rust, i've
decided to try something new:

**Instead of exposing every widget as a complete type to the user, I decided to completely hide the implementation behind a "manager" (UI) object**

Widgets are then exposed as an opaque type choosen by the user himself (example: `i32`, `&'static str`, etc).

For this POC, I implemented **actions**(what would normally be methods), **callbacks** and **control templates** (what would normally be the widgets). Each of
these concept is exposed by a single method on the manager object. **The NWG api has only 5 functions/methods**. See `tests/ui.rs` for an example.


## The manager

The manager `Ui`, is the object that handle the UI on a single thread. The type passed to the manager define the type of the widgets identifier.
A widget identifier is unique and can't be used twice.

Also, nwg offers the `dispatch_events` that dispatches events until a quit event is reveived.

Example:

```rust
let mut ui: nwg::Ui<&'static str> = nwg::Ui::new();

nwg::dispatch_events();
```

## Control templates

Control templates replaces "normal" widgets. They are transparent structures that implements the `ControlTemplate` trait.
These structures describe a Windows Control (example: `Windows`, `Buttons`).

Once a template structure has been filled, it can be passed to the Window Manager in order to create the widget in the interface.  
This is done through the `new_control` methods.

Example:

```rust
let hello_btn = nwg::controls::Button {
        text: "Say hello!".to_string(),
        size: (480, 50),
        position: (10, 10),
        parent: "MainWindow"
};

ui.new_control("HelloBtn", hello_btn).unwrap();
```

## Actions

Actions replace what would normally be widgets methods. Action is a big enum of pretty much any that can be applied to a widget. When an action is
sent through an manager, the widget evals the action, and then, if it is supported, it can return an ActionReturn value. ActionReturn, just like action
is a big enum that can return anything.

Notes:

* Some action (ex: `SetText`) could be applied to many widgets.
* Actions/Action return that are bigger than 8 bytes will be boxed
* Big action (ex: `Message`) can have action helper to make the action creation easier
* If a control receive a unsupported action, it must return `ActionReturn::NotSupported`.

Actions are sont to a control using the `exec` method

Example:

```rust
ui.exec("MainWindow", nwga::message("Hello", "Hello World!", 0)).unwrap();
```

## Callback

Callbacks are defined in a big enum: `EventCallback`. This enum contains any callback that can be applied to a widget.
The enum member each take a Boxed `Fn` that will be called internally when system events are processed.

Note:

* Right now, any callback can be bound to any widget. I think it would be best to add a `supported_callback` to the `ControlTemplate` trait and then raise error if someone try to bind an unsupported callback.
* Right now callback binding is quite ugly, i'd like to change that if possible.

Callbacks are bound to a widget using the `bind` method.

Example:

```rust
ui.bind("MainWindow", EventCallback::MouseUp(Box::new(|ui, caller, x, y, btn, modifiers| {
        println!("Caller: {:?}", caller);
        println!("Left mouse button pressed: {:?}", (btn & nwgc::BTN_MOUSE_LEFT) != 0 );
        println!("Ctrl pressed: {:?}", (modifiers & nwgc::MOD_MOUSE_CTRL) != 0 );
        println!("Mouse position: {:?} {:?}", x, y);
    })));
```