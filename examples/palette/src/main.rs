use palette::{State, icon};

#[granita::app(theme = icedxdy::Theme)]
fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .title("icedxdy • palette demo")
        .window_size([1280.0, 860.0])
        .theme(State::theme)
        .font(icon::FONT)
        .run()
}
