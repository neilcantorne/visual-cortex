mod opencl_context;

use std::rc::Rc;
pub(crate) use opencl_context::OpenCLContext;

pub struct Accelerator<'a> {
    context: Rc<dyn Context + 'a>
}

impl<'a> Accelerator<'a> {
    pub fn new(device: &super::Device) -> super::Result<Self> {
        Ok(Self {
            context: device.internal.create_context()?
        })
    }
}

pub(super) trait Context {
    
}