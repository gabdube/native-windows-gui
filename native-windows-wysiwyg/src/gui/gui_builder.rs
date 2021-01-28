/*!
    Main window gui components
*/
use nwd::NwgUi;
use nwg::{NativeUi, NwgError};
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};

use std::cell::{RefCell, RefMut, Ref};
use crate::AppState;
use super::{gui_error::*, widget_box::WidgetBox, object_inspector::ObjectInspector};


#[derive(Default, NwgUi)]
pub struct GuiBuilder {
    /// Application state
    state: Option<RefCell<AppState>>,

    /// Name of the gui process that currently borrow the state
    /// Used for debugging if the app gets borrowed twice
    debug_borrow: RefCell<Option<&'static str>>,

    #[nwg_control(size: (1200, 800), center: true, title: "Native Windows WYSIWYG", flags: "MAIN_WINDOW")]
    #[nwg_events( 
        OnInit: [GuiBuilder::init],
        OnWindowClose: [GuiBuilder::close],
    )]
    window: nwg::Window,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    layout: nwg::FlexboxLayout,

    //
    // Resources & extra
    //

    #[nwg_resource(action: nwg::FileDialogAction::OpenDirectory)]
    directory_dialog: nwg::FileDialog,

    //
    // File menu
    //
    #[nwg_control(text: "&File")]
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
    // Controls List
    //
    #[nwg_control(parent: window, flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, size: Size { width: Points(400.0) , height: Percent(1.0) } )]
    widget_box_frame: nwg::Frame,

    #[nwg_partial(parent: widget_box_frame)]
    widget_box: WidgetBox,

    //
    // Control inspector
    //
    #[nwg_control(parent: window, flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0) , height: Percent(1.0) } )]
    object_inspector_frame: nwg::Frame,

    #[nwg_partial(parent: object_inspector_frame)]
    object_inspector: ObjectInspector,
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
        self.window.set_visible(true);
    }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn create_new_project(&self) {
        let err_title = "Failed to create new project";
        let window = &self.window;

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
                    self.widget_box.set_enabled(true);
                    self.object_inspector.set_enabled(true);
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

    /**
        Gui helper that query and parse the output of the directory dialog
    */
    fn select_folder(&self, title: &str) -> Result<Option<String>, CreateProjectError>  {
        self.directory_dialog.set_title(title);
        if !self.directory_dialog.run(Some(&self.window)) {
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
                    nwg::modal_error_message(&self.window, "State borrow error", &content);
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
                    nwg::modal_error_message(&self.window, "State borrow error", &content);
                    Err(())
                }
            },
            None => unreachable!("State should be set to Some at GUI creation")
        }
    }

}
