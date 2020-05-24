use iced::{
    Application,
    Column,
    Command,
    Container,
    Element,
    Length,
    Scrollable,
    Settings,
    Subscription,
    TextInput,
    scrollable,
    text_input,
};

mod message;
use message::{Message, QueryProgress};

mod query;
use query::Query;

mod query_result_view;
use query_result_view::QueryResultView;

pub fn main() {
    Launcher::run(Settings::default())
}

#[derive(Debug, Default, Clone)]
struct State {
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    query_results: Vec<QueryResultView>,
}

impl State {
    pub fn new() -> State {
        return State::default()
    }
}

enum Launcher {
    Idle { state: State },
    Querying { state: State },
}

// TODO: Implement https://docs.rs/iced_native/0.2.2/iced_native/input/keyboard/enum.Event.html to
// track keypress and mouse click events

impl Application for Launcher {
    type Executor = iced_futures::executor::Tokio;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Launcher, Command<Self::Message>) {
        let state = State::new();
        (Launcher::Idle{ state }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Launcher")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::QueryProgress(query_result) => match self {
                Launcher::Idle { .. } => { Command::none() },
                Launcher::Querying { state } => {
                    match query_result {
                        QueryProgress::NewLine(line) => {
                            state.query_results.push(QueryResultView::new(line));
                        },
                        QueryProgress::Finished {
                            exit_status
                        } => {
                            if !exit_status.success() {
                                println!("This is where a plugin process failure surfaces atm");
                            }

                            *self = Launcher::Idle { state: state.clone() };
                        },
                        _ => {},
                    }
                    Command::none()
                },
            },
            Message::InputChanged(new_input) => match self {
                Launcher::Idle { .. } => {
                    let mut state = State::new();
                    state.input_value = new_input.clone();
                    *self = Launcher::Querying { state };
                    Command::none()
                },
                // If we're not idle we need to cancel everything currently running
                // and then start a new query
                _ => Command::none(),
            },
            Message::SubmitInput => {
                // Launch some action with the selected result and then clear state
                *self = Launcher::Idle { state: State::new() };
                Command::none()
            },
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self {
            Launcher::Idle { .. } => Subscription::none(),
            Launcher::Querying { state } => {
                 Query::new(state.input_value.clone())
                     .get_output()
                     .map(Message::QueryProgress)
            },
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self {
            Launcher::Querying { state } | Launcher::Idle { state } => {
                let input = TextInput::new(
                    &mut state.input,
                    "",
                    &state.input_value,
                    Message::InputChanged,
                )
                    .padding(15)
                    .size(30)
                    .on_submit(Message::SubmitInput);

                let query_results: Element<_> = state.query_results
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new().spacing(20), |column, (_, query_result)| {
                        column.push(query_result.view())
                    })
                .into();

                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(input)
                    .push(query_results);

                return Scrollable::new(&mut state.scroll)
                    .padding(40)
                    .push(
                        Container::new(content).width(Length::Fill).center_x(),
                    )
                    .into()
            },
        }
    }
}
