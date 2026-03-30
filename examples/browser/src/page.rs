use std::collections::BTreeSet;
use std::path::PathBuf;

use iced::Task;
use iced::widget::{Id, markdown};

use icedxdy::Theme;

use crate::example;

/// Static metadata for a component page.
pub struct Meta {
    pub title: &'static str,
    pub description: &'static str,
    pub examples: &'static [example::Meta],
}

/// Events produced by page statestructure and component-specific interactions.
#[derive(Debug, Clone)]
pub enum Event {
    SnippetLoaded(usize, Result<String, String>),
    LinkClicked(markdown::Uri),
    ExampleShown(usize),
    ExampleHidden(usize),
    ScrollTo(usize),
    /// Input field changed: (field index within the page, new value)
    InputChanged(usize, String),
    /// Generic no-op for stateless component actions (e.g. button presses in previews)
    Noop,
}

/// Object-safe trait for a component page.
pub trait Page {
    fn meta(&self) -> &'static Meta;
    fn snippets(&self) -> &[Option<Vec<markdown::Item>>];
    fn visible(&self) -> &BTreeSet<usize>;
    fn example_ids(&self) -> &[Id];
    fn scrollable_id(&self) -> Id;

    fn init(&self) -> Task<super::Message>;
    fn update(&mut self, event: Event) -> Task<super::Message>;
    fn preview(&self, index: usize) -> iced::Element<'_, super::Message, Theme>;
}

/// Shared structure that each page owns via composition.
pub struct State {
    pub page_index: usize,
    pub snippets: Vec<Option<Vec<markdown::Item>>>,
    pub visible: BTreeSet<usize>,
    pub example_ids: Vec<Id>,
    pub scrollable_id: Id,
    meta: &'static Meta,
}

impl State {
    pub fn new(page_index: usize, meta: &'static Meta) -> Self {
        let count = meta.examples.len();
        Self {
            page_index,
            snippets: vec![None; count],
            visible: BTreeSet::new(),
            example_ids: (0..count).map(|_| Id::unique()).collect(),
            scrollable_id: Id::unique(),
            meta,
        }
    }

    pub fn init(&self) -> Task<super::Message> {
        let pi = self.page_index;
        let tasks: Vec<Task<super::Message>> = self
            .meta
            .examples
            .iter()
            .enumerate()
            .filter(|(_, ex)| !ex.snippet.is_empty())
            .map(|(i, ex)| {
                let path = PathBuf::from(ex.snippet);
                Task::perform(
                    async move {
                        tokio::fs::read_to_string(path)
                            .await
                            .map_err(|e| e.to_string())
                    },
                    move |result| super::Message::Page(pi, Event::SnippetLoaded(i, result)),
                )
            })
            .collect();
        Task::batch(tasks)
    }

    /// Handle statestructure events. Returns Some(task) if handled, None if not.
    pub fn update(&mut self, event: &Event) -> Option<Task<super::Message>> {
        use icedxdy::widget::operation::scroll;

        match event {
            Event::SnippetLoaded(index, result) => {
                if let Some(slot) = self.snippets.get_mut(*index) {
                    if let Ok(content) = result {
                        *slot = Some(markdown::parse(content).collect());
                    }
                }
                Some(Task::none())
            }
            Event::LinkClicked(_) => Some(Task::none()),
            Event::ExampleShown(idx) => {
                self.visible.insert(*idx);
                Some(Task::none())
            }
            Event::ExampleHidden(idx) => {
                self.visible.remove(idx);
                Some(Task::none())
            }
            Event::ScrollTo(idx) => Some(scroll::scroll_into_view(
                self.scrollable_id.clone(),
                self.example_ids[*idx].clone(),
            )),
            Event::Noop => Some(Task::none()),
            _ => None,
        }
    }
}
