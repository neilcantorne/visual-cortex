use std::rc::Rc;

#[derive(Clone)]
pub struct Device<'a> {
    internal: Rc<dyn DeviceInternal + 'a>
}

impl<'a> Device<'a> {
    
}

pub(super) trait DeviceInternal {
    fn get_name(&self) -> String;
    fn get_vendor(&self) -> String;
    fn get_type(&self) -> super::DeviceType;
    fn get_global_memory(&self) -> usize;
    fn get_clock_rate(&self) -> f32;
}
