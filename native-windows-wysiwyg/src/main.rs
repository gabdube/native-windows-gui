/*!
    Main application state.

    A good way to engineer large NWG app is to define the app data at the base level of the project and define the
    gui code in a different module. The app state is loaded here and the the ownedship is passed to the gui.

    As the owner of the state, the gui callback will trigger call the state methods. Optimally,
    no gui logic should be in the state code. See `gui::GuiBuilder::create_new_project` for a good
    example on how the gui should communicate with the state.
*/

mod gui;
use gui::GuiTask;

extern crate native_windows_gui as nwg;
extern crate  native_windows_derive as nwd;
use std::{
    fs,
    path::PathBuf,
    time::SystemTime,
    process::{exit, Command}
};


struct CargoToml {
    modified: SystemTime,
    content: toml::Value
}

pub struct Project {
    cargo_toml: CargoToml,
    path: String,
}

impl Project {

    /// Name of the cargo project
    pub fn name(&self) -> String {
        let name = self.cargo_toml.content
            .as_table()
            .and_then(|t| t.get("package"))
            .and_then(|v| v.as_table() )
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .map(|name| name.to_owned());

        match name {
            Some(n) => n,
            None => {
                println!("Failed to read project name from toml file: {:#?}", self.cargo_toml.content);
                "Failed to read project name".to_owned()
            }
        }
    }

    /// Check if native-windows-gui & native-window-derive are in the dependencies table
    pub fn dependencies_ok(&self) -> bool {
        let dep = self.cargo_toml.content
            .as_table()
            .and_then(|t| t.get("dependencies"))
            .and_then(|d| d.as_table());

        match dep {
            Some(dep) => {
                dep.get("native-windows-gui").is_some() &&
                dep.get("native-windows-derive").is_some()
            },
            None => {
                false
            }
        }
    }

    /// Check the missing dependencies
    /// Sets a value to `true` if the dependency is missing
    pub fn missing_dependencies(&self, nwg: &mut bool, nwd: &mut bool) {
        let dep = self.cargo_toml.content
            .as_table()
            .and_then(|t| t.get("dependencies"))
            .and_then(|d| d.as_table());

        match dep {
            Some(dep) => {
                *nwg = dep.get("native-windows-gui").is_none();
                *nwd = dep.get("native-windows-derive").is_none();
            },
            None => {
                println!("WARNING! Failed to fetch missing dependencies");
            }
        }
    }

    pub fn cargo_path(&self) -> PathBuf {
        let mut cargo_path = PathBuf::from(&self.path);
        cargo_path.push("Cargo.toml");
        cargo_path
    }

}

/**
    Main application state
*/
pub struct AppState {
    /// Current project data
    project: Option<Project>,

    /// List of tasks the GUI should do after the app state was updated
    gui_tasks: Vec<GuiTask>,
}

impl AppState {

    pub fn init() -> AppState {
        AppState {
            project: None,
            gui_tasks: Vec::new(),
        }
    }

    pub fn project_loaded(&self) -> bool {
        self.project.is_some()
    }

    pub fn project(&self) -> Option<&Project> {
        self.project.as_ref()
    }

    pub fn project_mut(&mut self) -> Option<&mut Project> {
        self.project.as_mut()
    }

    pub fn tasks(&mut self) -> &mut Vec<GuiTask> {
        &mut self.gui_tasks
    }

    /**
        Initialize a new rust project using cargo

        On failure, return a message that should be displayed by the GUI app.
    */
    pub fn create_new_project(&mut self, path: String) -> Result<(), String> {
        self.validate_new_project_path(&path)?;
        self.cargo_init(&path)?;

        let cargo_toml = self.read_cargo_toml(&path)?;
        self.init_project(path.clone(), cargo_toml);

        self.gui_tasks.push(GuiTask::EnableUi(true));
        self.gui_tasks.push(GuiTask::UpdateWindowTitle(format!("Native Windows WYSIWYG - {}", path)));
        self.gui_tasks.push(GuiTask::ReloadProjectSettings);

        Ok(())
    }

    /**
        Open an existing rust project

        On failure, return a message that should be displayed by the GUI app.
    */
    pub fn open_project(&mut self, path: String) -> Result<(), String> {
        let cargo_toml = self.read_cargo_toml(&path)?;
        self.init_project(path.clone(), cargo_toml);

        self.gui_tasks.push(GuiTask::EnableUi(true));
        self.gui_tasks.push(GuiTask::UpdateWindowTitle(format!("Native Windows WYSIWYG - {}", path)));
        self.gui_tasks.push(GuiTask::ReloadProjectSettings);

        // Check if the dependencies are OK
        let project = self.project().unwrap();
        if !project.dependencies_ok() {
            self.gui_tasks.push(GuiTask::AskUserUpdateDependencies);
        }

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

        self.project = None;

        self.gui_tasks.push(GuiTask::EnableUi(false));
        self.gui_tasks.push(GuiTask::UpdateWindowTitle("Native Windows WYSIWYG".to_owned()));
    }

