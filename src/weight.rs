//! Font weight for icedxdy components.

/// Font weight, matching CSS/shadcn conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    /// 100
    Thin,
    /// 200
    ExtraLight,
    /// 300
    Light,
    /// 400
    Normal,
    /// 500
    Medium,
    /// 600
    Semibold,
    /// 700
    Bold,
    /// 800
    ExtraBold,
    /// 900
    Black,
    /// Arbitrary weight (1–999)
    Custom(u16),
}

impl Default for Weight {
    fn default() -> Self {
        Weight::Normal
    }
}

impl From<u16> for Weight {
    fn from(v: u16) -> Self {
        Weight::Custom(v)
    }
}

impl From<Weight> for iced_core::font::Weight {
    fn from(w: Weight) -> Self {
        match w {
            Weight::Thin => iced_core::font::Weight::Thin,
            Weight::ExtraLight => iced_core::font::Weight::ExtraLight,
            Weight::Light => iced_core::font::Weight::Light,
            Weight::Normal => iced_core::font::Weight::Normal,
            Weight::Medium => iced_core::font::Weight::Medium,
            Weight::Semibold => iced_core::font::Weight::Semibold,
            Weight::Bold => iced_core::font::Weight::Bold,
            Weight::ExtraBold => iced_core::font::Weight::ExtraBold,
            Weight::Black => iced_core::font::Weight::Black,
            Weight::Custom(v) => match v {
                0..=149 => iced_core::font::Weight::Thin,
                150..=249 => iced_core::font::Weight::ExtraLight,
                250..=349 => iced_core::font::Weight::Light,
                350..=449 => iced_core::font::Weight::Normal,
                450..=549 => iced_core::font::Weight::Medium,
                550..=649 => iced_core::font::Weight::Semibold,
                650..=749 => iced_core::font::Weight::Bold,
                750..=849 => iced_core::font::Weight::ExtraBold,
                _ => iced_core::font::Weight::Black,
            },
        }
    }
}
