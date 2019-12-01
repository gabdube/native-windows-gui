use crate::*;
use std::cell::RefCell;

#[derive(Default)]
pub struct ThreadResources {
    count: usize
}

#[derive(Default)]
pub struct ThreadTest {
    resources: RefCell<ThreadResources>,
    pub window: Window,

    font: Font,

    counter: TextInput,
    timer_start_btn: Button,
    timer_stop_btn: Button,
    no_thread_count_btn: Button,
    thread_count_btn: Button,

    timer: Timer,
}

fn timer_tick(app: &ThreadTest) {
    let mut rc = app.resources.borrow_mut();
    rc.count += 1;
    app.counter.set_text(&format!("{}", rc.count));
}

fn start_timer(app: &ThreadTest) {
    app.timer.start();
}

fn stop_timer(app: &ThreadTest) {
    app.timer.stop();
}


mod partial_canvas_test_ui {
    use super::*;
    use crate::{PartialUi, SystemError, ControlHandle};

    impl PartialUi<ThreadTest> for ThreadTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut ThreadTest, _parent: Option<W>) -> Result<(), SystemError> {
            
            Font::builder()
                .size(40)
                .family("Consolas")
                .build(&mut data.font)?;
            
            Window::builder()
                .flags(WindowFlags::WINDOW)
                .size((300, 300))
                .position((250, 100))
                .title("Threads")
                .build(&mut data.window)?;

            TextInput::builder()
                .parent(&data.window)
                .font(Some(&data.font))
                .build(&mut data.counter)?;

            Button::builder()
                .text("Start timer")
                .parent(&data.window)
                .build(&mut data.timer_start_btn)?;

            Button::builder()
                .text("Stop timer")
                .parent(&data.window)
                .build(&mut data.timer_stop_btn)?;

            Button::builder()
                .text("Count to 1 billion")
                .parent(&data.window)
                .build(&mut data.no_thread_count_btn)?;
            
            Button::builder()
                .text("Count to 1 billion (off thread)")
                .parent(&data.window)
                .build(&mut data.thread_count_btn)?;

            Timer::builder()
                .parent(&data.window)
                .interval(25)
                .build(&mut data.timer)?;

            VBoxLayout::builder()
                .parent(&data.window)
                .child(0, &data.counter)
                .child(1, &data.timer_start_btn)
                .child(2, &data.timer_stop_btn)
                .child(3, &data.no_thread_count_btn)
                .child(4, &data.thread_count_btn)
                .build();

            Ok(())
        }

        fn process_event<'a>(&self, evt: Event, mut _evt_data: &EventData, handle: ControlHandle) {
            use crate::Event as E;

            match evt {
                E::OnButtonClick =>
                    if &handle == &self.timer_start_btn {
                        start_timer(self);
                    } else if &handle == &self.timer_stop_btn {
                        stop_timer(self);
                    },
                E::OnTimerTick => 
                    if &handle == &self.timer {
                        timer_tick(self)
                    },
                _ => {}
            }
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle]
        }

    }

}
