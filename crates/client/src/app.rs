use crate::components::modal::{Modal, MsgModal};

use iced::Element;
use iced::widget::{Column, button, container, stack};
use iced::{Alignment, Color, Length};

#[derive(Default)]
pub struct App {
    modal: Modal,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Modal(MsgModal),
}

impl App {
    pub fn app_update(&mut self, message: Msg) {
        match message {
            Msg::Modal(msg) => {
                self.modal.modal_update(msg);
            }
        }
    }

    pub fn app_view(&self) -> Element<'_, Msg> {
        let corner_button =
            button("+")
                .on_press(Msg::Modal(MsgModal::OpenModal))
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
            .padding(10);

        if !self.modal.show_modal {
            return main_content.into();
        };

        stack![main_content, self.modal.modal_view()].into()
    }
}
