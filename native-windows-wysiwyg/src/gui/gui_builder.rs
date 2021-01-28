/*!
    Main window gui components
*/
use nwd::NwgUi;
use nwg::{NativeUi, NwgError};
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};

use std::cell::{RefCell, RefMut, Ref};
use crate::AppState;
use super::{gui_error::*, widget_box::WidgetBox, object_inspector::ObjectInspector};

use winapi::shared::windef::HBRUSH;

/// Holds GDI objects for painting
struct PaintData {
    background: HBRUSH,
}

#[derive(Default, NwgUi)]
pub struct GuiBuilder {
    /// Application state
    state: Option<RefCell<AppState>>,

    /// GDI object for painting
    paint_data: RefCell<Option<PaintData>>,

    /// Name of the gui process that currently borrow the state
    /// Used for debugging if the app gets borrowed twice
    debug_borrow: RefCell<Option<&'static str>>,

    #[nwg_control(size: (900, 800), title: "Native Windows WYSIWYG", flags: "MAIN_WINDOW")]
    #[nwg_events( 
        OnInit: [GuiBuilder::init],
        OnWindowClose: [GuiBuilder::close],
    )]
    main_window: nwg::Window,

    #[nwg_control(size: (800, 800), title: "Demo", flags: "MAIN_WINDOW")]
    #[nwg_events(OnPaint: [GuiBuilder::fill_demo_background(SELF, EVT_DATA)])]
    demo_window: nwg::Window,

    #[nwg_layout(
        parent: main_window,
        flex_direction: FlexDirection::Row,
        padding: Rect { start: Points(10.0), end: Points(10.0), top: Points(10.0), bottom: Points(10.0), },
    )]
    layout: nwg::FlexboxLayout,

    //
    // Resources & extra
    //

    #[nwg_resource(action: nwg::FileDialogAction::OpenDirectory)]
    directory_dialog: nwg::FileDialog,

    //
    // File menu
    //
    #[nwg_control(parent: main_window, text: "&File")]
    file_menu: nwg::Menu,

    #[nwg_control(parent: file_menu, text: "&New Project")]
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::create_new_project] )]
    new_project_item: nwg::MenuItem,

    #[nwg_control(parent: file_menu)]
    sp1: nwg::MenuSeparator,

    #[nwg_control(parent: file_menu, text: "&Open project")]
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::open_project] )]
    open_project_item: nwg::MenuItem,

    #[nwg_control(parent: file_menu)]
    sp2: nwg::MenuSeparator,

    #[nwg_control(parent: file_menu, text: "E&xit")]
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::close] )]
    exit_item: nwg::MenuItem,

    //
    // Window menu
    //
    #[nwg_control(parent: main_window, text: "&Window")]
    window_menu: nwg::Menu,

    #[nwg_control(parent: window_menu, text: "&Show demo window")]
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::show_demo] )]
    show_demo_item: nwg::MenuItem,

    //
    // Controls List
    //
    #[nwg_control(parent: main_window, flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, flex_shrink: 0.0, size: Size { width: Points(300.0) , height: Percent(1.0) } )]
    widget_box_frame: nwg::Frame,

    #[nwg_partial(parent: widget_box_frame)]
    widget_box: WidgetBox,

    //
    // Main option container
    //
    #[nwg_control(parent: main_window)]
    #[nwg_layout_item(
        layout: layout,
        size: Size { width: Percent(1.0) , height: Percent(1.0) },
        margin: Rect { start: Points(10.0), ..Default::default() },
    )]
    options_container: nwg::TabsContainer,

    //
    // Project general settings
    //
    #[nwg_control(parent: options_container, text: "Project settings")]
    project_settings_tab: nwg::Tab,

    //
    // Control inspector
    //
    #[nwg_control(parent: options_container, text: "Control Data")]
    object_inspector_tab: nwg::Tab,

    #[nwg_partial(parent: object_inspector_tab)]
    object_inspector: ObjectInspector,

    //
    // Resources Manager
    //
    #[nwg_control(parent: options_container, text: "Resources")]
    resources_tab: nwg::Tab,

    //
    // Events Manager
    //
    #[nwg_control(parent: options_container, text: "Events")]
    events_tab: nwg::Tab,
}

impl GuiBuilder {

    pub fn build(state: AppState) -> Result<self::gui_builder_ui::GuiBuilderUi, NwgError> {
        let mut font = nwg::Font::default();
        nwg::Font::builder()
            .family("Segoe UI")
            .build(&mut font)?;
        nwg::Font::set_global_default(Some(font));

        let mut builder = Self::default();
        builder.state = Some(RefCell::new(state));
        GuiBuilder::build_ui(builder)
    }

    pub fn destroy(&self) {

    }

