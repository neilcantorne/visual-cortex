use std::iter::Map;


#[derive(Debug, Copy, Clone)]
pub(crate) struct StackBuffer<const SIZE: usize, T> 
    where T : Copy + Sized {
    buffer: [T; SIZE]
}

impl<const SIZE: usize, T> StackBuffer<SIZE, T>
    where T : Copy + Sized {
    
    pub fn new(init_value: T) -> Self {
        Self { buffer: [init_value; SIZE] }
    }

    pub fn get_count(&self) -> usize { self.buffer.len() }

    pub fn get_ptr(&mut self) -> *mut T {
        self.buffer.as_mut_ptr()
    }

    pub fn map<U>(&self, length: usize, f: fn(T) -> U) -> Map<std::iter::Take<std::array::IntoIter<T, SIZE>>, fn(T) -> U>  {
        self.buffer.into_iter().take(length).map(f)
    }
}

