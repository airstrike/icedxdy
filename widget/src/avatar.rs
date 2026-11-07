//! Avatar — circular image with text fallback.
//!
//! Mirrors shadcn nova's `<Avatar>`/`<AvatarImage>`/`<AvatarFallback>` triple.
//! Three sizes (`Sm` 24px, `Default` 32px, `Lg` 40px), an optional corner
//! [`badge`], and a [`group`] constructor for negative-space overlap with an
//! optional trailing count chip via [`group::Group::count`].
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::avatar;
//!
//! // Stand-alone, with an image and an initials fallback.
//! avatar(Some(my_image_handle)).fallback("CN")
//!
//! // No image — the fallback initials always render.
//! avatar(None).fallback("CN")
//!
//! // Sized — `.size(avatar::Lg)` for 40px, `.size(avatar::Sm)` for 24px.
//! avatar(None).fallback("ER").size(avatar::Lg)
//!
//! // Corner badge (presence dot).
//! avatar::badge(avatar(None).fallback("CN"), avatar::dot())
//!
//! // Group with three avatars and a `+3` count.
//! avatar::group([
//!     avatar(None).fallback("CN"),
//!     avatar(None).fallback("LR"),
//!     avatar(None).fallback("ER"),
//! ])
//! .count("+3")
//! ```
//!
//! # Image-load failure — explicit shadcn deviation
//!
//! Shadcn's `<AvatarImage>` falls back to `<AvatarFallback>` on **runtime**
//! image-load failure (Radix surfaces this via `onLoadingStatusChange`).
//! Iced's `Image` widget exposes no equivalent event surface, so our
//! [`avatar`] builder takes `Option<image::Handle>` and the fallback
//! triggers only when the caller passes `None`. **A `Some(handle)` whose
//! file/URL is broken will render iced's blank/broken-image affordance,
//! NOT the fallback.** Promoting runtime-detection requires an upstream
//! iced change to surface load status from the renderer; tracked as a
//! follow-up minimap.
//!
//! No fade-in animation either — shadcn's
//! `data-[state=loaded]:opacity-100` cascade depends on the same load
//! event we don't have. Avatars render at full opacity from frame 1.

pub mod badge;
pub mod group;

use iced_core::Length;
use iced_core::Length::Fill;
use iced_core::image::Handle;
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;
use icedxdy_core::theme::avatar::fallback_style;

use crate::compose::{Compose, Inherited, Slot};
use crate::container::Container;

#[doc(inline)]
pub use badge::{badge, dot};
#[doc(inline)]
pub use group::group;

// Re-export the size variant unit structs so callers reach them via
// `avatar::Sm`, `avatar::Default`, `avatar::Lg` — the same name shape the
// theme catalog uses, kept consistent across the call site.
#[doc(inline)]
pub use icedxdy_core::theme::avatar::{Default, Geometry, Lg, Sm};

/// Size variant for an [`Avatar`].
///
/// Stored as the same `Geometry` tuple the theme exposes so we don't
/// duplicate diameter / font-size / badge-size constants on the widget
/// side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size(Geometry);

impl Size {
    /// Resolved geometry for this size — diameter, font size, and badge
    /// chip dimensions in logical pixels.
    pub fn geometry(self) -> Geometry {
        self.0
    }
}

impl From<Default> for Size {
    fn from(_: Default) -> Self {
        Self(Geometry::DEFAULT)
    }
}

impl From<Sm> for Size {
    fn from(_: Sm) -> Self {
        Self(Geometry::SM)
    }
}

impl From<Lg> for Size {
    fn from(_: Lg) -> Self {
        Self(Geometry::LG)
    }
}

impl std::default::Default for Size {
    fn default() -> Self {
        Self(Geometry::DEFAULT)
    }
}

