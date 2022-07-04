use std::rc::Rc;

#[derive(Clone)]
pub struct Device<'a> {
    internal: Rc<dyn DeviceInternal + 'a>
}

impl<'a> Device<'a> {
    
}

pub(super) trait DeviceInternal {
    fn get_name(&self) -> String;
}
