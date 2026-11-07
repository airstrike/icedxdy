//! Field component
//!
//! A wrapper around a form input with optional label, description, and error
//! slots. Mirrors shadcn's `Field`/`FieldLabel`/`FieldDescription`/`FieldError`
//! (the MVP subset). Other shadcn field exports — `FieldSet`, `FieldLegend`,
//! `FieldSeparator`, `FieldContent`, `FieldTitle` — are not yet ported.
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
//! // Explicit — pass any pre-styled `Text` (or `Slot`) through unchanged.
//! field::field(input("Email", &value).on_input(Message::Changed))
//!     .label(text::label("Email"))
//!     .error(text::error("This email is already taken."));
//!
//! // Horizontal card — checkbox with label + description in a bordered card.
//! // Pressing the label toggles the checkbox; the card tints while it is
//! // checked.
//! field::field(checkbox().default_checked(true))
//!     .orientation(field::Orientation::Horizontal)
//!     .label("Enable notifications")
//!     .description("You can enable or disable notifications at any time.")
//!     .card();
//! ```
//!
//! # Label and control
//!
//! Like shadcn's `<FieldLabel htmlFor>`, pressing the label activates the
//! field's control through the [`control`](crate::operation::control)
//! operations: a checkbox, switch or radio toggles, an input takes focus.
//!
//! # Slot order — Vertical (default)
//!
//! Rendered top-to-bottom in a `Column` with 8px (`gap-2`) spacing per
//! shadcn's `field.tsx`:
//!
//! 1. `label`
//! 2. `content` (the input widget)
//! 3. `description`
//! 4. `error`
//!
//! # Slot order — Horizontal
//!
//! The content widget sits on the left; a FieldContent column (label +
//! description + error stacked at 2px gap) sits on the right. Both are
//! in a `Row` with 8px spacing per shadcn's `flex-row items-center
//! has-[>[data-slot=field-content]]:items-start`. Matches
//! `field-checkbox.tsx` and `field-switch.tsx`.
//!
//! Slots are independently optional; missing slots collapse out of the
//! column entirely (no empty rows). Composition follows the same
//! conditional-push pattern `alert::Alert` uses for its title/description
//! pair.

use iced_widget::Widget;
pub mod widget;

use iced_core::Length;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme::field::Card;
use icedxdy_core::theme::field::CardActive;

use crate::compose::{Compose, Inherited, Slot};
use crate::marker::{self, Press};

// Re-export the crate-root [`icedxdy::Orientation`] (and its bare variants)
// at this module path. The underlying type is shared across widgets — a
// `field::Orientation` value and a `button::group::Orientation` value are
// the SAME enum, so callers can build `use icedxdy::{Vertical, Horizontal};`
// once and feed into every `.orientation()` builder via `impl Into`.
//
// Field's default is Vertical (shadcn's default field shape used by Input,
// Textarea, etc. — `<Field><FieldLabel>...</FieldLabel><Input/></Field>`).
// Horizontal is for compact controls where the label sits inline with the
// control (Checkbox, Switch — `field-checkbox.tsx`, `field-switch.tsx`).
pub use icedxdy_core::Horizontal;
pub use icedxdy_core::Orientation;
pub use icedxdy_core::Vertical;

/// A labeled control with optional description, error and card chrome.
///
/// Shape: `transformation-method` (per the OPAQUE guide) — the constructor
/// is born from the inner content widget; builder methods carry forward state;
/// `compose` consumes `self` into the final layout.
pub struct Field<'a, Message> {
    label: Option<Slot<'a, Message>>,
    content: Slot<'a, Message>,
    description: Option<Slot<'a, Message>>,
    error: Option<Slot<'a, Message>>,
    orientation: Orientation,
    card: bool,
    width: Option<Length>,
}

