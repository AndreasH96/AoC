#[macro_use]
extern crate derive_new;

#[derive(Debug, Clone, new,Hash)]
pub struct Valve {
    pub name:String,
    pub flow_rate:i32,
    pub edges: Vec<String>,
    pub open:bool,
}
impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Valve {}


impl Valve {
    pub fn open(&mut self){
        self.open=true;
    }
}