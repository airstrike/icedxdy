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

use iced::Element;

use std::collections::HashSet;
use std::f32::consts::PI;

use iced::font;
use iced::gradient::Linear;
use iced::widget::Id;
use iced::widget::stack;
use iced::{Bottom, Fill, Font, Padding, Pixels, Right, Task};
use iced_widget::button::Status;
use icedxdy::Theme;
use icedxdy::core::{Background, Border};
use icedxdy::theme;
use icedxdy::theme::container::Style;
use icedxdy::widget::Size;
use icedxdy::widget::qr_code;
use icedxdy::widget::scrollable::{Direction, thin_scrollbar};
use icedxdy::widget::{button, column, container, row, scrollable};

pub mod chrome;
pub mod dashboard;
pub mod fonts;
pub mod icon;
pub mod sidebar;
pub mod tiles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewPage {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceivingMethod {
    BankTransfer,
    Paypal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadePosition {
    Open,
    Half,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KitchenMode {
    Hue,
    Color,
    Ambient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KitchenScene {
    Cooking,
    Dining,
    Nightlight,
    Focus,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetMode(Mode),
    SetPreviewPage(PreviewPage),
    ToggleConnection(bool),
    SetReceivingMethod(ReceivingMethod),
    SetShadePosition(ShadePosition),
    SetKitchenMode(KitchenMode),
    SetKitchenScene(KitchenScene),
    SetPayoutThreshold(f32),
    TransferAmountChanged(String),
    ToggleNotificationSetting(String),
    SetAllNotificationSettings(bool),
    Font(fonts::Message),
    GitHubStarsLoaded(Result<String, String>),
    SaveForm,
    CancelForm,
    NotificationsOpened(Vec<String>),
    ArmDelete,
    Delete,
    Deploy,
    NotificationCleared,
    Noop,
}

pub struct State {
    pub mode: Mode,
    pub preview_page: PreviewPage,
    pub connected: bool,
    pub receiving_method: ReceivingMethod,
    pub shade_position: ShadePosition,
    pub kitchen_mode: KitchenMode,
    pub kitchen_scene: KitchenScene,
    pub payout_threshold: f32,
    pub transfer_amount: String,
    pub notification_settings: HashSet<String>,
    pub open_notifications: Vec<String>,
    pub armed: bool,
    pub deploying: bool,
    pub qr_connect: Option<qr_code::Data>,
    fount: fount::Fount,
    geist_loaded: bool,
    pub github_stars: Option<String>,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let mode = if theme::is_dark_mode() {
            Mode::Dark
        } else {
            Mode::Light
        };

        (
            Self::with_mode(mode),
            Task::batch([
                fonts::init().map(Message::Font),
                Task::perform(fetch_github_stars(), Message::GitHubStarsLoaded),
            ]),
        )
    }

    pub fn dark() -> Self {
        Self::with_mode(Mode::Dark)
    }

    fn with_mode(mode: Mode) -> Self {
        Self {
            mode,
            preview_page: PreviewPage::One,
            connected: true,
            receiving_method: ReceivingMethod::BankTransfer,
            shade_position: ShadePosition::Half,
            kitchen_mode: KitchenMode::Hue,
            kitchen_scene: KitchenScene::Dining,
            payout_threshold: 2500.0,
            transfer_amount: "$ 1,200.00".into(),
            notification_settings: ["transactions".to_owned(), "security".to_owned()]
                .into_iter()
                .collect(),
            open_notifications: vec!["welcome".to_owned()],
            armed: false,
            deploying: false,
            qr_connect: qr_code::Data::new("https://ledger.app/connect/jd-4829").ok(),
            fount: fount::Fount::new(),
            geist_loaded: false,
            github_stars: None,
        }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::SetMode(m) => self.mode = m,
            Message::SetPreviewPage(page) => self.preview_page = page,
            Message::ToggleConnection(v) => self.connected = v,
            Message::SetReceivingMethod(v) => self.receiving_method = v,
            Message::SetShadePosition(v) => self.shade_position = v,
            Message::SetKitchenMode(v) => self.kitchen_mode = v,
            Message::SetKitchenScene(v) => self.kitchen_scene = v,
            Message::SetPayoutThreshold(v) => self.payout_threshold = v,
            Message::TransferAmountChanged(v) => self.transfer_amount = v,
            Message::ToggleNotificationSetting(id) => {
                if !self.notification_settings.remove(&id) {
                    self.notification_settings.insert(id);
                }
            }
            Message::SetAllNotificationSettings(enabled) => {
                self.notification_settings.clear();
                if enabled {
                    self.notification_settings.extend(
                        ["transactions", "security", "goals", "market"]
                            .into_iter()
                            .map(str::to_owned),
                    );
                }
            }
            Message::Font(msg) => return self.update_font(msg),
            Message::GitHubStarsLoaded(Ok(stars)) => self.github_stars = Some(stars),
            Message::GitHubStarsLoaded(Err(e)) => {
                eprintln!("GitHub star count failed: {e}");
            }
            Message::SaveForm | Message::CancelForm => {}
            Message::NotificationsOpened(open) => self.open_notifications = open,
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

    fn update_font(&mut self, msg: fonts::Message) -> Task<Message> {
        match msg {
            fonts::Message::SystemFontsLoaded(families) => {
                let names: Vec<String> = families
                    .into_iter()
                    .map(|f| f.to_string())
                    .filter(|name| !name.starts_with('.'))
                    .collect();
                self.fount.set_system_families(names);

                if !self.geist_loaded && self.fount.system_families().iter().any(|n| n == "Geist") {
                    self.geist_loaded = true;
                    font::set_defaults(Font::new("Geist"), Pixels(14.0))
                } else {
                    Task::none()
                }
            }
            fonts::Message::CatalogLoaded(Ok(catalog)) => {
                self.fount.set_google_catalog(catalog);

                if self.geist_loaded {
                    Task::none()
                } else {
                    fonts::load("Geist".to_string(), self.fount.google_catalog()).map(Message::Font)
                }
            }
            fonts::Message::CatalogLoaded(Err(e)) => {
                eprintln!("Font catalog loading failed: {e}");
                Task::none()
            }
            fonts::Message::Loaded(name, Err(e)) => {
                eprintln!("Font loading failed ({name}): {e}");
                Task::none()
            }
            fonts::Message::Loaded(name, Ok(())) => {
                if name == "Geist" && !self.geist_loaded {
                    self.geist_loaded = true;
                    font::set_defaults(Font::new("Geist"), Pixels(14.0))
                } else {
                    Task::none()
                }
            }
        }
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
        let preview = container(
            scrollable(dashboard::view(self))
                .id(Id::new("create-preview-scroll-v2"))
                .direction(Direction::Both {
                    vertical: thin_scrollbar(),
                    horizontal: thin_scrollbar(),
                })
                .width(Fill)
                .height(Fill),
        )
        .width(Fill)
        .height(Fill)
        .clip(true)
        .style(preview_frame);

        let preview = stack![
            Element::from(preview),
            Element::from(
                container(page_switcher(self.preview_page))
                    .width(Fill)
                    .height(Fill)
                    .padding(12)
                    .align_x(Right)
                    .align_y(Bottom)
            )
        ]
        .width(Fill)
        .height(Fill);

        let body = row![sidebar::view(self), preview]
            .spacing(24)
            .height(Fill)
            .width(Fill);

        let designer = container(body)
            .padding(Padding {
                top: 6.0,
                right: 24.0,
                bottom: 24.0,
                left: 24.0,
            })
            .width(Fill)
            .height(Fill);

        let page = column![chrome, designer]
            .spacing(0)
            .width(Fill)
            .height(Fill);

        container(page)
            .width(Fill)
            .height(Fill)
            .style(section_soft)
            .into()
    }
}

#[derive(Debug, serde::Deserialize)]
struct GithubRepo {
    stargazers_count: u64,
}

async fn fetch_github_stars() -> Result<String, String> {
    let repo = reqwest::Client::new()
        .get("https://api.github.com/repos/shadcn-ui/ui")
        .header(reqwest::header::USER_AGENT, "icedxdy-palette")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json::<GithubRepo>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(format_stars(repo.stargazers_count))
}

fn format_stars(count: u64) -> String {
    if count >= 1000 {
        format!("{}k", ((count as f64) / 1000.0).round() as u64)
    } else {
        count.to_string()
    }
}

fn page_switcher(page: PreviewPage) -> iced::Element<'static, Message, Theme> {
    let one = page_button("01", PreviewPage::One, page == PreviewPage::One);
    let two = page_button("02", PreviewPage::Two, page == PreviewPage::Two);

    container(row![one, two].spacing(4).align_y(iced::Center))
        .padding(4)
        .style(switcher_style)
        .into()
}

fn page_button(
    label: &'static str,
    page: PreviewPage,
    active: bool,
) -> iced::Element<'static, Message, Theme> {
    let button = button(label)
        .style(button::Ghost)
        .size(Size::Sm)
        .width(36.0)
        .style(page_button_style(active))
        .on_press(Message::SetPreviewPage(page));

    button.into()
}

fn preview_frame(theme: &Theme) -> Style {
    Style {
        background: Some(theme.neutral.base.into()),
        border: Border {
            radius: 18.0.into(),
            width: 1.0,
            color: theme.neutral.foreground.alpha(0.10),
        },
        ..Style::default()
    }
}

fn section_soft(theme: &Theme) -> Style {
    let top: iced::Color = theme.neutral.base.into();
    let bottom: iced::Color = theme.code.base.alpha(0.40).into();

    Style {
        background: Some(Background::Gradient(
            Linear::new(PI)
                .add_stop(0.0, top)
                .add_stop(1.0, bottom)
                .into(),
        )),
        ..Style::default()
    }
}

fn switcher_style(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.neutral.foreground),
        background: Some(theme.card.base.scale_alpha(0.90).into()),
        border: Border {
            radius: 14.0.into(),
            ..Border::default()
        },
        ..Style::default()
    }
}

fn page_button_style(active: bool) -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(move |theme, status| {
        let mut style = iced_widget::button::Style {
            text_color: if active {
                theme.accent.foreground.into()
            } else {
                theme.muted.foreground.into()
            },
            border: Border {
                radius: 10.0.into(),
                ..Border::default()
            }
            .into(),
            ..Default::default()
        };

        if active || matches!(status, Status::Hovered) {
            style.background = Some(theme.accent.base.into());
        }

        style
    })
}
