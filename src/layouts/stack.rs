use crate::*;

#[derive(Default)]
pub struct Stack;

impl Component for Stack {
    type State = ();
    type Msg = ();
    type Event = ();

    fn init(_props: &Self) -> Self::State {}

    fn layout(
        _state: &Self::State,
        children: &[Cid],
        constraints: BoxConstraints,
        ui: &mut UiLayout,
    ) -> Size {
        let constraints = constraints.min(Size::zero());
        let mut largest = Size::zero();

        for child in children.iter().cloned() {
            let size = ui.size(child, constraints);
            largest = largest.max(size);
        }

        for child in children.iter().cloned() {
            let size = ui.get_size(child);
            let x = (largest.width - size.width) / 2.0;
            let y = (largest.height - size.height) / 2.0;
            ui.position(child, Position::new(x, y));
        }

        largest
    }
}
