use iced::mouse::Interaction;
use iced::widget::{column, combo_box, row, scrollable, space, svg};
use iced_widget::button::StyleFn;

use iced::{Fill, Font, Pixels, Subscription, Task};
use icedxdy::theme::tweak::Bg;
use icedxdy::theme::tweak::token::Token;
use icedxdy::widget::container;
use icedxdy::widget::markdown::highlighter::Map;

use icedxdy::widget::{button, rule, separator, sidebar, text};
use icedxdy::{Size, Theme, Weight, theme};

/// Logo SVG, embedded at compile time so the binary is self-contained
/// (no runtime file lookup, no working-directory dependence).
const LOGO_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/logo.svg"
));

use crate::component;
use crate::fonts;
use crate::icon;
use crate::page::{self, SnippetViewer};

/// Fonts to pin at the top of the font list.
const PINNED_FONTS: &[&str] = &["Geist", "Inter"];

pub struct App {
    dark_mode: bool,
    current_page: usize,
    pages: Vec<Box<dyn page::Page>>,
    /// Both code-syntax theme maps are parsed once at startup (in `main.rs`)
    /// and handed in here. `ToggleTheme` rebuilds the viewer from the
    /// already-parsed map; no re-parse on toggle.
    gh_light: Map,
    gh_dark: Map,
    snippet_viewer: SnippetViewer,
    fount: fount::Fount,
    font_list: combo_box::State<String>,
    current_font: String,
}

/// Top-level docs-site nav sections, mirroring shadcn's main-nav.
///
/// Replicates `apps/v4/components/main-nav.tsx` minus `Directory` (deferred
/// per current scope). No real routing wired yet — the buttons emit
/// [`Message::NavSelect`] which `update` ignores; this preserves the visual
/// affordance while the navigation backend lands later.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavSection {
    Docs,
    Components,
    Blocks,
    Charts,
    Create,
}

impl NavSection {
    const ALL: &'static [Self] = &[
        Self::Docs,
        Self::Components,
        Self::Blocks,
        Self::Charts,
        Self::Create,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Docs => "Docs",
            Self::Components => "Components",
            Self::Blocks => "Blocks",
            Self::Charts => "Charts",
            Self::Create => "Create",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme,
    SelectPage(usize),
    FontSelected(String),
    Page(usize, page::Message),
    Font(fonts::Message),
    NavSelect(NavSection),
    /// Emitted by the splash-screen progress bar each time its easing
    /// animation settles. `App` ignores it; `Browser` intercepts and uses
    /// the `value == 100` case to dismiss the splash so it stays up until
    /// the bar visibly reaches the right edge.
    SplashIdle(f32),
}

impl App {
    pub fn new(gh_light: Map, gh_dark: Map) -> (Self, Task<Message>) {
        let mut pages: Vec<Box<dyn page::Page>> = Vec::new();
        let mut tasks: Vec<Task<Message>> = Vec::new();

        use component::{
            alert, aspect_ratio, badge, breadcrumb, button, button_group, card, checkbox, field,
            input, label, progress, sidebar, switch, text,
        };

        let constructors: Vec<fn(usize) -> Box<dyn page::Page>> = vec![
            alert::AlertPage::new,
            aspect_ratio::AspectRatioPage::new,
            badge::BadgePage::new,
            breadcrumb::BreadcrumbPage::new,
            button::ButtonPage::new,
            button_group::ButtonGroupPage::new,
            card::CardPage::new,
            checkbox::CheckboxPage::new,
            field::FieldPage::new,
            input::InputPage::new,
            label::LabelPage::new,
            progress::ProgressPage::new,
            sidebar::SidebarPage::new,
            switch::SwitchPage::new,
            text::TextPage::new,
        ];

        for (i, ctor) in constructors.into_iter().enumerate() {
            let page = ctor(i);
            tasks.push(page.init());
            pages.push(page);
        }

        let default_font = "Geist".to_string();

        let dark_mode = theme::is_dark_mode();
        let active_theme = if dark_mode {
            Theme::dark()
        } else {
            Theme::light()
        };
        let snippet_viewer = SnippetViewer::new(
            if dark_mode {
                gh_dark.clone()
            } else {
                gh_light.clone()
            },
            active_theme.palette(),
        );

        (
            Self {
                dark_mode,
                current_page: 0,
                pages,
                gh_light,
                gh_dark,
                snippet_viewer,
                fount: fount::Fount::new(),
                font_list: combo_box::State::new(vec![default_font.clone()]),
                current_font: default_font,
            },
            Task::batch(tasks),
        )
    }

    pub fn is_dark(&self) -> bool {
        self.dark_mode
    }

