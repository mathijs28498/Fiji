pub mod converter;
pub mod input_enums;

use std::collections::HashSet;

use winit::event::{ElementState, ModifiersState};

use nalgebra_glm::Vec2;

use self::{
    converter::{convert_mouse_button, convert_virtual_key_code},
    input_enums::{KeyCode, MouseButton},
};

#[derive(Clone)]
pub enum InteractionEvent {
    MouseEvent(ElementState, winit::event::MouseButton),
    MouseMovedEvent(Vec2),
    MouseDeltaEvent(Vec2),
    KeyEvent(ElementState, Option<winit::event::VirtualKeyCode>),
    ModifiersEvent(ModifiersState),
}

pub struct Input {
    key_pressed: HashSet<KeyCode>,
    key_held: HashSet<KeyCode>,
    key_released: HashSet<KeyCode>,
    mouse_button_pressed: HashSet<MouseButton>,
    mouse_button_held: HashSet<MouseButton>,
    mouse_button_released: HashSet<MouseButton>,
    mouse_position: Vec2,
    mouse_delta: Vec2,
    modifier_state: ModifiersState,
}

#[allow(dead_code)]
impl Input {
    pub(crate) fn new() -> Self {
        Input {
            key_pressed: HashSet::new(),
            key_held: HashSet::new(),
            key_released: HashSet::new(),
            mouse_button_pressed: HashSet::new(),
            mouse_button_held: HashSet::new(),
            mouse_button_released: HashSet::new(),
            mouse_position: Vec2::new(0., 0.),
            mouse_delta: Vec2::new(0., 0.),
            modifier_state: ModifiersState::empty(),
        }
    }

    pub fn key_pressed(&self, key: &KeyCode) -> bool {
        self.key_pressed.contains(key)
    }

    pub fn key_held(&self, key: &KeyCode) -> bool {
        self.key_held.contains(key)
    }

    pub fn key_released(&self, key: &KeyCode) -> bool {
        self.key_released.contains(key)
    }

    pub fn mouse_button_pressed(&self, button: &MouseButton) -> bool {
        self.mouse_button_pressed.contains(button)
    }

    pub fn mouse_button_held(&self, button: &MouseButton) -> bool {
        self.mouse_button_held.contains(button)
    }

    pub fn mouse_button_released(&self, button: &MouseButton) -> bool {
        self.mouse_button_released.contains(button)
    }

    pub fn mouse_position(&self) -> &Vec2 {
        &self.mouse_position
    }

    pub fn mouse_delta(&self) -> &Vec2 {
        &self.mouse_delta
    }

    pub fn modifier_state(&self) -> &ModifiersState {
        &self.modifier_state
    }

    pub(crate) fn reset_single_iteration_values(&mut self) {
        self.key_pressed.clear();
        self.key_released.clear();
        self.mouse_button_pressed.clear();
        self.mouse_button_released.clear();
        self.mouse_delta = Vec2::new(0., 0.);
    }

    pub(crate) fn handle_interaction_event(&mut self, event: InteractionEvent) {
        match event {
            InteractionEvent::MouseEvent(state, button) => match state {
                ElementState::Pressed => {
                    println!("{state:?}");
                    self.mouse_button_pressed
                        .insert(convert_mouse_button(button));
                    self.mouse_button_held.insert(convert_mouse_button(button));
                }
                ElementState::Released => {
                    self.mouse_button_released
                        .insert(convert_mouse_button(button));
                    self.mouse_button_held.remove(&convert_mouse_button(button));
                }
            },
            InteractionEvent::MouseMovedEvent(position) => {
                self.mouse_position = position;
            }
            InteractionEvent::MouseDeltaEvent(delta) => {
                self.mouse_delta = delta;
            }
            InteractionEvent::KeyEvent(state, key) => {
                if let Some(key) = key {
                    match state {
                        ElementState::Pressed => {
                            if self.key_held(&convert_virtual_key_code(key)) {
                                return;
                            }
                            
                            self.key_pressed.insert(convert_virtual_key_code(key));
                            self.key_held.insert(convert_virtual_key_code(key));
                        }
                        ElementState::Released => {
                            self.key_released.insert(convert_virtual_key_code(key));
                            self.key_held.remove(&convert_virtual_key_code(key));
                        }
                    }
                }
            }
            InteractionEvent::ModifiersEvent(state) => {
                self.modifier_state = state;
            }
        }
    }
}
