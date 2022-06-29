use std::ptr::NonNull;
use std::alloc::{alloc, Layout};

use crate::neu::TensorValue;

#[derive(Clone, Copy)]
pub struct Tensor<const D: usize, T>
    where T: TensorValue + Copy + Sized {
    sizes: [u16; D],
    buffer: NonNull<T>
}

impl <const D: usize, T> Tensor<D, T> 
    where T: TensorValue + Copy + Sized {
    
    #[inline(always)]
    unsafe fn new_internal(count: usize, sizes: [u16; D]) -> Result<Self, TensorError> {
        let ptr = {
            let layout = Layout::array::<T>(count).unwrap();
            NonNull::new(alloc(layout) as *mut T)
        };

        if let Some(buffer) = ptr {
            return Ok(Self {
                sizes,
                buffer
            });
        }

        Err(TensorError{ kind: TensorErrorKind::MemoryAllocationFailed })
    }
}

impl<T> Tensor<1, T>
    where T: TensorValue + Copy + Sized {

    pub unsafe fn new(s1: u16) -> Result<Self, TensorError> {
        Self::new_internal(s1.into(), [s1])
    }
}

impl<T> Tensor<2, T>
    where T: TensorValue + Copy + Sized {
    
    pub unsafe fn new(s1: u16, s2: u16) -> Result<Self, TensorError> {
        let count = s1 as usize
        * s2 as usize;

        Self::new_internal(count, [s1, s2])
    }
}

impl<T> Tensor<3, T>
    where T: TensorValue + Copy + Sized {

    pub unsafe fn new(s1: u16, s2: u16, s3: u16) -> Result<Self, TensorError> {
        let count = s1 as usize
        * s2 as usize
        * s3 as usize;

        Self::new_internal(count, [s1, s2, s3])
    }
}

#[derive(Debug)]
pub struct TensorError {
    kind: TensorErrorKind
}

#[derive(Debug)]
enum TensorErrorKind {
    MemoryAllocationFailed
}