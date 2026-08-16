use crate::app::Msg;
use iced::Element;
use iced::widget::{container, text};

#[derive(Debug, Clone)]
pub(crate) enum FormMode {
    CreateForm,
}

#[derive(Default)]
pub struct Forms {}

impl Forms {
    pub fn create_view<'a>() -> Element<'a, Msg> {
        let input = text("This is a form");

        container(input).into()
    }
}
