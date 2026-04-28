use iced::widget::{column, combo_box, row, scrollable, space};
use iced_widget::button::StyleFn;

use iced::{Fill, Font, Pixels, Subscription, Task};
use icedxdy::widget::container;
use icedxdy::widget::markdown::highlighter::Map;

use icedxdy::Size::*;
use icedxdy::widget::{button, rule, separator, text};
use icedxdy::{Theme, theme};

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

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme,
    SelectPage(usize),
    FontSelected(String),
    Page(usize, page::Message),
    Font(fonts::Message),
}

impl App {
    pub fn new(gh_light: Map, gh_dark: Map) -> (Self, Task<Message>) {
        let mut pages: Vec<Box<dyn page::Page>> = Vec::new();
        let mut tasks: Vec<Task<Message>> = Vec::new();

        use component::{alert, badge, button, input, progress, text};

        let constructors: Vec<fn(usize) -> Box<dyn page::Page>> = vec![
            button::ButtonPage::new,
            badge::BadgePage::new,
            input::InputPage::new,
            progress::ProgressPage::new,
            alert::AlertPage::new,
            text::TextPage::new,
        ];

        for (i, ctor) in constructors.into_iter().enumerate() {
            let page = ctor(i);
            tasks.push(page.init());
            pages.push(page);
        }

        let default_font = "Geist".to_string();

        let dark_mode = theme::is_dark_mode();
        let snippet_viewer = SnippetViewer::with_map(if dark_mode {
            gh_dark.clone()
        } else {
            gh_light.clone()
        });

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
                self.snippet_viewer = SnippetViewer::with_map(map);
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
        let branding = text::h4("icedxdy");

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

        let header = row![branding, space::horizontal(), font_selector, theme_toggle]
            .spacing(8)
            .padding(16)
            .align_y(iced::Center);

        // Sidebar
        let sidebar_header = container(text::muted("Components")).padding([0, 8]);

        let sidebar_items = column(self.pages.iter().enumerate().map(|(i, page)| {
            let is_active = i == self.current_page;
            let style: StyleFn<'_, Theme> = if is_active {
                theme::button::SidebarActive.into()
            } else {
                theme::button::Sidebar.into()
            };
            button(page.meta().title)
                .size(Sm)
                .translate(false)
                .style(style)
                .on_press(Message::SelectPage(i))
                .into()
        }))
        .spacing(2);

        let sidebar = scrollable(
            column![sidebar_header, sidebar_items]
                .spacing(16)
                .padding([24, 13])
                .width(150),
        )
        .height(Fill);

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
