use queues::{IsQueue, Queue};
use winit::{
    event::{DeviceEvent, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use nalgebra_glm::Vec2;

use crate::{input::{Input, InteractionEvent, fiji_events::FijiEventHandler}, public::context::Context};

pub struct EventLoopContainer {
    pub(super) event_loop: EventLoop<()>,
    events: Queue<InteractionEvent>,
    input: Input,
    fiji_event_handler: FijiEventHandler,
    
}

impl EventLoopContainer {
    pub(super) fn new() -> EventLoopContainer {
        EventLoopContainer {
            event_loop: EventLoop::new(),
            events: Queue::new(),
            input: Input::new(),
            fiji_event_handler: FijiEventHandler::new(),
        }
    }

    pub fn run<F>(mut self, mut event_handler: F)
    where
        F: 'static + FnMut(&Input, &mut FijiEventHandler),
    {
        self.event_loop
            .run(move |event, _, control_flow: &mut ControlFlow| {
                *control_flow = ControlFlow::Poll;
                
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::Resized(_) => {
                            self.fiji_event_handler.recreate_pipelines = true;
                        }
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            self.events
                                .add(InteractionEvent::MouseEvent(state, button))
                                .unwrap();
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let position = Vec2::new(position.x as f32, position.y as f32);
                            self.events
                                .add(InteractionEvent::MouseMovedEvent(position))
                                .unwrap();
                        }
                        WindowEvent::ModifiersChanged(state) => {
                            self.events
                                .add(InteractionEvent::ModifiersEvent(state))
                                .unwrap();
                        }
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state,
                                    virtual_keycode,
                                    ..
                                },
                            ..
                        } => {
                            self.events
                                .add(InteractionEvent::KeyEvent(state, virtual_keycode))
                                .unwrap();
                        }
                        _ => (),
                    },
                    Event::DeviceEvent {
                        event: DeviceEvent::MouseMotion { delta },
                        ..
                    } => {
                        let mouse_delta = Vec2::new(delta.0 as f32, delta.1 as f32);
                        self.events
                            .add(InteractionEvent::MouseDeltaEvent(mouse_delta))
                            .unwrap();
                    }
                    Event::RedrawEventsCleared => {
                        self.input.reset_single_iteration_values();
                        while let Ok(event) = self.events.remove() {
                            self.input.handle_interaction_event(event);
                        }

                        event_handler(&self.input, &mut self.fiji_event_handler);

                        self.fiji_event_handler.handle_events(control_flow);
                    }
                    _ => (),
                };
            });
    }
}
