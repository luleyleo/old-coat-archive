
struct PropsBuilder<C: Component> {
    props: C::Props
}

impl<C: Component> PropsBuilder<C> {

    pub fn new(props: C::Props) -> Self {
        PropsBuilder { props }
    }

    pub fn set(self) {
        C::finish(self.props);
    }

}

trait Component : Sized {

    type Props: Sized;

    fn new() -> PropsBuilder<Self>;

    fn finish(props: Self::Props);

}

impl<C: Component> std::ops::Deref for PropsBuilder<C> {
    type Target = C::Props;
    fn deref(&self) -> &Self::Target {
        &self.props
    }
}

impl<C: Component> std::ops::DerefMut for PropsBuilder<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.props
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

struct Button<'a>(&'a ());

struct Props<'a> {
    label: Option<&'a str>
}

impl<'a> PropsBuilder<Button<'a>> {
    pub fn label(mut self, value: &'a str) -> Self {
        self.label = Some(value);
        self
    }
}

impl<'a> Component for Button<'a> {
    type Props = Props<'a>;

    fn new() -> PropsBuilder<Self> {
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
