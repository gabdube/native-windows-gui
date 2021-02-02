/*!
    Main window gui components
*/
use nwd::NwgUi;
use nwg::{NativeUi, NwgError};
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};

use std::cell::{RefCell, RefMut, Ref};
use crate::AppState;
use super::{
    gui_error::*,
    widget_box::WidgetBox,
    project_settings_ui::ProjectSettingsUi,
    object_inspector::ObjectInspector
};

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
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::prepare_new_project, GuiBuilder::tasks] )]
    new_project_item: nwg::MenuItem,

    #[nwg_control(parent: file_menu)]
    sp1: nwg::MenuSeparator,

    #[nwg_control(parent: file_menu, text: "&Open project")]
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::prepare_open_project, GuiBuilder::tasks] )]
    open_project_item: nwg::MenuItem,

    #[nwg_control(parent: file_menu)]
    sp2: nwg::MenuSeparator,

    #[nwg_control(parent: file_menu, text: "&Close project")]
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::close_project, GuiBuilder::tasks] )]
    close_project_item: nwg::MenuItem,

    #[nwg_control(parent: file_menu)]
    sp3: nwg::MenuSeparator,

    #[nwg_control(parent: file_menu, text: "E&xit")]
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::close] )]
    exit_item: nwg::MenuItem,

    //
    // Window menu
    //
    #[nwg_control(parent: main_window, text: "&Window")]
    window_menu: nwg::Menu,

    #[nwg_control(parent: window_menu, text: "&Show demo window")]
    #[nwg_events( OnMenuItemSelected: [GuiBuilder::show_demo_window] )]
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

    #[nwg_partial(parent: project_settings_tab)]
    #[nwg_events(
        (on_settings_saved, OnCustomEvent): [GuiBuilder::save_project_settings],
        (on_settings_refresh, OnCustomEvent): [GuiBuilder::reload_project_settings],
    )]
    project_settings: ProjectSettingsUi,

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

    /// Extra ui initialization (after the native-windows-derive boilerplate)
    fn init(&self) {
        self.widget_box.init();
        self.project_settings.init();
        self.object_inspector.init();

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

        // Disable ui until a project is loaded
        self.enable_ui(false);

        // Execute waiting tasks
        self.tasks();
    }

    /// Close the app
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    /// Close the current project in the app
    fn close_project(&self) {
        if let Ok(mut state) = self.state_mut("close_project") {
            state.close_project();
        }
    }

    /// Update the UI based on the awaiting tasks in the application state
    fn tasks(&self) {
        use super::GuiTask::*;

        let tasks = match self.state_mut("tasks") {
            Ok(mut state) => {
                // Lets not borrow the app state for longer than necessary
                // Also, some task may require to borrow the app state again.
                let tasks = state.tasks().clone();
                state.tasks_mut().clear();
                tasks
            },
            Err(_) => {
                return;
            }
        };
        
        for task in tasks {
            match task {
                EnableUi(enable) => self.enable_ui(enable),
                UpdateWindowTitle(title) => self.main_window.set_text(&title),
                ReloadProjectSettings => self.reload_project_settings(),
                AskUserUpdateDependencies => self.ask_user_update_dependencies(),
                ClearData => {
                    self.project_settings.clear();
                }
            }
        }

        // Tasks may generate new tasks
        let new_tasks = self.state("tasks")
            .map(|state| state.tasks().len() > 0)
            .unwrap_or(false);
        
        if new_tasks {
            self.tasks();
        }
    }

    /// GUI checks before creating a new project
    fn prepare_new_project(&self) {
        // Check if project is already open
        if !self.swap_project("prepare_new_project") {
            return;
        }

        // Fetch project path
        let new_project_path = match self.select_folder("Create a new project") {
            Ok(Some(path)) => path,
            Ok(None) => { return; },
            Err(e) => {
                let err_title = "Failed to create new project";
                let err = format!("{}", e);
                nwg::modal_error_message(&self.main_window, err_title, &err);
                return;
            }
        };

        self.create_new_project(new_project_path);
    }

    /// Project creation & UI update
    /// Public for testing purpose
    fn create_new_project(&self, new_project_path: String) {
        let window = &self.main_window;
        let err_title = "Failed to create new project";

        if let Ok(mut state) = self.state_mut("create_new_project") {
            if let Err(reason) = state.create_new_project(new_project_path) {
                let content = format!("Impossible to create a new project at the selected location:\r\n\r\n{}", reason);
                nwg::modal_error_message(window, err_title, &content);
            }
        }
    }

    /// GUI checks before opening a new project
    fn prepare_open_project(&self) {
        // Check if project is already open
        if !self.swap_project("prepare_open_project") {
            return;
        }

        // Fetch project path
        let project_path = match self.select_folder("Open project") {
            Ok(Some(path)) => path,
            Ok(None) => { return; },
            Err(e) => {
                let err_title = "Failed to open project";
                let err = format!("{}", e);
                nwg::modal_error_message(&self.main_window, err_title, &err);
                return;
            }
        };
     
        self.open_project(project_path);
    }

    /// Project open & UI update
    fn open_project(&self, project_path: String) {
        let window = &self.main_window;
        let err_title = "Failed to open project";

        if let Ok(mut state) = self.state_mut("open_project") {
            if let Err(reason) = state.open_project(project_path) {
                let content = format!("Failed to open project at the selected location:\r\n\r\n{}", reason);
                nwg::modal_error_message(window, err_title, &content);
            }
        }
    }

    /**
        Saves the main project settings
    */
    pub fn save_project_settings(&self) {
    }

    /**
        Reload the project settings tab with the information from the project
    */
    fn reload_project_settings(&self) {
        if let Ok(state) = self.state("reload_project_settings") {
            if !state.project_loaded() {
                return;
            }

            let project = state.project().unwrap();
            self.project_settings.reload(project);
        }
    }

    /**
        If the deps of the project do not include nwg, ask the user if the app can add them
    */
    fn ask_user_update_dependencies(&self) {
        let title = "Project missing dependencies";

        let msg = nwg::MessageParams {
            title,
            content: "This project does not reference native-windows-gui in the dependencies. Do you want to app to add it for you?",
            buttons: nwg::MessageButtons::YesNo,
            icons: nwg::MessageIcons::Warning
        };

        if nwg::modal_message(&self.main_window, &msg) == nwg::MessageChoice::Yes {
            if let Ok(mut state) = self.state_mut("ask_user_update_dependencies") {
                if let Err(reason) = state.fix_dependencies() {
                    let content = format!("Failed to add dependencies to the current project:\r\n\r\n{}", reason);
                    nwg::modal_error_message(&self.main_window, title, &content);
                }
            }
        }
    }

    /// Enable/Disable ui
    fn enable_ui(&self, enable: bool) {
        self.project_settings.enable_ui(enable);
        self.object_inspector.enable_ui(enable);
        self.widget_box.widgets_tree.set_enabled(enable);
    }

    /// Duh, don't close the demo window
    fn show_demo_window(&self) {
        self.demo_window.set_visible(true);
    }

    /**
        Fill the demo window background with a single color.
        The render resources are initialized in `init`
    */
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
        Gui helper to close an existing project when a new project is loaded/created.

        Returns true if the user replaced the project or false otherwise.
    */
    fn swap_project(&self, borrower: &'static str) -> bool {
        let mut swap = true;
        if let Ok(mut state) = self.state_mut(borrower) {
            if state.project_loaded() {
                // Ask the user if he wishes to close the current project
                let msg = nwg::MessageParams {
                    title: "Project already loaded",
                    content: "A project is already open. Do you wish to close it?",
                    buttons: nwg::MessageButtons::YesNo,
                    icons: nwg::MessageIcons::Warning
                };

                match nwg::modal_message(&self.main_window, &msg) {
                    nwg::MessageChoice::Yes => {
                        state.close_project();
                    },
                    nwg::MessageChoice::No => {
                        swap = false;
                    },
                    _ => unreachable!()
                }
            }
        }

        swap
    }

    /**
        Borrow the app state. In a NWG app, refcell double borrow may happen so it is not recommended to use `state.borrow_mut` directly (unless you want your app to randomly crash that is).

        This function tries to borrow the state and if it succeed, it returns a borrowed mutable reference. As an added precaution,
        this function also store the name of the last borrower so that if a double borrow happens, we can easily find the troublemaker.
    */
    pub fn state_mut(&self, mew_borrower: &'static str) -> Result<RefMut<AppState>, ()> {
        match &self.state {
            Some(state) => match state.try_borrow_mut() {
                Ok(state) => {
                    *self.debug_borrow.borrow_mut() = Some(mew_borrower);
                    Ok(state)
                },
                Err(_) => {
                    let borrower = self.debug_borrow.borrow().unwrap_or("No borrower set!");
                    let content = format!(concat!(
                        "Internal error! {:?} is trying to borrow the application state is already borrowed by {:?}.\r\n\r\n",
                        "This is most likely my fault, trying again may fix the issue.\r\n\r\n",
                        "If you have 5 minutes to spare, please screenshot this message and open an issue of the githup repo."
                    ), mew_borrower, borrower);
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
    pub fn state(&self, new_borrower: &'static str) -> Result<Ref<AppState>, ()> {
        match &self.state {
            Some(state) => match state.try_borrow() {
                Ok(state) => {
                    *self.debug_borrow.borrow_mut() = Some(new_borrower);
                    Ok(state)
                },
                Err(_) => {
                    let borrower = self.debug_borrow.borrow().unwrap_or("No borrower set!");
                    let content = format!(concat!(
                        "Internal error! {:?} is trying to borrow the application state is already borrowed by {:?}.\r\n\r\n",
                        "This is most likely the developer fault, trying again may fix the issue.\r\n\r\n",
                        "If you have 5 minutes to spare, please screenshot this message and open an issue of the githup repo."
                    ), new_borrower, borrower);
                    
                    nwg::modal_error_message(&self.main_window, "State borrow error", &content);
                    Err(())
                }
            },
            None => unreachable!("State should be set to Some at GUI creation")
        }
    }

}
