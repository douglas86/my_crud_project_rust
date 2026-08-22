use crate::app::Msg;
use iced::Element;
use iced::widget::image::Handle;
use iced::widget::{container, image as image_widget};

const FERRIES_SVG: &[u8] = include_bytes!("../../assets/rustacean-flat-happy.png");

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
            profile_image: Handle::from_bytes(FERRIES_SVG),
        }
    }

    pub fn create_view(&self) -> Element<'static, Msg> {
        let img = image_widget(Clone::clone(&self.profile_image))
            .width(100)
            .height(100);

        container(Element::from(img)).into()
    }
}
