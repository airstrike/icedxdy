use iced::clipboard;
use icedxdy::widget::markdown::highlighter::Map;
use icedxdy::widget::{button, markdown};
use icedxdy::{Component, Theme};

use crate::page;

use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    button!["Button"].on_press(Message::Pressed).into()
}

/// Docs snippet + code theme embedded so the standalone preview renders
/// the code block without the async file load the live page performs.
/// The preview viewer renders light, so only `GH_LIGHT` is needed.
const SNIPPET: &str = include_str!("../../../snippets/button-default.md");
const GH_LIGHT: &str = include_str!("../../../themes/github-light-default.json");

/// Backing state for the full-card preview: the parsed snippet and a
/// `SnippetViewer` baked once, plus the "View Code" expand latch.
struct Showcase {
    viewer: page::SnippetViewer,
    snippet: Vec<markdown::Item>,
    expanded: bool,
}

impl Default for Showcase {
    fn default() -> Self {
        let map = Map::from_vscode_theme(GH_LIGHT).expect("vendored github theme parses");
        Self {
            viewer: page::SnippetViewer::new(map, &Theme::light()),
            snippet: markdown::parse(SNIPPET).collect(),
            expanded: false,
        }
    }
}

impl Showcase {
    fn update(&mut self, message: crate::Message) -> iced::Task<crate::Message> {
        if let crate::Message::Page(_, page_msg) = message {
            match page_msg {
                page::Message::ExpandCode(_) => self.expanded = true,
                page::Message::CopyCode(_) => {
                    return clipboard::write(page::extract_code_from_markdown(SNIPPET))
                        .map(|_| crate::Message::Page(0, page::Message::Noop));
                }
                _ => {}
            }
        }
        iced::Task::none()
    }
}

/// The "Default" button example wrapped in its full shadcn card chrome —
/// `h3` heading, 288px preview surface, and the collapsible code block
/// with the "View Code" button — exactly as the Button page lays it out
/// in `examples/browser`. Backed by [`Showcase`] so "View Code" expands
/// live in the granita viewer.
#[granita::preview(group = "Button")]
fn default(state: &Showcase) -> iced::Element<'_, crate::Message, Theme> {
    let preview = view().map(|_| crate::Message::Page(0, page::Message::Noop));
    let code = page::code_panel(
        0,
        0,
        page::snippet(0, &state.snippet, &state.viewer),
        state.expanded,
    );
    page::example_card(1, "Default", None, preview.into(), Some(code))
}
