use crate::app::Msg;
use iced::Element;
use iced::widget::image::Handle;
use iced::widget::{Image, container, image as image_widget};

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

    pub fn create_view<'a>(&self) -> Element<'static, Msg> {
        let img = image_widget(self.profile_image.clone())
            .width(100)
            .height(100);

        // TODO: Create an image drag and drop feature using subscriptions

        println!("img: {:?}", self.profile_image);

        container(img).into()
    }
}

impl Default for Forms {
    fn default() -> Self {
        Self::new()
    }
}
