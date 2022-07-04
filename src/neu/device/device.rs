use std::rc::Rc;

use crate::utils::StackBuffer;

#[derive(Clone)]
pub struct Device<'a> {
    internal: Rc<dyn DeviceInternal + 'a>
}

impl<'a> Device<'a> {
    pub fn get_devices() -> Vec<Self> {
        let devices : Vec<Self>;

        { // Get all devices with OpenCL
            let mut buffer = StackBuffer::<16, cl::cl_device_id>::new(std::ptr::null_mut());
            let mut length = 0u32;

            unsafe {
                cl::clGetDeviceIDs(std::ptr::null_mut(),
                    cl::CL_DEVICE_TYPE_ALL,
                    buffer.get_count() as u32,
                    buffer.get_ptr(), &mut length);
            }

            devices = buffer.map(length as usize, 
                |device_id| Self {
                    internal: Rc::new(super::OpenCLDevice {
                        id: device_id
                    })
                }
            ).collect()
        }

        { // TODO: implement get devices for Nvidia CUDA

        }

        return devices;
    }

    #[inline(always)]
    pub fn get_name(&self) -> String { self.internal.get_name() }

    #[inline(always)]
    pub fn get_vendor(&self) -> String { self.internal.get_vendor() }

    #[inline(always)]
    pub fn get_type(&self) -> super::DeviceType { self.internal.get_type() }

    #[inline(always)]
    pub fn get_global_memory(&self) -> usize { self.internal.get_global_memory() }

    #[inline(always)]
    pub fn get_clock_rate(&self) -> f32 { self.internal.get_clock_rate() }

}

pub(super) trait DeviceInternal {
    fn get_name(&self) -> String;
    fn get_vendor(&self) -> String;
    fn get_type(&self) -> super::DeviceType;
    fn get_global_memory(&self) -> usize;
    fn get_clock_rate(&self) -> f32;
}
