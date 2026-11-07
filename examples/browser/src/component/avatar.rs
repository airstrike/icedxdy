// API Reference is rendered via the `avatar-api-reference.md` snippet at
// the bottom of BLOCKS. Live docs API Reference covers `Avatar.size`
// (`Default` / `Sm` / `Lg`, default `Default`) plus the `AvatarImage`,
// `AvatarFallback`, `AvatarBadge`, `AvatarGroup`, and `AvatarGroupCount`
// sub-components. The snippet captures the icedxdy-shaped builder API for
// `avatar(...)`, `avatar::badge(...)`, `avatar::group(...)`, plus the
// image-load-failure caveat.
//
// Intentional skips:
// - Composition preamble (live-docs section between Usage and Examples) —
//   it's an ASCII tree of `Avatar`/`AvatarGroup` subcomponent shapes
//   (`Avatar ├── AvatarImage ├── AvatarFallback └── AvatarBadge`, etc.).
//   Pure docs scaffolding; nothing to render as a `Block::Example`. Will fold
//   in once a `Block::Md` (or equivalent prose block) primitive lands.
// - avatar-dropdown — needs a dropdown-menu primitive (not yet ported).
// - avatar-rtl — RTL is not in scope (per CLAUDE.md).
//
// Avatars are bundled as static PNGs under `examples/browser/assets/avatars/`
// and surfaced through small `*_handle()` helpers below. Snippets reference
// these same helpers verbatim so users can copy-paste the rendered example
// directly. iced has no URL fetcher; `include_bytes!` is the canonical path.

use std::sync::OnceLock;

use iced::Task;
use iced::widget::image;

use icedxdy::Theme;
use icedxdy::widget::{avatar, row};

use crate::Message;
use crate::icon;
use crate::page;

/// Andy Terra's GitHub avatar (the icedxdy author).
fn airstrike_handle() -> image::Handle {
    static H: OnceLock<image::Handle> = OnceLock::new();
    static BYTES: &[u8] = include_bytes!("../../assets/avatars/airstrike.png");
    H.get_or_init(|| image::Handle::from_bytes(BYTES)).clone()
}

/// Héctor Ramón's GitHub avatar (iced's creator).
fn hecrj_handle() -> image::Handle {
    static H: OnceLock<image::Handle> = OnceLock::new();
    static BYTES: &[u8] = include_bytes!("../../assets/avatars/hecrj.png");
    H.get_or_init(|| image::Handle::from_bytes(BYTES)).clone()
}

/// The iced framework's snowflake mark (iced-rs org avatar).
fn iced_handle() -> image::Handle {
    static H: OnceLock<image::Handle> = OnceLock::new();
    static BYTES: &[u8] = include_bytes!("../../assets/avatars/iced.png");
    H.get_or_init(|| image::Handle::from_bytes(BYTES)).clone()
}

pub static META: page::Meta = page::Meta {
    title: "Avatar",
    description: "An image element with a fallback for representing the user.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/avatar-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A basic avatar component with an image and a fallback."),
                snippet: "examples/browser/snippets/avatar-basic.md",
            },
            page::Block::Example {
                id: "badge",
                title: "Badge",
                description: Some(
                    "Use the AvatarBadge component to add a badge to the avatar. The badge is positioned at the bottom right of the avatar.",
                ),
                snippet: "examples/browser/snippets/avatar-badge.md",
            },
            page::Block::Example {
                id: "badge-icon",
                title: "Badge with Icon",
                description: Some("You can also use an icon inside <AvatarBadge>."),
                snippet: "examples/browser/snippets/avatar-badge-icon.md",
            },
            page::Block::Example {
                id: "group",
                title: "Avatar Group",
                description: Some("Use the AvatarGroup component to add a group of avatars."),
                snippet: "examples/browser/snippets/avatar-group.md",
            },
            page::Block::Example {
                id: "group-count",
                title: "Avatar Group Count",
                description: Some("Use <AvatarGroupCount> to add a count to the group."),
                snippet: "examples/browser/snippets/avatar-group-count.md",
            },
            page::Block::Example {
                id: "group-count-icon",
                title: "Avatar Group with Icon",
                description: Some("You can also use an icon inside <AvatarGroupCount>."),
                snippet: "examples/browser/snippets/avatar-group-count-icon.md",
            },
            page::Block::Example {
                id: "size",
                title: "Sizes",
                description: Some("Use the size prop to change the size of the avatar."),
                snippet: "examples/browser/snippets/avatar-size.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/avatar-api-reference.md"),
];

