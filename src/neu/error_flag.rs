
pub(crate) enum ErrorFlag {
    OpenCL(cl::cl_int),
    NvidiaCUDA(cuda::cudaError_enum)
}