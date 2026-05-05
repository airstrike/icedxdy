//! SidebarMenuButton — interactive sidebar menu item
//!
//! Uses `iced_widget::button()` (raw iced button) internally — NO press-down effect.
//!
//! # Example
//!
//! ```ignore
//! use icedxdy::widget::sidebar;
//!
//! sidebar::menu_button("Dashboard")
//!     .active(true)
//!     .on_press(Message::Navigate)
//!
//! sidebar::menu_button("Settings")
//!     .variant(MenuButtonVariant::Outline)
//!     .size(MenuButtonSize::Sm)
//!     .on_press(Message::Settings)
//! ```

use iced_core::mouse::Interaction;
use iced_core::Length::Fill;
use iced_core::{Alignment, Border};
use iced_widget::button::{Status, Style};
use iced_widget::tooltip::Position;
use iced_widget::Renderer;

use crate::button::widget;
use crate::component::{Component, IntoElement};
use crate::Size;
use icedxdy_core::radius;
use icedxdy_core::Theme;

/// Menu button variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuButtonVariant {
    #[default]
    Default,
    Outline,
}

/// Menu button size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuButtonSize {
    #[default]
    Default, // h-8 = 32px
    Sm,      // h-7 = 28px
    Lg,      // h-12 = 48px
}

impl MenuButtonSize {
    pub fn height(&self) -> f32 {
        match self {
            Self::Default => 32.0,
            Self::Sm => 28.0,
            Self::Lg => 48.0,
        }
    }

    pub fn text_size(&self) -> f32 {
        match self {
            Self::Default | Self::Lg => 14.0, // text-sm
            Self::Sm => 12.0,                  // text-xs
        }
    }

    pub fn padding(&self) -> iced_core::Padding {
        match self {
            Self::Default => iced_core::Padding::from([4.0, 8.0]),
            Self::Sm => iced_core::Padding::from([2.0, 8.0]),
            Self::Lg => iced_core::Padding::from([8.0, 8.0]),
        }
    }
}

/// A sidebar menu button.
///
/// Uses `iced_widget::button()` (raw iced button) internally so there is
/// no 1px press-down effect. Supports active state, collapsed mode, and tooltips.
pub struct SidebarMenuButton<'a, Message> {
    content: Component<'a, Message>,
    variant: MenuButtonVariant,
    size: MenuButtonSize,
    active: bool,
    collapsed: bool,
    tooltip: Option<String>,
    on_press: Option<Message>,
}

/// Create a sidebar menu button with default variant and size.
pub fn menu_button<'a, Message: Clone + 'a>(
    content: impl Into<Component<'a, Message>>,
) -> SidebarMenuButton<'a, Message> {
    SidebarMenuButton {
        content: content.into(),
        variant: MenuButtonVariant::Default,
        size: MenuButtonSize::Default,
        active: false,
        collapsed: false,
        tooltip: None,
        on_press: None,
    }
}

