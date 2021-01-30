/*!
    Main application state.

    A good way to engineer large NWG app is to define the app data at the base level of the project and define the
    gui code in a different module. The app state is loaded here and the the ownedship is passed to the gui.

    As the owner of the state, the gui callback will trigger call the state methods. Optimally,
    no gui logic should be in the state code. See `gui::GuiBuilder::create_new_project` for a good
    example on how the gui should communicate with the state.
*/

mod gui;


extern crate native_windows_gui as nwg;
extern crate  native_windows_derive as nwd;
use std::{fs, process::exit};

/**
    Main application state
*/
pub struct AppState {
    current_project_path: Option<String>
}

impl AppState {

    pub fn init() -> AppState {
        AppState {
            current_project_path: None
        }
    }

    pub fn project_loaded(&self) -> bool {
        self.current_project_path.is_some()
    }

    /**
        Initialize a new rust project using cargo
        Parse the cargo projec into the application state

        On failure, return a message that should be displayed by the GUI app
    */
    pub fn create_new_project(&mut self, path: String) -> Result<(), String> {
        self.validate_new_project_path(&path)?;
        self.current_project_path = Some(path);
        Ok(())
    }


    /**
        Saves the current change in the project and clear it from the app state.
        Does nothing if there is no current project.

        Cannot fail.
    */
    pub fn close_project(&mut self) {
        if !self.project_loaded() {
            return;
        }

        self.current_project_path = None;
    }

    fn validate_new_project_path(&self, path: &str) -> Result<(), String> {
        // Folder must exits and be writable
        let meta = match fs::metadata(path) {
            Ok(meta) => meta,
            Err(e) => {
                let msg = format!("Project path does not exist or you lack the permissions to access it ({:?})", e);
                return Err(msg)
            }
        };

        // Folder must be a directory
        if !meta.is_dir() {
            return Err("Project path is not a directory".into());
        }

        // Folder must be writable
        if meta.permissions().readonly() {
            return Err("You do not have write access to the project path".into());
        }
        
        // Folder must be empty
        match fs::read_dir(path) {
            Ok(mut it) => if it.next().is_some() {
                return Err("Project path must be empty".into());
            },
            Err(e) => {
                let msg = format!("Project path must be empty, but the app failed to read its content: ({:?})", e);
                return Err(msg)
            }
        }

        Ok(())
    }

}


fn main() {
    if let Err(e) = nwg::init() {
        let msg = format!("An internal error made it impossible to start the program: {:?}", e);
        nwg::error_message("Failed to launch application", &msg);
        exit(1);
    }

    let state = AppState::init();

    let app = match gui::GuiBuilder::build(state) {
        Ok(app) => app,
        Err(e) => {
            let msg = format!("An internal error made it impossible to start the program: {:?}", e);
            nwg::error_message("Failed to launch application", &msg);
            exit(1);
        }
    };

    {
        app.create_new_project("F:\\projects\\tmp\\gui_test_project".to_owned());
    }
    

    nwg::dispatch_thread_events();

    app.destroy();
}
