// API Reference is rendered via the `aspect-ratio-api-reference.md` snippet
// at the bottom of BLOCKS. The live docs API Reference covers `ratio`
// (`f32`, required) plus styling props that our snippet captures via the
// icedxdy-shaped builder API (`.width(...)`, `.max_width(...)`, ratio
// fallback semantics).
//
// Intentional skips:
// - aspect-ratio-rtl — RTL is not in scope (per CLAUDE.md).

use iced::Task;

use icedxdy::core::{Border, Color};
use icedxdy::theme::container::Style;
use icedxdy::widget::{aspect_ratio, container};
use icedxdy::{Theme, radius};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Aspect Ratio",
    description: "Displays content within a desired ratio.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/aspect-ratio-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "square",
                title: "Square",
                description: Some(
                    "A square aspect ratio component using a `1 / 1` ratio. \
                     This is useful for displaying images in a square format.",
                ),
                snippet: "examples/browser/snippets/aspect-ratio-square.md",
            },
            page::Block::Example {
                id: "portrait",
                title: "Portrait",
                description: Some(
                    "A portrait aspect ratio component using a `9 / 16` ratio. \
                     This is useful for displaying images in a portrait format.",
                ),
                snippet: "examples/browser/snippets/aspect-ratio-portrait.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/aspect-ratio-api-reference.md"),
];

pub struct AspectRatioPage {
    state: page::State,
}

impl AspectRatioPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// Stand-in for the shadcn next/image shown inside the AspectRatio. iced has
/// no built-in URL-fetcher and the project ships no checked-in raster
/// asset, so the demo uses a `bg-muted` rounded fill — visually equivalent
/// to shadcn's grayscale placeholder against the muted background card,
/// just without the photographic content. The geometry — what the example
/// is actually demonstrating — is unaffected.
fn image_placeholder<'a>() -> iced::Element<'a, Message, Theme> {
    container("")
        .width(iced::Fill)
        .height(iced::Fill)
        .style(|theme: &Theme| Style {
            background: Some(theme.muted.base.into()),
            border: Border {
                radius: radius::LG.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..Style::default()
        })
        .into()
}

impl page::Page for AspectRatioPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // Demo (16:9). shadcn `aspect-ratio-demo.tsx`: max-w-sm
            // (`24rem` = 384px) cap on the AspectRatio's parent. We apply
            // it directly via `aspect_ratio.max_width(...)` — see the
            // widget docs for why a wrapping `container(...).max_width(...)`
            // is the wrong shape (Shrink-width parent triggers
            // width-compression and collapses the child).
            "demo" => aspect_ratio(image_placeholder(), 16.0 / 9.0)
                .max_width(384.0)
                .into(),

            // Portrait (9:16). shadcn `aspect-ratio-portrait.tsx`:
            // max-w-[10rem] = 160px.
            "portrait" => aspect_ratio(image_placeholder(), 9.0 / 16.0)
                .max_width(160.0)
                .into(),

            // Square (1:1). shadcn `aspect-ratio-square.tsx`:
            // max-w-[12rem] = 192px.
            "square" => aspect_ratio(image_placeholder(), 1.0)
                .max_width(192.0)
                .into(),

            _ => unreachable!("unknown aspect_ratio preview id: {id}"),
        }
    }
}
