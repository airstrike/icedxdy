use iced::{Element, Fill, Task};
use icedxdy::Theme;
use icedxdy::core::Palette;
use icedxdy::core::style::{Status, Style, StyleFn, outline, primary};
use icedxdy::theme;
use icedxdy::widget::{Column, button, container, row, text};

mod armed;
mod gallery;
mod swatches;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetMode(Mode),
    ToggleArmed,
}

pub struct State {
    mode: Mode,
    armed: bool,
}

impl State {
    fn new() -> (Self, Task<Message>) {
        let mode = if theme::is_dark_mode() {
            Mode::Dark
        } else {
            Mode::Light
        };
        (Self { mode, armed: false }, Task::none())
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::SetMode(m) => {
                self.mode = m;
            }
            Message::ToggleArmed => {
                self.armed = !self.armed;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        let theme = self.theme();
        let header = header(self.mode);
        let body: Column<'_, Message> = Column::new()
            .push(header.into())
            .push(text::h2("Palette swatches").into())
            .push(swatches::view(&theme).into())
            .push(text::h2("Button gallery").into())
            .push(gallery::view().into())
            .push(text::h2("Armed delete").into())
            .push(armed::view(self.armed).into())
            .spacing(24)
            .padding(32)
            .width(Fill);

        container(body).width(Fill).height(Fill).into()
    }

    fn theme(&self) -> Theme {
        match self.mode {
            Mode::Light => Theme::light(),
            Mode::Dark => Theme::dark(),
        }
    }
}

fn header(mode: Mode) -> Element<'static, Message, Theme> {
    let style: StyleFn<fn(Palette, Status) -> Style> = theme::button::Default.into();

    let light_btn = if mode == Mode::Light {
        button("Light")
            .style(style + primary)
            .on_press(Message::SetMode(Mode::Light))
    } else {
        button("Light")
            .style(outline + primary)
            .on_press(Message::SetMode(Mode::Light))
    };

    let style2: StyleFn<fn(Palette, Status) -> Style> = theme::button::Default.into();
    let dark_btn = if mode == Mode::Dark {
        button("Dark")
            .style(style2 + primary)
            .on_press(Message::SetMode(Mode::Dark))
    } else {
        button("Dark")
            .style(outline + primary)
            .on_press(Message::SetMode(Mode::Dark))
    };

    let title = text::h1("icedxdy \u{2022} palette demo");
    let toggle = row()
        .spacing(8)
        .push(light_btn.into())
        .push(dark_btn.into());
    let header: Column<'_, Message> = Column::new()
        .push(title.into())
        .push(toggle.into())
        .spacing(10);
    header.into()
}

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .title("icedxdy \u{2022} palette demo")
        .theme(State::theme)
        .run()
}
