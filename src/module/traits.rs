pub trait Module {
    fn name(&self) -> &str;
    fn compatible_robot(&self) -> &str;
    fn activate(&self); 
}
