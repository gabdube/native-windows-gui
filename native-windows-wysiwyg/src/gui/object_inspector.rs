use nwd::NwgPartial;
use std::cell::Cell;


#[derive(Default)]
#[derive(NwgPartial)]
pub struct ObjectInspector {
    pub(super) user_width: Cell<u32>,

    #[nwg_control(size: (275, 0))]
    pub(super) container_frame: nwg::Frame,

    #[nwg_layout(spacing: 0, margin: [0,0,0,0])]
    layout: nwg::GridLayout,
}

impl ObjectInspector {

    pub(super) fn init(&self) {
        self.user_width.set(275);
    }

}
