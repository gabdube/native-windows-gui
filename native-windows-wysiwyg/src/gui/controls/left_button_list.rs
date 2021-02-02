/// Creates a list of buttons and push them left in a horizontal flexbox layout
#[derive(Default)]
pub struct LeftButtonList {
    layout: nwg::FlexboxLayout,
    pub frame: nwg::Frame,
    pub buttons: Vec<nwg::Button>,
}

// Implements default trait so that the control can be used by native windows derive
// The parameters are: subclass_control!(user type, base type, base field name)
nwg::subclass_control!(LeftButtonList, Frame, frame);

//
// Implement a builder API compatible with native window derive
//
impl LeftButtonList {
    pub fn builder() -> LeftButtonListBuilder {
        LeftButtonListBuilder {
            parent: None,
            width: 100.0,
            background_color: None,
            buttons: Vec::new(),
        }
    }

    pub fn set_enabled(&self, enable: bool) {
        for btn in self.buttons.iter() {
            btn.set_enabled(enable);
        }
    }
}

pub struct LeftButtonListBuilder {
    parent: Option<nwg::ControlHandle>,
    width: f32,
    background_color: Option<[u8; 3]>,
    buttons: Vec<String>,
}

impl LeftButtonListBuilder {

    pub fn buttons<S: Into<String>>(mut self, buttons: Vec<S>) -> LeftButtonListBuilder {
        self.buttons = buttons.into_iter().map(|name| name.into()).collect();
        self
    }

    pub fn width(mut self, width: f32) -> LeftButtonListBuilder {
        self.width = width;
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> LeftButtonListBuilder {
        self.background_color = color;
        self
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> LeftButtonListBuilder {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, out: &mut LeftButtonList) -> Result<(), nwg::NwgError> {
        use nwg::stretch::{geometry::Size, style::{Dimension as D, FlexDirection}};

        let parent = self.parent.expect("No parent associated with labeled field");

        nwg::Frame::builder()
            .parent(parent)
            .flags(nwg::FrameFlags::VISIBLE)
            .background_color(self.background_color)
            .build(&mut out.frame)?;

        for (i, text) in self.buttons.iter().enumerate() {
            out.buttons.push(nwg::Button::default());
            nwg::Button::builder()
                .parent(&out.frame)
                .text(&text)
                .build(&mut out.buttons[i])?;
        }

        let mut layout = nwg::FlexboxLayout::builder()
            .parent(&out.frame)
            .flex_direction(FlexDirection::RowReverse);

        for i in 0..self.buttons.len() {
            layout = layout.child(&out.buttons[i])
                           .child_size(Size { width: D::Points(self.width), height: D::Auto });
        }

        layout.build(&out.layout)?;

        Ok(())
    }
}
