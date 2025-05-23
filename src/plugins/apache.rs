use super::Plugin;

pub struct Apache {
    name: String,
    install_dir: String,
    is_on: bool,
}

impl Apache {
    pub fn init() -> Apache {
        Apache {
            name: "Apache".to_string(),
            install_dir: "bin/apache".to_string(),
            is_on: false,
        }
    }
}

impl Plugin for Apache {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn install(&self) {
        println!("todo...")
    }

    fn is_installed(&self) {
        println!("todo...")
    }

    fn start(&mut self) {
        if !self.is_on {
            println!("turning ON!");
            self.is_on = !self.is_on;
        }
    }

    fn stop(&mut self) {
        if self.is_on {
            println!("turning ON!");
            self.is_on = !self.is_on;
        }
    }
}
