use palette::State;

#[granita::app(theme = icedxdy::Theme)]
fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .title("icedxdy \u{2022} palette demo")
        .window_size([1280.0, 860.0])
        .theme(State::theme)
        .run()
}
