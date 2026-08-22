//! # Application
//!
//! This files provides the entire running of the application

use crate::components::form::{FormMode, Forms};
use crate::components::modal::{Modal, MsgModal};

use iced::Element;
use iced::widget::{Column, button, container, stack};
use iced::{Alignment, Color, Length};

#[derive(Default, Clone)]
pub struct App {
    /// Bring the Modal struct into scope for the app
    modal: Modal,
}

#[derive(Debug, Clone)]
pub enum Msg {
    /// Brings the Modal enums into scope for the update method
    Modal(MsgModal),
}

impl App {
    /// Updates the model of the main screen on button click
    ///
    /// # Arguments
    /// * `message` - The modal events passed to the Modal component on actions
    pub fn app_update(&mut self, message: Msg) -> iced::Task<Msg> {
        match message {
            Msg::Modal(msg) => self.modal.modal_update(msg),
        }

        iced::Task::none()
    }

    /// Creates the main page on application load
    pub fn app_view(&self) -> Element<'_, Msg> {
        // Display the button to add a new card with a "+"
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

        // Displays the main content of the page
        let main_content = container(Column::new().push(corner_button))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::End)
            .align_y(Alignment::Start)
            .padding(10);

        // Display the modal only if true
        if !self.modal.show_modal {
            return main_content.into();
        };

        // stack the modal and the main content together
        // stack macro is used when working with z-index
        stack![
            main_content,
            self.modal.modal_view(&FormMode::CreateForm(Forms::new()))
        ]
        .into()
    }
}
