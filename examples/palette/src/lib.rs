//! Dogfood demo for the icedxdy widget catalog.
//!
//! Layout mirrors shadcn's `/create` page: a top chrome bar (brand · nav ·
//! search · auth), a left rail (customizer), and a dense main dashboard of
//! cards exercising every widget in `widget/src/`. Tints appear thematically
//! — a success alert, a destructive armed-delete, a primary CTA — rather
//! than as abstract swatches.
//!
//! The root scrollable opts into both axes; tiles that genuinely need
//! horizontal overflow (transactions, etc.) wrap their own.

use std::collections::HashSet;

use iced::widget::qr_code;
use iced::{Fill, Task};
use icedxdy::Theme;
use icedxdy::theme;
use icedxdy::theme::container::background;
use icedxdy::widget::rule;
use icedxdy::widget::{column, container, row, scrollable};

pub mod chrome;
pub mod dashboard;
pub mod sidebar;
pub mod tiles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetMode(Mode),
    ToggleConnection(bool),
    EmailChanged(String),
    NameChanged(String),
    AcceptTerms(bool),
    SubscribeEmails(bool),
    SaveForm,
    CancelForm,
    ToggleNotification(String),
    ArmDelete,
    Delete,
    Deploy,
    NotificationCleared,
    Noop,
}

pub struct State {
    pub mode: Mode,
    pub connected: bool,
    pub email: String,
    pub name: String,
    pub accept_terms: bool,
    pub subscribe_emails: bool,
    pub open_notifications: HashSet<String>,
    pub armed: bool,
    pub deploying: bool,
}

fn qr_data() -> &'static qr_code::Data {
    use std::cell::OnceCell;
    thread_local! {
        static QR: OnceCell<&'static qr_code::Data> = const { OnceCell::new() };
    }
    QR.with(|cell| {
        *cell.get_or_init(|| {
            Box::leak(Box::new(
                qr_code::Data::new("https://github.com/airstrike/icedxdy")
                    .expect("static URL fits inside QR capacity"),
            ))
        })
    })
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let mode = if theme::is_dark_mode() {
            Mode::Dark
        } else {
            Mode::Light
        };
        (Self::with_mode(mode), Task::none())
    }

    pub fn dark() -> Self {
        Self::with_mode(Mode::Dark)
    }

    fn with_mode(mode: Mode) -> Self {
        Self {
            mode,
            connected: true,
            email: "ada@icedxdy.dev".into(),
            name: "Ada Lovelace".into(),
            accept_terms: true,
            subscribe_emails: false,
            open_notifications: ["welcome".to_owned()].into_iter().collect(),
            armed: false,
            deploying: false,
        }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::SetMode(m) => self.mode = m,
            Message::ToggleConnection(v) => self.connected = v,
            Message::EmailChanged(v) => self.email = v,
            Message::NameChanged(v) => self.name = v,
            Message::AcceptTerms(v) => self.accept_terms = v,
            Message::SubscribeEmails(v) => self.subscribe_emails = v,
            Message::SaveForm | Message::CancelForm => {}
            Message::ToggleNotification(id) => {
                if !self.open_notifications.remove(&id) {
                    self.open_notifications.insert(id);
                }
            }
            Message::ArmDelete => self.armed = !self.armed,
            Message::Delete => {
                self.armed = false;
            }
            Message::Deploy => self.deploying = !self.deploying,
            Message::NotificationCleared => {}
            Message::Noop => {}
        }
        Task::none()
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn theme(&self) -> Theme {
        match self.mode {
            Mode::Light => Theme::light(),
            Mode::Dark => Theme::dark(),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message, Theme> {
        let chrome = chrome::view(self);
        let body = row![sidebar::view(self), dashboard::view(self)]
            .spacing(0)
            .height(Fill)
            .width(Fill);

        let scroller = scrollable(body).width(Fill).height(Fill);

        let divider: iced::Element<'_, Message, Theme> = rule::horizontal(1).into();
        let page = column![chrome, divider, scroller]
            .spacing(0)
            .width(Fill)
            .height(Fill);

        container(page)
            .width(Fill)
            .height(Fill)
            .style(background)
            .into()
    }
}

#[granita::preview(state = State::new().0)]
pub fn chrome(state: &State) -> iced::Element<'_, Message, Theme> {
    chrome::view(state)
}

#[granita::preview(state = State::new().0)]
pub fn customizer(state: &State) -> iced::Element<'_, Message, Theme> {
    sidebar::view(state)
}

#[granita::preview(state = State::new().0)]
pub fn two_factor(state: &State) -> iced::Element<'_, Message, Theme> {
    use crate::tiles::two_factor;
    two_factor::view(state)
}

#[granita::preview(state = State::new().0)]
pub fn bento(state: &State) -> iced::Element<'_, Message, Theme> {
    dashboard::view(state)
}

#[granita::preview(state = State::new().0, group = "tiles")]
pub fn contributions(state: &State) -> iced::Element<'_, Message, Theme> {
    use crate::tiles::contributions;
    contributions::view(state)
}

#[granita::preview(state = State::new().0, group = "tiles")]
pub fn chart(state: &State) -> iced::Element<'_, Message, Theme> {
    use crate::tiles::chart;
    chart::view(state)
}

#[granita::preview(state = State::dark(), group = "tiles")]
pub fn env_vars(state: &State) -> iced::Element<'_, Message, Theme> {
    use crate::tiles::env_vars;
    env_vars::view(state)
}

#[granita::preview(state = State::new().0, group = "tiles")]
pub fn invite_team(state: &State) -> iced::Element<'_, Message, Theme> {
    use crate::tiles::invite_team;
    invite_team::view(state)
}

#[granita::preview(state = State::new().0, group = "tiles")]
pub fn codespaces(state: &State) -> iced::Element<'_, Message, Theme> {
    use crate::tiles::codespaces;
    codespaces::view(state)
}

#[granita::preview(state = State::new().0, group = "tiles")]
pub fn banner(state: &State) -> iced::Element<'_, Message, Theme> {
    use crate::tiles::banner;
    banner::view(state)
}

granita::previews!();