/// Builder for an avatar widget.
///
/// Born from an `Option<image::Handle>`: `Some(handle)` renders the image
/// (clipped to a circle); `None` renders [`fallback`](Self::fallback) text
/// centered on the muted surface. See the module docstring for the
/// runtime-load-failure caveat.
pub struct Avatar<'a, Message> {
    image: Option<Handle>,
    fallback: Option<Slot<'a, Message>>,
    size: Size,
    /// 2px outer ring colored to the page background. Set by
    /// [`group`](crate::avatar::group) on every avatar it composes so
    /// overlapping avatars look visually separated. Stand-alone avatars
    /// leave this `false`.
    ring: bool,
}

/// Create an [`Avatar`] from an optional image handle.
///
/// Pass `None` to always render the fallback content; pass `Some(handle)`
/// to render the image (clipped to a circle). See the module docstring
/// for the runtime-load-failure caveat.
pub fn avatar<'a, Message: 'a>(image: Option<Handle>) -> Avatar<'a, Message> {
    Avatar {
        image,
        fallback: None,
        size: Size::default(),
        ring: false,
    }
}

impl<'a, Message: 'a> Avatar<'a, Message> {
    /// Set the fallback content rendered when no image handle is provided.
    ///
    /// Typically the user's initials (`"CN"`, `"ER"`) but accepts any
    /// `Slot` — an icon, a single character, etc.
    pub fn fallback(mut self, content: impl Into<Slot<'a, Message>>) -> Self {
        self.fallback = Some(content.into());
        self
    }

    /// Set the avatar's size variant. Defaults to [`Default`] (32px).
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// Resolved geometry for this avatar — exposed so [`badge`] and
    /// [`group`] can compose the corner badge / count chip at the right
    /// dimensions without re-parsing the size.
    pub fn geometry(&self) -> Geometry {
        self.size.geometry()
    }

    /// Toggle the 2px outer ring. Internal API: [`group`] flips this on
    /// for every avatar it composes; stand-alone avatars don't expose it
    /// to keep the builder surface focused on shadcn parity.
    pub(crate) fn ring(mut self, ring: bool) -> Self {
        self.ring = ring;
        self
    }
}

impl<'a, Message: 'a> From<Avatar<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(avatar: Avatar<'a, Message>) -> Self {
        Box::new(avatar).compose()
    }
}

impl<'a, Message: 'a> From<Avatar<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Avatar<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Avatar<'a, Message> {
    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Avatar {
            image: handle,
            fallback,
            size,
            ring,
        } = *self;
        let geometry = size.geometry();
        let diameter = geometry.diameter;

        // Circular surface = container at fixed diameter, pill-radius
        // border. The fallback bg / text colors come from the theme
        // catalog so light/dark mode tracks correctly.
        let style_fn: iced_widget::container::StyleFn<'static, Theme> =
            Box::new(move |t| fallback_style(t, ring).into());

        // Inner content: image clipped to the circle if Some, fallback
        // text otherwise. iced's Image widget supports `border_radius`
        // directly — the pill clamp matches the surrounding container.
        let inner: Slot<'_, Message> = if let Some(handle) = handle {
            Slot::from(
                iced_widget::Image::new(handle)
                    .width(Fill)
                    .height(Fill)
                    .content_fit(iced_core::ContentFit::Cover)
                    .border_radius(u32::MAX as f32)
                    .boxed(),
            )
        } else {
            // Centered fallback. The fallback's text size is pushed down
            // through the standard size-propagation channel so a bare
            // string flows into `text::Text` and picks up the variant's
            // font size automatically.
            let body: Slot<'_, Message> = match fallback {
                Some(content) => content,
                None => Slot::from(""),
            };
            let mut sized = body;
            sized.inherit(&Inherited::default().with_size(crate::Size::Custom(geometry.font_size)));
            sized
        };

        let circle = Container::new(inner)
            .center(Length::from(diameter))
            .clip(true)
            .class(style_fn);
        iced_core::Element::from(circle)
    }
}
