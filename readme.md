# Native Windows GUI

<b>Native Windows GUI (NWG)</b> is a thin GUI toolkit built over the <b>Microsoft Windows WINAPI</b> for rust. The current version is <b>0.2.0 BETA 1</b>. The library is close to be production ready, but still lacks some important features and some useful controls and resources.

NWG uses [retep998/winapi-rs](https://github.com/retep998/winapi-rs) and works on all rust channels and most
rust versions. NWG was tested on Windows 8.1 and Windows 10 using the MSVC ABI build but any version of Microsoft Windows supported by Rust is supposed to be
supported by NWG (vista and up).

## Beta notes

<b>The beta release is a rewrite</b>, so the <b>ALPHA</b> code won't work anymore. Most of the concepts remain though.

NWG now supports macro templates in order to make interface definition much less painful.

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

NWG has a complete documentation available here:  https://gabdube.github.io/native-windows-gui/

The documentation alone should be enough to introduce to the basics of NWG.

(btw) If English is your first language (it's not mine), it would be super kind to give me feedback about quality of the docs.

# Example
Having cargo installed and in your PATH, execute the following code to run the included example:

```bash
git clone git@github.com:gabdube/native-windows-gui.git
cd native-windows-gui
cargo run --example showcase
cargo run --example canvas
cargo run --example templating
```

![A GUI](/img/showcase.png "Image")  

![A GUI](/img/canvas.png "Image")  
