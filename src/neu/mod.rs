mod tensor;
mod tensor_value;
mod device;
mod accelerator;
mod error;

pub use tensor::Tensor;
pub type Tensor1d<T> = Tensor<1, T>;
pub type Tensor2d<T> = Tensor<2, T>;
pub type Tensor3d<T> = Tensor<3, T>;
pub type Tensor4d<T> = Tensor<4, T>;

pub use tensor_value::TensorValue;
pub type Vec2f = (f32, f32);
pub type Vec3f = (f32, f32, f32);
pub type Vec4f = (f32, f32, f32, f32);

pub use device::Device;
pub use accelerator::Accelerator;

pub use error::Error;
use error::ErrorFlag;