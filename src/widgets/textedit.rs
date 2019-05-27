use crate::*;
use crate::widgets::text::{TextState, render_text};

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
    text: TextState,
}

// TODO: Handle the case where this is not beeing applied
/// IMPORTANT: This is *expected* to be applied!
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
            text: TextState {
                content: String::default(),
                size: props.size,
                font: props.font.clone(),
                layout: TextLayout::default(),
            },
        }
    }

    fn derive_state(props: &Self, state: &mut Self::State, ui: &mut UiDerive<Self>) {
        let mut changed = false;
        if props.content != state.text.content {
            state.text.content.replace_range(.., props.content);
            changed = true;
        }
        if props.size != state.text.size {
            state.text.size = props.size;
            changed = true;
        }
        if props.font != state.text.font {
            state.text.font = props.font.clone();
            changed = true;
        }
        if changed {
            state.text.layout = ui.layout(props.content, props.font.as_ref(), props.size);
        }
    }

    fn update(msg: Self::Msg, mut state: Mut<Self::State>, ui: &mut UiUpdate) {
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
        TextInputArea::new()
            .set(iid!(), ui)
            .on(ui, |event| match dbg!(event) {
                Add(ch) => Some(TextEditMsg::Insertion(ch)),
                Backspace => Some(TextEditMsg::Deletion),
                Delete => Some(TextEditMsg::Deletion),
            });
    }

    fn layout(state: &Self::State, children: &[Cid], _: BoxConstraints, ui: &mut UiLayout) -> Size {
        if children.len() != 1 {
            let name = ui.full_debug_name();
            log::error!(
                "The primitive Component {} has content attached to it but it will be ignored",
                name
            );
        }

        // TODO: Some sort of ellipsis or so if the constraints are to small
        let size = state.text.layout.size;
        ui.size(children[0], BoxConstraints::new_tight(size));

        size
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        render_text(&state.text, bounds, renderer);
    }
}
