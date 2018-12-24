use crate::{Bounds, Cid, Renderer, UiData};
use log::trace;

pub struct UiRender<'a> {
    data: &'a UiData,
    renderer: &'a mut Renderer,
}

impl<'a> UiRender<'a> {
    pub(crate) fn run(data: &'a UiData, renderer: &mut Renderer, root: Cid) {
        trace!("Running `UiRender`");
        let mut ui = UiRender { data, renderer };
        ui.render(root);
    }

    pub fn render(&mut self, cid: Cid) {
        {
            let pointer = self.data.pointer[cid.get()];
            let state = self.data.state[cid.get()].as_ref().unwrap();
            let position = self.data.position[cid.get()];
            let size = self.data.size[cid.get()];
            let bounds = Bounds::new(position, size);

            (pointer.render)(state, bounds, self.renderer);
        }

        let children = &self.data.children[cid.get()];

        for child in children {
            self.render(*child);
        }
    }
}
