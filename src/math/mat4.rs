pub struct Mat4 {
    data: [i32; 16],
}

impl Mat4 {
    pub fn new() -> Mat4 {
        Mat4 { data: [0; 16] }
    }
    pub fn add(&self, other: &Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..16 {
            result.data[i] = self.data[i] + other.data[i];
        }
        result
    }

    pub fn sub(&self, other: &Mat4) -> Mat4 {
        let mut result = Mat4::new();
        for i in 0..16 {
            result.data[i] = self.data[i] - other.data[i];
        }
        result
    }
}
