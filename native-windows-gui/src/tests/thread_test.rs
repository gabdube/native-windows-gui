use crate::*;
use std::cell::RefCell;
use std::thread;
use std::time::Duration;


#[derive(Default)]
pub struct ThreadResources {
    count: usize
}

#[derive(Default)]
pub struct ThreadTest {
    resources: RefCell<ThreadResources>,
    pub window: Window,

    layout: FlexboxLayout,

    font: Font,

    counter: TextInput,
    timer_start_btn: Button,
    timer_stop_btn: Button,
    sleep_btn: Button,
    thread_sleep_btn: Button,

    timer: AnimationTimer,
    notice: Notice
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

fn sleep() {
    thread::sleep(Duration::new(5, 0));
}

fn thread_sleep(app: &ThreadTest) {
    app.counter.set_text("Sleeping for 5 sec! (off the GUI thread)");

    let sender = app.notice.sender();
    thread::spawn(move || {
        thread::sleep(Duration::new(5, 0));
        sender.notice();
    });
}

fn notice_me(app: &ThreadTest) {
    app.counter.set_text("Done sleeping of the main thread!");
}


mod partial_canvas_test_ui {
    use super::*;
    use crate::{PartialUi, NwgError, ControlHandle};
    use stretch::style::*;

    impl PartialUi for ThreadTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut ThreadTest, _parent: Option<W>) -> Result<(), NwgError> {
            
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
                .text("Sleep")
                .parent(&data.window)
                .build(&mut data.sleep_btn)?;
            
            Button::builder()
                .text("Sleep (off thread)")
                .parent(&data.window)
                .build(&mut data.thread_sleep_btn)?;

            AnimationTimer::builder()
                .parent(&data.window)
                .interval(Duration::from_millis(25))
                .build(&mut data.timer)?;

            Notice::builder()
                .parent(&data.window)
                .build(&mut data.notice)?;

            FlexboxLayout::builder()
                .parent(&data.window)
                .flex_direction(FlexDirection::Column)
                .auto_size(true)
                .auto_spacing(Some(5))
                .child(&data.counter)
                .child(&data.timer_start_btn)
                .child(&data.timer_stop_btn)
                .child(&data.sleep_btn)
                .child(&data.thread_sleep_btn)
                .build(&data.layout)?;

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
                    } else if &handle == &self.sleep_btn {
                        sleep();
                    } else if &handle == &self.thread_sleep_btn {
                        thread_sleep(self);
                    },
                E::OnTimerTick => 
                    if &handle == &self.timer {
                        timer_tick(self)
                    },
                E::OnNotice => 
                    if &handle == &self.notice {
                        notice_me(self)
                    },
                _ => {}
            }
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle]
        }

    }

}
