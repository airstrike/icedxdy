//! Dogfood demo for the icedxdy widget catalog.
//!
//! Layout mirrors shadcn's `/create` page: a left rail (theme controls,
//! nav, and a tint legend) and a dense main dashboard of cards exercising
//! every widget in `widget/src/`. Tints appear thematically — a success
//! alert, a destructive armed-delete, a primary CTA — rather than as
//! abstract swatches.
//!
//! The header carries a `Switch` labeled `Connection`. When off, every
//! action button across the dashboard wires `on_press(...)` only when
//! `state.connected` is true, so flipping it surfaces the disabled
//! visual on every tint at once.
//!
//! The root is a vertical `Scrollable`; tiles that genuinely need
//! horizontal overflow (transactions, tints) wrap their own.

use std::collections::HashSet;

use iced::alignment;
use iced::widget::space;
use iced::{Center, Fill, Task};
use icedxdy::Component;
use icedxdy::Theme;
use icedxdy::theme;
use icedxdy::theme::container::background;
use icedxdy::widget::rule;
use icedxdy::widget::scrollable::{Direction, Scrollbar};
use icedxdy::widget::{button, column, container, row, scrollable, switch, text};

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
        let header = self.header();
        let body = row![sidebar::view(self), dashboard::view(self)]
            .spacing(0)
            .height(Fill)
            .width(Fill);

        let scroller = scrollable(body)
            .width(Fill)
            .height(Fill)
            .direction(both_scrollbars());

        let divider: iced::Element<'_, Message, Theme> = rule::horizontal(1).into();
        let page = column![header, divider, scroller]
            .spacing(0)
            .width(Fill)
            .height(Fill);

        container(page)
            .width(Fill)
            .height(Fill)
            .style(background)
            .into()
    }

    fn header(&self) -> iced::Element<'_, Message, Theme> {
        let title = text::h2("icedxdy \u{2022} theme explorer");
        let subtitle = text::muted("Catalog demo — flip the toggle to disable every action.");

        // Fixed-width label container so the switch's position doesn't
        // jump when the label flips between "Connected" / "Disconnected".
        let connection_label = container(
            text::small(if self.connected {
                "Connected"
            } else {
                "Disconnected"
            })
            .style(theme::text::Muted),
        )
        .width(96)
        .align_x(alignment::Horizontal::Right);

        let connection_switch = switch::switch(self.connected).on_toggle(Message::ToggleConnection);

        let connection = row![Component::from(connection_label), connection_switch]
            .spacing(8)
            .align_y(Center);

        let mode_button = match self.mode {
            Mode::Light => button("Dark").on_press(Message::SetMode(Mode::Dark)),
            Mode::Dark => button("Light").on_press(Message::SetMode(Mode::Light)),
        };

        let title_col = column([title.into(), subtitle.into()]).spacing(2);

        let bar = row![title_col, space::horizontal(), connection, mode_button,]
            .spacing(16)
            .align_y(Center)
            .width(Fill);

        container(bar).padding([16, 24]).width(Fill).into()
    }
}

fn both_scrollbars() -> Direction {
    Direction::Both {
        vertical: Scrollbar::default(),
        horizontal: Scrollbar::default(),
    }
}

granita::previews!();
