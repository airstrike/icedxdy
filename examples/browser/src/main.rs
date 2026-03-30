mod component;
mod example;
mod fonts;
mod icon;
mod page;

use iced::widget::{column, combo_box, container, markdown, row, scrollable, sensor, space};
use iced::{Fill, Font, Pixels, Task};

use icedxdy::widget::{button, rule, separator, text};
use icedxdy::{Component, Theme, theme};
use icedxdy::Size::*;

fn is_dark_mode() -> bool {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "Dark")
            .unwrap_or(true) // default to dark
    }
    #[cfg(not(target_os = "macos"))]
    {
        true // default to dark on other platforms
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("icedxdy \u{2022} components browser")
        .antialiasing(true)
        .theme(App::theme)
        .font(icon::FONT)
        .run()
}

/// Height for preview containers (shadcn uses h-72 = 288px)
const PREVIEW_HEIGHT: f32 = 288.0;

/// Fonts to pin at the top of the font list
const PINNED_FONTS: &[&str] = &["Geist", "Inter"];

struct App {
    dark_mode: bool,
    current_page: usize,
    pages: Vec<Box<dyn page::Page>>,
    // Font management
    fount: fount::Fount,
    font_list: combo_box::State<String>,
    current_font: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme,
    SelectPage(usize),
    FontSelected(String),
    Page(usize, page::Event),
    Font(fonts::Message),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let mut pages: Vec<Box<dyn page::Page>> = Vec::new();
        let mut tasks: Vec<Task<Message>> = Vec::new();

        use component::{alert, badge, button, input, text};

        let constructors: Vec<fn(usize) -> Box<dyn page::Page>> = vec![
            button::ButtonPage::new,
            badge::BadgePage::new,
            input::InputPage::new,
            alert::AlertPage::new,
            text::TextPage::new,
        ];

        for (i, ctor) in constructors.into_iter().enumerate() {
            let page = ctor(i);
            tasks.push(page.init());
            pages.push(page);
        }

        let default_font = "Geist".to_string();

        tasks.push(fonts::init().map(Message::Font));