pub struct AvatarPage {
    state: page::State,
}

impl AvatarPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for AvatarPage {
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

    /// Each example wires a real `image::Handle` (PNGs bundled under
    /// `examples/browser/assets/avatars/` via `include_bytes!`). The
    /// fallback initials still match each persona — they only render if
    /// decoding fails, which mirrors the image-load-failure deviation
    /// documented on the avatar widget itself.
    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // Hero (avatar-demo.tsx): three avatars in a row at gap-6 (24px),
            // demonstrating the basic avatar, an avatar with a status badge,
            // and a group with a +3 count.
            "demo" => row![
                avatar(Some(airstrike_handle())).fallback("AT"),
                avatar::badge(avatar(Some(hecrj_handle())).fallback("HR"), avatar::dot(),),
                avatar::group([
                    avatar(Some(airstrike_handle())).fallback("AT"),
                    avatar(Some(hecrj_handle())).fallback("HR"),
                    avatar(Some(iced_handle())).fallback("IC"),
                ])
                .count("+3"),
            ]
            .spacing(24)
            .align_y(iced::Center)
            .into(),

            // Basic (avatar-basic.tsx): single avatar — image + fallback.
            "basic" => avatar(Some(airstrike_handle())).fallback("AT").into(),

            // Badge (avatar-badge.tsx): avatar + corner status dot.
            "badge" => {
                avatar::badge(avatar(Some(hecrj_handle())).fallback("HR"), avatar::dot()).into()
            }

            // Badge with Icon (avatar-badge-icon.tsx): avatar + corner badge
            // containing a `+` icon (lucide PlusIcon).
            "badge-icon" => {
                avatar::badge(avatar(Some(iced_handle())).fallback("IC"), icon::plus()).into()
            }

            // Avatar Group (avatar-group.tsx): three overlapping avatars.
            "group" => avatar::group([
                avatar(Some(airstrike_handle())).fallback("AT"),
                avatar(Some(hecrj_handle())).fallback("HR"),
                avatar(Some(iced_handle())).fallback("IC"),
            ])
            .into(),

            // Avatar Group Count (avatar-group-count.tsx): three avatars +
            // a `+3` text count chip on the right.
            "group-count" => avatar::group([
                avatar(Some(airstrike_handle())).fallback("AT"),
                avatar(Some(hecrj_handle())).fallback("HR"),
                avatar(Some(iced_handle())).fallback("IC"),
            ])
            .count("+3")
            .into(),

            // Avatar Group with Icon (avatar-group-count-icon.tsx): three
            // avatars + a `+` icon count chip on the right.
            "group-count-icon" => avatar::group([
                avatar(Some(airstrike_handle())).fallback("AT"),
                avatar(Some(hecrj_handle())).fallback("HR"),
                avatar(Some(iced_handle())).fallback("IC"),
            ])
            .count(icon::plus())
            .into(),

            // Sizes (avatar-size.tsx): three avatars at sm / default / lg in
            // a row at gap-2 (8px). Same persona at three sizes mirrors the
            // shadcn example (which uses the same avatar three times).
            "size" => row![
                avatar(Some(airstrike_handle()))
                    .fallback("AT")
                    .size(avatar::Sm),
                avatar(Some(airstrike_handle())).fallback("AT"),
                avatar(Some(airstrike_handle()))
                    .fallback("AT")
                    .size(avatar::Lg),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into(),

            _ => unreachable!("unknown avatar preview id: {id}"),
        }
    }
}
