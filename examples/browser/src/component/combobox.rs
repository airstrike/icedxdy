// API Reference is rendered via the `combobox-api-reference.md` snippet at
// the bottom of BLOCKS.
//
// Intentional skips:
// - combobox-rtl: RTL is not in scope (per CLAUDE.md).
// - combobox-multiple (chips), -clear, -groups, -auto-highlight,
//   -input-group, -popup, -custom, -invalid: need chips, a clear button,
//   grouped items, keyboard highlighting or a custom trigger; not ported.

use iced::Task;

use icedxdy::Theme;
use icedxdy::widget::combobox;

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Combobox",
    description: "Autocomplete input and command palette with a list of suggestions.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/combobox-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[page::Block::Example {
            id: "disabled",
            title: "Disabled",
            description: None,
            snippet: "examples/browser/snippets/combobox-disabled.md",
        }],
    },
    page::Block::Md("examples/browser/snippets/combobox-api-reference.md"),
];

const FRAMEWORKS: [&str; 5] = ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"];

/// The combobox page. Every combobox is uncontrolled.
pub struct ComboboxPage {
    state: page::State,
}

impl ComboboxPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for ComboboxPage {
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
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // shadcn `combobox-demo.tsx` / `combobox-basic.tsx`.
            "demo" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .icon(icon::chevron_down)
                .into(),

            // shadcn `combobox-disabled.tsx`.
            "disabled" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .icon(icon::chevron_down)
                .disabled(true)
                .into(),

            _ => unreachable!("unknown combobox preview id: {id}"),
        }
    }
}
