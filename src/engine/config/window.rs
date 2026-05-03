use serde::Deserialize;
use winit::{
    dpi::PhysicalSize, 
    window::WindowButtons
};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,

    pub fullscreen: bool,
    pub active: bool,
    pub decorations: bool,

    pub close_button: bool,
    pub maximize_button: bool,
    pub minimize_button: bool,

    pub resizable: bool,
    pub window_icon_path: Option<String>,
    pub taskbar_icon_path: Option<String>,
}

impl WindowConfig {
    pub fn new() -> Self {
        Self {
            title: "Osmium".to_string(),
            width: 1024,
            height: 1024,
            fullscreen: false,
            active: true,
            decorations: true,
            close_button: true,
            maximize_button: true,
            minimize_button: true,
            resizable: true,
            window_icon_path: Some("./resources/osmium.ico".to_string()),
            taskbar_icon_path: Some("./resources/osmium_tb.ico".to_string())
        }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(self.width, self.height)
    }

    pub fn enabled_buttons(&self) -> WindowButtons {
        let mut buttons = WindowButtons::empty();

        if self.close_button {
            buttons |= WindowButtons::CLOSE;
        }

        if self.maximize_button {
            buttons |= WindowButtons::MAXIMIZE;
        }

        if self.minimize_button {
            buttons |= WindowButtons::MINIMIZE;
        }

        buttons
    }
}