impl<'a, Message: Clone + 'a> SidebarMenuButton<'a, Message> {
    /// Set the button variant.
    pub fn variant(mut self, variant: MenuButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the button size.
    pub fn size(mut self, size: MenuButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether the button is in the active (selected) state.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set whether the sidebar is collapsed (icon-only mode).
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Set a tooltip to show when collapsed.
    pub fn tooltip(mut self, tooltip: impl ToString) -> Self {
        self.tooltip = Some(tooltip.to_string());
        self
    }

    /// Set the message to emit on press.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

// -- Style functions ----------------------------------------------------------
// These match shadcn's docs-sidebar.tsx override (what the rendered docs site
// actually shows): `font-medium` always, `bg-accent` (main palette) on active,
// muted text color for inactive. They do NOT match the base SidebarMenuButton
// CSS (which uses `bg-sidebar-accent`) — the icedxdy browser mimics the docs
// site, so docs-sidebar styling is the intended default here.

fn style_default(active: bool) -> impl Fn(&Theme, Status) -> Style {
    move |theme, status| {
        let p = theme.palette();

        let base = Style {
            background: if active {
                Some(p.accent.base.into())
            } else {
                None
            },
            text_color: if active {
                p.accent.foreground.into()
            } else {
                p.muted.foreground.into()
            },
            border: Border {
                radius: radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        };

        match status {
            Status::Active => base,
            Status::Hovered | Status::Pressed => Style {
                background: Some(p.accent.base.into()),
                text_color: p.accent.foreground.into(),
                ..base
            },
            Status::Disabled => Style {
                background: if active {
                    Some(p.accent.base.alpha(0.5).into())
                } else {
                    None
                },
                text_color: p.muted.foreground.alpha(0.5).into(),
                ..base
            },
        }
    }
}

fn style_outline(active: bool) -> impl Fn(&Theme, Status) -> Style {
    move |theme, status| {
        let p = theme.palette();

        let base = Style {
            background: if active {
                Some(p.accent.base.into())
            } else {
                Some(p.background.into())
            },
            text_color: if active {
                p.accent.foreground.into()
            } else {
                p.foreground.into()
            },
            border: Border {
                radius: radius::SM.into(),
                width: 1.0,
                color: p.border.into(),
            },
            ..Default::default()
        };

        match status {
            Status::Active => base,
            Status::Hovered | Status::Pressed => Style {
                background: Some(p.accent.base.into()),
                text_color: p.accent.foreground.into(),
                ..base
            },
            Status::Disabled => Style {
                background: if active {
                    Some(p.accent.base.alpha(0.5).into())
                } else {
                    Some(p.background.alpha(0.5).into())
                },
                text_color: p.foreground.alpha(0.5).into(),
                border: Border {
                    color: p.border.alpha(0.5).into(),
                    ..base.border
                },
                ..base
            },
        }
    }
}

// -- From impls ---------------------------------------------------------------

impl<'a, Message: Clone + 'a> From<SidebarMenuButton<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(mut btn: SidebarMenuButton<'a, Message>) -> Self {
        let text_size = btn.size.text_size();
        btn.content.size(Size::Custom(text_size));
        btn.content.weight(crate::Weight::Medium);

        let content_iced = btn.content.into_iced();
        let height = btn.size.height();

        // Use our raw button widget with translate(false) — no press-down
        let mut raw_btn = if btn.collapsed {
            widget::Button::new(iced_widget::center(content_iced))
                .width(height)
                .height(height)
                .padding(0)
        } else {
            widget::Button::new(
                iced_widget::container(content_iced)
                    .align_x(Alignment::Start)
                    .width(Fill),
            )
            .width(Fill)
            .height(height)
            .padding(btn.size.padding())
        };

        raw_btn = raw_btn
            .translate(false)
            .interaction(Interaction::Pointer);

        // Style: convert iced button::Style → our widget::Style via Into
        let active = btn.active;
        let variant = btn.variant;
        let style: widget::StyleFn<'a, Theme> = Box::new(move |t, s: widget::Status| {
            let status: Status = s.into();
            let style: Style = match variant {
                MenuButtonVariant::Default => style_default(active)(t, status),
                MenuButtonVariant::Outline => style_outline(active)(t, status),
            };
            style.into()
        });
        raw_btn = raw_btn.style(style);

        if let Some(on_press) = btn.on_press {
            raw_btn = raw_btn.on_press(on_press);
        }

        if btn.collapsed {
            if let Some(tooltip_text) = btn.tooltip {
                return iced_widget::tooltip(
                    raw_btn,
                    iced_widget::text(tooltip_text).size(12.0),
                    Position::Right,
                )
                .gap(4.0)
                .into();
            }
        }

        raw_btn.into()
    }
}

impl<'a, Message: Clone + 'a> From<SidebarMenuButton<'a, Message>>
    for crate::Component<'a, Message>
{
    fn from(b: SidebarMenuButton<'a, Message>) -> Self {
        crate::Component::new(b)
    }
}

impl<'a, Message: Clone + 'a> IntoElement<'a, Message> for SidebarMenuButton<'a, Message> {
    fn size(&mut self, _size: Size) {
        // SidebarMenuButton has its own size system (MenuButtonSize),
        // so we ignore parent size hints.
    }

    fn into_element(self: Box<Self>) -> iced_core::Element<'a, Message, Theme, Renderer> {
        (*self).into()
    }
}
