// API Reference is rendered via the `sidebar-api-reference.md` snippet at
// the bottom of BLOCKS. Live docs API Reference covers the full suite —
// `SidebarProvider.{defaultOpen, open, onOpenChange}`,
// `Sidebar.{side, variant, collapsible}`, plus a structural enumeration of
// `SidebarHeader`/`SidebarFooter`/`SidebarContent`/`SidebarGroup`/
// `SidebarMenu`/`SidebarMenuItem`/`SidebarMenuButton`/`SidebarMenuAction`/
// `SidebarMenuBadge`/`SidebarMenuSub*`/`SidebarTrigger`/`SidebarRail`/
// `SidebarInset` and the `useSidebar()` hook — and the snippet captures
// the icedxdy-shaped helpers (`sidebar::fixed`/`floating`/`inset`,
// section helpers, `menu_button` builder, `State`/`Side`/`Collapsible`
// constants) plus the caller-owns-state model that replaces the React
// hook.
//
// Intentional skips: sidebar-rtl (RTL).
// Examples shipped: sidebar-menu (demo), sidebar-group, sidebar-header.

use iced::Task;

use icedxdy::widget::{column, sidebar, text};
use icedxdy::{Slot, Theme};

use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Sidebar",
    description: "A composable, themeable sidebar suite.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/sidebar-demo.md",
    },
    page::Block::Md("examples/browser/snippets/sidebar-usage.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "group",
                title: "Group",
                description: Some(
                    "Use sidebar::group and sidebar::group_label to create labeled sections.",
                ),
                snippet: "examples/browser/snippets/sidebar-group.md",
            },
            page::Block::Example {
                id: "header",
                title: "Header",
                description: Some(
                    "Use sidebar::header to add a workspace switcher or branding at the top.",
                ),
                snippet: "examples/browser/snippets/sidebar-header.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/sidebar-api-reference.md"),
];

#[derive(Debug, Clone)]
pub enum Message {
    Select(usize),
}

pub struct SidebarPage {
    state: page::State,
    selected: usize,
}

impl SidebarPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            selected: 0,
        })
    }
}

fn demo<'a>(selected: usize) -> Slot<'a, Message> {
    // Mirrors `sidebar-menu.tsx`: a floating sidebar with a "Projects" group
    // and five menu items, the active one highlighted.
    let items: &[&'static str] = &[
        "Design Engineering",
        "Sales & Marketing",
        "Travel",
        "Support",
        "Feedback",
    ];

    let menu_items: Vec<Slot<'a, Message>> = items
        .iter()
        .enumerate()
        .map(|(i, label)| {
            sidebar::menu_item([sidebar::menu_button(*label)
                .active(i == selected)
                .on_press(Message::Select(i))
                .into()])
            .into()
        })
        .collect();

    sidebar::floating(sidebar::content([sidebar::group([
        sidebar::group_label("Projects").into(),
        sidebar::group_content([sidebar::menu(menu_items).into()]).into(),
    ])
    .into()]))
    .width(sidebar::WIDTH)
    .height(iced::Fill)
    .into()
}

fn group_example<'a>() -> Slot<'a, Message> {
    // Mirrors `sidebar-group.tsx`: a floating sidebar with a "Help" group.
    sidebar::floating(sidebar::content([sidebar::group([
        sidebar::group_label("Help").into(),
        sidebar::group_content([sidebar::menu([
            sidebar::menu_item([sidebar::menu_button("Support").into()]).into(),
            sidebar::menu_item([sidebar::menu_button("Feedback").into()]).into(),
        ])
        .into()])
        .into(),
    ])
    .into()]))
    .width(sidebar::WIDTH)
    .height(iced::Fill)
    .into()
}

fn header_example<'a>() -> Slot<'a, Message> {
    // Mirrors `sidebar-header.tsx`: a floating sidebar with a workspace
    // switcher in the header.
    sidebar::floating(
        column![
            sidebar::header([sidebar::menu([sidebar::menu_item([sidebar::menu_button(
                "Select Workspace",
            )
            .into()])
            .into()])
            .into()]),
            sidebar::content([text::p("Workspace content goes here.").into()]),
        ]
        .width(sidebar::WIDTH),
    )
    .height(iced::Fill)
    .into()
}

impl page::Page for SidebarPage {
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

    fn init(&self) -> Task<crate::Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<crate::Message> {
        if let Some(task) = self.state.update(&event) {
            return task;
        }
        if let page::Message::ValueChanged(0, v) = event {
            self.selected = v as usize;
        }
        Task::none()
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        let pi = self.state.page_index;
        let selected = self.selected;
        let view: Slot<'_, Message> = match id {
            "demo" => demo(selected),
            "group" => group_example(),
            "header" => header_example(),
            _ => unreachable!("unknown sidebar preview id: {id}"),
        };
        view.map(move |msg| {
            let inner = match msg {
                Message::Select(i) => page::Message::ValueChanged(0, i as f32),
            };
            crate::Message::Page(pi, inner)
        })
        .into()
    }
}
