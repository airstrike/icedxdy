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

use iced::{Fill, Task};
use icedxdy::Theme;
use icedxdy::theme;
use icedxdy::theme::container::background;
use icedxdy::widget::rule;
use icedxdy::widget::scrollable::{Direction, Scrollbar};
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

/// Sidebar nav items. Visual only — clicking updates `State::nav` and
/// highlights the active row, but every section shares the same dashboard
/// body (we're dogfooding the widget catalog, not building a real router).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavItem {
    Overview,
    Transactions,
    Targets,
    Notifications,
    Settings,
}

impl NavItem {
    pub const ALL: &'static [Self] = &[
        Self::Overview,
        Self::Transactions,
        Self::Targets,
        Self::Notifications,
        Self::Settings,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Transactions => "Transactions",
            Self::Targets => "Targets",
            Self::Notifications => "Notifications",
            Self::Settings => "Settings",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetMode(Mode),
    ToggleConnection(bool),
    SelectNav(NavItem),
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
    pub nav: NavItem,
    pub email: String,
    pub name: String,
    pub accept_terms: bool,
    pub subscribe_emails: bool,
    pub open_notifications: HashSet<String>,
    pub armed: bool,
    pub deploying: bool,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let mode = if theme::is_dark_mode() {
            Mode::Dark
        } else {
            Mode::Light
        };
        (
            Self {
                mode,
                connected: true,
                nav: NavItem::Overview,
                email: "ada@icedxdy.dev".into(),
                name: "Ada Lovelace".into(),
                accept_terms: true,
                subscribe_emails: false,
                open_notifications: ["welcome".to_owned()].into_iter().collect(),
                armed: false,
                deploying: false,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::SetMode(m) => self.mode = m,
            Message::ToggleConnection(v) => self.connected = v,
            Message::SelectNav(n) => self.nav = n,
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

        let scroller = scrollable(body)
            .width(Fill)
            .height(Fill)
            .direction(both_scrollbars());

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

fn both_scrollbars() -> Direction {
    Direction::Both {
        vertical: Scrollbar::default(),
        horizontal: Scrollbar::default(),
    }
}

#[granita::preview(state = State::new().0)]
pub fn chrome(state: &State) -> iced::Element<'_, Message, Theme> {
    chrome::view(state)
}

granita::previews!();
