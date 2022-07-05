
pub(crate) struct OpenCLContext {
    pub(crate) handle: cl::cl_context
}

impl super::Context for OpenCLContext {
    fn free(&self) {
        unsafe {  cl::clReleaseContext(self.handle) };
    }
}