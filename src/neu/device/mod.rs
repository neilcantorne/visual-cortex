use std::ffi::c_void;

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

    pub fn get_name(&self) -> String {
        let mut buffer = StackBuffer::<256, u8>::new(0u8);
        let mut length = 0usize;

        unsafe {
            cl::clGetDeviceInfo(self.device_id, cl::CL_DEVICE_NAME, buffer.get_count(), buffer.get_ptr() as *mut c_void, &mut length);
        }

        buffer.get_string(length)
    }

}