use crate::utils::StackBuffer;


#[derive(Copy, Clone)]
pub(crate) struct OpenCLDevice {
    id: cl::cl_device_id
}

impl super::device::DeviceInternal for OpenCLDevice {

    
}