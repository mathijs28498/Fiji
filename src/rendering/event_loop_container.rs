use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use nalgebra_glm as glm;

pub struct EventLoopContainer {
    pub(super) event_loop: EventLoop<()>,
}

impl EventLoopContainer {
    pub(super) fn new() -> EventLoopContainer {
        EventLoopContainer {
            event_loop: EventLoop::new(),
        }
    }

    pub fn run<F>(self, mut event_handler: F)
    where
        F: 'static + FnMut(),
    {
        self.event_loop.run(
            move |event, _, control_flow: &mut ControlFlow| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        println!("{state:?} - {button:?}")
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let _position = glm::Vec2::new(position.x as f32, position.y as f32);
                    }
                    WindowEvent::ModifiersChanged(state) => {
                        println!("{state:?}");
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        let state = input.state;
                        let key = input.virtual_keycode.unwrap();
                        println!("{state:?} - {key:?}")
                    }
                    _ => (),
                },
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    let _mouse_delta = glm::Vec2::new(delta.0 as f32, delta.1 as f32);
                }
                // Event::MouseInput {
                //     state,
                //     button,
                //     ..
                // } => {

                //     // let mut mouse_button_input_events = world
                //     //     .get_resource_mut::<Events<MouseButtonInput>>()
                //     //     .unwrap();
                //     // mouse_button_input_events.send(MouseButtonInput {
                //     //     button: converters::convert_mouse_button(button),
                //     //     state: converters::convert_element_state(state),
                //     });
                // },
                // Event::WindowEvent {
                Event::RedrawEventsCleared => {
                    event_handler();
                }
                _ => (),
            },
        );
    }
}
