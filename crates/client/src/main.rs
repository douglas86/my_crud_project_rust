use iced::Element;
use iced::widget::{Column, button, container, stack, text};
use iced::{Alignment, Color, Length};

#[derive(Default)]
struct State {
    show_modal: bool,
}

#[derive(Debug, Clone)]
enum Message {
    OpenModal,
    CloseModal,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::OpenModal => {
            state.show_modal = true;
        }
        Message::CloseModal => {
            state.show_modal = false;
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let corner_button =
        button("Click Me!")
            .on_press(Message::OpenModal)
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

    let main_content = container(Column::new().push(corner_button))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::End)
        .align_y(Alignment::Start)
        .padding(20);

    if !state.show_modal {
        return main_content.into();
    };

    let modal_box = container(
        Column::new()
            .spacing(20)
            .align_x(Alignment::Center)
            .push(text("Create New Modal").size(24))
            .push(button("Close").on_press(Message::CloseModal)),
    )
    .padding(20)
    .style(|_theme| container::Style {
        background: Some(iced::Background::Color(Color::from_rgb8(35, 38, 42))),
        border: iced::Border {
            radius: 10.0.into(),
            width: 1.0,
            color: Color::from_rgb8(60, 65, 72),
        },
        ..container::Style::default()
    });

    let modal_overlay = container(modal_box)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(
                0.0, 0.0, 0.0, 0.6,
            ))),
            ..container::Style::default()
        });

    stack![main_content, modal_overlay].into()
}

fn main() -> iced::Result {
    iced::run(update, view)
}
