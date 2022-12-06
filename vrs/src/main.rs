use {
    iced::{
        window,
        Sandbox,
        Settings,
    },
    vrs::ui::*,
};

fn main() {
    InterpreterUi::run(Settings {
        window: window::Settings {
            size: (480, 640),
            ..Default::default()
        },
        ..Default::default()
    })
    .expect("Failed")
}
