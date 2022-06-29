pub trait TensorValue {
    const LENGTH: usize;
    const ZERO: Self;
}

impl TensorValue for f32 {
    const LENGTH: usize = 1;
    const ZERO: Self = 0f32;
}

impl TensorValue for (f32, f32) {
    const LENGTH: usize = 2;
    const ZERO: Self = (0f32, 0f32);
}

impl TensorValue for (f32, f32, f32) {
    const LENGTH: usize = 3;
    const ZERO: Self = (0f32, 0f32, 0f32);
}

impl TensorValue for (f32, f32, f32, f32) {
    const LENGTH: usize = 4;
    const ZERO: Self = (0f32, 0f32, 0f32, 0f32);
}