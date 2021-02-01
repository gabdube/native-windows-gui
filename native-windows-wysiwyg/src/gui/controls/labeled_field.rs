/// Binds a label and a textinput field together
#[derive(Default)]
pub struct LabeledField {
    layout: nwg::FlexboxLayout,
    pub frame: nwg::Frame,
    pub label: nwg::Label,
    pub input: nwg::TextInput,
}

// Implements default trait so that the control can be used by native windows derive
// The parameters are: subclass_control!(user type, base type, base field name)
nwg::subclass_control!(LabeledField, Frame, frame);

//
// Implement a builder API compatible with native window derive
//
impl LabeledField {
    pub fn builder<'a>() -> LabeledFieldBuilder<'a> {
        LabeledFieldBuilder {
            parent: None,
            label_text: "",
            value: "",
            label_width: 200.0,
            disabled: false,
            background_color: None
        }
    }

    pub fn set_text(&self, text: &str) {
        self.input.set_text(text);
    }

    pub fn text(&self) -> String {
        self.input.text()
    }

    pub fn set_enabled(&self, enable: bool) {
        self.input.set_readonly(!enable);
    }

}

pub struct LabeledFieldBuilder<'a> {
    parent: Option<nwg::ControlHandle>,
    label_text: &'a str,
    label_width: f32,
    value: &'a str,
    disabled: bool,
    background_color: Option<[u8; 3]>,
}

impl<'a> LabeledFieldBuilder<'a> {
    pub fn text(mut self, text: &'a str) -> LabeledFieldBuilder<'a> {
        self.label_text = text;
        self
    }

    pub fn value(mut self, val: &'a str) -> LabeledFieldBuilder<'a> {
        self.value = val;
        self
    }

    pub fn disabled(mut self, val: bool) -> LabeledFieldBuilder<'a> {
        self.disabled = val;
        self
    }

    pub fn label_width(mut self, val: f32) -> LabeledFieldBuilder<'a> {
        self.label_width = val;
        self
    }

    pub fn background_color(mut self, color: Option<[u8;3]>) -> LabeledFieldBuilder<'a> {
        self.background_color = color;
        self
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> LabeledFieldBuilder<'a> {
        self.parent = Some(p.into());
        self
    }

    pub fn build(self, labeled: &mut LabeledField) -> Result<(), nwg::NwgError> {
        let parent = self.parent.expect("No parent associated with labeled field");

        nwg::Frame::builder()
            .parent(parent)
            .flags(nwg::FrameFlags::VISIBLE)
            .background_color(self.background_color)
            .build(&mut labeled.frame)?;

        nwg::Label::builder()
            .parent(&labeled.frame)
            .text(self.label_text)
            .h_align(nwg::HTextAlign::Left)
            .background_color(self.background_color)
            .build(&mut labeled.label)?;

        nwg::TextInput::builder()
            .parent(&labeled.frame)
            .readonly(self.disabled)
            .text(self.value)
            .build(&mut labeled.input)?;

        use nwg::stretch::{geometry::Size, style::{Dimension as D, FlexDirection}};

        nwg::FlexboxLayout::builder()
            .parent(&labeled.frame)
            .flex_direction(FlexDirection::Row)
            .child(&labeled.label)
                .child_flex_shrink(0.0)
                .child_size(Size { width: D::Points(self.label_width), height: D::Auto })
            .child(&labeled.input)
                .child_size(Size { width: D::Percent(1.0), height: D::Auto })
            .build(&labeled.layout)?;

        Ok(())
    }
}
