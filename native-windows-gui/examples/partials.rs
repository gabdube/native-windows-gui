/*!
    An application that load different interfaces using the partial feature.
    Partials can be used to split large GUI application into smaller bits.

    Requires the following features: `cargo run --example partials --features "listbox frame combobox checkbox"`
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct PartialDemo {
    window: nwg::Window,
    layout: nwg::BoxLayout,
    menu: nwg::ListBox<&'static str>,
    frame1: nwg::Frame,
    frame2: nwg::Frame,
    frame3: nwg::Frame,

    people_ui: PeopleUi,
    animal_ui: AnimalUi,
    food_ui: FoodUi,
}

impl PartialDemo {

    fn change_interface(&self) {
        self.frame1.set_visible(false);
        self.frame2.set_visible(false);
        self.frame3.set_visible(false);

        match self.menu.selection() {
            None | Some(0) => self.frame1.set_visible(true),
            Some(1) => self.frame2.set_visible(true),
            Some(2) => self.frame3.set_visible(true),
            Some(_) => unreachable!()
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default)]
pub struct PeopleUi {
    layout: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,
    label3: nwg::Label,

    name_input: nwg::TextInput,
    age_input: nwg::TextInput,
    job_input: nwg::TextInput,
}

#[derive(Default)]
pub struct AnimalUi {
    layout: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,
    label3: nwg::Label,

    name_input: nwg::TextInput,
    race_input: nwg::ComboBox<&'static str>,
    is_soft_input: nwg::CheckBox
}

#[derive(Default)]
pub struct FoodUi {
    layout: nwg::GridLayout,

    label1: nwg::Label,
    label2: nwg::Label,

    name_input: nwg::TextInput,
    tasty_input: nwg::CheckBox,
}


//
// ALL of this stuff is handled by native-windows-derive
//
mod partial_demo_ui {
    use native_windows_gui as nwg;
    use self::nwg::PartialUi;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct PartialDemoUi {
        inner: PartialDemo
    }

    impl nwg::NativeUi<PartialDemo, PartialDemoUi> for PartialDemo {
        fn build_ui(mut data: PartialDemo) -> Result<Rc<PartialDemoUi>, nwg::NwgError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .size((500, 400))
                .position((300, 300))
                .title("Many UI")
                .build(&mut data.window)?;

            nwg::ListBox::builder()
                .collection(vec!["People", "Animals", "Food"])
                .parent(&data.window)
                .build(&mut data.menu)?;

            nwg::Frame::builder()
                .parent(&data.window)
                .build(&mut data.frame1)?;

            nwg::Frame::builder()
                .flags(nwg::FrameFlags::BORDER)
                .parent(&data.window)
                .build(&mut data.frame2)?;

            nwg::Frame::builder()
                .flags(nwg::FrameFlags::BORDER)
                .parent(&data.window)
                .build(&mut data.frame3)?;

            // Partials
            PeopleUi::build_partial(&mut data.people_ui, Some(&data.frame1))?;
            AnimalUi::build_partial(&mut data.animal_ui, Some(&data.frame2))?;
            FoodUi::build_partial(&mut data.food_ui, Some(&data.frame3))?;

            // Wrap-up
            let ui = Rc::new(PartialDemoUi { inner: data });

            // Events
            let mut window_handles = vec![&ui.window.handle];
            window_handles.append(&mut ui.people_ui.handles());
            window_handles.append(&mut ui.animal_ui.handles());
            window_handles.append(&mut ui.food_ui.handles());

            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, evt_data, handle| {
                    evt_ui.people_ui.process_event(evt, &evt_data, handle);
                    evt_ui.animal_ui.process_event(evt, &evt_data, handle);
                    evt_ui.food_ui.process_event(evt, &evt_data, handle);
                    
                    match evt {
                        E::OnListBoxSelect => 
                            if &handle == &evt_ui.menu {
                                PartialDemo::change_interface(&evt_ui.inner);
                            },
                        E::OnWindowClose => 
                            if &handle == &evt_ui.window {
                                PartialDemo::exit(&evt_ui.inner);
                            },
                        _ => {}
                    }
                };

                nwg::full_bind_event_handler(handle, handle_events);
            }

            // Layout

            nwg::BoxLayout::builder()
                .parent(&ui.window)
                .layout_type(nwg::BoxLayoutType::Horizontal)
                .cell_count(Some(4))
                .child(0, &ui.menu)
                .child_item(nwg::BoxLayoutItem::new(&ui.frame1, 1, 3))
                .child_item(nwg::BoxLayoutItem::new(&ui.frame2, 1, 3))
                .child_item(nwg::BoxLayoutItem::new(&ui.frame3, 1, 3))
                .build(&ui.layout);
            
            return Ok(ui);
        }
    }


    impl Deref for PartialDemoUi {
        type Target = PartialDemo;

        fn deref(&self) -> &PartialDemo {
            &self.inner
        }
    }

}

mod partial_people_ui {
    use native_windows_gui as nwg;
    use self::nwg::{PartialUi, NwgError, ControlHandle};
    use super::*;
    
    impl PartialUi<PeopleUi> for PeopleUi {

        fn build_partial<W: Into<ControlHandle>>(data: &mut PeopleUi, parent: Option<W>) -> Result<(), NwgError> {
            let parent = parent.unwrap().into();

            nwg::Label::builder()
                .text("Name:")
                .parent(&parent)
                .build(&mut data.label1)?;

            nwg::Label::builder()
                .text("Age:")
                .parent(&parent)
                .build(&mut data.label2)?;

            nwg::Label::builder()
                .text("Job:")
                .parent(&parent)
                .build(&mut data.label3)?;

            nwg::TextInput::builder()
                .text("John Doe")
                .parent(&parent)
                .build(&mut data.name_input)?;

            nwg::TextInput::builder()
                .text("75")
                .flags(nwg::TextInputFlags::VISIBLE | nwg::TextInputFlags::NUMBER)
                .parent(&parent)
                .build(&mut data.age_input)?;

            nwg::TextInput::builder()
                .text("Programmer")
                .parent(&parent)
                .build(&mut data.job_input)?;

            nwg::GridLayout::builder()
                .parent(&parent)
                .max_size([1000, 150])
                .min_size([100, 120])
                .child(0, 0, &data.label1)
                .child(0, 1, &data.label2)
                .child(0, 2, &data.label3)
                .child(1, 0, &data.name_input)
                .child(1, 1, &data.age_input)
                .child(1, 2, &data.job_input)
                .build(&data.layout);

            Ok(())
        }

        fn process_event<'a>(&self, _evt: nwg::Event, _evt_data: &nwg::EventData, _handle: ControlHandle) {
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            Vec::new()
        }
    }
}

mod partial_animal_ui {
    use native_windows_gui as nwg;
    use self::nwg::{PartialUi, NwgError, ControlHandle};
    use super::*;

    impl PartialUi<AnimalUi> for AnimalUi {

        fn build_partial<W: Into<ControlHandle>>(data: &mut AnimalUi, parent: Option<W>) -> Result<(), NwgError> {
            let parent = parent.unwrap().into();

            nwg::Label::builder()
                .text("Name:")
                .parent(&parent)
                .build(&mut data.label1)?;

            nwg::Label::builder()
                .text("Race:")
                .parent(&parent)
                .build(&mut data.label2)?;

            nwg::Label::builder()
                .text("Is fluffy:")
                .parent(&parent)
                .build(&mut data.label3)?;

            nwg::TextInput::builder()
                .text("Mittens")
                .parent(&parent)
                .build(&mut data.name_input)?;

            nwg::ComboBox::builder()
                .collection(vec!["Cat", "Dog", "Pidgeon", "Monkey"])
                .selected_index(Some(0))
                .parent(&parent)
                .build(&mut data.race_input)?;

            nwg::CheckBox::builder()
                .text("")
                .check_state(nwg::CheckBoxState::Checked)
                .parent(&parent)
                .build(&mut data.is_soft_input)?;

            nwg::GridLayout::builder()
                .parent(&parent)
                .max_size([1000, 150])
                .min_size([100, 120])
                .child(0, 0, &data.label1)
                .child(0, 1, &data.label2)
                .child(0, 2, &data.label3)
                .child(1, 0, &data.name_input)
                .child(1, 1, &data.race_input)
                .child(1, 2, &data.is_soft_input)
                .build(&data.layout);

            Ok(())
        }

        fn process_event<'a>(&self, _evt: nwg::Event, _evt_data: &nwg::EventData, _handle: ControlHandle) {
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            Vec::new()
        }
    }
}

mod partial_food_ui {
    use native_windows_gui as nwg;
    use self::nwg::{PartialUi, NwgError, ControlHandle};
    use super::*;

    impl PartialUi<FoodUi> for FoodUi {
        fn build_partial<W: Into<ControlHandle>>(data: &mut FoodUi, parent: Option<W>) -> Result<(), NwgError> {
            let parent = parent.unwrap().into();

            nwg::Label::builder()
                .text("Name:")
                .parent(&parent)
                .build(&mut data.label1)?;

            nwg::Label::builder()
                .text("Tasty:")
                .parent(&parent)
                .build(&mut data.label2)?;

            nwg::TextInput::builder()
                .text("Banana")
                .parent(&parent)
                .build(&mut data.name_input)?;

            nwg::CheckBox::builder()
                .text("")
                .check_state(nwg::CheckBoxState::Checked)
                .parent(&parent)
                .build(&mut data.tasty_input)?;

            nwg::GridLayout::builder()
                .parent(&parent)
                .max_size([1000, 90])
                .min_size([100, 80])
                .child(0, 0, &data.label1)
                .child(0, 1, &data.label2)
                .child(1, 0, &data.name_input)
                .child(1, 1, &data.tasty_input)
                .build(&data.layout);

            Ok(())
        }

        fn process_event<'a>(&self, _evt: nwg::Event, _evt_data: &nwg::EventData, _handle: ControlHandle) {
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            Vec::new()
        }
    }
}



fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _ui = PartialDemo::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