    /**
        Add `native-windows-gui` && `native-windows-derive` to the dependency of an already existing project

        On failure, return a message that should be displayed by the GUI app.
    */
    pub fn fix_dependencies(&mut self) -> Result<(), String> {
        use std::io::prelude::Write;

        if !self.project_loaded() {
            println!("fix_dependencies called without an active project");
            return Ok(());
        }

        let project = self.project_mut().unwrap();

        // Check missing
        let (mut nwg, mut nwd) = (false, false);
        project.missing_dependencies(&mut nwg, &mut nwd);
        if !nwg && !nwd {
            return Ok(());
        }

        // Read content
        let cargo_path = project.cargo_path();
        let cargo_str = fs::read_to_string(&cargo_path)
            .map_err(|e| format!("Failed to read Cargo.toml: {:?}", e) )?;
        
        // Dep index
        let dep_index: usize = {
            let dep_str = "[dependencies]";
            let mut i = cargo_str.match_indices(dep_str);
            
            match i.next().map(|(index, _)| index + dep_str.len()) {
                Some(i) => i,
                None => {
                    return Err(format!("Cannot find \"[dependencies]\" in Cargo.toml"));
                }
            }
        };

        // Write dependencies
        let (first, last) = cargo_str.split_at(dep_index);
        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(&cargo_path)
            .map_err(|e| format!("Failed to open `Cargo.toml`:\r\n\r\n{:#?}", e) )?;
        
        file.write_all(first.as_bytes())
            .and_then(|_| file.write_all("\nnative-windows-gui = \"~1.0\"\n".as_bytes()))
            .and_then(|_| file.write_all("native-windows-derive = \"~1.0\"\n".as_bytes()))
            .and_then(|_| file.write_all(last.as_bytes()))
            .map_err(|e| format!("Failed to write dependencies to `Cargo.toml`:\r\n\r\n{:#?}", e) )?;

        // Reload Cargo.toml
        self.reload_cargo()?;

        Ok(())
    }

    fn init_project(&mut self, path: String, cargo_toml: CargoToml) {
        let project = Project {
            cargo_toml,
            path,
        };

        self.project = Some(project);
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

    fn cargo_init(&self, path: &str) -> Result<(), String> {
        use std::io::prelude::Write;

        // `cargo init --bin`
        let cargo_output = Command::new("cargo")
            .arg("init")
            .arg("--bin")
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to run `cargo init`: {:?}", e) )?;

        if !cargo_output.status.success() {
            let msg = match cargo_output.status.code() {
                Some(code) => format!("`cargo init` terminated with exit code {}", code),
                None => format!("`cargo init` process terminated by signal")
            };
            return Err(msg);
        }

        // Add native-windows-gui dependencies
        let mut cargo_path = PathBuf::from(path);
        cargo_path.push("Cargo.toml");

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&cargo_path)
            .map_err(|e| format!("Failed to read `Cargo.toml`:\r\n\r\n{:#?}", e) )?;

        let dep = concat!(
            "native-windows-gui = \"~1.0\"\n",
            "native-windows-derive = \"~1.0\"\n",
        );

        file.write_all(dep.as_bytes())
            .map_err(|e| format!("Failed to write dependencies to `Cargo.toml`:\r\n\r\n{:#?}", e) )?;

        Ok(())
    }

    fn read_cargo_toml(&self, path: &str) -> Result<CargoToml, String> {
        let mut cargo_path = PathBuf::from(path);
        cargo_path.push("Cargo.toml");

        let meta = fs::metadata(path)
            .map_err(|e| format!("Failed to read `Cargo.toml`:\r\n\r\n{:#?}", e) )?;

        let cargo_str = fs::read_to_string(&cargo_path)
            .map_err(|e| format!("Failed to read `Cargo.toml`:\r\n\r\n{:#?}", e))?;

        let content: toml::Value = toml::from_str(&cargo_str)
            .map_err(|e| format!("Failed to parse `Cargo.toml`:\r\n\r\n{:#?}", e))?;

        let toml = CargoToml {
            modified: meta.modified().unwrap_or(SystemTime::now()),
            content,
        };


        Ok(toml)
    }

    /// Reload the cargo file if the file was modified
    fn reload_cargo(&mut self) -> Result<(), String> {
        let project = self.project_mut().unwrap();
        let cargo_path = project.cargo_path();

        let meta = fs::metadata(&cargo_path)
            .map_err(|e| format!("Failed to read `Cargo.toml`:\r\n\r\n{:#?}", e) )?;

        let modified = meta.modified().unwrap_or(SystemTime::now());
        if modified == project.cargo_toml.modified {
            return Ok(());
        }

        let cargo_str = fs::read_to_string(&cargo_path)
            .map_err(|e| format!("Failed to read `Cargo.toml`:\r\n\r\n{:#?}", e))?;

        let content: toml::Value = toml::from_str(&cargo_str)
            .map_err(|e| format!("Failed to parse `Cargo.toml`:\r\n\r\n{:#?}", e))?;

        project.cargo_toml = CargoToml {
            modified,
            content,
        };

        Ok(())
    }

}


fn main() {
    if let Err(e) = nwg::init() {
        let msg = format!("An internal error made it impossible to start the program:\r\n\r\n{:?}", e);
        nwg::error_message("Failed to launch application", &msg);
        exit(1);
    }

    let state = AppState::init();

    let app = match gui::GuiBuilder::build(state) {
        Ok(app) => app,
        Err(e) => {
            let msg = format!("An internal error made it impossible to start the program:\r\n\r\n{:?}", e);
            nwg::error_message("Failed to launch application", &msg);
            exit(1);
        }
    };

    {
        //let mut state = app.state_mut("main").unwrap();
        //state.open_project("F:\\projects\\tmp\\gui_test_project".to_owned()).unwrap();
    }
    
    nwg::dispatch_thread_events();

    app.destroy();
}
