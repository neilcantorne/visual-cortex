use crate::utils::StackBuffer;


#[derive(Copy, Clone)]
pub(crate) struct OpenCLDevice {
    id: cl::cl_device_id
}

impl super::device::DeviceInternal for OpenCLDevice {
    fn get_name(&self) -> String {
        let mut buffer = StackBuffer::<256, u8>::new(0u8);
        let mut length = 0usize;

        unsafe {
            cl::clGetDeviceInfo(
                self.id, 
                cl::CL_DEVICE_NAME, 
                buffer.get_count(), 
                buffer.get_ptr() as *mut std::ffi::c_void, 
                &mut length);
        }

        buffer.get_string(length)
    }

    fn get_vendor(&self) -> String {
        let mut buffer = StackBuffer::<256, u8>::new(0u8);
        let mut length = 0usize;

        unsafe {
            cl::clGetDeviceInfo(
                self.id, 
                cl::CL_DEVICE_VENDOR, 
                buffer.get_count(), 
                buffer.get_ptr() as *mut std::ffi::c_void, 
                &mut length);
        }

        buffer.get_string(length)
    }
}