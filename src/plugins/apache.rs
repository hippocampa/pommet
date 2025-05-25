use super::Plugin;

// https://www.apachelounge.com/download/VS17/binaries/httpd-2.4.63-250207-win64-VS17.zip
pub struct Apache {
    name: String,
    download_url: String,
    // pid: int?
    // status: on/ofF
    is_installed: bool, // needs to be checked
    install_dir: String,
}

impl Apache {
    pub fn new() -> Self {
        Self {
            name: "Apache Server v2.4.63".to_string(),
            download_url: "https://www.apachelounge.com/download/VS17/binaries/httpd-2.4.63-250207-win64-VS17.zip".to_string(),
            is_installed: false,
            install_dir: "bin/apache".to_string(),
        }
    }
}

impl Plugin for Apache {
    fn name(&self) -> &String {
        &self.name
    }
    fn install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Installing {}", self.name());
        Ok(())
    }
    fn is_installed(&self) -> bool {
        self.is_installed
    }
    fn toggle(&mut self) {
        println!("Toggling");
        todo!();
    }
}
