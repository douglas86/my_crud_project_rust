use iced::Element;
use iced::widget::{Column, button, container};
use iced::{Alignment, Color, Length};

#[derive(Default)]
struct State;

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked,
}

fn update(_state: &mut State, message: Message) {
    match message {
        Message::ButtonClicked => {
            println!("button clicked!");
        }
    }
}

fn view(_state: &State) -> Element<'_, Message> {
    let button = button("Click Me!")
        .on_press(Message::ButtonClicked)
        .style(|_theme, _status| button::Style {
            background: Some(iced::Background::Color(Color::from_rgb(
                19.0 / 255.0,
                82.0 / 255.0,
                36.0 / 255.0,
            ))),
            text_color: Color::WHITE,
            border: iced::Border {
                radius: 6.0.into(),
                ..iced::Border::default()
            },
            ..button::Style::default()
        });

    container(Column::new().push(button))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::End)
        .align_y(Alignment::Start)
        .padding(20)
        .into()
}

fn main() -> iced::Result {
    iced::run(update, view)
}