        (
            Self {
                dark_mode: is_dark_mode(),
                current_page: 0,
                pages,
                fount: fount::Fount::new(),
                font_list: combo_box::State::new(vec![default_font.clone()]),
                current_font: default_font,
            },
            Task::batch(tasks),
        )
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

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                Task::none()
            }
            Message::SelectPage(page) => {
                self.current_page = page;
                Task::none()
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
            Message::Page(i, event) => {
                if let Some(page) = self.pages.get_mut(i) {
                    page.update(event)
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

    /// Generic page content: title, description, example cards with previews and code.
    fn view_page(&self) -> Component<'_, Message> {
        let page_index = self.current_page;
        let page = &*self.pages[page_index];
        let meta = page.meta();

        let mut items: Vec<Component<'_, Message>> = Vec::new();

        for (i, example_meta) in meta.examples.iter().enumerate() {
            let preview = container(page.preview(i))
                .center_x(Fill)
                .center_y(Fill)
                .width(Fill)
                .height(PREVIEW_HEIGHT)
                .padding(40)
                .style(theme::container::card_preview);

            let has_snippet = !example_meta.snippet.is_empty();

            let card: Component<'_, Message> = if has_snippet {
                let code: Component<'_, Message> = page
                    .snippets()[i]
                    .as_ref()
                    .map(|s| {
                        let mut style = markdown::Style::from(&iced::Theme::TokyoNight);
                        style.code_block_font = Font::MONOSPACE;

                        let settings = markdown::Settings {
                            code_size: Pixels(13.0),
                            ..markdown::Settings::with_style(style)
                        };

                        markdown::view(s, settings)
                            .map(move |url| Message::Page(page_index, page::Event::LinkClicked(url)))
                            .into()
                    })
                    .unwrap_or_else(|| text("Loading...").into());

                let code = container(code)
                    .padding([12, 16])
                    .style(theme::container::code_block);

                container(column![preview, code])
                    .style(theme::container::card_muted)
                    .width(Fill)
                    .into()
            } else {
                container(preview)
                    .style(theme::container::card_muted)
                    .width(Fill)
                    .into()
            };

            let example_ids = page.example_ids();

            if i == 0 {
                // Demo card: no title header, just the card with sensor
                let with_id = container(card).id(example_ids[i].clone());
                let sensored: Component<'_, Message> = sensor(with_id)
                    .on_show(move |_| Message::Page(page_index, page::Event::ExampleShown(i)))
                    .on_hide(Message::Page(page_index, page::Event::ExampleHidden(i)))
                    .anticipate(50)
                    .into();
                items.push(sensored);
            } else {
                if i == 1 {
                    let heading: Component<'_, Message> = text::h2("Examples").into();
                    items.push(heading);
                }

                let mut section = column![text::h3(example_meta.title)].spacing(16);

                if let Some(desc) = example_meta.description {
                    section = section.push(text::p(desc).style(theme::text::muted));
                }

                let section_card: Component<'_, Message> = section.push(card).spacing(16).into();

                let with_id = container(section_card).id(example_ids[i].clone());
                let sensored: Component<'_, Message> = sensor(with_id)
                    .on_show(move |_| Message::Page(page_index, page::Event::ExampleShown(i)))
                    .on_hide(Message::Page(page_index, page::Event::ExampleHidden(i)))
                    .anticipate(50)
                    .into();
                items.push(sensored);
            }
        }

        icedxdy::widget::column(items)
            .spacing(48)
            .max_width(640.0)
            .width(Fill)
            .into()
    }

    /// "On This Page" navigation panel.
    fn on_this_page(&self) -> Component<'_, Message> {
        let page_index = self.current_page;
        let page = &*self.pages[page_index];
        let meta = page.meta();
        let visible = page.visible();

        let count = visible.len();
        let alpha_for = |i: usize| -> f32 {
            if count == 0 || !visible.contains(&i) {
                return 0.25;
            }
            let rank = visible.iter().position(|&v| v == i).unwrap();
            let center = (count - 1) as f32 / 2.0;
            let distance = (rank as f32 - center).abs() / center.max(1.0);
            1.0 - 0.5 * distance
        };

        // Skip the first example (demo) in the ToC
        let nav_items = meta.examples.iter().enumerate().skip(1).map(|(i, example)| {
            let alpha = alpha_for(i);
            use theme::button::{Status, Style};
            let toc_style = move |theme: &Theme, status: Status| {
                let p = theme.palette();
                let base_color = match status {
                    Status::Active => p.foreground.alpha(alpha),
                    _ => p.foreground,
                };
                Style {
                    background: None,
                    text_color: base_color.into(),
                    ..Style::default()
                }
            };
            button(example.title)
                .size(Sm)
                .translate(false)
                .style(toc_style)
                .on_press(Message::Page(page_index, page::Event::ScrollTo(i)))
                .into()
        });

        let header = container(text::muted("On This Page")).padding([0, 8]);

        column![header]
            .push(column(nav_items).spacing(2))
            .spacing(16)
            .padding([24, 8])
            .width(200)
            .into()
    }

    fn view(&self) -> iced::Element<'_, Message, Theme> {
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
            let style: fn(&Theme, _) -> _ = if is_active {
                theme::button::sidebar_active
            } else {
                theme::button::sidebar
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
        let description = text::p(meta.description).style(theme::text::muted);

        let preview = container(self.view_page()).width(Fill);

        let content = container(
            column![title, description, preview]
                .spacing(24)
                .padding(32)
                .width(600),
        )
        .center_x(Fill);

        let thin_scrollbar = scrollable::Scrollbar::default().width(1).scroller_width(2);

        let main = scrollable(content)
            .direction(scrollable::Direction::Vertical(thin_scrollbar))
            .height(Fill)
            .id(current.scrollable_id());

        let sidebar_divider = rule::vertical(1).style(|theme: &Theme| {
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
        });

        // Show "On This Page" panel if there are more than 1 example
        let body = if meta.examples.len() > 1 {
            let on_this_page = self.on_this_page();
            row![sidebar, sidebar_divider, main, on_this_page]
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

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::dark()
        } else {
            Theme::light()
        }
    }
}
