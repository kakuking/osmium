pub struct Gravity {
    pub g: f32
}

impl Gravity {
    pub fn new() -> Self {
        Self {
            g: 9.8
        }
    }
}