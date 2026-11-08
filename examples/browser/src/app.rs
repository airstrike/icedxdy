use iced::Element;
use iced::Widget;
use iced::mouse::Interaction;
use iced::widget::{column, combo_box, row, space, stack, svg};
use icedxdy::widget::scrollable;

use iced::{Center, Fill, Font, Pixels, Subscription, Task};
use icedxdy::theme::tweak::Bg;
use icedxdy::theme::tweak::token::Token;
use icedxdy::widget::container;
use icedxdy::widget::markdown::highlighter::Map;
use icedxdy::widget::sidebar::MenuButtonSize;

use icedxdy::widget::{button, rule, separator, sidebar, text};
use icedxdy::{Size, Theme, Weight, theme};

const LOGO_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/logo.svg"
));

use crate::component;
use crate::fonts;
use crate::icon;
use crate::page::{self, SnippetViewer};

const PINNED_FONTS: &[&str] = &["Geist", "Inter"];

pub struct App {
    dark_mode: bool,
    current_page: usize,
    pages: Vec<Box<dyn page::Page>>,
    gh_light: Map,
    gh_dark: Map,
    snippet_viewer: SnippetViewer,
    fount: fount::Fount,
    font_list: combo_box::State<String>,
    current_font: String,
}

/// No real routing yet — `Message::NavSelect` is a no-op sink in `update`,
/// preserving the visual affordance until the navigation backend lands.
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
    /// `App` ignores; `Browser` intercepts the `value == 100` case to dismiss
    /// the splash so it stays up until the bar visibly reaches the right edge.
    SplashIdle(f32),
}

