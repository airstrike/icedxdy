//! The widget behind [`Toaster`](super::Toaster): its content in place,
//! its toasts in an overlay, and the clock that dismisses them.

use std::time::Duration;

use iced_core::Length::Fill;
use iced_core::time::Instant;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme::toast;

use super::{Icons, Inbox, Position, Raise, Toast};
use crate::button::button;
use crate::column::Column;
use crate::compose::Slot;
use crate::container::Container;
use crate::core::Renderer as _;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::window;
use crate::core::{
    Alignment, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector, Widget,
};
use crate::marker::{self, Press};
use crate::row::Row;
use crate::size;
use crate::text::{self, Text};

/// Sonner's `--width`.
const WIDTH: f32 = 356.0;
/// Sonner's viewport offset from the window edges.
const OFFSET: f32 = 24.0;
/// Sonner's gap between expanded toasts.
const GAP: f32 = 14.0;
/// Sonner's `visibleToasts`.
const VISIBLE: usize = 3;

/// The signal a toast's action records: the toast to dismiss.
#[derive(Debug, Clone, Copy)]
struct Dismiss(u64);

/// Content with a stack of toasts over it.
pub struct Toaster<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    position: Position,
    duration: Duration,
    icons: Icons<'a, Message>,
    /// The cards of the live toasts, rebuilt from the tree in `diff`.
    cards: Vec<Card<'a, Message>>,
}

/// A live toast's card.
struct Card<'a, Message> {
    position: Position,
    element: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message> Toaster<'a, Message> {
    /// Show the toasts raised in `content` at `position` for `duration`.
    pub fn new(
        content: Element<'a, Message, Theme, Renderer>,
        position: Position,
        duration: Duration,
        icons: Icons<'a, Message>,
    ) -> Self {
        Self {
            content,
            position,
            duration,
            icons,
            cards: Vec::new(),
        }
    }
}

impl<'a, Message: Clone + 'a> Toaster<'a, Message> {
    /// `flex items-center gap-1.5 p-4` around the icon, the title over the
    /// description, and the action.
    fn card(&self, id: u64, toast: &Toast<Message>) -> Element<'a, Message, Theme, Renderer> {
        let mut copy = vec![Slot::from(Text::new(toast.title.clone()).size(13).medium())];

        if let Some(description) = &toast.description {
            copy.push(Slot::from(text::muted(description.clone()).size(13)));
        }

        let mut row: Vec<Slot<'a, Message>> = Vec::new();

        if let Some(icon) = (self.icons)(toast.kind) {
            row.push(icon);
        }

        row.push(Slot::from(
            Column::with_children(copy).spacing(2).width(Fill),
        ));

        if let Some((label, message)) = &toast.action {
            // `h-6 px-2 text-xs font-medium`.
            let action = button(Text::new(label.clone()))
                .size(size::Size::Xs)
                .style(
                    Box::new(|theme: &Theme, status| toast::action(theme, status).into())
                        as iced_widget::button::StyleFn<'a, Theme>,
                )
                .on_press(message.clone());

            let action: Element<'a, Message, Theme, Renderer> =
                marker::marker(Slot::from(action).compose(), Dismiss(id), Press::Captured).into();
            row.push(Slot::from(action));
        }

        Container::new(
            Row::with_children(row)
                .spacing(6)
                .align_y(Alignment::Center)
                .width(Fill),
        )
        .width(WIDTH)
        .padding(16)
        .style(toast::card)
        .into()
    }
}

impl<Message> Meta for Toaster<'_, Message> {}

