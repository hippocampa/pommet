use super::Plugin;

pub struct Apache {
    name: String,
    install_dir: String,
    status: String,
}

impl Apache {
    pub fn new() -> Apache {
        Apache {
            name: "Apache".to_string(),
            install_dir: "bin/apache".to_string(),
            status: "on".to_string(),
        }
    }
}

impl Plugin for Apache {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn install(&self) {
        println!("apache todo...")
    }

    fn is_installed(&self) {
        println!("apache todo...")
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