impl App {
    pub fn new(gh_light: Map, gh_dark: Map) -> (Self, Task<Message>) {
        let mut pages: Vec<Box<dyn page::Page>> = Vec::new();
        let mut tasks: Vec<Task<Message>> = Vec::new();

        use component::{
            accordion, alert, aspect_ratio, avatar, badge, breadcrumb, button, button_group, card,
            checkbox, collapsible, dialog, empty, field, input, item, kbd, label, native_select,
            pagination, progress, radio_group, separator, sheet, sidebar, skeleton, slider,
            spinner, switch, table, tabs, text, toggle, toggle_group, tooltip,
        };

        let constructors: Vec<fn(usize) -> Box<dyn page::Page>> = vec![
            accordion::AccordionPage::new,
            alert::AlertPage::new,
            aspect_ratio::AspectRatioPage::new,
            avatar::AvatarPage::new,
            badge::BadgePage::new,
            breadcrumb::BreadcrumbPage::new,
            button::ButtonPage::new,
            button_group::ButtonGroupPage::new,
            card::CardPage::new,
            checkbox::CheckboxPage::new,
            collapsible::CollapsiblePage::new,
            dialog::DialogPage::new,
            empty::EmptyPage::new,
            field::FieldPage::new,
            input::InputPage::new,
            item::ItemPage::new,
            kbd::KbdPage::new,
            label::LabelPage::new,
            native_select::NativeSelectPage::new,
            pagination::PaginationPage::new,
            progress::ProgressPage::new,
            radio_group::RadioGroupPage::new,
            separator::SeparatorPage::new,
            sheet::SheetPage::new,
            sidebar::SidebarPage::new,
            skeleton::SkeletonPage::new,
            slider::SliderPage::new,
            spinner::SpinnerPage::new,
            switch::SwitchPage::new,
            table::TablePage::new,
            tabs::TabsPage::new,
            text::TextPage::new,
            toggle::TogglePage::new,
            toggle_group::ToggleGroupPage::new,
            tooltip::TooltipPage::new,
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
            &active_theme,
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
                self.snippet_viewer = SnippetViewer::new(map, &active_theme);
                Task::none()
            }
            Message::SelectPage(page) => {
                if page != self.current_page
                    && let Some(prev) = self.pages.get_mut(self.current_page)
                {
                    prev.reset_volatile();
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

        // Reverse iteration so the first pinned font ends up first.
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
        let theme_toggle = button(theme_icon)
            .style(button::Ghost)
            .square()
            .on_press(Message::ToggleTheme);

        let nav = row(NavSection::ALL.iter().copied().map(|section| {
            Element::from(
                button(section.label())
                    .style(button::Ghost)
                    .size(Size::Sm)
                    .on_press(Message::NavSelect(section)),
            )
        }))
        .spacing(0)
        .align_y(Center);

        let header = row![
            branding,
            nav,
            space::horizontal(),
            font_selector,
            Element::from(theme_toggle)
        ]
        .spacing(8)
        .padding(16)
        .align_y(Center);

        let sidebar_header = container(
            text::xs("Components")
                .weight(Weight::Medium)
                .style(theme::text::Muted),
        )
        .height(32)
        .padding([0, 8])
        .align_y(Center);

        let sidebar_items = column(self.pages.iter().enumerate().map(|(i, page)| {
            Element::from(
                sidebar::menu_button(page.meta().title)
                    .size(MenuButtonSize::Docs)
                    .inset(1)
                    .active(i == self.current_page)
                    .on_press(Message::SelectPage(i)),
            )
        }))
        .width(Fill);

        // padding-left=49px aligns item text with "Docs" in the navbar:
        // header(16) + branding(24) + row_spacing(8) + button_pad_l(10)
        //   - inset(1) - item_pad_l(8) = 49.
        let sidebar_scrollable = scrollable::scrollable(
            column![Element::from(sidebar_header), sidebar_items]
                .padding(iced::Padding::ZERO.top(24).right(8).bottom(8).left(49))
                .width(272),
        )
        .height(Fill);

        // Top fade-to-bg overlay (`apps/v4/components/docs-sidebar.tsx`):
        // `h-8 from-bg via-bg/80 to-bg/50`. Pinned to top via column+space.
        // Layered over the scrollable so wheel events pass through — the
        // container has no on_press, so its `mouse_interaction` returns
        // None and `stack!` levitates the cursor through.
        let top_fade = container(space::vertical())
            .width(Fill)
            .height(32)
            .style(|t: &Theme| {
                use iced::gradient::Linear;
                use std::f32::consts::PI;
                use theme::container::Style;
                let bg: iced::Color = t.neutral.base.into();
                Style {
                    background: Some(
                        Linear::new(PI)
                            .add_stop(0.0, bg)
                            .add_stop(0.5, iced::Color { a: 0.8, ..bg })
                            .add_stop(1.0, iced::Color { a: 0.5, ..bg })
                            .into(),
                    ),
                    ..Style::default()
                }
            });
        let fade_overlay = column![Element::from(top_fade), space::vertical()]
            .width(Fill)
            .height(Fill);

        let sidebar = sidebar::fixed(
            stack![Element::from(sidebar_scrollable), fade_overlay]
                .width(272)
                .height(Fill),
        )
        .height(Fill)
        .class(theme::sidebar::Default + Bg(Token::Background));

        let current = &*self.pages[self.current_page];
        let meta = current.meta();
        let title = text::h1(meta.title);
        let description = text::p(meta.description).style(theme::text::Muted);

        // Anchor-style controls: pointer cursor + no press translate (matches
        // shadcn's `<Link>`-wrapped `<Button variant="secondary" size="icon">`
        // at `apps/v4/app/(app)/docs/[[...slug]]/page.tsx`). SelectPage with
        // a same-index target is a no-op, so saturating arithmetic at the
        // edges replaces a separate disabled state.
        // TODO: Copy Page button (needs DropdownMenu + clipboard write).
        let total = self.pages.len();
        let curr = self.current_page;
        let prev_btn = button(icon::arrow_left())
            .style(button::Secondary)
            .size(Size::Sm)
            .square()
            .translate(false)
            .interaction(Interaction::Pointer)
            .on_press(Message::SelectPage(curr.saturating_sub(1)));
        let next_btn = button(icon::arrow_right())
            .style(button::Secondary)
            .size(Size::Sm)
            .square()
            .translate(false)
            .interaction(Interaction::Pointer)
            .on_press(Message::SelectPage((curr + 1).min(total.saturating_sub(1))));

        let title_row = row![
            Element::from(title),
            space::horizontal(),
            row![Element::from(prev_btn), Element::from(next_btn)].spacing(8),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Start);
        let page_header = column![title_row, Element::from(description)].spacing(8);

        let (page_body, page_nav) = page::view(current, &self.snippet_viewer);

        let content = container(
            column![page_header, Element::from(container(page_body).width(Fill))]
                .spacing(24)
                .padding(32)
                .width(700),
        )
        .center_x(Fill);

        let main = scrollable::scrollable(content)
            .width(Fill)
            .height(Fill)
            .id(current.scrollable_id());

        let sidebar_divider = rule::vertical(1).style(Box::new(|theme: &Theme| {
            use iced::gradient::Linear;
            use std::f32::consts::PI;
            let border = theme.neutral.border;
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
            row![
                Element::from(sidebar),
                sidebar_divider,
                Element::from(main),
                Element::from(nav)
            ]
            .width(Fill)
            .height(Fill)
        } else {
            row![Element::from(sidebar), sidebar_divider, Element::from(main)]
                .width(Fill)
                .height(Fill)
        };

        column![header, Element::from(separator()), body]
            .width(Fill)
            .height(Fill)
            .boxed()
    }
}
