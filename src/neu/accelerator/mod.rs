mod opencl_context;

use std::rc::Rc;
pub(crate) use opencl_context::OpenCLContext;

pub struct Accelerator<'a> {
    context: Rc<dyn Context + 'a>
}

pub(super) trait Context {
    
}