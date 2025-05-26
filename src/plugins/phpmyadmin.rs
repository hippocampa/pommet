use std::{
    error::Error,
    path::Path,
    process::{Child, Command},
};

use super::{Plugin, PluginStatus};
use crate::plugins::utils;

const PMA_CONFIG: &[u8] = include_bytes!("../config/config.inc.php");

pub struct PMA {
    name: String,
    download_url: String,
    status: PluginStatus,
    is_installed: bool,
    install_dir: String,
}

impl PMA {
    pub fn new() -> Self {
        let mut is_installed = false;
        if Path::new("C:/pommet/bin/Apache24/htdocs/phpMyAdmin-5.2.2-english").exists() {
            is_installed = true
        }
        Self {
            name: "PHPMyAdmin v5.2.2".to_string(),
            download_url:
                "https://files.phpmyadmin.net/phpMyAdmin/5.2.2/phpMyAdmin-5.2.2-english.zip"
                    .to_string(),
            is_installed: is_installed,
            install_dir: "bin/Apache24/htdocs".to_string(),
            status: PluginStatus::Off,
        }
    }

    async fn download(&self) -> Result<(), Box<dyn std::error::Error>> {
        utils::download_plugin(&self.download_url, "phpmyadmin.zip").await?;
        Ok(())
    }
}

impl Plugin for PMA {
    fn install(&mut self) -> Result<(), Box<dyn Error>> {
        let full_install_dir = format!("C:/pommet/{}", self.install_dir);
        println!("Downloading {}", self.name());
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.download())?;

        println!("Extracting {} to {}", self.name(), full_install_dir);
        utils::unzip("phpmyadmin.zip", &full_install_dir)?;

        println!("Installing {}", self.name());
        utils::write_conf(
            &PMA_CONFIG,
            Path::new(&full_install_dir)
                .join("phpMyAdmin-5.2.2-english")
                .join("config.inc.php"),
        )?;

        self.is_installed = true;
        println!("{} is installed", self.name);
        Ok(())
    }

    fn is_installed(&self) -> bool {
        self.is_installed
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn status(&self) -> &PluginStatus {
        &self.status
    }

    fn toggle(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
