//! The widget behind [`Textarea`](super::Textarea): iced's text editor over
//! a [`Content`] kept in the widget's own tree, so the textarea can be
//! uncontrolled like an Input.
//!
//! The editor publishes its edit actions into a local shell; the textarea
//! performs them on its content and forwards everything else the editor
//! asked of the shell (capture, redraws, layout, IME, clipboard).

use iced_widget::Renderer;
use iced_widget::text_editor::{Action, Content, StyleFn, TextEditor};
use icedxdy_core::Theme;

use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::shell::Bus;
use crate::core::text::parser::PlainText;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::{Meta, Operation};
use crate::core::{Event, Layout, Length, Padding, Rectangle, Shell, Size, Widget};

/// A multi-line text field.
pub struct Textarea<'a, Message> {
    placeholder: String,
    value: Option<String>,
    default_value: String,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    is_disabled: bool,
    min_height: f32,
    style: StyleFn<'a, Theme>,
}

impl<'a, Message> Textarea<'a, Message> {
    /// A textarea showing `placeholder` while empty, painted by `style`.
    pub fn new(placeholder: String, style: StyleFn<'a, Theme>) -> Self {
        Self {
            placeholder,
            value: None,
            default_value: String::new(),
            on_input: None,
            is_disabled: false,
            min_height: 64.0,
            style,
        }
    }

    /// Control the textarea: it shows `value`.
    pub fn value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }

    /// The text of an uncontrolled textarea.
    pub fn default_value(mut self, value: String) -> Self {
        self.default_value = value;
        self
    }

    /// Publish the complete new text on every edit.
    pub fn on_input(mut self, on_input: Option<Box<dyn Fn(String) -> Message + 'a>>) -> Self {
        self.on_input = on_input;
        self
    }

    /// Whether the textarea ignores input.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    /// The least height of the textarea, in logical pixels.
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }

    /// The editor over `content`, built anew for every call: it borrows the
    /// content from the tree.
    fn editor<'b>(
        &'b self,
        content: &'b Content<Renderer>,
    ) -> TextEditor<'b, PlainText, Action, Theme, Renderer> {
        let editor = TextEditor::new(content)
            .placeholder(self.placeholder.as_str())
            .size(14)
            .padding(Padding {
                top: 8.0,
                right: 10.0,
                bottom: 8.0,
                left: 10.0,
            })
            .height(Length::Fit.min(self.min_height))
            .style(|theme: &Theme, status| (self.style)(theme, status));

        if self.is_disabled {
            editor
        } else {
            editor.on_action(|action| action)
        }
    }
}

impl<Message> Meta for Textarea<'_, Message> {}

impl<Message> Widget<Message, Theme, Renderer> for Textarea<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let text = self.value.as_deref().unwrap_or(&self.default_value);

        tree::State::new(State {
            content: Content::with_text(text),
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();

        if let Some(value) = &self.value
            && state.content.text() != *value
        {
            state.content = Content::with_text(value);
        }

        let mut editor = self.editor(&state.content);

        if children.is_empty() {
            children.push(Tree::new(&editor));
        }

        children[0].diff(&mut editor);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fit)
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        let Tree {
            state,
            children,
            size,
            ..
        } = tree;
        let state = state.downcast_ref::<State>();

        self.editor(&state.content)
            .layout(&mut children[0], renderer, limits);
        *size = children[0].size;
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
        let mut bus = Bus::new();

        {
            let Tree {
                state, children, ..
            } = &mut *tree;
            let state = state.downcast_ref::<State>();
            let (child_layout, child_tree) = layout
                .iter_mut(children)
                .next()
                .expect("a textarea has one child");

            let mut local = shell.local(&mut bus);

            self.editor(&state.content).update(
                child_tree,
                event,
                child_layout,
                cursor,
                renderer,
                &mut local,
                viewport,
            );

            if local.is_event_captured() {
                shell.capture_event();
            }

            shell.request_redraw_at(local.redraw_request());
            shell.invalidate(local.invalidation());
            shell.request_input_method(local.input_method());
            shell.clipboard_mut().merge(local.clipboard_mut());
        }

        let state = tree.state.downcast_mut::<State>();
        let mut edited = false;

        for (action, _receipt) in bus.drain() {
            edited |= action.is_edit();
            state.content.perform(action);
        }

        if edited && let Some(on_input) = &self.on_input {
            shell.publish(on_input(state.content.text()));
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
        let state = tree.state.downcast_ref::<State>();
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a textarea has one child");

        self.editor(&state.content).draw(
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
        if self.is_disabled {
            return if cursor.is_over(layout.bounds()) {
                mouse::Interaction::NotAllowed
            } else {
                mouse::Interaction::default()
            };
        }

        let state = tree.state.downcast_ref::<State>();
        let (child_layout, child_tree) = layout
            .iter(&tree.children)
            .next()
            .expect("a textarea has one child");

        self.editor(&state.content).mouse_interaction(
            child_tree,
            child_layout,
            cursor,
            viewport,
            renderer,
        )
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
        let state = state.downcast_ref::<State>();
        let (child_layout, child_tree) = layout
            .iter_mut(children)
            .next()
            .expect("a textarea has one child");

        self.editor(&state.content).operate(
            child_tree,
            child_layout,
            viewport,
            renderer,
            operation,
        );
    }
}

struct State {
    content: Content<Renderer>,
}
