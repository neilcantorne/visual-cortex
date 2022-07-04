
#[derive(Debug, Copy, Clone)]
pub enum DeviceType {
    Gpu,
    Cpu,
    Accelerator,
    Unknown
}

impl DeviceType {
    pub (super) fn from_opencl_flag(device_type: cl::cl_bitfield) -> Self {
        match device_type {
            cl::CL_DEVICE_TYPE_GPU => Self::Gpu,
            cl::CL_DEVICE_TYPE_CPU => Self::Cpu,
            cl::CL_DEVICE_TYPE_ACCELERATOR => Self::Accelerator,
            _ => Self::Unknown
        }
    }
}