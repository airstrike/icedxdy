use iced_core as core;

pub mod accordion;
pub mod alert;
pub mod animation;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod checkbox;
pub mod column;
pub mod component;
pub mod container;
pub mod empty;
pub mod field;
pub mod input;
pub mod item;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod operation;
pub mod progress;
pub mod rich_text;
pub mod row;
pub mod rule;
pub mod scrollable;
pub mod separator;
pub mod sidebar;
pub mod size;
pub mod skeleton;
pub mod slider;
#[cfg(feature = "spinner")]
pub mod spinner;
pub mod switch;
pub mod table;
pub mod text;
pub mod text_input;
pub mod toggle_group;
pub mod weight;

pub use iced_widget::Radio;
pub use iced_widget::space;
pub use iced_widget::{QRCode, qr_code};

pub use accordion::Accordion;
pub use alert::Alert;
pub use aspect_ratio::AspectRatio;
pub use avatar::Avatar;
pub use badge::Badge;
pub use breadcrumb::Breadcrumb;
pub use button::Button;
pub use button::ButtonGroup;
pub use checkbox::Checkbox;
pub use column::Column;
pub use component::{Component, IntoElement};
pub use empty::Empty;
pub use field::Field;
pub use input::Input;
pub use item::{Item, Media};
pub use progress::Progress;
pub use rich_text::{RichText, Span, rich_text, span};
pub use row::Row;
pub use size::Size;
pub use skeleton::Skeleton;
pub use slider::Slider;
pub use switch::Switch;
pub use table::Table;
pub use text::Text;
pub use toggle_group::ToggleGroup;
pub use weight::Weight;

/// Create a [`Button`] with the given content and default style.
pub fn button<'a, Message: Clone + 'a>(
    content: impl Into<Component<'a, Message>>,
) -> Button<'a, Message> {
    button::button(content)
}

/// Create a [`Text`] widget at small size — matches shadcn's default body
/// text (`text-sm`, 14px). For other sizes use the explicit helpers
/// (`text::h1`, `text::large`, `text::muted`, etc.).
pub fn text<'a>(content: impl iced_core::text::IntoFragment<'a>) -> Text<'a> {
    text::small(content)
}

/// Create a [`Column`] with the given children.
pub fn column<'a, Message: 'a>(
    children: impl IntoIterator<Item = component::Component<'a, Message>>,
) -> Column<'a, Message> {
    Column::with_children(children)
}

/// Create a [`Container`] wrapping the given content.
pub fn container<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> container::Container<'a, Message> {
    container::Container::new(content)
}

/// Create a [`Container`] that centers its content (both axes), filling
/// the available space. Mirrors `iced::widget::center` but returns our
/// wrapped Container so the tweak-aware `.style()` is available.
pub fn center<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> container::Container<'a, Message> {
    container::Container::new(content).center(iced_core::Length::Fill)
}

/// Create a [`Row`].
pub fn row<'a, Message: 'a>() -> Row<'a, Message> {
    Row::new()
}

/// Create a nova-styled [`Slider`].
pub fn slider<'a, T, Message>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> Slider<'a, T, Message>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
{
    slider::slider(range, value, on_change)
}

/// Create a [`scrollable::Scrollable`] wrapping the given content. The
/// project-wide default style is the thin invisible-rail spec from
/// the `default` fn in core's theme/scrollable — 1px transparent rail,
/// 2px scroller at faint `palette.muted.foreground` alpha. Variant
/// overrides via `.class(theme::scrollable::Hidden)` and friends.
pub fn scrollable<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
) -> scrollable::Scrollable<'a, Message> {
    scrollable::scrollable(content)
}

/// Create a [`Table`] with the given table child slots.
pub fn table<'a, Message: 'a, C>(children: impl IntoIterator<Item = C>) -> Table<'a, Message>
where
    C: Into<Component<'a, Message>>,
{
    table::table(children)
}

/// Create a [`Skeleton`] loading placeholder.
pub fn skeleton<'a, Message: 'a>() -> Skeleton<'a, Message> {
    skeleton::skeleton()
}

