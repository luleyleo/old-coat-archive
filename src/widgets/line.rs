use crate::{
    iid, BoxConstraints, Cid, Component, Font, FontSize, Glyphs, Size, TextLayout, UiDerive,
    UiLayout, UiView,
};

pub struct Line<'a> {
    content: &'a str,
    size: FontSize,
    font: Option<Font>,
}

impl<'a> Default for Line<'a> {
    fn default() -> Self {
        Self {
            content: "",
            size: 12,
            font: None,
        }
    }
}

impl<'a> Line<'a> {
    pub fn content(mut self, content: &'a str) -> Self {
        self.content = content;
        self
    }

    pub fn size(mut self, size: FontSize) -> Self {
        self.size = size;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }
}

pub struct LineState {
    pub content: String,
    pub size: FontSize,
    pub font: Option<Font>,
    pub layout: TextLayout,
}

impl<'a> Component for Line<'a> {
    type State = LineState;
    type Msg = ();
    type Event = ();

    fn init(props: &Self) -> Self::State {
        LineState {
            content: String::default(),
            size: props.size,
            font: props.font.clone(),
            layout: TextLayout::default(),
        }
    }

    fn derive_state(props: &Self, state: &mut Self::State, ui: &UiDerive) {
        let mut changed = false;
        if props.content != state.content {
            state.content.replace_range(.., props.content);
            changed = true;
        }
        if props.size != state.size {
            state.size = props.size;
            changed = true;
        }
        if props.font != state.font {
            state.font = props.font.clone();
            changed = true;
        }
        if changed {
            ui.layout(props.content, props.font.as_ref(), props.size, &mut state.layout);
        }
    }

    fn layout(
        state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.len() != 1 {
            let name = ui.full_debug_name();
            log::error!(
                "The primitive Component {} has content attached to it but it will be ignored",
                name
            );
        }

        // TODO: Some sort of ellipsis or so if the constraints are to small
        let size = constraints.check_size(state.layout.size);
        ui.size(children[0], BoxConstraints::new_tight(size));

        size
    }

    fn view(_props: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        Glyphs::new()
            .size(state.size)
            .text(&state.layout)
            .set(iid!(), ui);
    }
}
