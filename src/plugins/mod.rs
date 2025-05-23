pub mod apache;

pub trait Plugin {
    fn get_name(&self) -> &String;
    fn is_installed(&self);
    fn install(&self);
    fn start(&mut self);
    fn stop(&mut self);
}
