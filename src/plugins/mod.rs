pub mod apache;
pub mod msql;

pub trait Plugin {
    fn get_name(&self) -> &String;
    fn is_installed(&self) -> bool;
    fn install(&mut self);
    fn toggle(&mut self);
    fn status(&self) -> &String;
    fn ref_array(&self) -> [&String; 2];
}
