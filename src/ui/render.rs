use crate::{Bounds, Cid, Renderer, UiData, Position};

pub struct UiRender<'a> {
    data: &'a UiData,
    renderer: &'a mut Renderer,
}

impl<'a> UiRender<'a> {
    pub(crate) fn run(data: &'a UiData, renderer: &mut Renderer, root: Cid) {
        log::trace!("Running `UiRender`");
        let mut ui = UiRender { data, renderer };
        ui.render(root, Position::default());
    }

    pub fn render(&mut self, cid: Cid, offset: Position) {
        let pointer = self.data.pointer[cid.get()];
        let position = self.data.position[cid.get()] + offset;
        let size = self.data.size[cid.get()];
        let bounds = Bounds::new(position, size);
        log::trace!("Rendering {:?} at {:?} with {:?}", cid, position, size);

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
