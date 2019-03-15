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

    pub fn for_all_events(&mut self, mut handler: impl FnMut(bool, &Event) -> bool) {
        for (ref event, ref mut handled) in &mut self.events {
            if handler(*handled, event) {
                *handled = true;
            }
        }
    }

    pub fn for_new_events(&mut self, mut handler: impl FnMut(&Event) -> bool) {
        for (ref event, ref mut handled) in &mut self.events {
            if !(*handled) {
                if handler(event) {
                    *handled = true;
                }
            }
        }
    }
}