    /// Wire the active page's subscription. Inactive pages don't tick — the
    /// user only ever sees one at a time, and animation work for hidden
    /// pages would just heat the CPU.
    pub fn subscription(&self) -> Subscription<Message> {
        let i = self.current_page;
        match self.pages.get(i) {
            Some(page) => page
                .subscription()
                .with(i)
                .map(|(i, m)| Message::Page(i, m)),
            None => Subscription::none(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                let map = if self.dark_mode {
                    self.gh_dark.clone()
                } else {
                    self.gh_light.clone()
                };
                let active_theme = if self.dark_mode {
                    Theme::dark()
                } else {
                    Theme::light()
                };
                self.snippet_viewer = SnippetViewer::new(map, active_theme.palette());
                Task::none()
            }
            Message::SelectPage(page) => {
                if page != self.current_page {
                    if let Some(prev) = self.pages.get_mut(self.current_page) {
                        prev.reset_volatile();
                    }
                }
                self.current_page = page;
                self.pages
                    .get_mut(page)
                    .map(|p| p.on_visible())
                    .unwrap_or_else(Task::none)
            }
            Message::FontSelected(name) => {
                self.current_font = name.clone();

                if self.fount.system_families().contains(&name) {
                    let name: &'static str = Box::leak(name.into_boxed_str());
                    iced::font::set_defaults(Font::new(name), Pixels(14.0))
                } else {
                    fonts::load(name, self.fount.google_catalog()).map(Message::Font)
                }
            }
            Message::Page(i, message) => {
                if let Some(page) = self.pages.get_mut(i) {
                    page.update(message)
                } else {
                    Task::none()
                }
            }
            Message::NavSelect(_) => Task::none(),
            Message::SplashIdle(_) => Task::none(),
            Message::Font(msg) => match msg {
                fonts::Message::SystemFontsLoaded(families) => {
                    let names: Vec<String> = families
                        .into_iter()
                        .map(|f| f.to_string())
                        .filter(|name| !name.starts_with('.'))
                        .collect();
                    self.fount.set_system_families(names);
                    self.rebuild_font_list();
                    Task::none()
                }
                fonts::Message::CatalogLoaded(Ok(catalog)) => {
                    self.fount.set_google_catalog(catalog);
                    self.rebuild_font_list();
                    fonts::load(self.current_font.clone(), self.fount.google_catalog())
                        .map(Message::Font)
                }
                fonts::Message::CatalogLoaded(Err(_)) => Task::none(),
                fonts::Message::Loaded(name, Err(e)) => {
                    eprintln!("Font loading failed ({name}): {e}");
                    Task::none()
                }
                fonts::Message::Loaded(name, Ok(())) => {
                    let name: &'static str = Box::leak(name.into_boxed_str());
                    iced::font::set_defaults(Font::new(name), Pixels(14.0))
                }
            },
        }
    }

    fn rebuild_font_list(&mut self) {
        let mut names = self.fount.families();

        // Move pinned fonts to the top (in reverse so first pinned ends up first)
        for &pinned in PINNED_FONTS.iter().rev() {
            if let Some(pos) = names.iter().position(|n| n == pinned) {
                let name = names.remove(pos);
                names.insert(0, name);
            }
        }

        self.font_list = combo_box::State::new(names);
    }

    pub fn view(&self) -> iced::Element<'_, Message, Theme> {
        let branding = svg(svg::Handle::from_memory(LOGO_BYTES))
            .width(24)
            .height(24);

        let font_selector = combo_box(
            &self.font_list,
            "Font...",
            Some(&self.current_font),
            Message::FontSelected,
        )
        .width(160)
        .size(12);

        let theme_icon = if self.dark_mode {
            icon::sun().size(16)
        } else {
            icon::moon().size(16)
        };
        let theme_toggle = button::ghost(theme_icon).on_press(Message::ToggleTheme);

        // Top-level docs-site nav, per `main-nav.tsx` — ghost buttons,
        // size=sm. No active-state highlighting yet; the navigation backend
        // isn't wired (`Message::NavSelect` is a no-op sink in `update`).
        //
        // We pass the label as `&str` rather than `text::small(...)` because
        // `text::small` sets `width(Fill)` (intended for paragraph-style
        // body text), which propagates up through Button's `size_hint`
        // resolution and forces every button to Fill width — adjacent
        // buttons end up sharing a single row's worth of space and the
        // labels run together. `&str` flows through `Component::from(&str)`
        // → `Text::new(s)` (no width set, default Shrink) so each button
        // sizes to its content.
        let nav = row(NavSection::ALL.iter().copied().map(|section| {
            button::ghost(section.label())
                .size(Size::Sm)
                .on_press(Message::NavSelect(section))
                .into()
        }))
        .spacing(0)
        .align_y(iced::Center);

        let header = row![branding, nav, space::horizontal(), font_selector, theme_toggle]
            .spacing(8)
            .padding(16)
            .align_y(iced::Center);

