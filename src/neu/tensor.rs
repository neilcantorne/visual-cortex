use std::ptr::NonNull;
use std::alloc::{alloc, Layout};

use crate::neu::TensorValue;

#[derive(Clone, Copy)]
pub struct Tensor<const D: usize, T>
    where T: TensorValue + Copy + Sized {
    sizes: [u16; D],
    buffer: NonNull<T>
}

impl<T> Tensor<1, T>
    where T: TensorValue + Copy + Sized {

    pub unsafe fn new(s1: u16) -> Result<Self, TensorError> {
        let ptr = {
            let layout = Layout::array::<T>(s1 as usize).unwrap();
            NonNull::new(alloc(layout) as *mut T)
        };

        if let Some(buffer) = ptr {
            return Ok(Self {
                sizes: [s1],
                buffer
            });
        }

        Err(TensorError{ kind: TensorErrorKind::MemoryAllocationFailed })
    }
}

impl<T> Tensor<2, T>
    where T: TensorValue + Copy + Sized {
    
    pub unsafe fn new(s1: u16, s2: u16) -> Result<Self, TensorError> {
        let ptr = {
            let size = 
            s1 as usize
            * s2 as usize;

            let layout = Layout::array::<T>(size).unwrap();

            NonNull::new(alloc(layout) as *mut T)
        };

        if let Some(buffer) = ptr {
            return Ok(Self {
                sizes: [s1, s2],
                buffer
            });
        }

        Err(TensorError{ kind: TensorErrorKind::MemoryAllocationFailed })
    }
}

impl<T> Tensor<3, T>
    where T: TensorValue + Copy + Sized {

    pub unsafe fn new(s1: u16, s2: u16, s3: u16) -> Result<Self, TensorError> {
        let ptr = {
            let size = 
                s1 as usize
                * s2 as usize
                * s3 as usize;

            let layout = Layout::array::<T>(size).unwrap();

            NonNull::new(alloc(layout) as *mut T)
        };

        if let Some(buffer) = ptr {
            return Ok(Self {
                sizes: [s1, s2, s3],
                buffer
            });
        }

        Err(TensorError{ kind: TensorErrorKind::MemoryAllocationFailed })
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