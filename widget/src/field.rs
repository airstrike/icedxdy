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
//! // Explicit — pass any pre-styled `Text` (or `Component`) through unchanged.
//! field::field(input("Email", &value).on_input(Message::Changed))
//!     .label(text::label("Email"))
//!     .error(text::error("This email is already taken."));
//!
//! // Horizontal card — checkbox with label + description in a bordered card.
//! // Active tint tracks the checkbox state (caller wires both on_toggle and
//! // on_press, see `.on_press` docs).
//! field::field(checkbox(state).on_toggle(Message::Toggle))
//!     .orientation(field::Orientation::Horizontal)
//!     .label("Enable notifications")
//!     .description("You can enable or disable notifications at any time.")
//!     .card(state)
//!     .on_press(Message::Toggle(!state));
//! ```
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

mod animated_card;

use iced_core::Length;
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme::field::Card;
use icedxdy_core::theme::field::CardActive;

use crate::component::{Component, IntoElement};
use animated_card::AnimatedCard;

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

/// A labeled-input wrapper with optional card chrome and click-anywhere handling.
///
/// Shape: `transformation-method` (per the OPAQUE guide) — the constructor
/// is born from the inner content widget; builder methods carry forward state;
/// `into_element` consumes `self` into the final layout.
pub struct Field<'a, Message> {
    label: Option<Component<'a, Message>>,
    content: Component<'a, Message>,
    description: Option<Component<'a, Message>>,
    error: Option<Component<'a, Message>>,
    orientation: Orientation,
    /// `Some(true)` = card + active tint; `Some(false)` = card + inactive border;
    /// `None` = no card wrapper.
    card: Option<bool>,
    on_press: Option<Message>,
    width: Option<Length>,
}

/// A vertical group of fields.
///
/// Mirrors shadcn's `FieldGroup`: `flex w-full flex-col gap-5`. Nested
/// group-specific CSS variants are intentionally deferred until the matching
/// checkbox/radio group primitives exist.
pub struct Group<'a, Message> {
    children: Vec<Component<'a, Message>>,
    spacing: f32,
    width: Length,
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
        orientation: Orientation::Vertical,
        card: None,
        on_press: None,
        width: None,
    }
}

/// Create a [`Group`] wrapping related fields.
pub fn group<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Group<'a, Message> {
    Group {
        children: children.into_iter().collect(),
        spacing: 20.0,
        width: Length::Fill,
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
    /// `is_active = true` → primary-tinted border + bg (`border-primary/30`,
    /// `bg-primary/5`). Matches shadcn's `has-data-checked:border-primary/30
    /// has-data-checked:bg-primary/5` on `FieldLabel`.
    ///
    /// `is_active = false` → 1px `border-border`, transparent bg.
    ///
    /// The card adds 10px padding (`p-2.5`) on all sides and `rounded-lg`
    /// corners. Caller drives the active/inactive choice — iced has no
    /// `data-checked` CSS cascade.
    pub fn card(mut self, is_active: bool) -> Self {
        self.card = Some(is_active);
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
        Box::new(g).into_element()
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>> for Component<'a, Message> {
    fn from(g: Group<'a, Message>) -> Self {
        Component::new(g)
    }
}

impl<'a, Message: 'a> IntoElement<Message> for Group<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Group {
            children,
            spacing,
            width,
        } = *self;

        let elements: Vec<iced_core::Element<'a, Message, Theme, Renderer>> =
            children.into_iter().map(Component::into_iced).collect();

        iced_widget::Column::with_children(elements)
            .spacing(spacing)
            .width(width)
            .into()
    }
}

impl<'a, Message: Clone + 'a> Field<'a, Message> {
    /// Fire `message` when the user clicks anywhere in the field area.
    ///
    /// The inner content widget (e.g. a `Checkbox`) retains its own
    /// `.on_toggle(...)` handler. Iced's `mouse_area` propagates the press
    /// event up without consuming it from children, so clicking the checkbox
    /// itself fires the checkbox toggle; clicking the surrounding card area
    /// fires `on_press`. Wire both for full click-anywhere coverage:
    ///
    /// ```ignore
    /// field::field(checkbox(state).on_toggle(Message::Toggle(true)))
    ///     .on_press(Message::Toggle(!state))
    /// ```
    ///
    /// Callers should use idempotent state-set messages (set `true`/`false`)
    /// rather than flip-flop toggles when both handlers fire on the same
    /// click, since iced's mouse_area may fire on_press for clicks that also
    /// reach child widgets.
    ///
    /// Requires `Message: Clone` (iced's `mouse_area` clones the message
    /// for each press event).
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<'a, Message: Clone + 'a> From<Field<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(f: Field<'a, Message>) -> Self {
        Box::new(f).into_element()
    }
}

