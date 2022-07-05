
pub struct Error {
    flag: ErrorFlag
}

pub(super) enum ErrorFlag {
    OpenCL(cl::cl_int),
    NvidiaCUDA(cuda::cudaError_enum)
}

impl Error  {
    pub(super) fn new(flag: ErrorFlag) -> Self {
        Self { flag: flag }
    }
}