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
use message::{LauncherMessage, QueryProgress};

mod query;
use query::Query;

mod query_result_view;
use query_result_view::QueryResultView;

mod launcher_widget;
use launcher_widget::LauncherWidget;

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

#[derive(Debug)]
enum Launcher {
    Idle { state: State },
    Querying { state: State },
}

// TODO: Implement https://docs.rs/iced_native/0.2.2/iced_native/input/keyboard/enum.Event.html to
// track keypress and mouse click events
// Do I have to reimplement TextInput widget to always accept key events even if it isn't selected?

impl Application for Launcher {
    type Executor = iced_futures::executor::Tokio;
    type Message = LauncherMessage;
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
            LauncherMessage::QueryProgress(query_result) => match self {
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
            // Message::EventOccurred(event) => match self {
            //     Launcher::Idle{ state } | Launcher::Querying { state } =>{
            //         // state.input.send_event(event.clone());
            //         Command::none()
            //     }
            // },
            LauncherMessage::FocusTextInput => match self {
                Launcher::Idle { state } => {
                    let mut state = state.clone();
                    state.input = text_input::State::focused();
                    *self = Launcher::Querying { state };
                    Command::none()
                },
                // If we're not idle we need to cancel everything currently running
                // and then start a new query
                _ => Command::none(),
            },
            LauncherMessage::InputChanged(new_input) => match self {
                Launcher::Idle { .. } => {
                    let mut state = State::new();
                    state.input_value = new_input.clone();
                    *self = Launcher::Querying { state };
                    Command::none()
                },
                Launcher::Querying { state } => {
                    let mut new_state = State::new();
                    new_state.input_value = format!("{}{}", state.input_value, new_input);
                    *self = Launcher::Querying { state: new_state };
                    Command::none()
                },
            },
            LauncherMessage::SubmitInput => {
                // Launch some action with the selected result and then clear state
                *self = Launcher::Idle { state: State::new() };
                Command::none()
            },
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let query_subscription = match self {
            Launcher::Idle { .. } => Subscription::none(),
            Launcher::Querying { state } => {
                 Query::new(state.input_value.clone())
                     .get_output()
                     .map(LauncherMessage::QueryProgress)
            },
        };

        Subscription::batch(vec![
            // iced_native::subscription::events().map(Message::EventOccurred),
            query_subscription,
        ])
    }

    // I think the whole view needs to be wrapped in a custom element so I can
    // implement on_event on it and use that to route events properly. Eg always
    // send characters to the text input
    // fn on_event(
    //     &mut self,
    //     event: Event,
    //     layout: Layout<'_>,
    //     cursor_position: Point,
    //     messages: &mut Vec<Message>,
    //     renderer: &Renderer,
    //     clipboard: Option<&dyn Clipboard>,
    // ) {
    //     println!("{:?}", event);
    // }

    fn view(&mut self) -> Element<Self::Message> {
        match self {
            Launcher::Querying { state } | Launcher::Idle { state } => {
                let input = TextInput::new(
                    &mut state.input,
                    "",
                    &state.input_value,
                    LauncherMessage::InputChanged,
                )
                    .padding(15)
                    .size(30)
                    .on_submit(LauncherMessage::SubmitInput);

                let query_results: Element<_> = state.query_results
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new().spacing(20), |column, (_, query_result)| {
                        column.push(query_result.view())
                    }).into();

                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(LauncherWidget::new(Box::new(|m| m)))
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
