//! Media-slot framing shared by `ItemMedia` and `EmptyMedia`.
//!
//! shadcn's media `variant` classes bundle paint with geometry
//! (`size-8 rounded-lg bg-muted`, `size-10 overflow-hidden rounded-sm`), so a
//! media style is a [`Frame`]: a container [`Spec`] plus the square tile and
//! clipping the variant asks for.

use std::ops::Add;

use super::container::Spec;
use super::tweak::Tweak;

/// How a media slot frames its content: its paint, an optional square tile
/// size and whether the tile clips its content.
#[derive(Debug, Clone)]
pub struct Frame {
    paint: Spec,
    tile: Option<f32>,
    clip: bool,
}

impl Frame {
    pub(crate) fn new(paint: Spec, tile: Option<f32>, clip: bool) -> Self {
        Self { paint, tile, clip }
    }

    /// The container style painted behind the content.
    pub fn paint(&self) -> Spec {
        self.paint.clone()
    }

    /// The side of the square tile, in logical pixels, when the variant
    /// fixes one.
    pub fn tile(&self) -> Option<f32> {
        self.tile
    }

    /// Whether the tile clips its content (`overflow-hidden`).
    pub fn clip(&self) -> bool {
        self.clip
    }
}

impl<T: Into<Tweak>> Add<T> for Frame {
    type Output = Frame;

    fn add(mut self, rhs: T) -> Frame {
        self.paint = self.paint + rhs;
        self
    }
}
