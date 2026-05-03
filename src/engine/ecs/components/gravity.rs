pub struct Gravity {
    pub g: f32
}

impl Gravity {
    pub fn new() -> Self {
        Self {
            g: 9.8
        }
    }

    pub fn init(g: f32) -> Self {
        Self {
            g
        }
    }
}