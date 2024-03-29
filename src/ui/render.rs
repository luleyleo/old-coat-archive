use crate::{Bounds, Cid, Position, Renderer, UiData};

pub struct UiRender<'a> {
    data: &'a UiData,
    renderer: &'a mut Renderer,
}

impl<'a> UiRender<'a> {
    pub(crate) fn run(data: &'a UiData, renderer: &mut Renderer, root: Cid) {
        log::trace!("Running `UiRender`");
        let mut ui = UiRender { data, renderer };
        ui.render(root, Position::zero());
    }

    pub fn render(&mut self, cid: Cid, offset: Position) {
        let pointer = self.data.pointer[cid.get()];
        let position = self.data.position[cid.get()] + offset.to_vector();
        let size = self.data.size[cid.get()];
        let bounds = Bounds::new(position, size);

        {
            let state = self.data.state[cid.get()].as_ref().unwrap();
            (pointer.render)(state, bounds, self.renderer);
        }

        let children = &self.data.children[cid.get()];

        for child in children {
            self.render(*child, position);
        }
    }
}
