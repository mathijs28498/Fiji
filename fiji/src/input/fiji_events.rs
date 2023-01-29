use queues::{IsQueue, Queue};
use winit::event_loop::ControlFlow;

#[derive(Clone)]
enum FijiEvent {
    Exit,
}

pub struct FijiEventHandler {
    events: Queue<FijiEvent>,
    pub(crate) recreate_pipelines: bool,
}

impl FijiEventHandler {
    pub(crate) fn new() -> Self {
        Self {
            events: Queue::new(),
            recreate_pipelines: false
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
