mod tensor;
mod tensor_value;

pub use tensor::Tensor;
pub type Tensor1d<T> = Tensor<1, T>;
pub type Tensor2d<T> = Tensor<2, T>;
pub type Tensor3d<T> = Tensor<3, T>;
pub type Tensor4d<T> = Tensor<4, T>;

pub use tensor_value::TensorValue;