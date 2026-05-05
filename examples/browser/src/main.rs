use browser::{Browser, font, icon};

#[granita::app(theme = icedxdy::Theme)]
fn main() -> iced::Result {
    iced::application(Browser::new, Browser::update, Browser::view)
        .title("icedxdy \u{2022} components browser")
        .window_size([1440.0, 900.0])
        .antialiasing(true)
        .subscription(Browser::subscription)
        .theme(Browser::theme)
        .font(icon::FONT)
        .font(font::GEIST_MONO)
        .run()
}
