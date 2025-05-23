use super::Plugin;

pub struct MySQL {
    name: String,
    install_dir: String,
    status: String,
}

impl MySQL {
    pub fn new() -> MySQL {
        MySQL {
            name: "MySQL".to_string(),
            install_dir: "bin/mysql".to_string(),
            status: "off".to_string(),
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
