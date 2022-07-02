#[derive(Copy, Clone)]
pub struct Device<'a> {
    internal: Box<dyn DeviceInternal + 'a>
}

impl<'a> Device<'a> {
    
}

pub(super) trait DeviceInternal {
    fn get_name(&self) -> String;
}
