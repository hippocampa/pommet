use super::Plugin;

pub struct MySQL {
    name: String,
    install_dir: String,
    status: String,
    is_installed: bool,
}

impl MySQL {
    pub fn new() -> MySQL {
        MySQL {
            name: "MySQL".to_string(),
            install_dir: "bin/mysql".to_string(),
            status: "off".to_string(),
            is_installed: false,
        }
    }
}

impl Plugin for MySQL {
    fn get_name(&self) -> &String {
        &self.name
    }    fn install(&mut self) {
        use crossterm::style::{Stylize, Color};
        
        println!("{}", "Installing MySQL database...".bold().with(Color::Magenta));
        println!("Installing to directory: {}", &self.install_dir.as_str().yellow());
        println!("{}", "Configuring MySQL...".italic().with(Color::Blue));
        println!("{}", "Setting up default databases...".with(Color::DarkCyan));
        println!("{}", "MySQL installation completed!".bold().green());
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