        // Sidebar — values lifted from `.nova/apps/v4/components/docs-sidebar.tsx`,
        // not the registry default. The docs site overrides several values:
        //   --sidebar-menu-width: spacing(56) = 14rem = 224px (not 256)
        //   item height = h-[30px] (not h-8 = 32)
        //   item font = text-[0.8rem] ≈ 13px (not text-sm = 14)
        //   inter-item gap = gap-0.5 = 2px (not gap-0)
        //   active bg = bg-accent (palette.accent, NOT sidebar.accent)
        //   active border = border-accent 1px
        //   hover bg = hover:bg-sidebar-accent (registry default)
        // Both nova radix and docs override agree: data-[active=true]:font-medium.
        let sidebar_header = container(
            text::xs("Components")
                .weight(Weight::Medium)
                .style(theme::text::Muted),
        )
        .height(32)
        .padding([0, 8])
        .align_y(iced::Center);

        // Per-item: 30px height + 13px text + Medium when active, Normal
        // otherwise. Our button defaults to Medium, so the inactive drop
        // is explicit. Items are wrapped in `sidebar::menu` (gap-1, full
        // width) so the suite's column geometry is the one in play; the
        // raw `Button` underneath preserves the docs-faithful 30px/13px/
        // `inset([1, 0])` continuous-hover affordance landed in earlier
        // phases — the suite's own `menu_button` builder targets the base
        // shadcn registry sizes (h-8/text-sm), which the docs site
        // overrides. See the comment block above for the divergence list.
        let sidebar_items = sidebar::menu(self.pages.iter().enumerate().map(|(i, page)| {
            let is_active = i == self.current_page;
            let (style, weight): (StyleFn<'_, Theme>, Weight) = if is_active {
                (theme::button::SidebarActive.into(), Weight::Medium)
            } else {
                (theme::button::Sidebar.into(), Weight::Normal)
            };
            button(page.meta().title)
                .translate(false)
                .size(13.0)
                .weight(weight)
                .style(style)
                .inset([1, 0])
                .interaction(Interaction::Pointer)
                .on_press(Message::SelectPage(i))
                .into()
        }));

        // Atom-system demonstration on the sidebar root: `sidebar::Default`
        // (the flush variant) with `Bg(Token::Background)` overriding the
        // variant's default sidebar bg with the page bg. Mirrors the
        // docs-site `<Sidebar className="bg-transparent ...">` override
        // (`apps/v4/components/docs-sidebar.tsx`) — the docs site renders
        // its sidebar flush with the page rather than as a distinct surface,
        // so we override `palette.sidebar.background` with `palette.background`
        // to match. Cross-widget atom system is exercised on the third
        // catalog (after button + container).
        // padding([top, right, bottom, left]):
        //   top=24px — matches shadcn docs sidebar; the "Components" label
        //             starts ~24px below the header rule, creating visible
        //             breathing room (image 5 reference).
        //   left=24px — shifts content inset so it aligns with the "Docs"
        //              text in the top nav (shadcn's sidebar content is
        //              inset ~8px more than our old left=16 value).
        //   right/bottom=8px — unchanged from prior padding.
        let sidebar = sidebar::fixed(
            scrollable(
                column![sidebar_header, sidebar_items]
                    .padding(iced::Padding::ZERO.top(24).right(8).bottom(8).left(24))
                    .width(224),
            )
            .height(Fill),
        )
        .height(Fill)
        .class(theme::sidebar::Default + Bg(Token::Background));

        // Main content
        let current = &*self.pages[self.current_page];
        let meta = current.meta();
        let title = text::h1(meta.title);
        let description = text::p(meta.description).style(theme::text::Muted);

        let (page_body, page_nav) = page::view(current, &self.snippet_viewer);

        let content = container(
            column![title, description, container(page_body).width(Fill)]
                .spacing(24)
                .padding(32)
                .width(700),
        )
        .center_x(Fill);

        let thin_scrollbar = scrollable::Scrollbar::default().width(1).scroller_width(2);

        let main = scrollable(content)
            .direction(scrollable::Direction::Vertical(thin_scrollbar))
            .height(Fill)
            .id(current.scrollable_id());

        let sidebar_divider = rule::vertical(1).style(Box::new(|theme: &Theme| {
            use iced::gradient::Linear;
            use std::f32::consts::PI;
            let border = theme.palette().border;
            rule::Style {
                background: Linear::new(PI)
                    .add_stop(0.0, iced::Color::TRANSPARENT)
                    .add_stop(0.5, border.into())
                    .add_stop(1.0, iced::Color::TRANSPARENT)
                    .into(),
                ..rule::Style::default()
            }
        }) as rule::StyleFn<'_, Theme>);

        let body = if let Some(nav) = page_nav {
            row![sidebar, sidebar_divider, main, nav]
                .width(Fill)
                .height(Fill)
        } else {
            row![sidebar, sidebar_divider, main]
                .width(Fill)
                .height(Fill)
        };

        column![header, separator::horizontal(), body]
            .width(Fill)
            .height(Fill)
            .into()
    }
}
