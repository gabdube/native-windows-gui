# Native Windows GUI

<b>NWG is currently being rewritten. Please see the `v2` branch for the lastest api and features</b>

<b>Native Windows GUI (NWG)</b> is a thin GUI toolkit built over the <b>Microsoft Windows WINAPI</b> for rust. The current version is
<b>0.1.0 ALPHA</b>. The library is not production ready, but it has enough features implemented in order 
to create simple GUI applications.

NWG uses [retep998/winapi-rs](https://github.com/retep998/winapi-rs) and works on all rust channels and most
rust versions. NWG was tested on Windows 8.1 and Windows 10 using the MSVC ABI build but any version of Microsoft Windows supported by Rust is supposed to be
supported by NWG (vista and up).

Native Windows GUI do not work like your average GUI library, it works like some kind of opaque
service. It's kinda hard to explain it in a few words so you should check the [first chapter of the docs](https://gabdube.github.io/native-windows-gui/book_20.html) .

# Installation
To use NWG in your project add it to cargo.toml: 

```toml
[dependencies]
native-windows-gui = "0.1.1"
```

And then, in main.rs or lib.rs : 

```rust
extern crate native_windows_gui as nwg;
```

# Documentation

NWG has a complete documentation available here:  https://gabdube.github.io/native-windows-gui/

Have I mentionned that you should REALLY read the [first chapter of the docs](https://gabdube.github.io/native-windows-gui/book_20.html) ? I mean, it explains
the whole API and there's a simple example included.

(btw) If English is your first language (it's not mine), it would be super kind to give me feedback about quality of the docs.

# Example
Having cargo installed and in your PATH, execute the following code to run the included example:

```bash
git clone git@github.com:gabdube/native-windows-gui.git
cd native-windows-gui
cargo run --example simple_form
```


![A GUI](/img/simple_form.PNG "Image")  
