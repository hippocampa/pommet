use super::Plugin;

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
    }    fn install(&mut self) {
        use crossterm::style::{Stylize, Color};
        
        println!("{}", "Installing Apache server...".bold().with(Color::Cyan));
        println!("Installing to directory: {}", &self.install_dir.as_str().yellow());
        println!("{}", "Configuring Apache...".italic().with(Color::Blue));
        println!("Setting up Apache modules...");
        println!("{}", "Apache installation completed!".bold().green());
        self.is_installed = true;
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