/// A vertical group of fields.
///
/// Mirrors shadcn's `FieldGroup`: `flex w-full flex-col gap-5`. Nested
/// group-specific CSS variants are intentionally deferred until the matching
/// checkbox/radio group primitives exist.
pub struct Group<'a, Message> {
    children: Vec<Slot<'a, Message>>,
    spacing: f32,
    width: Length,
}

/// Create a [`Field`] wrapping the given input content.
///
/// The input widget is the only required argument. Add a label,
/// description, or error message via the builder methods.
pub fn field<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Field<'a, Message> {
    Field {
        label: None,
        content: content.into(),
        description: None,
        error: None,
        orientation: Orientation::Vertical,
        card: false,
        width: None,
    }
}

/// Create a [`Group`] wrapping related fields.
pub fn group<'a, Message: 'a>(
    children: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Group<'a, Message> {
    Group {
        children: children.into_iter().collect(),
        spacing: 20.0,
        width: Length::Fill,
    }
}

impl<'a, Message: 'a> Field<'a, Message> {
    /// Set the field label. Plain strings auto-style as `text::label`
    /// (`text-sm font-medium`); pre-styled `Text` or `Slot` values
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

    /// Set the layout orientation.
    ///
    /// [`Orientation::Vertical`] (default): label → content → description →
    /// error stacked in a Column with 8px gap.
    ///
    /// [`Orientation::Horizontal`]: content on the left, FieldContent column
    /// (label / description / error at 2px gap) on the right, in a Row with
    /// 8px gap. Matches shadcn's `orientation="horizontal"` on `Field`.
    pub fn orientation(mut self, orientation: impl Into<Orientation>) -> Self {
        self.orientation = orientation.into();
        self
    }

    /// Wrap the entire field in a bordered card container.
    ///
    /// While the field's control is checked the card takes a primary-tinted
    /// border and background (`border-primary/30`, `bg-primary/5`), shadcn's
    /// `has-data-checked:` cascade on `FieldLabel`; otherwise a 1px
    /// `border-border` on a transparent background. The control's state is
    /// read through the `checked` operation
    /// ([`control`](crate::operation::control)), so the card needs no flag of its own.
    ///
    /// The card adds 10px padding (`p-2.5`) on all sides and `rounded-lg`
    /// corners.
    pub fn card(mut self) -> Self {
        self.card = true;
        self
    }

    /// Set the field's outer width. Applied to the final rendered element so
    /// callers don't need to wrap a single Field in a `column![field].width(N)`
    /// just to constrain its width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }
}

impl<'a, Message: 'a> Group<'a, Message> {
    /// Set the group gap in logical pixels.
    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Set the group width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(g: Group<'a, Message>) -> Self {
        Box::new(g).compose()
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Group<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Group<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Group {
            children,
            spacing,
            width,
        } = *self;

        let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
            children.into_iter().map(Slot::compose).collect();

        iced_widget::Column::with_children(elements)
            .spacing(spacing)
            .width(width)
            .boxed()
    }
}

impl<'a, Message: Clone + 'a> From<Field<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(f: Field<'a, Message>) -> Self {
        Box::new(f).compose()
    }
}

