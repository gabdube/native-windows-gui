/*!
    Application project structure.
*/

use std::{
    time::SystemTime,
    path::{Path, PathBuf},
};

use crate::parser::{parse, GuiStruct};


pub struct CargoToml {
    pub modified: SystemTime,
    pub content: toml::Value
}

pub struct Project {
    cargo_toml: CargoToml,
    path: String,
    gui_structs: Vec<GuiStruct>,
}

impl Project {

    pub fn new(path: String, cargo_toml: CargoToml) -> Project {
        Project {
            cargo_toml,
            path,
            gui_structs: Vec::new(),
        }
    }

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
            None => "Undefined".to_owned()
        }
    }

    /// Version of native-windows-gui
    /// `N/A` for single file project
    pub fn nwg_version(&self) -> String {
        let version = self.cargo_toml.content
            .as_table()
            .and_then(|t| t.get("dependencies"))
            .and_then(|v| v.as_table() )
            .and_then(|v| v.get("native-windows-gui"));

        if version.is_none() {
            return "N/A".to_owned();
        }

        let version = version.unwrap();
        if version.is_table() {
            let version = version.as_table().unwrap();
            version
                .get("version")
                .and_then(|v| v.as_str() )
                .unwrap_or("N/A")
                .to_owned()
        } else if version.is_str() {
            version.as_str().unwrap().to_owned()
        } else {
            "N/A".to_owned()
        }
    }

    /// Version of native-windows-derive
    /// `N/A` for single file project
    pub fn nwd_version(&self) -> String {
        let version = self.cargo_toml.content
            .as_table()
            .and_then(|t| t.get("dependencies"))
            .and_then(|v| v.as_table() )
            .and_then(|v| v.get("native-windows-derive"));

        if version.is_none() {
            return "N/A".to_owned();
        }

        let version = version.unwrap();
        if version.is_table() {
            let version = version.as_table().unwrap();
            version
                .get("version")
                .and_then(|v| v.as_str() )
                .unwrap_or("N/A")
                .to_owned()
        } else if version.is_str() {
            version.as_str().unwrap().to_owned()
        } else {
            "N/A".to_owned()
        }
    }

    /// Return the location of the resource file
    /// `N/A` for single file project
    pub fn resource_file(&self) -> String {
        if self.is_file_project() {
            return "N/A".to_owned();
        }

        "".to_owned()
    }

    /// Return the folder for the gui resources
    /// `N/A` for single file project
    pub fn resources_path(&self) -> String {
        if self.is_file_project() {
            return "N/A".to_owned();
        }

        "".to_owned()
    }

    /// Check if native-windows-gui & native-window-derive are in the dependencies table
    pub fn dependencies_ok(&self) -> bool {
        if self.is_file_project() {
            // File project do not have dependencies
            return true;
        }

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
    pub fn missing_dependencies(&self, nwg: &mut bool, nwd: &mut bool) -> Result<(), String> {
        let dep = self.cargo_toml.content
            .as_table()
            .and_then(|t| t.get("dependencies"))
            .and_then(|d| d.as_table());

        match dep {
            Some(dep) => {
                *nwg = dep.get("native-windows-gui").is_none();
                *nwd = dep.get("native-windows-derive").is_none();
                Ok(())
            },
            None => {
                Err("Failed to fetch dependencies. Does cargo.toml have a [dependencies] table?".to_owned())
            }
        }
    }

    /// Returns true if the project is a single file
    pub fn is_file_project(&self) -> bool {
        Path::new(&self.path)
            .extension()
            .map(|name| name == "rs")
            .unwrap_or(false)
    }

    /**
        Reload the project gui struct.

        If the project is a single file, just checks if the file changed on disk since the last load. If so, it reloads it.

        If we have a full project, iterates over all the rust source code file to find any new gui structs.
        Gui struct in files that haven't changed since the last load won't be touched, just like in a single file project.
    */
    pub fn reload_gui_struct(&mut self) -> Result<(), String> {
        if !self.is_file_project() {
            println!("TODO");
        } else {
            let gui_struct = match parse(&self.path) {
                Ok(Some(s)) => s,
                Ok(None) => { return Ok(()); },
                Err(e) => {
                    return Err(format!("Failed to parse {:?} for rust struct file: {:?}", self.path, e));
                }
            };

            self.gui_structs.push(gui_struct);
        }


        Ok(())
    }

    //
    // Other getter/setters
    //

    pub fn cargo_path(&self) -> PathBuf {
        let mut cargo_path = PathBuf::from(&self.path);
        cargo_path.push("Cargo.toml");
        cargo_path
    }

    pub fn cargo_toml(&self) -> &CargoToml {
        &self.cargo_toml
    }

    pub fn cargo_toml_mut(&mut self) -> &mut CargoToml {
        &mut self.cargo_toml
    }

    pub fn gui_structs(&self) -> &[GuiStruct] {
        &self.gui_structs
    }

}
