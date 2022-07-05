use std::rc::Rc;
use crate::neu;

pub struct Accelerator<'a> {
    context: Rc<dyn Context + 'a>
}

impl<'a> Accelerator<'a> {
    pub fn new(device: &neu::Device) -> neu::Result<Self> {
        Ok(Self {
            context: device.internal.create_context()?
        })
    }
}

impl<'a> Drop for Accelerator<'a> {
    fn drop(&mut self) {
        self.context.free();
    }
}

pub(crate) trait Context {
    fn free(&self);
}