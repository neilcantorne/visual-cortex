use crate::utils::StackBuffer;


#[derive(Debug, Copy, Clone)]
pub struct Device {
    device_id: cl::cl_device_id
}

impl Device {
    pub fn get_devices() -> Vec<Device> {
        let mut buffer = StackBuffer::<16, cl::cl_device_id>::new(std::ptr::null_mut());
        let mut length = 0u32;

        unsafe {
            cl::clGetDeviceIDs(std::ptr::null_mut(),
                cl::CL_DEVICE_TYPE_ALL,
                buffer.get_count() as u32,
                buffer.get_ptr(), &mut length);
        }

        buffer.map(length as usize, |device_id| {
            Device{ device_id }
        }).collect()
    }
}