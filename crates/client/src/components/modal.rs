//! # Modal Component
//!
//! Provides the UI dialog overlay layout and state handler for user modal

use crate::app::Msg;

use iced::widget::{Column, button, container, text};
use iced::{Alignment, Length};
use iced::{Color, Element};

#[derive(Default)]
pub struct Modal {
    /// Controls whether the modal dialog is currently visible or not
    pub show_modal: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum MsgModal {
    /// Opens the modal based on the update method
    OpenModal,
    /// Closes the modal based on the update method
    CloseModal,
}

impl Modal {
    /// Updates the internal state of the modal based on incoming messages
    ///
    /// # Arguments
    /// * `message` - The modal event (`OpenModal` or `CloseModal`) to process.
    pub fn modal_update(&mut self, message: MsgModal) {
        match message {
            MsgModal::CloseModal => {
                self.show_modal = false;
            }
            MsgModal::OpenModal => {
                self.show_modal = true;
            }
        }
    }

    /// Creates the UI content and overlay for the modal dialog.
    pub fn modal_view(&self) -> Element<'_, Msg> {
        // creates the content inside the modal
        let modal_box = container(
            Column::new()
                .spacing(20)
                .align_x(Alignment::Center)
                .push(text!("Create New Modal").size(24))
                .push(button("Close").on_press(Msg::Modal(MsgModal::CloseModal))),
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

        // creating the out layer or the overlay of the modal
        container(modal_box)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(
                    0.0, 0.0, 0.0, 0.6,
                ))),
                ..container::Style::default()
            })
            .into()
    }
}
