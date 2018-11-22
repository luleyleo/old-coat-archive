
struct PropsBuilder<'a, C: Component<'a>> {
    props: C::Props
}

impl<'a, C: Component<'a>> PropsBuilder<'a, C> {

    pub fn new(props: C::Props) -> Self {
        PropsBuilder { props }
    }

    pub fn set(self) {
        C::finish(self.props);
    }

}

trait Component<'a> : Sized {

    type Props: Sized + 'a;

    fn new() -> PropsBuilder<'a, Self>;

    fn finish(props: Self::Props);

}

impl<'a, C: Component<'a>> std::ops::Deref for PropsBuilder<'a, C> {
    type Target = C::Props;
    fn deref(&self) -> &Self::Target {
        &self.props
    }
}

impl<'a, C: Component<'a>> std::ops::DerefMut for PropsBuilder<'a, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.props
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

enum Button {}

struct Props<'a> {
    label: Option<&'a str>
}

impl<'a> PropsBuilder<'a, Button> {
    pub fn label(mut self, value: &'a str) -> Self {
        self.label = Some(value);
        self
    }
}

impl<'a> Component<'a> for Button {
    type Props = Props<'a>;

    fn new() -> PropsBuilder<'a, Self> {
        PropsBuilder::new(Props {
            label: None
        })
    }

    fn finish(props: Props<'a>) {
        println!("Hello {}!", props.label.unwrap_or("World"));
    }
}

fn main() {
    Button::new()
        .set();

    Button::new()
        .label("Rust")
        .set();
}
