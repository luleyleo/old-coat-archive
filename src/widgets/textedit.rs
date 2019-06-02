use crate::*;

pub struct TextEdit<'a> {
    buffer: Option<&'a Buffer>,
    size: FontSize,
    font: Option<Font>,
}

impl<'a> Default for TextEdit<'a> {
    fn default() -> Self {
        Self {
            buffer: None,
            size: 12,
            font: None,
        }
    }
}

impl<'a> TextEdit<'a> {
    pub fn buffer(mut self, buffer: &'a Buffer) -> Self {
        self.buffer = Some(buffer);
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

pub struct TextEditState {
    pub content: String,
    pub size: FontSize,
    pub font: Option<Font>,
    pub layout: TextLayout,
}

pub enum TextEditMsg {
    Insertion(char),
    Deletion,
}

impl<'a> Component for TextEdit<'a> {
    type State = TextEditState;
    type Msg = TextEditMsg;
    type Event = BufferUpdate;

    fn init(props: &Self) -> Self::State {
        TextEditState {
            content: String::default(),
            size: props.size,
            font: props.font.clone(),
            layout: TextLayout::default(),
        }
    }

    fn derive_state(props: &Self, state: &mut Self::State, ui: &mut UiDerive<Self>) {
        let mut changed = false;
        if let Some(buffer) = props.buffer {
            if buffer.text() != state.content {
                state.content.replace_range(.., buffer.text());
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
        } else {
            changed = !state.content.is_empty();
            state.content.clear();
        }
        if changed {
            state.layout = ui.layout(&state.content, props.font.as_ref(), props.size);
        }
    }

    fn update(msg: Self::Msg, _state: Mut<Self::State>, ui: &mut UiUpdate) {
        match msg {
            TextEditMsg::Insertion(ch) => {
                ui.emit(BufferUpdate::Insert(ch));
            }
            TextEditMsg::Deletion => {
                ui.emit(BufferUpdate::Delete(-1));
            }
        }
    }

    fn view(_props: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        use crate::widgets::TextInputAreaEvent::*;

        Glyphs::new()
            .size(state.size)
            .text(&state.layout)
            .set(iid!(), ui);

        TextInputArea::new()
            .set(iid!(), ui)
            .on_event(ui, |event| match event {
                Add(ch) => Some(TextEditMsg::Insertion(ch)),
                Backspace => Some(TextEditMsg::Deletion),
                Delete => Some(TextEditMsg::Deletion),
            });
    }

    fn layout(
        state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        if children.len() != 2 {
            let name = ui.full_debug_name();
            log::error!(
                "The primitive Component {} has content attached to it but it will be ignored",
                name
            );
        }

        // TODO: Some sort of ellipsis or so if the constraints are to small
        let size = constraints.check_size(state.layout.size);
        ui.size(children[0], BoxConstraints::new_tight(size));
        ui.size(children[1], BoxConstraints::new_tight(size));

        size
    }
}
