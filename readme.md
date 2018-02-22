# Native Windows GUI

**Native Windows GUI is no longer maintained. The dev branch is where the lastest features are if you are in dire need of a Windows only GUI library**

**Native Windows GUI (NWG)** is a thin GUI toolkit built over the **Microsoft Windows WINAPI** for rust. The 
current version is **0.2.0 BETA 1**. The library is close to production ready, but still lacks
some important features and some useful controls and resources.

NWG uses [retep998/winapi-rs](https://github.com/retep998/winapi-rs) and works on all rust channels and most
rust versions. NWG was tested on Windows 8.1 and Windows 10 using the MSVC ABI build, but any version of Microsoft Windows supported by Rust is supposed to be
supported by NWG (vista and up).

**NWG will not compile on the GNU toolchain**. This is because `comctl32.lib` doesn't include a function required by NWG.

## Why NWG?

Is native-windows-gui the gui framework you are looking for? It is ...

* For those who want to develop a Windows application with the smallest executable and memory footprint possible. 
* For those who don't like dependencies. NWG only requires some `winapi-rs` crates and do not depend on external "executable" code 
* For those who want a canvas to draw pretty things, NWG has a very powerful (and light-weight) canvas build over Direct2D
* For those who don't like to manage widgets (aka controls, aka stuff the user clicks on). The UI manages the controls and the resources for you.
* For those who like documentation, NWG has one (and I think it's pretty good). Oh and its API is available online too: https://gabdube.github.io/native-windows-gui/ 
* For those who want a light and simple API, NWG might be for you

And it isn't...

* For those who want portability across systems. Maybe it will work with WINE though...
* For those who want to deploy a production ready application as soon as possible. The first stable version will take some time to come out.
* For those who want a safe api to create custom controls, nwg is not there YET...
* For those who want a UI to track a killers API address, Visual Basic is better (source: CSI)

# Installation
To use NWG in your project add it to cargo.toml: 

```toml
[dependencies]
native-windows-gui = "0.2.0"
```

And then, in main.rs or lib.rs : 

```rust
extern crate native_windows_gui as nwg;
```

# Documentation

NWG has complete documentation, available here:  https://gabdube.github.io/native-windows-gui/

The documentation alone should be enough to introduce to the basics of NWG.

(btw) If English is your first language (it's not mine), it would be super kind to give me feedback about quality of the docs.

# Example
Having cargo installed and in your PATH, execute the following code to run the included example:  
(scroll further down for a code example)

```bash
git clone git@github.com:gabdube/native-windows-gui.git
cd native-windows-gui
cargo run --example showcase
cargo run --example canvas
cargo run --example templating
```

![A GUI](/img/showcase.png "Image")  

![A GUI](/img/canvas.png "Image")  

# Code example
```rust
/**
    Simple example on how to use the nwg template system.
*/

//To hide console window
#![windows_subsystem = "windows"]

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Event, Ui, simple_message, fatal_message, dispatch_events};

/// Custom enums are the preferred way to define ui ids. It's clearer and more extensible than any other types (such as &'str).
#[derive(Debug, Clone, Hash)]
pub enum AppId {
    // Controls
    MainWindow,
    NameInput, 
    HelloButton,
    Label(u8),   // Ids for static controls that won't be referenced in the Ui logic can be shortened this way.

    // Events
    SayHello,

    // Resources
    MainFont,
    TextFont
}

use AppId::*; // Shortcut

nwg_template!(
    head: setup_ui<AppId>,
    controls: [
        (MainWindow, nwg_window!( title="Template Example"; size=(280, 105) )),
        (Label(0), nwg_label!( parent=MainWindow; text="Your Name: "; position=(5,15); size=(80, 25); font=Some(TextFont) )),
        (NameInput, nwg_textinput!( parent=MainWindow; position=(85,13); size=(185,22); font=Some(TextFont) )),
        (HelloButton, nwg_button!( parent=MainWindow; text="Hello World!"; position=(5, 45); size=(270, 50); font=Some(MainFont) ))
    ];
    events: [
        (HelloButton, SayHello, Event::Click, |ui,_,_,_| {
            let your_name = nwg_get!(ui; (NameInput, nwg::TextInput));
            simple_message("Hello", &format!("Hello {}!", your_name.get_text()) );
        })
    ];
    resources: [
        (MainFont, nwg_font!(family="Arial"; size=27)),
        (TextFont, nwg_font!(family="Arial"; size=17))
    ];
    values: []
);

fn main() {
    let app: Ui<AppId>;

    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    if let Err(e) = setup_ui(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    dispatch_events();
}
```