impl<'a, Message: Clone + Send + 'static> Widget<Message, Theme, Renderer>
    for Toaster<'a, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Message>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::<Message> {
            live: Vec::new(),
            inbox: Inbox(Vec::new()),
            next_id: 0,
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        let state = tree.state.downcast_ref::<State<Message>>();

        self.cards = state
            .live
            .iter()
            .map(|live| Card {
                position: live.toast.position.unwrap_or(self.position),
                element: self.card(live.id, &live.toast),
            })
            .collect();

        let mut children: Vec<&mut Element<'a, Message, Theme, Renderer>> =
            std::iter::once(&mut self.content)
                .chain(self.cards.iter_mut().map(|card| &mut card.element))
                .collect();

        tree.diff_children_custom(
            &mut children,
            |tree, element| tree.diff(&mut **element),
            |element| Tree::new(&**element),
        );
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State<Message>>();
        let (child_layout, child_tree) = layout
            .iter_mut(&mut children[..1])
            .next()
            .expect("a toaster has its content");

        self.content.update(
            child_tree,
            event,
            child_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        let mut raised = marker::drain::<Raise<Message>>();
        self.content
            .operate(child_tree, child_layout, viewport, renderer, &mut raised);

        let mut toasts: Vec<Toast<Message>> = raised
            .into_signals()
            .into_iter()
            .map(|Raise(toast)| toast)
            .collect();
        toasts.append(&mut state.inbox.0);

        if !toasts.is_empty() {
            for toast in toasts {
                state.live.push(Live {
                    id: state.next_id,
                    toast,
                    shown_at: None,
                });
                state.next_id += 1;
            }

            shell.invalidate_widgets();
            shell.request_redraw();
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let before = state.live.len();
            let duration = self.duration;

            for live in &mut state.live {
                live.shown_at.get_or_insert(*now);
            }

            state.live.retain(|live| {
                live.shown_at
                    .is_none_or(|at| *now < at + live.toast.duration.unwrap_or(duration))
            });

            if state.live.len() != before {
                shell.invalidate_widgets();
            }

            if let Some(next) = state
                .live
                .iter()
                .filter_map(|live| {
                    live.shown_at
                        .map(|at| at + live.toast.duration.unwrap_or(duration))
                })
                .min()
            {
                shell.request_redraw_at(window::RedrawRequest::At(next));
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let (child_layout, child_tree) = layout
            .iter(&tree.children[..1])
            .next()
            .expect("a toaster has its content");

        self.content.draw(
            child_tree,
            renderer,
            theme,
            style,
            child_layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let (child_layout, child_tree) = layout
            .iter(&tree.children[..1])
            .next()
            .expect("a toaster has its content");

        self.content
            .mouse_interaction(child_tree, child_layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State<Message>>();
        operation.custom(None, layout.bounds(), &mut state.inbox);

        let (child_layout, child_tree) = layout
            .iter_mut(&mut children[..1])
            .next()
            .expect("a toaster has its content");

        self.content
            .operate(child_tree, child_layout, viewport, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let Tree {
            state, children, ..
        } = tree;
        let (content_tree, card_trees) = children.split_at_mut(1);

        let (child_layout, content_tree) = layout
            .iter_mut(content_tree)
            .next()
            .expect("a toaster has its content");

        let mut overlays = self.content.overlay(
            content_tree,
            child_layout,
            renderer,
            viewport,
            translation,
            window,
        );

        if self.cards.is_empty() {
            return overlays;
        }

        let state = state.downcast_mut::<State<Message>>();
        let width = WIDTH.min(window.width - OFFSET * 2.0).max(0.0);
        let limits = layout::Limits::new(Size::ZERO, Size::new(width, window.height));

        // Per position, the newest toasts sit nearest their edge; older
        // ones past the visible count wait off screen.
        let mut stacks: Vec<(Position, f32, usize)> = Vec::new();
        let mut entries = Vec::new();

        for (card, tree) in self.cards.iter_mut().zip(card_trees.iter_mut()).rev() {
            let index = match stacks.iter().position(|(at, ..)| *at == card.position) {
                Some(index) => index,
                None => {
                    stacks.push((card.position, 0.0, 0));
                    stacks.len() - 1
                }
            };
            let (_, offset, count) = &mut stacks[index];

            if *count == VISIBLE {
                continue;
            }

            card.element.layout(tree, renderer, &limits);
            let size = tree.size;

            let x = match card.position {
                Position::TopLeft | Position::BottomLeft => OFFSET,
                Position::TopCenter | Position::BottomCenter => (window.width - size.width) / 2.0,
                Position::TopRight | Position::BottomRight => window.width - OFFSET - size.width,
            };
            let y = match card.position {
                Position::TopLeft | Position::TopCenter | Position::TopRight => OFFSET + *offset,
                Position::BottomLeft | Position::BottomCenter | Position::BottomRight => {
                    window.height - OFFSET - *offset - size.height
                }
            };

            *offset += size.height + GAP;
            *count += 1;

            entries.push(Entry {
                element: &mut card.element,
                tree,
                layout: Layout::new(size).move_to(Point::new(x, y)),
            });
        }

        overlays.push(overlay::Element::new(Box::new(Stack {
            entries,
            state,
            window,
        })));

        overlays
    }
}

/// The toaster's tree state.
struct State<Message> {
    live: Vec<Live<Message>>,
    inbox: Inbox<Message>,
    next_id: u64,
}

/// A toast on screen, timed from its first frame.
struct Live<Message> {
    id: u64,
    toast: Toast<Message>,
    shown_at: Option<Instant>,
}

/// A laid-out card in the overlay.
struct Entry<'a, 'b, Message> {
    element: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    layout: Layout,
}

/// The toasts over the window.
struct Stack<'a, 'b, Message> {
    entries: Vec<Entry<'a, 'b, Message>>,
    state: &'b mut State<Message>,
    window: Size,
}

impl<Message: Clone + Send + 'static> overlay::Overlay<Message, Theme, Renderer>
    for Stack<'_, '_, Message>
{
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        let viewport = Rectangle::with_size(self.window);

        for entry in &self.entries {
            renderer.with_layer(viewport, |renderer| {
                entry.element.draw(
                    entry.tree,
                    renderer,
                    theme,
                    style,
                    entry.layout,
                    cursor,
                    &viewport,
                );
            });
        }
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn Operation) {
        let viewport = Rectangle::with_size(self.window);

        for entry in &mut self.entries {
            entry
                .element
                .operate(entry.tree, entry.layout, &viewport, renderer, operation);
        }
    }

    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        let viewport = Rectangle::with_size(self.window);
        let mut dismissed = Vec::new();

        for entry in &mut self.entries {
            entry.element.update(
                entry.tree,
                event,
                entry.layout,
                cursor,
                renderer,
                shell,
                &viewport,
            );

            let mut drain = marker::drain::<Dismiss>();
            entry
                .element
                .operate(entry.tree, entry.layout, &viewport, renderer, &mut drain);
            dismissed.extend(drain.into_signals().into_iter().map(|Dismiss(id)| id));
        }

        if !dismissed.is_empty() {
            self.state.live.retain(|live| !dismissed.contains(&live.id));
            shell.invalidate_widgets();
        }
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        let viewport = Rectangle::with_size(self.window);

        self.entries
            .iter()
            .map(|entry| {
                entry.element.mouse_interaction(
                    entry.tree,
                    entry.layout,
                    cursor,
                    &viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }
}
