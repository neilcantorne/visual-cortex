use std::ptr::NonNull;

use crate::neu::TensorValue;

pub struct Tensor<const D: usize, T>
    where T: TensorValue {
    sizes: [u16; D],
    buffer: NonNull<T>
}