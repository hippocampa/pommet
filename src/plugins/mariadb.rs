//https://mirror.citrahost.com/mariadb//mariadb-11.4.7/winx64-packages/mariadb-11.4.7-winx64.zip

use std::{
    error::Error, path::Path, process::{Child, Command}
};

use super::{Plugin, PluginStatus};
use crate::plugins::utils;

const MARIADB_CONFIG: &[u8] = include_bytes!("../config/my.ini");

pub struct Mariadb {
    name: String,
    download_url: String,
    child_process: Option<Child>,
    status: PluginStatus,
    is_installed: bool,
    install_dir: String,
}

impl Mariadb {
    pub fn new() -> Self {
        let mut is_installed = false;
        if Path::new("C:/pommet/bin/mariadb-11.4.7-winx64/bin/mysqld.exe").exists() {
            is_installed = true
        }
        Self {
            name: "MariaDB v11.4.7".to_string(),
            download_url: "https://mirror.citrahost.com/mariadb//mariadb-11.4.7/winx64-packages/mariadb-11.4.7-winx64.zip".to_string(),
            is_installed: is_installed,
            install_dir: "bin".to_string(),
            status:PluginStatus::Off,
            child_process:None,
    
        }
    }

    async fn download(&self) -> Result<(), Box<dyn std::error::Error>> {
        utils::download_plugin(&self.download_url, "mariadb.zip").await?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        match std::net::TcpStream::connect("127.0.0.1:3306") {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    pub fn wait_for_start(&self, max_attempts: u32) -> Result<(), Box<dyn Error>> {
        let mut attempts = 0;
        while attempts < max_attempts {
            if self.is_running() {
                return Ok(());
            }
            attempts += 1;
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Err(format!("MariaDB failed to start after {} attempts", max_attempts).into())
    }
    fn initialize_db(&self) -> Result<(), Box<dyn Error>> {
        let install_path = format!("C:/pommet/bin/mariadb-11.4.7-winx64");
        let data_dir = format!("{}/data", install_path);

        // Create the data directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)?;

        Command::new("cmd")
            .args(&[
                "/C",
                &format!(
                    "cd /D {} && bin\\mysqld --initialize-insecure --user=root --datadir={}",
                    install_path, data_dir
                )
            ])
            .status()?;
        Ok(())
    }
    }

impl Plugin for Mariadb {
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
        utils::unzip("mariadb.zip", &full_install_dir)?;

        println!("Intializing database");
        self.initialize_db()?;

        println!("Writing configuration for {}", self.name());
        utils::write_conf(
            &MARIADB_CONFIG,
            Path::new(&full_install_dir)
            .join( "mariadb-11.4.7-winx64")
            .join( "my.ini")
        )?;
        self.is_installed = true;
        println!("{} is installed!", self.name);
        Ok(())
    }
    fn is_installed(&self) -> bool {
        self.is_installed
    }
    fn toggle(&mut self) -> Result<(), Box<dyn Error>> {
        // First check if installed
        if !self.is_installed {
            return Err("MariaDB is not installed".into());
        }

        match self.status {
            PluginStatus::On => {
                if let Some(mut child) = self.child_process.take() {
                    let shutdown = Command::new("C:/pommet/bin/mariadb-11.4.7-winx64/bin/mysqladmin").args(&["-u", "root", "shutdown"]).status();
                    if shutdown.is_err(){
                        child.kill()?;
                    }
                    child.wait()?;
                    self.status = PluginStatus::Off;
                }
            }
            PluginStatus::Off => {
                let command_path = Path::new("C:/pommet/bin/mariadb-11.4.7-winx64/bin/mysqld.exe");
                
                if !command_path.exists() {
                    return Err(format!("MariaDB executable not found at: {}", command_path.display()).into());
                }
                
                let child = Command::new(command_path)
                .arg("--defaults-file=C:/pommet/bin/mariadb-11.4.7-winx64/my.ini")
                    .spawn()
                    .map_err(|e| format!("Failed to start MariaDB: {}", e))?;
                    
                self.child_process = Some(child);
                
                if let Err(e) = self.wait_for_start(10) {
                    if let Some(mut child) = self.child_process.take() {
                        let _ = child.kill();
                        let _ = child.wait();
                    }
                    return Err(e);
                }
                
                self.status = PluginStatus::On;
            }
        }
        Ok(())
    }
    fn status(&self) -> &PluginStatus {
        &self.status
    }
    fn is_toggleable(&self) -> bool {
        true
    }
}
