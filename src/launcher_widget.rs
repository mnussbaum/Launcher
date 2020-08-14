use iced_native::{
    layout, mouse, Element, Hasher, Layout, Length,
    Point, Widget, Clipboard, Event,
};
use iced_graphics::{Backend, Defaults, Primitive, Renderer};

pub struct LauncherWidget<'a, Message, B: iced_graphics::Backend> {
    text_input: iced_native::Element<'a, Message, Renderer<B>>,
}

impl<'a, Message, B: iced_graphics::Backend> LauncherWidget<'a, Message, B> {
    pub fn new(text_input: iced_native::Element<'a, Message, Renderer<B>>) -> Self {
        Self { text_input }
    }
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for LauncherWidget<'a, Message, B>
where
    B: Backend,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }
    fn layout(
        &self,
        renderer: &Renderer<B>,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.text_input.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer<B>,
        clipboard: Option<&dyn Clipboard>,
    ) {
        self.text_input.on_event(
            event,
            layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.text_input.hash_layout(state)
    }

    fn draw(
        &self,
        renderer: &mut Renderer<B>,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> (Primitive, mouse::Interaction) {
        self.text_input.draw(renderer, defaults, layout, cursor_position)
    }
}

impl<'a, Message: 'a, B: iced_graphics::backend::Backend + 'a> Into<Element<'a, Message, Renderer<B>>> for LauncherWidget<'a, Message, B> {
    fn into(self) -> Element<'a, Message, Renderer<B>>
    where
        B: Backend + 'a,
    {

        Element::new(self)
    }
}
