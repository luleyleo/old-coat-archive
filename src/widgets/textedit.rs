use crate::*;

pub struct TextEdit<'a> {
    content: &'a str,
    size: FontSize,
    font: Option<Font>,
}

impl<'a> Default for TextEdit<'a> {
    fn default() -> Self {
        Self {
            content: "",
            size: 12,
            font: None,
        }
    }
}

impl<'a> TextEdit<'a> {
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

pub struct TextEditState {
    pub content: String,
    pub size: FontSize,
    pub font: Option<Font>,
    pub layout: TextLayout,
}

pub enum TextEditEvent {
    Insertion(char),
    Deletion,
}

impl TextEditEvent {
    pub fn apply(self, target: &mut String) {
        use TextEditEvent::*;
        match self {
            Insertion(ch) => {
                target.push(ch);
            }
            Deletion => {
                target.pop();
            }
        }
    }
}

pub enum TextEditMsg {
    Insertion(char),
    Deletion,
}

impl<'a> Component for TextEdit<'a> {
    type State = TextEditState;
    type Msg = TextEditMsg;
    type Event = TextEditEvent;

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
            state.layout = ui.layout(props.content, props.font.as_ref(), props.size);
        }
    }

    fn update(msg: Self::Msg, _state: Mut<Self::State>, ui: &mut UiUpdate) {
        match msg {
            TextEditMsg::Insertion(ch) => {
                ui.emit(TextEditEvent::Insertion(ch));
            }
            TextEditMsg::Deletion => {
                ui.emit(TextEditEvent::Deletion);
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
