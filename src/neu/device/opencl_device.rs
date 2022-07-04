use crate::utils::StackBuffer;


#[derive(Copy, Clone)]
pub(crate) struct OpenCLDevice {
    pub (super) id: cl::cl_device_id
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

    fn get_global_memory(&self) -> usize {
        let mut length = 0usize;
        let mut global_memory : cl::cl_ulong = 0;
        unsafe {

            cl::clGetDeviceInfo(
                self.id, 
                cl::CL_DEVICE_GLOBAL_MEM_SIZE, 
                std::mem::size_of::<cl::cl_ulong>(), 
                (&mut global_memory as *mut cl::cl_ulong) as *mut std::ffi::c_void, 
                &mut length);
        }

        global_memory as usize
    }

    fn get_type(&self) -> super::DeviceType {
        let mut length = 0usize;
        let mut device_type : cl::cl_bitfield = 0;
        unsafe {
            cl::clGetDeviceInfo(
                self.id, 
                cl::CL_DEVICE_TYPE, 
                std::mem::size_of::<cl::cl_bitfield>(), 
                (&mut device_type as *mut cl::cl_bitfield) as *mut std::ffi::c_void, 
                &mut length);
        }

        super::DeviceType::from_opencl_flag(device_type)
    }

    fn get_clock_rate(&self) -> f32 {
        let mut length = 0usize;
        let mut clock_rate : cl::cl_uint = 0;

        unsafe {
            cl::clGetDeviceInfo(
                self.id, 
                cl::CL_DEVICE_TYPE, 
                std::mem::size_of::<cl::cl_uint>(), 
                (&mut clock_rate as *mut cl::cl_uint) as *mut std::ffi::c_void, 
                &mut length);
        }

        clock_rate as f32
    }

}