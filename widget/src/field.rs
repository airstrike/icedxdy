//! Field component
//!
//! A vertical wrapper around a form input with optional label, description,
//! and error slots. Mirrors shadcn's `Field` (the MVP subset:
//! `Field`/`FieldLabel`/`FieldDescription`/`FieldError`); other shadcn
//! field exports — `FieldGroup`, `FieldSet`, `FieldLegend`, `FieldSeparator`,
//! `FieldContent`, `FieldTitle` — are not yet ported.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::{field, input, text};
//!
//! // Terse — strings auto-style with the matching typography helper
//! // (`text::label`, `text::muted`, `text::error`).
//! field::field(input("Username", &value).on_input(Message::Changed))
//!     .label("Username")
//!     .description("Choose a unique username for your account.");
//!
//! // Explicit — pass any pre-styled `Text` (or `Component`) through unchanged.
//! field::field(input("Email", &value).on_input(Message::Changed))
//!     .label(text::label("Email"))
//!     .error(text::error("This email is already taken."));
//! ```
//!
//! # Slot order
//!
//! Rendered top-to-bottom in a `Column` with 8px (`gap-2`) spacing per
//! shadcn's `field.tsx`:
//!
//! 1. `label`
//! 2. `content` (the input widget)
//! 3. `description`
//! 4. `error`
//!
//! Slots are independently optional; missing slots collapse out of the
//! column entirely (no empty rows). Composition follows the same
//! conditional-push pattern `alert::Alert` uses for its title/description
//! pair — `Column::new().spacing(...)` plus `if let Some(x) = x { col =
//! col.push(x); }` for each optional slot. The `column!` macro is
//! variadic and rejects `Option<Component>`, so it's not the right
//! shape here.

use iced_widget::Renderer;
use icedxdy_core::Theme;

use crate::component::{Component, IntoElement};

/// A vertical labeled-input wrapper.
///
/// Shape: `transformation-method` (per the OPAQUE guide) — the constructor
/// is born from the inner content widget; builder methods carry forward
/// state; `into_element` consumes `self` into the final layout column.
pub struct Field<'a, Message> {
    label: Option<Component<'a, Message>>,
    content: Component<'a, Message>,
    description: Option<Component<'a, Message>>,
    error: Option<Component<'a, Message>>,
}

/// Create a [`Field`] wrapping the given input content.
///
/// The input widget is the only required argument. Add a label,
/// description, or error message via the builder methods.
pub fn field<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Field<'a, Message> {
    Field {
        label: None,
        content: content.into(),
        description: None,
        error: None,
    }
}

impl<'a, Message: 'a> Field<'a, Message> {
    /// Set the field label. Plain strings auto-style as `text::label`
    /// (`text-sm font-medium`); pre-styled `Text` or `Component` values
    /// pass through unchanged.
    pub fn label(mut self, label: impl IntoLabel<'a, Message>) -> Self {
        self.label = Some(label.into_label());
        self
    }

    /// Set the field description. Plain strings auto-style as `text::muted`
    /// (`text-sm text-muted-foreground`); pre-styled values pass through.
    pub fn description(mut self, description: impl IntoDescription<'a, Message>) -> Self {
        self.description = Some(description.into_description());
        self
    }

    /// Set the field error message. Plain strings auto-style as
    /// `text::error` (`text-sm text-destructive`); pre-styled values pass
    /// through.
    ///
    /// Note: presence of an error message does NOT auto-style the inner
    /// content widget as invalid — iced has no context system for that
    /// propagation. Callers wire invalid styling on the input separately
    /// (e.g. `input(...).style(theme::text_input::Invalid)`).
    pub fn error(mut self, error: impl IntoError<'a, Message>) -> Self {
        self.error = Some(error.into_error());
        self
    }
}

impl<'a, Message: 'a> From<Field<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(f: Field<'a, Message>) -> Self {
        Box::new(f).into_element()
    }
}

impl<'a, Message: 'a> From<Field<'a, Message>> for Component<'a, Message> {
    fn from(f: Field<'a, Message>) -> Self {
        Component::new(f)
    }
}

impl<'a, Message: 'a> IntoElement<'a, Message> for Field<'a, Message> {
    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        let Field {
            label,
            content,
            description,
            error,
        } = *self;

        // shadcn `field.tsx` line 55: `flex w-full gap-2` — 8px (`gap-2`)
        // between every present slot. Composition follows the same
        // conditional-push shape `alert::Alert::into_element` uses.
        let mut col = iced_widget::Column::new().spacing(8);
        if let Some(label) = label {
            col = col.push(label.into_iced());
        }
        col = col.push(content.into_iced());
        if let Some(description) = description {
            col = col.push(description.into_iced());
        }
        if let Some(error) = error {
            col = col.push(error.into_iced());
        }
        col.into()
    }
}

// ---------------------------------------------------------------------------
// Hybrid string / pre-styled intermediary traits
//
// Each slot accepts either a plain string (auto-styled with the matching
// typography helper) or a pre-styled `Text<'a>` / `Component<'a, Message>`.
// Implemented as three separate marker traits — one per slot — with concrete
// impls per acceptable input type. No blanket `impl<T: Into<Component>>`,
// because that would conflict with the auto-styling impls on `&str` /
// `String` (a string is `Into<Component>` via the bare-text path that
// skips the typography helper).
//
// Trait names are deliberately `IntoLabel` / `IntoDescription` / `IntoError`
// — NOT `IntoFieldLabel` / `IntoFieldDescription` / `IntoFieldError` — so
// the module path (`field::IntoLabel`) carries the "field" disambiguation
// per the project's "no composite names" rule.
// ---------------------------------------------------------------------------

