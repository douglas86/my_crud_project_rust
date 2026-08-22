//! # Main File
//!
//! Main entry point into the entire client side of the project
//!
//! This file is only created to call the app file and connect all others together

mod app;
mod components;

use app::App;

fn main() -> iced::Result {
    // iced run was used as this is a stateless/simple sync application
    // There are not async operations, background tasks or side effects
    iced::application(
        || (App::default(), iced::Task::none()),
        App::app_update,
        App::app_view,
    )
    .title("My Application")
    .run()

    // Will need to convert from iced::run to iced::application
    // when I need to connect to server
}
