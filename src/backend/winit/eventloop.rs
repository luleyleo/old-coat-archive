use winit::{ControlFlow, Event, EventsLoop, EventsLoopProxy};

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
    events_loop: EventsLoop,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
            events_loop: EventsLoop::new(),
        }
    }

    pub fn create_proxy(&self) -> EventsLoopProxy {
        self.events_loop.create_proxy()
    }

    pub fn events_loop(&self) -> &EventsLoop {
        &self.events_loop
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self) -> Vec<Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        self.events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            self.events_loop.run_forever(|event| {
                events.push(event);
                ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }
}
