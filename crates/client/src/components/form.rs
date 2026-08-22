use crate::app::Msg;
use iced::Element;
use iced::widget::image::Handle;
use iced::widget::{container, image as image_widget};

#[derive(Debug, Clone)]
pub(crate) enum FormMode {
    CreateForm(Forms),
}

#[derive(Debug, Clone)]
pub struct Forms {
    pub profile_image: Handle,
}

impl Forms {
    pub fn new() -> Self {
        Self {
            profile_image: Handle::from_rgba(1, 1, vec![218, 77, 38, 255]),
        }
    }

    pub fn create_view(&self) -> Element<'static, Msg> {
        let img = image_widget(Clone::clone(&self.profile_image))
            .width(100)
            .height(100);

        container(Element::from(img)).into()
    }
}
