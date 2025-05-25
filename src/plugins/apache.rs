use std::path::Path;

use super::Plugin;
use crate::plugins::utils;

const APACHE_CONFIG: &[u8] = include_bytes!("../config/httpd.conf");

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
            install_dir: "bin".to_string(),
        }
    }

    async fn download(&self) -> Result<(), Box<dyn std::error::Error>> {
        utils::download_plugin(&self.download_url, "apache.zip").await?;
        Ok(())
    }
}

impl Plugin for Apache {
    fn name(&self) -> &String {
        &self.name
    }
    fn install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let full_install_dir = format!("C:/pommet/{}", self.install_dir);
        // Create a runtime to run the async download function
        println!("Downloading {}", self.name());
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.download())?;

        println!("Extracting {} to {}", self.name(), full_install_dir);
        utils::unzip("apache.zip", &full_install_dir)?;

        println!("Installing {}", self.name());
        utils::write_conf(
            &APACHE_CONFIG,
            Path::new(&full_install_dir)
                .join("Apache24")
                .join("conf")
                .join("httpd.conf"),
        )?;
        self.is_installed = true;
        println!("{} is installed", self.name);
        Ok(())
    }
    fn is_installed(&self) -> bool {
        // if the install_dir exist, then return true
        self.is_installed
    }
    fn toggle(&mut self) {
        println!("Toggling");
        todo!();
    }
}
