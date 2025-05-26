use std::error::Error;

pub mod apache;
pub mod mariadb;
pub mod php;
pub mod phpmyadmin;
mod utils;

pub enum PluginStatus {
    On,
    Off,
}

pub trait Plugin {
    // maybe add installable?
    fn name(&self) -> &String;
    fn install(&mut self) -> Result<(), Box<dyn Error>>;
    fn is_installed(&self) -> bool;
    fn status(&self) -> &PluginStatus;
    fn toggle(&mut self) -> Result<(), Box<dyn Error>>;
}
