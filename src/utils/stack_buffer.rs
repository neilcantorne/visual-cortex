
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
}

