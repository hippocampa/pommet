use std::{
    error::Error, path::Path, process::{Child, Command}
};

use super::{Plugin, PluginStatus};
use crate::plugins::utils;

const APACHE_CONFIG: &[u8] = include_bytes!("../config/httpd.conf");

pub struct Apache {
    name: String,
    download_url: String,
    child_process: Option<Child>,
    status: PluginStatus,
    is_installed: bool, // needs to be checked
    install_dir: String,
}

impl Apache {
    pub fn new() -> Self {
        let mut is_installed = false;
        if Path::new("C:/pommet/bin/Apache24/bin/httpd.exe").exists() {
            is_installed = true
        }
        Self {
            name: "Apache Server v2.4.63".to_string(),
            download_url: "https://www.apachelounge.com/download/VS17/binaries/httpd-2.4.63-250207-win64-VS17.zip".to_string(),
            is_installed: is_installed,
            install_dir: "bin".to_string(),
            status:PluginStatus::Off,
            child_process:None,
    
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
        self.is_installed
    }
    fn toggle(&mut self) -> Result<(), Box<dyn Error>> {
        // First check if installed
        if !self.is_installed {
            return Err("Apache is not installed".into());
        }

        match self.status {
            PluginStatus::On => {
                if let Some(mut child) = self.child_process.take() {
                    child.kill()?;
                    child.wait()?;
                    self.status = PluginStatus::Off;
                }
            }
            PluginStatus::Off => {
                let command_path = Path::new("C:/pommet/bin/Apache24/bin/httpd.exe");
                
                if !command_path.exists() {
                    return Err(format!("Apache executable not found at: {}", command_path.display()).into());
                }
                
                let child = Command::new(&command_path)
                    .spawn()
                    .map_err(|e| format!("Failed to start Apache: {}", e))?;
                    
                self.child_process = Some(child);
                self.status = PluginStatus::On;
            }
        }
        Ok(())
    }
    fn status(&self) -> &PluginStatus {
        &self.status
    }
}