impl<'a, Message: Clone + 'a> From<Field<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Field<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Field<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
        for slot in [&mut self.label, &mut self.description, &mut self.error]
            .into_iter()
            .flatten()
        {
            slot.inherit(from);
        }
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Field {
            label,
            content,
            description,
            error,
            orientation,
            card,
            width,
        } = *self;

        // The label records presses for the field to activate its control.
        // A card is a label as a whole (shadcn wraps the field in its
        // `FieldLabel`), so it marks the entire field below instead.
        let label = label.map(
            |label| -> iced_core::Element<'a, Message, Theme, Renderer> {
                if card {
                    label.compose()
                } else {
                    marker::marker(label.compose(), widget::Label, Press::Any).into()
                }
            },
        );

        // Build the inner layout depending on orientation.
        let inner: iced_core::Element<'a, Message, Theme, Renderer> = match orientation {
            Orientation::Vertical => {
                // shadcn `field.tsx` line 55: `flex w-full gap-2` — 8px between
                // every present slot. Composition follows the conditional-push
                // pattern `alert::Alert::compose` uses.
                let mut col = iced_widget::Column::new().spacing(8);
                if let Some(label) = label {
                    col = col.push(label);
                }
                col = col.push(content.compose());
                if let Some(description) = description {
                    col = col.push(description.compose());
                }
                if let Some(error) = error {
                    col = col.push(error.compose());
                }
                col.boxed()
            }
            Orientation::Horizontal => {
                // shadcn `field.tsx` horizontal: `flex-row items-center
                // has-[>[data-slot=field-content]]:items-start`
                // The Center → Start switch is driven by FieldContent presence,
                // which in shadcn appears when there's a description / error /
                // multi-element block alongside the label. We mirror this by
                // checking whether description or error is set: with neither,
                // the right column is just the label (single-line) and
                // alignment stays Center; with either, the right column is a
                // multi-line stack and alignment switches to Start.
                // Slot order: content (left) — FieldContent column (right).
                // FieldContent inner: `gap-0.5` (2px). Row gap: `gap-2` (8px).
                let has_right = label.is_some() || description.is_some() || error.is_some();
                if has_right {
                    let has_field_content = description.is_some() || error.is_some();
                    let align = if has_field_content {
                        iced_core::Alignment::Start
                    } else {
                        iced_core::Alignment::Center
                    };

                    let content_element = content.compose();
                    let mut right = iced_widget::Column::new().spacing(2);
                    if let Some(label) = label {
                        right = right.push(label);
                    }
                    if let Some(description) = description {
                        right = right.push(description.compose());
                    }
                    if let Some(error) = error {
                        right = right.push(error.compose());
                    }
                    iced_widget::Row::new()
                        .spacing(8)
                        .align_y(align)
                        .push(content_element)
                        .push(right.boxed())
                        .boxed()
                } else {
                    // No right-hand content — just content on its own in a row.
                    iced_widget::Row::new()
                        .spacing(8)
                        .align_y(iced_core::Alignment::Center)
                        .push(content.compose())
                        .boxed()
                }
            }
        };

        let inner = if card {
            marker::marker(inner, widget::Label, Press::Any).into()
        } else {
            inner
        };
        let field = widget::Field::new(inner);
        let field: iced_core::Element<'a, Message, Theme, Renderer> = if card {
            // shadcn FieldLabel: `has-[>[data-slot=field]]:rounded-lg
            //   has-[>[data-slot=field]]:border *:data-[slot=field]:p-2.5
            //   has-data-checked:border-primary/30 has-data-checked:bg-primary/5`
            field.card(Card.into(), CardActive.into(), 10).into()
        } else {
            field.into()
        };

        // Optionally apply the outer width via a thin container wrapper.
        // Cheaper than threading width through every layout branch (Row, Column,
        // AnimatedCard, mouse_area), and matches the convention of similar
        // wrapper widgets.
        match width {
            Some(w) => iced_widget::container(field).width(w).boxed(),
            None => field,
        }
    }
}

// ---------------------------------------------------------------------------
// Hybrid string / pre-styled intermediary traits
//
// Each slot accepts either a plain string (auto-styled with the matching
// typography helper) or a pre-styled `Text<'a>` / `Slot<'a, Message>`.
// Implemented as three separate marker traits — one per slot — with concrete
// impls per acceptable input type. No blanket `impl<T: Into<Slot>>`,
// because that would conflict with the auto-styling impls on `&str` /
// `String` (a string is `Into<Slot>` via the bare-text path that
// skips the typography helper).
//
// Trait names are deliberately `IntoLabel` / `IntoDescription` / `IntoError`
// — NOT `IntoFieldLabel` / `IntoFieldDescription` / `IntoFieldError` — so
// the module path (`field::IntoLabel`) carries the "field" disambiguation
// per the project's "no composite names" rule.
// ---------------------------------------------------------------------------

