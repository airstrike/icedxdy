//! Button Group component page.
//!
//! 5 of 11 shadcn examples ported. Live docs page:
//! <https://ui.shadcn.com/docs/components/radix/button-group>
//!
//! # Intentional skips
//!
//! Per the radix-batch-2 minimap (Phase 5) the compound examples that
//! depend on primitives we haven't ported yet are deferred:
//!
//! - `button-group-split` — same primitive surface that `button-group-separator`
//!   already exercises; revisit when there's a meaningfully distinct shape
//!   to demonstrate.
//! - `button-group-input` — needs an `Input` integrated into the group
//!   so the field shares the cluster's seam radius. Out of scope until
//!   button_group integrates non-button items.
//! - `button-group-input-group` — needs `InputGroup` (not yet ported).
//! - `button-group-dropdown` — needs `DropdownMenu` (not yet ported).
//! - `button-group-select` — needs `Select` (not yet ported).
//! - `button-group-popover` — needs `Popover` + `Textarea` (not yet ported).
//!
//! The hero (`Demo`) example is rendered without the `MoreHorizontal`
//! drop-down since `DropdownMenu` is out of scope; the icon button stays
//! but doesn't open a menu — the visual cluster still demonstrates a
//! shadcn-faithful 5-button group composed of 3 nested sub-clusters.
//!
//! The `Nested` example's second sub-group wraps an `<InputGroup>` in
//! the original; since `InputGroup` isn't ported yet, the second
//! sub-group is two outline buttons instead — keeps the visual
//! demonstration of "outer group separates sub-clusters with 8px gap"
//! intact.
//!
//! # Animation note
//!
//! shadcn's `button.tsx` carries `transition-all`, but the edge-radius
//! collapse the group performs is static layout (driven by sibling
//! position, not state) — there is no transition into adjacent slots
//! on hover/focus, matching shadcn's behavior. Hover/focus animation
//! still happens at the per-button level via the button widget itself.
//!
//! # API Reference
//!
//! Rendered via the `button-group-api-reference.md` snippet at the bottom
//! of BLOCKS. Live docs covers `ButtonGroup.orientation`,
//! `ButtonGroupSeparator.orientation` (auto-derived from the surrounding
//! group in our port), and `ButtonGroupText.asChild` (not yet ported — no
//! examples in the 5-example slice depend on it). The snippet captures
//! the icedxdy-shaped builder API and the typed-children rationale
//! (style mutation requires the typed `Item` enum, not erased elements).

use iced::Task;

use icedxdy::widget::{button, button_group, column, row};
use icedxdy::{Component, Theme};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Button Group",
    description: "A container that groups related buttons together with consistent styling.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/button-group-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "orientation",
                title: "Orientation",
                description: Some(
                    "Set the orientation prop to change the button group layout.",
                ),
                snippet: "examples/browser/snippets/button-group-orientation.md",
            },
            page::Block::Example {
                id: "size",
                title: "Size",
                description: Some(
                    "Control the size of buttons using the size prop on individual buttons.",
                ),
                snippet: "examples/browser/snippets/button-group-size.md",
            },
            page::Block::Example {
                id: "nested",
                title: "Nested",
                description: Some(
                    "Nest ButtonGroup components to create button groups with spacing.",
                ),
                snippet: "examples/browser/snippets/button-group-nested.md",
            },
            page::Block::Example {
                id: "separator",
                title: "Separator",
                description: Some(
                    "The ButtonGroupSeparator component visually divides buttons \
                     within a group. Buttons with variant outline do not need a \
                     separator since they have a border. For other variants, a \
                     separator is recommended to improve the visual hierarchy.",
                ),
                snippet: "examples/browser/snippets/button-group-separator.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/button-group-api-reference.md"),
];

pub struct ButtonGroupPage {
    state: page::State,
}

impl ButtonGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for ButtonGroupPage {
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
        view(id).into_iced()
    }
}

fn view(id: &str) -> Component<'static, Message> {
    use icedxdy::Size::{Lg, Sm};

    match id {
        // Demo (hero). shadcn `button-group-demo.tsx` nests three child
        // `<ButtonGroup>` elements inside an outer one to produce three
        // visual clusters with 8px spacing between them. The third cluster
        // ends with a `<DropdownMenu>` trigger styled as an icon button —
        // we render the icon button without the menu since DropdownMenu
        // isn't ported yet (see module docstring for the deferred-examples
        // list).
        "demo" => row![
            button_group([button::icon![arrow_left].into()]),
            button_group([
                button::outline("Archive").into(),
                button::outline("Report").into(),
            ]),
            button_group([
                button::outline("Snooze").into(),
                button::icon![ellipsis].into(),
            ]),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .into(),

        // Orientation. shadcn `button-group-orientation.tsx`: vertical
        // group of two icon buttons (`<PlusIcon>`, `<MinusIcon>`).
        "orientation" => button_group([
            button::icon![plus].into(),
            button::icon![minus].into(),
        ])
        .orientation(button_group::Vertical)
        .into(),

        // Size. shadcn `button-group-size.tsx`: three groups stacked
        // vertically, each with three text buttons + one icon button.
        // Sizes: sm / default / lg respectively. icon-sm/icon/icon-lg map
        // to a `square()` icon button at the matching text size, since our
        // Size enum drives both the height and the icon-cell width.
        "size" => column([
            button_group([
                button::outline("Small").size(Sm).into(),
                button::outline("Button").size(Sm).into(),
                button::outline("Group").size(Sm).into(),
                button::icon![plus].size(Sm).into(),
            ])
            .into(),
            button_group([
                button::outline("Default").into(),
                button::outline("Button").into(),
                button::outline("Group").into(),
                button::icon![plus].into(),
            ])
            .into(),
            button_group([
                button::outline("Large").size(Lg).into(),
                button::outline("Button").size(Lg).into(),
                button::outline("Group").size(Lg).into(),
                button::icon![plus].size(Lg).into(),
            ])
            .into(),
        ])
        .spacing(32)
        .align_x(iced::Alignment::Start)
        .into(),

        // Nested. shadcn `button-group-nested.tsx`: an outer `<ButtonGroup>`
        // containing two `<ButtonGroup>` children, demonstrating the 8px
        // gap shadcn applies via `has-[>[data-slot=button-group]]:gap-2`.
        // We approximate the cascade with an explicit `row![...].spacing(8)`
        // since iced has no parent-keyed sibling selector. See module
        // docstring for the `<InputGroup>` degradation in the second
        // sub-group.
        "nested" => row![
            button_group([button::icon![plus].into()]),
            button_group([
                button::outline("Add").into(),
                button::outline("Remove").into(),
            ]),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .into(),

        // Separator. shadcn `button-group-separator.tsx`: two secondary
        // buttons with a `<ButtonGroupSeparator>` between them.
        "separator" => button_group([
            button::secondary("Copy").size(Sm).into(),
            button_group::separator(),
            button::secondary("Paste").size(Sm).into(),
        ])
        .into(),

        _ => unreachable!("unknown button_group preview id: {id}"),
    }
}