/// Create a stock iced [`Radio`] styled by the icedxdy [`Theme`].
pub fn radio<'a, Message, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> Message,
) -> Radio<'a, Message, icedxdy_core::Theme>
where
    Message: Clone + 'a,
    V: Eq + Copy,
{
    iced_widget::radio(label, value, selected, on_click)
}

/// Create a [`ToggleGroup`] from toggle-group items.
pub fn toggle_group<'a, Message: Clone + 'a>(
    items: impl IntoIterator<Item = toggle_group::Item<'a, Message>>,
) -> ToggleGroup<'a, Message> {
    toggle_group::toggle_group(items)
}

/// Create a [`Badge`] with the given content.
pub fn badge<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Badge<'a, Message> {
    badge::badge(content)
}

/// Create a [`Breadcrumb`] from a list of items.
///
/// Item constructors stay namespaced under [`breadcrumb`] (`breadcrumb::link`,
/// `breadcrumb::page`, `breadcrumb::ellipsis`) — the bare crate-root re-export
/// is the trail builder, not the items.
pub fn breadcrumb<'a, Message: 'a>(
    items: impl IntoIterator<Item = breadcrumb::Item<'a, Message>>,
) -> Breadcrumb<'a, Message> {
    breadcrumb::breadcrumb(items)
}

/// Create an [`Alert`].
pub fn alert<'a, Message: 'a>() -> Alert<'a, Message> {
    alert::alert()
}

/// Create an [`AspectRatio`] wrapper that constrains `content` to a
/// `width / height` ratio. Mirrors shadcn's `<AspectRatio ratio={...}>`.
pub fn aspect_ratio<'a, Message: 'a>(
    content: impl Into<Component<'a, Message>>,
    ratio: f32,
) -> AspectRatio<'a, Message> {
    aspect_ratio::aspect_ratio(content, ratio)
}

/// Create an [`Avatar`] from an optional image handle.
///
/// Pass `None` to always render the fallback content; pass `Some(handle)`
/// to render the image clipped to a circle. See the `avatar` module for
/// the runtime image-load deviation note.
pub fn avatar<'a, Message: 'a>(image: Option<iced_widget::image::Handle>) -> Avatar<'a, Message> {
    avatar::avatar(image)
}

/// Create an empty [`Input`] with the given placeholder.
pub fn input<'a, Message: Clone + 'a>(
    placeholder: impl iced_core::text::IntoFragment<'a>,
) -> Input<'a, Message> {
    input::input(placeholder)
}

/// Create a [`Checkbox`] in the given state.
pub fn checkbox<'a, Message: 'a>(state: impl Into<checkbox::State>) -> Checkbox<'a, Message> {
    checkbox::checkbox(state)
}

/// Create a [`Progress`] bar with the given value (0..=100).
pub fn progress<'a, Message: 'a>(value: f32) -> Progress<'a, Message> {
    progress::progress(value)
}

/// Create a [`Field`] wrapping the given input content.
pub fn field<'a, Message: 'a>(content: impl Into<Component<'a, Message>>) -> Field<'a, Message> {
    field::field(content)
}

/// Create an [`Empty`] state root.
pub fn empty<'a, Message: 'a>() -> Empty<'a, Message> {
    empty::empty()
}

/// Create an [`Item`] from children.
pub fn item<'a, Message: 'a>(
    children: impl IntoIterator<Item = Component<'a, Message>>,
) -> Item<'a, Message> {
    item::item(children)
}

/// Create a [`Switch`] with the given on/off state.
pub fn switch<'a, Message: 'a>(is_toggled: bool) -> Switch<'a, Message> {
    switch::switch(is_toggled)
}

/// Create a [`Spinner`].
#[cfg(feature = "spinner")]
pub fn spinner<'a>() -> spinner::Spinner<'a> {
    spinner::Spinner::new()
}

/// Create an [`Accordion`] from an iterable of items.
pub fn accordion<'a, Message, I>(items: I) -> Accordion<'a, Message>
where
    Message: 'a,
    I: IntoIterator<Item = accordion::Item<'a, Message>>,
{
    accordion::accordion(items)
}
