use std::error::Error;

pub mod apache;
mod utils;

pub enum PluginStatus {
    On,
    Off,
}

pub trait Plugin {
    fn name(&self) -> &String;
    fn install(&mut self) -> Result<(), Box<dyn Error>>;
    fn is_installed(&self) -> bool;
    fn status(&self) -> &PluginStatus;
    fn toggle(&mut self) -> Result<(), Box<dyn Error>>;
}
