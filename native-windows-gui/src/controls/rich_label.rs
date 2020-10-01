/**
A rich label is a label that supports rich text. This control is built on top of the rich text box control and as such
require the `rich-textbox` feature.

Unlike the basic `Label`, this version supports:

* Colored text
* Multiple fonts
* Styled text such as bold, underscore, strikeout, etc
* Bullet point list
* Paragraph with custom indent/offset
* Custom line spacing

**Control events:**

* `MousePress(_)`: Generic mouse press events on the label
* `OnMouseMove`: Generic mouse mouse event
* `OnMouseWheel`: Generic mouse wheel event

** Example **

```rust
use native_windows_gui as nwg;
fn build_label(label: &mut nwg::RichLabel, window: &nwg::Window) {
    nwg::RichLabel::builder()
        .text("Hello")
        .parent(window)
        .build(label);
}
```

*/
pub struct RichLabel {

}