/// Convert a value into a [`Field`] label slot. Plain strings are auto-styled
/// as `text::label`; pre-styled `Text` / `Slot` values pass through.
pub trait IntoLabel<'a, Message> {
    fn into_label(self) -> Slot<'a, Message>;
}

impl<'a, Message: 'a> IntoLabel<'a, Message> for &'a str {
    fn into_label(self) -> Slot<'a, Message> {
        crate::text::label(self).into()
    }
}

impl<'a, Message: 'a> IntoLabel<'a, Message> for String {
    fn into_label(self) -> Slot<'a, Message> {
        crate::text::label(self).into()
    }
}

impl<'a, Message: 'a> IntoLabel<'a, Message> for crate::text::Text<'a> {
    fn into_label(self) -> Slot<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoLabel<'a, Message> for Slot<'a, Message> {
    fn into_label(self) -> Slot<'a, Message> {
        self
    }
}

/// Convert a value into a [`Field`] description slot. Plain strings are
/// auto-styled as `text::muted`; pre-styled values pass through.
pub trait IntoDescription<'a, Message> {
    fn into_description(self) -> Slot<'a, Message>;
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for &'a str {
    fn into_description(self) -> Slot<'a, Message> {
        crate::text::muted(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for String {
    fn into_description(self) -> Slot<'a, Message> {
        crate::text::muted(self).into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for crate::text::Text<'a> {
    fn into_description(self) -> Slot<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoDescription<'a, Message> for Slot<'a, Message> {
    fn into_description(self) -> Slot<'a, Message> {
        self
    }
}

/// Convert a value into a [`Field`] error slot. Plain strings are
/// auto-styled as `text::error`; pre-styled values pass through.
pub trait IntoError<'a, Message> {
    fn into_error(self) -> Slot<'a, Message>;
}

impl<'a, Message: 'a> IntoError<'a, Message> for &'a str {
    fn into_error(self) -> Slot<'a, Message> {
        crate::text::error(self).into()
    }
}

impl<'a, Message: 'a> IntoError<'a, Message> for String {
    fn into_error(self) -> Slot<'a, Message> {
        crate::text::error(self).into()
    }
}

impl<'a, Message: 'a> IntoError<'a, Message> for crate::text::Text<'a> {
    fn into_error(self) -> Slot<'a, Message> {
        self.into()
    }
}

impl<'a, Message: 'a> IntoError<'a, Message> for Slot<'a, Message> {
    fn into_error(self) -> Slot<'a, Message> {
        self
    }
}

#[cfg(test)]
mod tests {
    //! Composition-shape coverage for the alert-style builder loop.
    //!
    //! These tests inspect the builder fields BEFORE `compose` runs —
    //! once an iced widget is produced, the slot identities are erased.
    //! Asserting on the `Option<Slot>` shape here proves the conditional-
    //! push pattern in `compose` won't try to render empty slots.

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

    #[test]
    fn group_defaults_to_field_group_spacing() {
        let g: field::Group<'_, Msg> =
            field::group([field::field("First").into(), field::field("Second").into()]);

        assert_eq!(g.spacing, 20.0);
        assert_eq!(g.width, iced_core::Length::Fill);
    }

    /// Horizontal orientation is the default-opposite — builder field
    /// tracks what was set.
    #[test]
    fn horizontal_orientation_is_set() {
        let f: field::Field<'_, Msg> = field::field("content")
            .orientation(field::Orientation::Horizontal)
            .label("Label")
            .description("Desc");
        assert_eq!(f.orientation, field::Orientation::Horizontal);
        assert!(f.label.is_some());
        assert!(f.description.is_some());
        assert!(!f.card, "no card set by default");
    }

    #[test]
    fn card_is_set() {
        let f: field::Field<'_, Msg> = field::field("content").card();
        assert!(f.card);
    }
}
