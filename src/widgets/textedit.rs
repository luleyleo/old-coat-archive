use crate::*;

pub struct TextEdit<'a> {
    buffer: Option<&'a Buffer>,
    size: FontSize,
    font: Option<Font>,
    cursor: CursorStyle,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CursorStyle {
    width: Scalar,
    color: Color,
}

impl<'a> Default for TextEdit<'a> {
    fn default() -> Self {
        Self {
            buffer: None,
            size: 12,
            font: None,
            cursor: CursorStyle {
                width: 2.0,
                color: Color::rgb(0.1, 0.1, 0.1),
            },
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

    pub fn style(mut self, style: CursorStyle) -> Self {
        self.cursor = style;
        self
    }
}

pub struct TextEditState {
    pub content: String,
    pub size: FontSize,
    pub font: Option<Font>,
    pub layout: TextLayout,
    pub cursor: CursorStyle,
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
            cursor: props.cursor,
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
        if props.cursor != state.cursor {
            state.cursor = props.cursor;
            changed = true;
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

    fn view(props: &Self, state: &Self::State, ui: &mut UiView<Self>) {
        use crate::widgets::TextInputAreaEvent::*;

        Glyphs::new()
            .size(state.size)
            .text(&state.layout)
            .set(iid!(), ui);

        let x_offset = state.layout.size.width;
        Offset::new().x(x_offset + props.cursor.width).set(iid!(), ui).add(|| {
            let height = state.layout.size.height;
            Constrained::new()
                .max_width(props.cursor.width)
                .max_height(height)
                .set(iid!(), ui)
                .add(|| {
                    Rectangle::new()
                        .color(props.cursor.color)
                        .set(iid!(Cursor), ui);
                });
        });

        TextInputArea::new()
            .set(iid!(), ui)
            .map_events(ui, |event| match event {
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
        const CHILD_COUNT: usize = 3;
        if children.len() != CHILD_COUNT {
            let name = ui.full_debug_name();
            log::error!(
                "The primitive Component {} has content attached to it but it will be ignored",
                name
            );
        }

        let constraints = constraints.min_width(constraints.min_width.max(state.cursor.width));

        // TODO: Some sort of ellipsis or so if the constraints are to small
        let size = constraints.check_size(state.layout.size);
        for child in 0..CHILD_COUNT {
            ui.size(children[child], BoxConstraints::new_tight(size));
        }

        size
    }
}
