/// Creates a button and push it left in a horizontal flexbox layout
#[derive(Default)]
pub struct LeftButton {
    layout: nwg::FlexboxLayout,
    pub frame: nwg::Frame,
    pub button: nwg::Button,
}

// Implements default trait so that the control can be used by native windows derive
// The parameters are: subclass_control!(user type, base type, base field name)
nwg::subclass_control!(LeftButton, Frame, frame);

//
// Implement a builder API compatible with native window derive
//
impl LeftButton {
    pub fn builder<'a>() -> LeftButtonBuilder<'a> {
        LeftButtonBuilder {
            parent: None,
            text: "",
            width: 100.0,
            background_color: None
        }
    }
}

pub struct LeftButtonBuilder<'a> {
    parent: Option<nwg::ControlHandle>,
    text: &'a str,
    width: f32,
    background_color: Option<[u8; 3]>,
}

impl<'a> LeftButtonBuilder<'a> {

    pub fn text(mut self, text: &'a str) -> LeftButtonBuilder<'a> {
        self.text = text;
        self
    }

    pub fn width(mut self, width: f32) -> LeftButtonBuilder<'a> {
        self.width = width;
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> LeftButtonBuilder<'a> {
        self.background_color = color;
        self
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> LeftButtonBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, btn: &mut LeftButton) -> Result<(), nwg::NwgError> {
        let parent = self.parent.expect("No parent associated with labeled field");

        nwg::Frame::builder()
            .parent(parent)
            .flags(nwg::FrameFlags::VISIBLE)
            .background_color(self.background_color)
            .build(&mut btn.frame)?;

        nwg::Button::builder()
            .parent(&btn.frame)
            .text(self.text)
            .build(&mut btn.button)?;

        use nwg::stretch::{geometry::Size, style::{Dimension as D, FlexDirection}};

        nwg::FlexboxLayout::builder()
            .parent(&btn.frame)
            .flex_direction(FlexDirection::RowReverse)
            .child(&btn.button)
                .child_size(Size { width: D::Points(self.width), height: D::Auto })
            .build(&btn.layout)?;

        Ok(())
    }
}
