pub struct FirstPersonController {
    pub yaw: f32,
    pub pitch: f32,

    pub move_speed: f32,
    pub mouse_sensitivity: f32,

    pub pitch_limit: f32,
}

impl FirstPersonController {
    pub fn new() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            move_speed: 5.0,
            mouse_sensitivity: 0.002,
            pitch_limit: 89.0_f32.to_radians(),
        }
    }
}