use iced::{
    Align,
    Element,
    Row,
    Text,
};

use crate::Message;

#[derive(Debug, Clone)]
pub struct QueryResultView {
    text: String,
}

impl QueryResultView {
    pub fn new(text: String) -> Self {
        QueryResultView {
            text,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Text::new(&self.text)
            )
            .into()
    }
}
