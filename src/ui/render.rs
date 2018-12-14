use crate::{UiData, Cid, Renderer, Bounds};

pub struct UiRender<'a> {
    data: &'a UiData,
    current: Cid,
}

impl<'a> UiRender<'a> {
    pub(crate) fn new(data: &'a UiData, root: Cid) -> Self {
        UiRender {
            data,
            current: root,
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        let current = self.current;

        {
            let pointer = self.data.pointer[self.current.get()];
            let state = self.data.state[self.current.get()].as_ref().unwrap();
            let position = self.data.position[self.current.get()];
            let size = self.data.size[self.current.get()];
            let bounds = Bounds::new(position, size);

            (pointer.render)(state, bounds, renderer);
        }

        let children = &self.data.children[self.current.get()];

        for child in children {
            self.current = *child;
            self.render(renderer);
        }
        self.current = current;
    }
}
