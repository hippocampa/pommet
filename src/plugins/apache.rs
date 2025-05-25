use std::path::Path;

use crate::plugins::utils::extract_bz2;

use super::Plugin;
use crossterm::style::{Color, Stylize};

const APACHE_EXECUTABLE: &[u8] = include_bytes!("../../assets/apache.tar.bz2");
pub struct Apache {
    name: String,
    install_dir: String,
    status: String,
    is_installed: bool,
}

impl Apache {
    pub fn new() -> Apache {
        Apache {
            name: "Apache".to_string(),
            install_dir: "bin/apache".to_string(),
            status: "on".to_string(),
            is_installed: false,
        }
    }
}

impl Plugin for Apache {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn install(&mut self) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        let install_dest = format!("c:/pommet/{}", self.install_dir);
        println!("{}", "Installing Apache server...".bold().with(Color::Cyan));
        extract_bz2(APACHE_EXECUTABLE, Path::new(&install_dest))?;
        println!(
            "Installing to directory: {}",
            &install_dest.as_str().yellow()
        );
        // TODO:
        // println!("{}", "Configuring Apache...".italic().with(Color::Blue));
        // println!("Setting up Apache modules...");
        println!("{}", "Apache installation completed!".bold().green());
        self.is_installed = true;
        Ok(())
    }

    fn is_installed(&self) -> bool {
        self.is_installed
    }

    fn toggle(&mut self) {
        if self.status == "off".to_string() {
            self.status = "on".to_string();
        } else {
            self.status = "off".to_string();
        }
    }

    fn status(&self) -> &String {
        &self.status
    }

    fn ref_array(&self) -> [&String; 2] {
        [&self.get_name(), &self.status]
    }
}
