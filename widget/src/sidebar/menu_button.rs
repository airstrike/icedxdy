//! SidebarMenuButton — interactive sidebar menu item
//!
//! Uses our raw button widget with `translate(false)` — NO press-down effect.
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
//!     .size(MenuButtonSize::Sm)
//!     .on_press(Message::Settings)
//! ```

use iced_core::Length::{Fill, Shrink};
use iced_core::Padding;
use iced_core::mouse::Interaction;
use iced_widget::Renderer;
use iced_widget::button::Status;
use iced_widget::tooltip::Position;

use crate::Size;
use crate::button::widget;
use crate::component::{Component, IntoElement};
use icedxdy_core::Theme;
use icedxdy_core::theme::button;

/// Menu button size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuButtonSize {
    #[default]
    Default, // h-8 = 32px, text-sm
    Sm, // h-7 = 28px, text-xs
    Lg, // h-12 = 48px, text-sm
    /// Docs-sidebar override (`apps/v4/components/docs-sidebar.tsx`):
    /// `h-[30px] text-[0.8rem] font-medium` with `border border-transparent`.
    /// We render as h=32 + inset(1) elsewhere, so the painted fill lands
    /// at 30 px while the 1px transparent halo around the fill mirrors the
    /// `border border-transparent` on the canonical class.
    Docs,
}

impl MenuButtonSize {
    pub fn height(&self) -> f32 {
        match self {
            Self::Default | Self::Docs => 32.0,
            Self::Sm => 28.0,
            Self::Lg => 48.0,
        }
    }

    pub fn text_size(&self) -> f32 {
        match self {
            Self::Default | Self::Lg => 14.0, // text-sm
            Self::Sm => 12.0,                 // text-xs
            Self::Docs => 13.0,               // text-[0.8rem] ≈ 12.8
        }
    }

    pub fn padding(&self) -> Padding {
        match self {
            Self::Default => Padding::from([4.0, 8.0]),
            Self::Sm => Padding::from([2.0, 8.0]),
            Self::Lg => Padding::from([8.0, 8.0]),
            // Canonical docs-sidebar items use `p-2` (8 px on every side).
            Self::Docs => Padding::from([8.0, 8.0]),
        }
    }
}

/// A sidebar menu button.
///
/// Uses our raw button widget with `translate(false)` so there is no
/// 1px press-down effect. Supports active state, collapsed mode, and tooltips.
pub struct SidebarMenuButton<'a, Message> {
    content: Component<'a, Message>,
    size: MenuButtonSize,
    active: bool,
    collapsed: bool,
    inset: Padding,
    tooltip: Option<String>,
    on_press: Option<Message>,
}

/// Create a sidebar menu button with default size.
pub fn menu_button<'a, Message: Clone + 'a>(
    content: impl Into<Component<'a, Message>>,
) -> SidebarMenuButton<'a, Message> {
    SidebarMenuButton {
        content: content.into(),
        size: MenuButtonSize::Default,
        active: false,
        collapsed: false,
        inset: Padding::ZERO,
        tooltip: None,
        on_press: None,
    }
}

impl<'a, Message: Clone + 'a> SidebarMenuButton<'a, Message> {
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

    /// Transparent inset between the button's bounds (hit-test + hover
    /// region) and the painted bg/border. Lets a parent column keep
    /// `spacing(0)` so adjacent items share boundaries (continuous hover,
    /// no dead-zone gap) while the visible fills sit apart by
    /// `inset.top + inset.bottom`. The docs-sidebar override pairs
    /// `inset(1)` with [`MenuButtonSize::Docs`] to mirror the canonical
    /// `border border-transparent` 1px halo.
    pub fn inset(mut self, inset: impl Into<Padding>) -> Self {
        self.inset = inset.into();
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

// -- From impls ---------------------------------------------------------------

impl<'a, Message: Clone + 'a> From<SidebarMenuButton<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(mut btn: SidebarMenuButton<'a, Message>) -> Self {
        let text_size = btn.size.text_size();
        btn.content.size(Size::Custom(text_size));
        btn.content.weight(crate::Weight::Medium);
        // Collapse the line-box to the Docs content area (16px = 32px -
        // 8/8 padding). Default 1.3 leading on text-13 produces a ~17px
        // box that overflows by 1px DOWN — iced's `padded` layout puts
        // the line-box at the TOP of the content area, leaving the glyph
        // ~1px below center. Pin to 16 so the baseline lands at the
        // midpoint without spilling. Other sizes have ample content slack;
        // `center_y(Fill)` below handles them.
        btn.content.line_height(16.0);

        let content_iced = btn.content.into_iced();
        let height = btn.size.height();

        // Use our raw button widget with translate(false) — no press-down
        let mut raw_btn = if btn.collapsed {
            widget::Button::new(iced_widget::center(content_iced))
                .width(height)
                .height(height)
                .padding(0)
        } else {
            // Width(Shrink) for both the inner container and the outer
            // button — shadcn's docs-sidebar items use `w-fit`, so the
            // active-state bg paint hugs the text + padding rather than
            // stretching the full sidebar column. Hit area follows.
            //
            // `center_y(Fill)` centers the line-box within the post-padding
            // content area for sizes whose content slack > line-box (Default,
            // Sm, Lg). Docs (16px content / 16px line-box) doesn't need it
            // but tolerates it — combination handles all sizes.
            widget::Button::new(
                iced_widget::container(content_iced)
                    .width(Shrink)
                    .center_y(Fill),
            )
            .width(Shrink)
            .height(height)
            .padding(btn.size.padding())
        };

        raw_btn = raw_btn
            .translate(false)
            .interaction(Interaction::Pointer)
            .inset(btn.inset);

        // Dispatch into existing theme::button variants. `button::sidebar`
        // (inactive) hovers with `palette.sidebar.accent`; `button::sidebar_active`
        // (active) uses the main `palette.accent` per the docs-sidebar override.
        // The chained `.into()` lands core::Style → iced::Style → widget::Style.
        let style: widget::StyleFn<'a, Theme> = if btn.active {
            Box::new(move |t, s: widget::Status| {
                let status: Status = s.into();
                let iced: iced_widget::button::Style = button::sidebar_active(t, status).into();
                iced.into()
            })
        } else {
            Box::new(move |t, s: widget::Status| {
                let status: Status = s.into();
                let iced: iced_widget::button::Style = button::sidebar(t, status).into();
                iced.into()
            })
        };
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
