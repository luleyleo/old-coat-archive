mod event;
pub use self::event::*;

mod key_code;
pub use self::key_code::VirtualKeyCode;

pub struct Input {
    events: Vec<(Event, bool)>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Input { events: Vec::new() }
    }

    pub(crate) fn clear_events(&mut self) {
        self.events.clear();
    }

    pub(crate) fn push_event(&mut self, event: Event) {
        self.events.push((event, false));
    }

    /// Only iterates events that have not been handled yet.
    /// If the `&mut bool` gets set to `true` the event is considered handled.
    pub fn iter_fresh_events(&mut self) -> impl Iterator<Item = (&Event, &mut bool)> {
        self.events
            .iter_mut()
            .filter(|ev| ev.1 == false)
            .map(|ev| (&ev.0, &mut ev.1))
    }

    /// Only iterates events that have been handled already.
    pub fn iter_spoiled_events(&mut self) -> impl Iterator<Item = &Event> {
        self.events
            .iter_mut()
            .filter(|ev| ev.1 == true)
            .map(|ev| &ev.0)
    }

    pub fn iter_all_events(&mut self) -> impl Iterator<Item = (&Event, bool)> {
        self.events.iter().map(|ev| (&ev.0, ev.1))
    }
}
