//! Sidebar state management

/// Which side the sidebar is on
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Side {
    Left,
    Right,
}

/// How the sidebar collapses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Collapsible {
    Offcanvas,
    Icon,
    None,
}

/// Default sidebar width (256px)
pub const WIDTH: f32 = 256.0;
/// Mobile sidebar width (288px)
pub const WIDTH_MOBILE: f32 = 288.0;
/// Collapsed icon-only width (48px)
pub const WIDTH_ICON: f32 = 48.0;

/// Tracks sidebar open/collapsed state
#[derive(Debug, Clone, Copy)]
pub struct State {
    pub open: bool,
    pub side: Side,
    pub collapsible: Collapsible,
}

impl State {
    pub fn new() -> Self {
        Self {
            open: true,
            side: Side::Left,
            collapsible: Collapsible::Offcanvas,
        }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn is_collapsed(&self) -> bool {
        !self.open && self.collapsible != Collapsible::None
    }

    pub fn width(&self) -> f32 {
        if self.is_collapsed() {
            match self.collapsible {
                Collapsible::Icon => WIDTH_ICON,
                _ => 0.0,
            }
        } else {
            WIDTH
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
