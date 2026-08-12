use iced::Element;
use iced::widget::{container, text};

#[derive(Default)]
struct State;

#[derive(Debug, Clone)]
enum Message {}

fn update(_state: &mut State, _message: Message) {}

fn view(_state: &State) -> Element<'_, Message> {
    container(text("Hello World!")).into()
}

fn main() -> iced::Result {
    iced::run(update, view)
}
