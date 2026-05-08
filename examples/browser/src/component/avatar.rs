// TODO: API Reference section once a table widget lands.
//
// Live docs API Reference covers: `Avatar.size` (`"default" | "sm" | "lg"`,
// default `"default"`), `Avatar.className`, `AvatarImage.src/alt/className`,
// `AvatarFallback.className`, `AvatarBadge.className`, `AvatarGroup.className`,
// `AvatarGroupCount.className`. Captured here so the gap is auditable when
// the table widget lands.
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
// Snippet-vs-page asymmetry:
// - Snippets reference aspirational `shadcn_handle()` / `evilrabbit_handle()`
//   / `maxleiter_handle()` / `pranathip_handle()` helpers (mirroring the
//   shadcn .tsx imports) so the snippet reads as the canonical shape a user
//   would copy-paste. The rendered page passes `None` to `avatar(...)` and
//   relies on the fallback — see the `preview()` docstring and the avatar
//   widget docstring for the image-load-failure deviation that motivates it.

use iced::Task;

use icedxdy::widget::{avatar, row};
use icedxdy::Theme;

use crate::Message;
use crate::icon;
use crate::page;

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

    /// Per the avatar module docstring: iced has no URL fetcher and the
    /// project ships no checked-in raster asset, so every example here passes
    /// `None` to [`avatar`] and lets the fallback (initials) render. This
    /// faithfully demonstrates the fallback path; the image-rendering path
    /// itself remains exercised by callers who hand in their own
    /// `image::Handle`.
    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // Hero (avatar-demo.tsx): three avatars in a row at gap-6 (24px),
            // demonstrating the basic avatar, an avatar with a status badge,
            // and a group with a +3 count.
            "demo" => row![
                avatar(None).fallback("CN"),
                avatar::badge(avatar(None).fallback("ER"), avatar::dot()),
                avatar::group([
                    avatar(None).fallback("CN"),
                    avatar(None).fallback("LR"),
                    avatar(None).fallback("ER"),
                ])
                .count("+3"),
            ]
            .spacing(24)
            .align_y(iced::Center)
            .into(),

            // Basic (avatar-basic.tsx): single avatar — image + fallback.
            // Image source is `None` per the page's image-asset deviation;
            // the fallback "CN" carries the example.
            "basic" => avatar(None).fallback("CN").into(),

            // Badge (avatar-badge.tsx): avatar + corner status dot.
            "badge" => avatar::badge(avatar(None).fallback("CN"), avatar::dot()).into(),

            // Badge with Icon (avatar-badge-icon.tsx): avatar + corner badge
            // containing a `+` icon (lucide PlusIcon).
            "badge-icon" => {
                avatar::badge(avatar(None).fallback("PP"), icon::plus()).into()
            }

            // Avatar Group (avatar-group.tsx): three overlapping avatars.
            "group" => avatar::group([
                avatar(None).fallback("CN"),
                avatar(None).fallback("LR"),
                avatar(None).fallback("ER"),
            ])
            .into(),

            // Avatar Group Count (avatar-group-count.tsx): three avatars +
            // a `+3` text count chip on the right.
            "group-count" => avatar::group([
                avatar(None).fallback("CN"),
                avatar(None).fallback("LR"),
                avatar(None).fallback("ER"),
            ])
            .count("+3")
            .into(),

            // Avatar Group with Icon (avatar-group-count-icon.tsx): three
            // avatars + a `+` icon count chip on the right.
            "group-count-icon" => avatar::group([
                avatar(None).fallback("CN"),
                avatar(None).fallback("LR"),
                avatar(None).fallback("ER"),
            ])
            .count(icon::plus())
            .into(),

            // Sizes (avatar-size.tsx): three avatars at sm / default / lg in
            // a row at gap-2 (8px).
            "size" => row![
                avatar(None).fallback("CN").size(avatar::Sm),
                avatar(None).fallback("CN"),
                avatar(None).fallback("CN").size(avatar::Lg),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into(),

            _ => unreachable!("unknown avatar preview id: {id}"),
        }
    }
}
