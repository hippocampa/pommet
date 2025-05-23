use super::Plugin;

pub struct MySQL {
    name: String,
    install_dir: String,
    is_on: bool,
}

impl MySQL {
    pub fn new() -> MySQL {
        MySQL {
            name: "MySQL".to_string(),
            install_dir: "bin/mysql".to_string(),
            is_on: false,
        }
    }
}

impl Plugin for MySQL {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn install(&self) {
        println!("MYSQL is installing...")
    }

    fn is_installed(&self) {
        println!("MYSQL is installed")
    }

    fn start(&mut self) {
        // ! fix me: turning on is spawning a process
        if !self.is_on {
            println!("MYSQL is on");
            self.is_on = !self.is_on;
        }
    }

    fn stop(&mut self) {
        // ! fix me: turning off is killing a process
        if self.is_on {
            println!("MYSQL is off");
            self.is_on = !self.is_on;
        }
    }
}
