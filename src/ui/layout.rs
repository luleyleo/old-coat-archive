use crate::{
    BoxConstraints, Cid, ComponentPointer, Font, FontSize, Position, Renderer, Size, UiData,
};
use std::any::Any;

pub struct UiLayout<'a> {
    name: &'a Vec<&'static str>,
    pointer: &'a Vec<ComponentPointer>,
    parent: &'a Vec<Option<Cid>>,
    children: &'a Vec<Vec<Cid>>,
    position: &'a mut Vec<Position>,
    size: &'a mut Vec<Size>,
    state: &'a Vec<Option<Box<Any>>>,
    renderer: &'a Renderer,
    current: Cid,
}

impl<'a> UiLayout<'a> {
    pub(crate) fn run(data: &'a mut UiData, renderer: &'a Renderer, root: Cid, window_size: Size) {
        log::trace!("Running `UiLayout`");

        let mut ui = UiLayout {
            name: &data.name,
            pointer: &data.pointer,
            parent: &data.parent,
            children: &data.children,
            position: &mut data.position,
            size: &mut data.size,
            state: &mut data.state,
            renderer: renderer,
            current: Cid::invalid(),
        };

        ui.size(root, BoxConstraints::new_tight(window_size));
    }

    pub fn size(&mut self, child: Cid, constraints: BoxConstraints) -> Size {
        let layout = self.pointer[child.get()].layout;

        let state = self.state[child.get()].as_ref().unwrap();
        let children = &self.children[child.get()];

        let previous = self.current;
        self.current = child;
        let proposed = layout(state, children, constraints, self);
        self.current = previous;

        let size = constraints.check_size(proposed);
        log::trace!(
            "Sized {} to {:?}",
            crate::full_debug_name_of(self.parent, self.name, child),
            size
        );

        self.size[child.get()] = size;

        size
    }

    pub fn get_size(&self, child: Cid) -> Size {
        self.size[child.get()]
    }

    pub fn position(&mut self, child: Cid, position: Position) {
        self.position[child.get()] = position;
    }

    pub fn full_debug_name(&self) -> String {
        crate::full_debug_name_of(self.parent, self.name, self.current)
    }

    pub fn get_text_size(&self, text: &str, font: Option<&Font>, size: FontSize) -> Size {
        let font = font.unwrap_or_else(|| self.renderer.font_manager.default_font());
        self.renderer.font_manager.dimensions(text, font, size)
    }
}
