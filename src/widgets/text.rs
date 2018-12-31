use crate::*;

pub struct Text<'a> {
    content: &'a str,
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Self {
            content: "",
        }
    }
}

impl<'a> PropsBuilder<Text<'a>> {
    pub fn content(mut self, content: &'a str) -> Self {
        self.content = content;
        self
    }
}

pub struct TextState {
    content: String,
}

impl<'a> Component for Text<'a> {
    type Props = Text<'a>;
    type State = TextState;
    type Msg = ();
    type Event = ();

    fn init(props: &Self::Props) -> Self::State {
        TextState {
            content: props.content.to_string(),
        }
    }

    fn render(state: &Self::State, bounds: Bounds, renderer: &mut Renderer) {
        
    }
}
