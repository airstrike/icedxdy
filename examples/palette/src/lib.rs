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
    EmailChanged(String),
    NameChanged(String),
    AcceptTerms(bool),
    SubscribeEmails(bool),
    SetReceivingMethod(ReceivingMethod),
    SetShadePosition(ShadePosition),
    ToggleShadeSchedule(bool),
    SetKitchenMode(KitchenMode),
    SetKitchenScene(KitchenScene),
    SetKitchenBrightness(f32),
    SetKitchenColorTemp(f32),
    SetPayoutThreshold(f32),
    SetShowcaseSlider(f32),
    SetShowcaseMiddleCheckbox(bool),
    SetShowcaseTrailingCheckbox(bool),
    ShowcaseNameChanged(String),
    BuyInvestmentAmountChanged(String),
    NewMilestoneGoalChanged(String),
    NewMilestoneAmountChanged(String),
    NewMilestoneDateChanged(String),
    ReceivingAccountHolderChanged(String),
    ReceivingRoutingChanged(String),
    InviteEmailChanged(String),
    AccountEmailChanged(String),
    AccountPasswordChanged(String),
    TransferAmountChanged(String),
    PreferencesPublicStatsChanged(bool),
    PreferencesEmailNotificationsChanged(bool),
    SocialSpotifyChanged(String),
    SocialInstagramChanged(String),
    SocialSoundCloudChanged(String),
    SocialWebsiteChanged(String),
    EnvDatabaseChanged(String),
    EnvApiChanged(String),
    EnvStripeChanged(String),
    ToggleNotificationSetting(String),
    SetAllNotificationSettings(bool),
    Font(fonts::Message),
    GitHubStarsLoaded(Result<String, String>),
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
    pub preview_page: PreviewPage,
    pub connected: bool,
    pub email: String,
    pub name: String,
    pub accept_terms: bool,
    pub subscribe_emails: bool,
    pub receiving_method: ReceivingMethod,
    pub shade_position: ShadePosition,
    pub shade_auto_schedule: bool,
    pub kitchen_mode: KitchenMode,
    pub kitchen_scene: KitchenScene,
    pub kitchen_brightness: f32,
    pub kitchen_color_temp: f32,
    pub payout_threshold: f32,
    pub showcase_slider: f32,
    pub showcase_middle_checkbox: bool,
    pub showcase_trailing_checkbox: bool,
    pub showcase_name: String,
    pub buy_investment_amount: String,
    pub new_milestone_goal: String,
    pub new_milestone_amount: String,
    pub new_milestone_date: String,
    pub receiving_account_holder: String,
    pub receiving_routing: String,
    pub invite_email: String,
    pub account_email: String,
    pub account_password: String,
    pub transfer_amount: String,
    pub preferences_public_stats: bool,
    pub preferences_email_notifications: bool,
    pub social_spotify: String,
    pub social_instagram: String,
    pub social_soundcloud: String,
    pub social_website: String,
    pub env_database: String,
    pub env_api: String,
    pub env_stripe: String,
    pub notification_settings: HashSet<String>,
    pub open_notifications: HashSet<String>,
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
            email: "ada@icedxdy.dev".into(),
            name: "Ada Lovelace".into(),
            accept_terms: true,
            subscribe_emails: false,
            receiving_method: ReceivingMethod::BankTransfer,
            shade_position: ShadePosition::Half,
            shade_auto_schedule: true,
            kitchen_mode: KitchenMode::Hue,
            kitchen_scene: KitchenScene::Dining,
            kitchen_brightness: 72.0,
            kitchen_color_temp: 44.0,
            payout_threshold: 2500.0,
            showcase_slider: 45.0,
            showcase_middle_checkbox: false,
            showcase_trailing_checkbox: true,
            showcase_name: String::new(),
            buy_investment_amount: "$ 1,000.00".into(),
            new_milestone_goal: String::new(),
            new_milestone_amount: "$15,000".into(),
            new_milestone_date: "Dec 2025".into(),
            receiving_account_holder: "Morgan Shah".into(),
            receiving_routing: "DE89 3704 0044 0532 0130 00".into(),
            invite_email: String::new(),
            account_email: "artist@studio.inc".into(),
            account_password: "***********".into(),
            transfer_amount: "$ 1,200.00".into(),
            preferences_public_stats: true,
            preferences_email_notifications: true,
            social_spotify: "spotify.com/artist/3j...2k".into(),
            social_instagram: "@julianduryea_music".into(),
            social_soundcloud: "soundcloud.com/username".into(),
            social_website: "https://yoursite.com".into(),
            env_database: "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}".into(),
            env_api: "https://api.example.com".into(),
            env_stripe: "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}".into(),
            notification_settings: ["transactions".to_owned(), "security".to_owned()]
                .into_iter()
                .collect(),
            open_notifications: ["welcome".to_owned()].into_iter().collect(),
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
            Message::EmailChanged(v) => self.email = v,
            Message::NameChanged(v) => self.name = v,
            Message::AcceptTerms(v) => self.accept_terms = v,
            Message::SubscribeEmails(v) => self.subscribe_emails = v,
            Message::SetReceivingMethod(v) => self.receiving_method = v,
            Message::SetShadePosition(v) => self.shade_position = v,
            Message::ToggleShadeSchedule(v) => self.shade_auto_schedule = v,
            Message::SetKitchenMode(v) => self.kitchen_mode = v,
            Message::SetKitchenScene(v) => self.kitchen_scene = v,
            Message::SetKitchenBrightness(v) => self.kitchen_brightness = v,
            Message::SetKitchenColorTemp(v) => self.kitchen_color_temp = v,
            Message::SetPayoutThreshold(v) => self.payout_threshold = v,
            Message::SetShowcaseSlider(v) => self.showcase_slider = v,
            Message::SetShowcaseMiddleCheckbox(v) => self.showcase_middle_checkbox = v,
            Message::SetShowcaseTrailingCheckbox(v) => self.showcase_trailing_checkbox = v,
            Message::ShowcaseNameChanged(v) => self.showcase_name = v,
            Message::BuyInvestmentAmountChanged(v) => self.buy_investment_amount = v,
            Message::NewMilestoneGoalChanged(v) => self.new_milestone_goal = v,
            Message::NewMilestoneAmountChanged(v) => self.new_milestone_amount = v,
            Message::NewMilestoneDateChanged(v) => self.new_milestone_date = v,
            Message::ReceivingAccountHolderChanged(v) => self.receiving_account_holder = v,
            Message::ReceivingRoutingChanged(v) => self.receiving_routing = v,
            Message::InviteEmailChanged(v) => self.invite_email = v,
            Message::AccountEmailChanged(v) => self.account_email = v,
            Message::AccountPasswordChanged(v) => self.account_password = v,
            Message::TransferAmountChanged(v) => self.transfer_amount = v,
            Message::PreferencesPublicStatsChanged(v) => self.preferences_public_stats = v,
            Message::PreferencesEmailNotificationsChanged(v) => {
                self.preferences_email_notifications = v;
            }
            Message::SocialSpotifyChanged(v) => self.social_spotify = v,
            Message::SocialInstagramChanged(v) => self.social_instagram = v,
            Message::SocialSoundCloudChanged(v) => self.social_soundcloud = v,
            Message::SocialWebsiteChanged(v) => self.social_website = v,
            Message::EnvDatabaseChanged(v) => self.env_database = v,
            Message::EnvApiChanged(v) => self.env_api = v,
            Message::EnvStripeChanged(v) => self.env_stripe = v,
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
            preview,
            container(page_switcher(self.preview_page))
                .width(Fill)
                .height(Fill)
                .padding(12)
                .align_x(Right)
                .align_y(Bottom)
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
    let button = button::ghost(label)
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
        text_color: Some(theme.neutral.foreground.into()),
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
