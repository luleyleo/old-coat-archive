mod event;
pub use self::event::*;

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

    pub fn iter_new_events(&mut self) -> impl Iterator<Item = (&Event, &mut bool)> {
        self.events
            .iter_mut()
            .filter(|ev| ev.1 == false)
            .map(|ev| (&ev.0, &mut ev.1))
    }

    pub fn iter_all_events(&mut self) -> impl Iterator<Item = (&Event, bool)> {
        self.events
            .iter()
            .map(|ev| (&ev.0, ev.1))
    }
}
