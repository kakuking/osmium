use std::collections::HashSet;

use winit::event::{MouseButton, VirtualKeyCode};

#[derive(Debug, Clone)]
pub enum EngineEvent {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),

    MousePressed(MouseButton),
    MouseReleased(MouseButton),

    MouseMoved {
        x: f64,
        y: f64,
    },

    WindowResized {
        width: u32,
        height: u32,
    },
}

#[derive(Default)]
pub struct EventManager {
    events: Vec<EngineEvent>,

    keys_pressed: HashSet<VirtualKeyCode>,
    keys_down: HashSet<VirtualKeyCode>,
    keys_up: HashSet<VirtualKeyCode>,

    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_down: HashSet<MouseButton>,
    mouse_buttons_up: HashSet<MouseButton>,

    mouse_position: Option<(f64, f64)>,
}

impl EventManager {
    pub fn send(&mut self, event: EngineEvent) {
        match &event {
            EngineEvent::KeyPressed(key) => {
                if !self.keys_pressed.contains(key) {
                    self.keys_down.insert(*key);
                }
                self.keys_pressed.insert(*key);
            }

            EngineEvent::KeyReleased(key) => {
                self.keys_pressed.remove(key);
                self.keys_up.insert(*key);
            }

            EngineEvent::MousePressed(button) => {
                if !self.mouse_buttons_pressed.contains(button) {
                    self.mouse_buttons_down.insert(*button);
                }
                self.mouse_buttons_pressed.insert(*button);
            }

            EngineEvent::MouseReleased(button) => {
                self.mouse_buttons_pressed.remove(button);
                self.mouse_buttons_up.insert(*button);
            }

            EngineEvent::MouseMoved { x, y } => {
                self.mouse_position = Some((*x, *y));
            }

            EngineEvent::WindowResized { .. } => {}
        }

        self.events.push(event);
    }

    pub fn events(&self) -> &[EngineEvent] {
        &self.events
    }

    pub fn key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn key_up(&self, key: VirtualKeyCode) -> bool {
        self.keys_up.contains(&key)
    }

    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn mouse_up(&self, button: MouseButton) -> bool {
        self.mouse_buttons_up.contains(&button)
    }

    pub fn mouse_position(&self) -> Option<(f64, f64)> {
        self.mouse_position
    }

    pub fn clear_frame_events(&mut self) {
        self.events.clear();

        self.keys_down.clear();
        self.keys_up.clear();

        self.mouse_buttons_down.clear();
        self.mouse_buttons_up.clear();
    }
}