impl<'a, Message: Clone + 'a> From<Field<'a, Message>> for Component<'a, Message> {
    fn from(f: Field<'a, Message>) -> Self {
        Component::new(f)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<Message> for Field<'a, Message> {
    fn into_element<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
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
            on_press,
            width,
        } = *self;

        // Read toggle_target BEFORE consuming content. Field uses this to wrap
        // the rendered label in mouse_area, matching shadcn's `<label for>`
        // click-target semantic. Message: Clone is already on the impl.
        let toggle_target: Option<Message> = content.toggle_target().cloned();

        // Build the inner layout depending on orientation.
        let inner: iced_core::Element<'a, Message, Theme, Renderer> = match orientation {
            Orientation::Vertical => {
                // shadcn `field.tsx` line 55: `flex w-full gap-2` — 8px between
                // every present slot. Composition follows the conditional-push
                // pattern `alert::Alert::into_element` uses.
                let mut col = iced_widget::Column::new().spacing(8);
                if let Some(label) = label {
                    let label_element = label.into_iced();
                    let label_element = match toggle_target.as_ref() {
                        Some(target) => iced_widget::mouse_area(label_element)
                            .on_press(target.clone())
                            .into(),
                        None => label_element,
                    };
                    col = col.push(label_element);
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

                    let content_element = content.into_iced();
                    let mut right = iced_widget::Column::new().spacing(2);
                    if let Some(label) = label {
                        // Wrap label in mouse_area when a toggle target is
                        // present — mirrors shadcn's `<FieldLabel htmlFor>`
                        // that fires a synthetic click on the linked control.
                        // ONLY the label is wrapped; description and error are
                        // passive text (no htmlFor in shadcn's cascade).
                        let label_element = label.into_iced();
                        let label_element = match toggle_target.as_ref() {
                            Some(target) => iced_widget::mouse_area(label_element)
                                .on_press(target.clone())
                                .into(),
                            None => label_element,
                        };
                        right = right.push(label_element);
                    }
                    if let Some(description) = description {
                        right = right.push(description.into_iced());
                    }
                    if let Some(error) = error {
                        right = right.push(error.into_iced());
                    }
                    iced_widget::Row::new()
                        .spacing(8)
                        .align_y(align)
                        .push(content_element)
                        .push(right)
                        .into()
                } else {
                    // No right-hand content — just content on its own in a row.
                    iced_widget::Row::new()
                        .spacing(8)
                        .align_y(iced_core::Alignment::Center)
                        .push(content.into_iced())
                        .into()
                }
            }
        };

        // Optionally wrap in a card container.
        // shadcn FieldLabel: `has-[>[data-slot=field]]:rounded-lg
        //   has-[>[data-slot=field]]:border
        //   *:data-[slot=field]:p-2.5
        //   has-data-checked:border-primary/30
        //   has-data-checked:bg-primary/5`
        //
        // We swap stock `iced_widget::container` for an [`AnimatedCard`]
        // when card mode is on — the card chrome (border + bg) cross-fades
        // in lockstep with the inner checkbox / switch when `is_active`
        // toggles, matching shadcn's `transition-colors` cascade. The
        // bezier curve and easing match the inner checkbox + switch
        // widgets exactly so a card + its inner control animate as one
        // synchronized state change.
        let carded: iced_core::Element<'a, Message, Theme, Renderer> = match card {
            Some(is_active) => {
                let inactive: iced_widget::container::StyleFn<'_, Theme> = Card.into();
                let active: iced_widget::container::StyleFn<'_, Theme> = CardActive.into();
                AnimatedCard::new(inner, is_active, inactive, active)
                    .padding(10)
                    .into()
            }
            None => inner,
        };

        // Optionally wrap in a mouse_area for click-anywhere handling.
        let pressed: iced_core::Element<'a, Message, Theme, Renderer> = match on_press {
            Some(message) => iced_widget::mouse_area(carded).on_press(message).into(),
            None => carded,
        };

        // Optionally apply the outer width via a thin container wrapper.
        // Cheaper than threading width through every layout branch (Row, Column,
        // AnimatedCard, mouse_area), and matches the convention of similar
        // wrapper widgets.
        match width {
            Some(w) => iced_widget::container(pressed).width(w).into(),
            None => pressed,
        }
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
        assert!(f.card.is_none(), "no card set by default");
    }

    /// `.card(false)` populates the card slot with `Some(false)` (inactive).
    #[test]
    fn card_inactive_is_set() {
        let f: field::Field<'_, Msg> = field::field("content").card(false);
        assert_eq!(
            f.card,
            Some(false),
            "card(false) must set card = Some(false)"
        );
        assert!(f.on_press.is_none());
    }

    /// `.card(true)` populates the card slot with `Some(true)` (active).
    #[test]
    fn card_active_is_set() {
        let f: field::Field<'_, Msg> = field::field("content").card(true);
        assert_eq!(f.card, Some(true), "card(true) must set card = Some(true)");
    }
}
