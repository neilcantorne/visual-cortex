mod device;
mod device_type;
mod opencl_device;
mod computing_api;

pub use device::Device;
pub use device_type::DeviceType;
pub use computing_api::ComputingApi;

use opencl_device::OpenCLDevice;