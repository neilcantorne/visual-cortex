
#[derive(Debug, Copy, Clone)]
pub struct Device {
    device_id: cl::cl_device_id
}

impl Device {
    pub fn get_devices() -> Vec<Device> {
        
    }
}