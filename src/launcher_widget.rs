use iced_native::{
    layout, mouse, Background, Color, Element, Hasher, Layout, Length,
    Point, Size, Widget, Clipboard, Event, keyboard
};
use iced_graphics::{Backend, Defaults, Primitive, Renderer};

use crate::LauncherMessage;

pub struct LauncherWidget<Message> {
    on_edit: Box<dyn Fn(LauncherMessage) -> Message>,
}

impl<Message> LauncherWidget<Message> {
    pub fn new(on_edit: Box<dyn Fn(LauncherMessage) -> Message>) -> Self {
        Self {
            on_edit,
        }
    }
}

impl<Message, B> Widget<Message, Renderer<B>> for LauncherWidget<Message>
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
        _renderer: &Renderer<B>,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(
            f32::from(10.0) * 2.0,
            f32::from(10.0) * 2.0,
        ))
    }

    fn on_event(
        &mut self,
        event: Event,
        _layout: Layout<'_>,
        _cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer<B>,
        _clipboard: Option<&dyn Clipboard>,
    ) {
        match event {
            Event::Keyboard(keyboard::Event::CharacterReceived(c)) => {
                messages.push((self.on_edit)(LauncherMessage::FocusTextInput));
                messages.push((self.on_edit)(LauncherMessage::InputChanged(c.to_string())));
            }
            _ => {},
        }
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;

        10.hash(state);
    }

    fn draw(
        &self,
        _renderer: &mut Renderer<B>,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> (Primitive, mouse::Interaction) {
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(Color::BLACK),
                border_radius: 10,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            },
            mouse::Interaction::default(),
        )
    }
}

impl<'a, Message: 'a, B: iced_graphics::backend::Backend> Into<Element<'a, Message, Renderer<B>>> for LauncherWidget<Message> {
    fn into(self) -> Element<'a, Message, Renderer<B>>
    where
        B: Backend,
    {

        Element::new(self)
    }
}
