mod accelerator;
mod opencl_context;

pub use accelerator::Accelerator;
pub(crate) use opencl_context::OpenCLContext;
pub(crate) use accelerator::Context;