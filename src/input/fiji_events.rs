use queues::{IsQueue, Queue};
use winit::event_loop::ControlFlow;

#[derive(Clone)]
enum FijiEvent {
    Exit,
}

// TODO:  Think of better name for this
pub struct FijiEventHandler {
    events: Queue<FijiEvent>,
}

impl FijiEventHandler {
    pub(crate) fn new() -> Self {
        Self {
            events: Queue::new(),
        }
    }

    pub(crate) fn handle_events(&mut self, control_flow: &mut ControlFlow) {
        while let Ok(event) = self.events.remove() {
            match event {
                FijiEvent::Exit => {
                    *control_flow = ControlFlow::Exit
                }                
            }
        }
    }

    pub fn exit(&mut self) {
        self.events.add(FijiEvent::Exit).unwrap();
    }
}
