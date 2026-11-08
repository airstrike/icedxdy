//! Link widgets to the form controls inside them, the way an HTML
//! `<label for>` links to its `<input>`.
//!
//! A form control (Checkbox, Switch, Radio, Toggle) keeps a [`Control`] in
//! its tree state and hands it to every [`Operation`] through
//! [`Operation::custom`]. A widget that owns a control without knowing its
//! type (Field, for one) runs the operations below over its content:
//!
//! - [`Activate`] presses the first control it reaches. A checkbox, switch,
//!   radio or toggle records the press and acts on it at its next event,
//!   toggling its uncontrolled value and publishing its change message as if
//!   it had been clicked. A text input takes focus instead, with the cursor at the
//!   end of its value.
//! - [`Checked`] reads whether the first control it reaches is checked.
//!
//! Operations traverse iced `Component` boundaries, so the link holds inside
//! a user's form component too.

use std::any::Any;

use crate::core::Rectangle;
use crate::core::widget::Id;
use crate::core::widget::Operation;
use crate::core::widget::operation::{Focusable, TextInput};

/// The state a form control exposes to [`Activate`] and [`Checked`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Control {
    checked: bool,
    activated: bool,
}

impl Control {
    /// A control in the given checked state.
    pub fn new(checked: bool) -> Self {
        Self {
            checked,
            activated: false,
        }
    }

    /// Record the control's current checked state, read by [`Checked`].
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Whether [`Activate`] pressed the control since the last call. The
    /// control acts on a `true` as it would on a click.
    pub fn take_activation(&mut self) -> bool {
        std::mem::take(&mut self.activated)
    }
}

/// An [`Operation`] that presses the first form control it reaches.
#[derive(Debug, Default)]
pub struct Activate {
    done: bool,
    text_input: bool,
}

/// Produces an [`Activate`] operation.
pub fn activate() -> Activate {
    Activate::default()
}

impl Activate {
    /// Whether the operation reached a control.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Operation for Activate {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        if !self.done {
            operate(self);
        }
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Any) {
        if self.done {
            return;
        }

        if let Some(control) = state.downcast_mut::<Control>() {
            control.activated = true;
            self.done = true;
        }
    }

    fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
        if !self.done {
            state.move_cursor_to_end();
            self.text_input = true;
        }
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        // A text input reports itself as `text_input` and then as
        // `focusable`; any other focusable (a button) is not a control.
        if self.text_input && !self.done {
            state.focus();
            self.done = true;
        }
    }
}

/// An [`Operation`] that reads whether the first form control it reaches is
/// checked.
#[derive(Debug, Default)]
pub struct Checked {
    checked: Option<bool>,
}

/// Produces a [`Checked`] operation.
pub fn checked() -> Checked {
    Checked::default()
}

impl Checked {
    /// The checked state of the control the operation reached, if any.
    pub fn get(&self) -> Option<bool> {
        self.checked
    }
}

impl Operation for Checked {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        if self.checked.is_none() {
            operate(self);
        }
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Any) {
        if self.checked.is_none()
            && let Some(control) = state.downcast_ref::<Control>()
        {
            self.checked = Some(control.checked);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::text::{Fragment, Position};

    struct Field {
        is_focused: bool,
        at_end: bool,
    }

    impl Focusable for Field {
        fn is_focused(&self) -> bool {
            self.is_focused
        }

        fn focus(&mut self) {
            self.is_focused = true;
        }

        fn unfocus(&mut self) {
            self.is_focused = false;
        }
    }

    impl TextInput for Field {
        fn text(&self) -> Fragment<'_> {
            "".into()
        }

        fn move_cursor_to_front(&mut self) {}

        fn move_cursor_to_end(&mut self) {
            self.at_end = true;
        }

        fn move_cursor_to(&mut self, _location: Position) {}

        fn select_all(&mut self) {}

        fn select_range(&mut self, _start: Position, _end: Position) {}
    }

    #[test]
    fn activate_presses_only_the_first_control() {
        let mut first = Control::new(false);
        let mut second = Control::new(false);
        let mut activate = activate();

        activate.custom(None, Rectangle::default(), &mut first);
        activate.custom(None, Rectangle::default(), &mut second);

        assert!(activate.is_done());
        assert!(first.take_activation());
        assert!(!first.take_activation(), "an activation is taken once");
        assert!(!second.take_activation());
    }

    #[test]
    fn activate_ignores_state_that_is_not_a_control() {
        let mut other = 42_u8;
        let mut activate = activate();

        activate.custom(None, Rectangle::default(), &mut other);

        assert!(!activate.is_done());
    }

    #[test]
    fn activate_focuses_a_text_input_but_not_a_plain_focusable() {
        let mut button = Field {
            is_focused: false,
            at_end: false,
        };
        let mut input = Field {
            is_focused: false,
            at_end: false,
        };
        let mut activate = activate();

        activate.focusable(None, Rectangle::default(), &mut button);
        assert!(!button.is_focused);

        activate.text_input(None, Rectangle::default(), &mut input);
        activate.focusable(None, Rectangle::default(), &mut input);

        assert!(input.is_focused);
        assert!(input.at_end);
        assert!(activate.is_done());
    }

    #[test]
    fn checked_reads_the_first_control() {
        let mut on = Control::new(true);
        let mut off = Control::new(false);
        let mut probe = checked();

        assert_eq!(probe.get(), None);

        probe.custom(None, Rectangle::default(), &mut on);
        probe.custom(None, Rectangle::default(), &mut off);

        assert_eq!(probe.get(), Some(true));
    }
}
