use std::{error::Error, path::Path};

use super::{Plugin, PluginStatus};
use crate::plugins::utils;

// TODO: php.ini
const PHP_CONFIG: &[u8] = include_bytes!("../config/php.ini");

pub struct PHP {
    name: String,
    download_url: String,
    status: PluginStatus,
    is_installed: bool,
    install_dir: String,
}

impl PHP {
    pub fn new() -> Self {
        let mut is_installed = false;
        if Path::new("C:/pommet/bin/php8/php.exe").exists() {
            is_installed = true
        }
        Self {
            name: "PHP v8.4.7".to_string(),
            download_url: "https://windows.php.net/downloads/releases/php-8.4.7-Win32-vs17-x64.zip"
                .to_string(),
            is_installed: is_installed,
            install_dir: "bin/php8".to_string(),
            status: match is_installed {
                true => PluginStatus::On,
                false => PluginStatus::Off,
            },
        }
    }

    async fn download(&self) -> Result<(), Box<dyn std::error::Error>> {
        utils::download_plugin(&self.download_url, "php.zip").await?;
        Ok(())
    }
}

impl Plugin for PHP {
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
        utils::unzip("php.zip", &full_install_dir)?;

        println!("Installing {}", self.name());
        utils::write_conf(&PHP_CONFIG, Path::new(&full_install_dir).join("php.ini"))?;
        self.is_installed = true;
        println!("{} is installed", self.name);
        Ok(())
    }
    fn is_installed(&self) -> bool {
        self.is_installed
    }
    fn toggle(&mut self) -> Result<(), Box<dyn Error>> {
        // First check if installed
        Ok(())
    }
    fn status(&self) -> &PluginStatus {
        &self.status
    }
}