/// Convert a value into a [`Field`] label slot. Plain strings are auto-styled
/// as `text::label`; pre-styled `Text` / `Component` values pass through.
pub trait IntoLabel<'a, Message> {
    fn into_label(self) -> Component<'a, Message>;
}

impl<'a, Message: 'a> IntoLabel<'a, Message> for &'a str {
    fn into_label(self) -> Component<'a, Message> {
        crate::text::label(self).into()
    }
}

impl<'a, Message: 'a> IntoLabel<'a, Message> for String {
    fn into_label(self) -> Component<'a, Message> {
        crate::text::label(self).into()
    }
}

impl<'a, Message: 'a> IntoLabel<'a, Message> for crate::text::Text<'a> {
    fn into_label(self) -> Component<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoLabel<'a, Message> for Component<'a, Message> {
    fn into_label(self) -> Component<'a, Message> {
        self
    }
}

/// Convert a value into a [`Field`] description slot. Plain strings are
/// auto-styled as `text::muted`; pre-styled values pass through.
pub trait IntoDescription<'a, Message> {
    fn into_description(self) -> Component<'a, Message>;
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for &'a str {
    fn into_description(self) -> Component<'a, Message> {
        crate::text::muted(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for String {
    fn into_description(self) -> Component<'a, Message> {
        crate::text::muted(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for crate::text::Text<'a> {
    fn into_description(self) -> Component<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for Component<'a, Message> {
    fn into_description(self) -> Component<'a, Message> {
        self
    }
}

/// Convert a value into a [`Field`] error slot. Plain strings are
/// auto-styled as `text::error`; pre-styled values pass through.
pub trait IntoError<'a, Message> {
    fn into_error(self) -> Component<'a, Message>;
}

impl<'a, Message: 'a> IntoError<'a, Message> for &'a str {
    fn into_error(self) -> Component<'a, Message> {
        crate::text::error(self).into()
    }
}

impl<'a, Message: 'a> IntoError<'a, Message> for String {
    fn into_error(self) -> Component<'a, Message> {
        crate::text::error(self).into()
    }
}

impl<'a, Message: 'a> IntoError<'a, Message> for crate::text::Text<'a> {
    fn into_error(self) -> Component<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoError<'a, Message> for Component<'a, Message> {
    fn into_error(self) -> Component<'a, Message> {
        self
    }
}

#[cfg(test)]
mod tests {
    //! Composition-shape coverage for the alert-style builder loop.
    //!
    //! These tests inspect the builder fields BEFORE `into_element` runs —
    //! once an iced widget is produced, the slot identities are erased.
    //! Asserting on the `Option<Component>` shape here proves the conditional-
    //! push pattern in `into_element` won't try to render empty slots.

    use crate::field;

    #[derive(Clone, Debug)]
    enum Msg {}

    #[test]
    fn empty_field_has_no_slots() {
        let f: field::Field<'_, Msg> = field::field("content");
        assert!(f.label.is_none(), "empty field must have no label");
        assert!(
            f.description.is_none(),
            "empty field must have no description"
        );
        assert!(f.error.is_none(), "empty field must have no error");
    }

    #[test]
    fn label_only_populates_label_slot() {
        let f: field::Field<'_, Msg> = field::field("content").label("My Label");
        assert!(f.label.is_some(), "label() must populate the label slot");
        assert!(f.description.is_none());
        assert!(f.error.is_none());
    }

    #[test]
    fn description_only_populates_description_slot() {
        let f: field::Field<'_, Msg> = field::field("content").description("Helper text");
        assert!(f.label.is_none());
        assert!(
            f.description.is_some(),
            "description() must populate the description slot"
        );
        assert!(f.error.is_none());
    }

    #[test]
    fn error_only_populates_error_slot() {
        let f: field::Field<'_, Msg> = field::field("content").error("Bad input");
        assert!(f.label.is_none());
        assert!(f.description.is_none());
        assert!(f.error.is_some(), "error() must populate the error slot");
    }

    #[test]
    fn full_field_populates_all_slots() {
        let f: field::Field<'_, Msg> = field::field("content")
            .label("Email")
            .description("We'll never share your email.")
            .error("Invalid email");
        assert!(f.label.is_some());
        assert!(f.description.is_some());
        assert!(f.error.is_some());
    }

    /// Pre-styled `Text` values pass through the intermediary trait
    /// without re-wrapping. The terse `&str` arm and the explicit
    /// `text::label(...)` arm both produce a populated slot — what
    /// matters is that both compile and produce `Some(_)`.
    #[test]
    fn pre_styled_text_passes_through() {
        let f: field::Field<'_, Msg> = field::field("content")
            .label(crate::text::label("Pre-styled label"))
            .description(crate::text::muted("Pre-styled description"))
            .error(crate::text::error("Pre-styled error"));
        assert!(f.label.is_some());
        assert!(f.description.is_some());
        assert!(f.error.is_some());
    }

    /// `String` (owned) is accepted alongside `&str`. Confirms the
    /// intermediary trait covers both string-shape inputs.
    #[test]
    fn owned_string_arm() {
        let label = String::from("Owned label");
        let description = String::from("Owned description");
        let error = String::from("Owned error");
        let f: field::Field<'_, Msg> = field::field("content")
            .label(label)
            .description(description)
            .error(error);
        assert!(f.label.is_some());
        assert!(f.description.is_some());
        assert!(f.error.is_some());
    }
}