    fn init(&self) {
        self.widget_box.init();
        self.object_inspector.init();
        self.options_container.set_enabled(false);

        // Setup paint data
        *self.paint_data.borrow_mut() = unsafe {
            use winapi::um::wingdi::{CreateSolidBrush, RGB};
            
            let data = PaintData {
                background: CreateSolidBrush(RGB(80, 80, 80))
            };

            Some(data)
        };

        // Position and show the window
        let (x, y) = self.main_window.position();
        let (w, _h) = self.main_window.size();

        self.demo_window.set_position(x + (w as i32) + 10, y);
        self.demo_window.set_visible(true);

        self.main_window.set_visible(true);
        self.main_window.set_focus();

        // Disable ui until the user load a project
        self.enable_ui(false);
    }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn create_new_project(&self) {
        let err_title = "Failed to create new project";
        let window = &self.main_window;

        // Check if project is already open
        if let Ok(mut state) = self.state_mut("create_new_project") {
            if state.project_loaded() {
                // Ask the user if he wishes to close the current project
                let msg = nwg::MessageParams {
                    title: "Project already loaded",
                    content: "A project is already open. Do you wish to close it?",
                    buttons: nwg::MessageButtons::YesNo,
                    icons: nwg::MessageIcons::Warning
                };

                match nwg::modal_message(window, &msg) {
                    nwg::MessageChoice::No => { return; },
                    nwg::MessageChoice::Yes => state.close_project(),
                    _ => unreachable!()
                }
            }
        }

        // Fetch project path
        let new_project_path = match self.select_folder("Create a new project") {
            Ok(Some(path)) => path,
            Ok(None) => { return; },
            Err(e) => {
                match e {
                    CreateProjectError::BadPath(path) => {
                        let content = format!("The selected path is not a valid utf-8 string: {:?}", path);
                        nwg::modal_error_message(window, err_title, &content);
                    },
                    CreateProjectError::Internal(e) => {
                        let content = format!("A system error ocurred while reading the path: {:?}", e);
                        nwg::modal_error_message(window, err_title, &content);
                    },
                }

                return;
            }
        };

        // Create new project
        if let Ok(mut state) = self.state_mut("create_new_project") {
            match state.create_new_project(new_project_path.clone()) {
                Ok(_) => {
                    self.enable_ui(true);
                    window.set_text(&format!("Native Windows WYSIWYG - {}", new_project_path));
                },
                Err(reason) => {
                    let content = format!("Impossible to create a new project at the selected location: {}", reason);
                    nwg::modal_error_message(window, err_title, &content);
                }
            }
        } 
    }

    fn open_project(&self) {

    }

    fn enable_ui(&self, enable: bool) {
        self.options_container.set_enabled(enable);
        self.widget_box.widgets_tree.set_enabled(enable);
    }

    fn show_demo(&self) {
        self.demo_window.set_visible(true);
    }

    fn fill_demo_background(&self, data: &nwg::EventData) {
        use winapi::um::winuser::FillRect;

        let paint = data.on_paint();
        let ps = paint.begin_paint();

        unsafe {
            let paint = self.paint_data.borrow();
            let p = paint.as_ref().unwrap();

            let hdc = ps.hdc;
            let rc = &ps.rcPaint;

            FillRect(hdc, rc, p.background as _);
        }

        paint.end_paint(&ps);
    }

    /**
        Gui helper that query and parse the output of the directory dialog
    */
    fn select_folder(&self, title: &str) -> Result<Option<String>, CreateProjectError>  {
        self.directory_dialog.set_title(title);
        if !self.directory_dialog.run(Some(&self.main_window)) {
            return Ok(None);
        }

        match self.directory_dialog.get_selected_item() {
            Ok(path) => match path.into_string() {
                Ok(path) => Ok(Some(path)),
                Err(path) => {
                    Err(CreateProjectError::BadPath(path))
                }
            },
            Err(err) => Err(CreateProjectError::Internal(err))
        }
    }

    /**
        Borrow the app state. In a NWG app, refcell double borrow may happen so it is not recommended to use `state.borrow_mut` directly (unless you want your app to randomly crash that is).

        This function tries to borrow the state and if it succeed, it returns a borrowed mutable reference. As an added precaution,
        this function also store the name of the last borrower so that if a double borrow happens, we can easily find the troublemaker.
    */
    fn state_mut(&self, borrower: &'static str) -> Result<RefMut<AppState>, ()> {
        match &self.state {
            Some(state) => match state.try_borrow_mut() {
                Ok(state) => {
                    *self.debug_borrow.borrow_mut() = Some(borrower);
                    Ok(state)
                },
                Err(_) => {
                    let borrower = self.debug_borrow.borrow().unwrap_or("No borrower set!");
                    let content = format!("Internal error! Application state is already borrowed by \"{}\". 
                    This is most likely the developer fault, trying again may fix the issue.\r\n
                    If you have 5 minutes to spare, please screenshot this message and open an issue of the githup repo.", borrower);
                    nwg::modal_error_message(&self.main_window, "State borrow error", &content);
                    Err(())
                }
            },
            None => unreachable!("State should be set to Some at GUI creation")
        }
    }

    /**
        See `Self::state_mut`
    */
    #[allow(unused)]
    fn state(&self, borrower: &'static str) -> Result<Ref<AppState>, ()> {
        match &self.state {
            Some(state) => match state.try_borrow() {
                Ok(state) => {
                    *self.debug_borrow.borrow_mut() = Some(borrower);
                    Ok(state)
                },
                Err(_) => {
                    let borrower = self.debug_borrow.borrow().unwrap_or("No borrower set!");
                    let content = format!("Internal error! Application state is already borrowed by \"{}\". 
                    This is most likely the developer fault, trying again may fix the issue.\r\n
                    If you have 5 minutes to spare, please screenshot this message and open an issue of the githup repo.", borrower);
                    nwg::modal_error_message(&self.main_window, "State borrow error", &content);
                    Err(())
                }
            },
            None => unreachable!("State should be set to Some at GUI creation")
        }
    }

}
