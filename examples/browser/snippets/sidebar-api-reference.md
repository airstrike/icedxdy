## API Reference

The sidebar suite is a family of helpers, not a single component. Compose the canonical shadcn shape by stacking constructors at the call site.

### Sidebar root constructors

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::fixed(content)` | `Container<'a>` | Sidebar flush against the window edge — solid background, no margin. Mirrors shadcn's default `sidebar` variant. |
| `sidebar::floating(content)` | `Container<'a>` | Sidebar floating with margin, rounded corners, border, and shadow. Mirrors shadcn's `floating` variant. |
| `sidebar::inset(content)` | `Container<'a>` | Inset sidebar — no border on the sidebar itself; the content area (paired via `sidebar::main_styled`) carries the rounded/border/shadow chrome. Mirrors shadcn's `inset` variant. |

These return a stock `iced_widget::Container`, so all the standard container builders chain (`.padding(...)`, `.width(...)`, etc.).

### Section helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::header(content)` | `Container` | Top section — typically holds the brand badge + a `sidebar::trigger` for collapse. Mirrors shadcn's `SidebarHeader`. |
| `sidebar::content(content)` | `Container` | Scrollable middle section that hosts groups + menus. Mirrors shadcn's `SidebarContent`. |
| `sidebar::footer(content)` | `Container` | Bottom section — typically a `sidebar::menu(...)` with an account / settings button. Mirrors shadcn's `SidebarFooter`. |
| `sidebar::group(content)` | `Container` | A logical group inside `content`. Mirrors shadcn's `SidebarGroup`. |
| `sidebar::group_label(label)` | `Text<'a>` | Group heading (`text-xs uppercase tracking-wider muted-foreground`). Mirrors shadcn's `SidebarGroupLabel`. |
| `sidebar::group_content(content)` | `Container` | Inner container for a group's items. Mirrors shadcn's `SidebarGroupContent`. |
| `sidebar::separator()` | `Container` | 1px horizontal divider scoped to the sidebar palette. Mirrors shadcn's `SidebarSeparator`. |
| `sidebar::menu(items)` | `Column` | Vertical list of menu items at consistent spacing. Mirrors shadcn's `SidebarMenu`. |
| `sidebar::menu_item(content)` | `Component` | Wraps a single menu row (typically a `menu_button`). Mirrors shadcn's `SidebarMenuItem`. |
| `sidebar::input(...)` | `Component` | Inline search box scoped to the sidebar palette. Mirrors shadcn's `SidebarInput`. |

### `sidebar::menu_button()`

The interactive sidebar menu item — NOT our `Button`. Uses the raw button widget with `translate(false)` so there's no press-down effect; sidebar palette (`palette.sidebar.accent`) for hover, main `palette.accent` for active.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `MenuButtonSize` | `Default` (32px) | Size variant: `Sm` (28px), `Default` (32px), `Lg` (48px). Drives both height and text size. |
| `.active(bool)` | `bool` | `false` | Selected-state — switches palette to main `palette.accent` and bumps weight to medium. |
| `.collapsed(bool)` | `bool` | `false` | Icon-only mode for `Collapsible::Icon` sidebars. Pairs with `.tooltip(...)` for the right-side tooltip. |
| `.tooltip(text)` | `impl ToString` | `None` | Tooltip rendered when collapsed (only fires while `.collapsed(true)`). |
| `.on_press(message)` | `Message` | `None` | Click handler. |

### Sub-menu helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::submenu(items)` | `Column` | Indented sub-list under a `menu_item`. Mirrors shadcn's `SidebarMenuSub`. |
| `sidebar::submenu_item(content)` | `Component` | One row inside a submenu. Mirrors shadcn's `SidebarMenuSubItem`. |
| `sidebar::submenu_button(content)` | `SidebarMenuButton` | Smaller variant of `menu_button` for submenu rows. Mirrors shadcn's `SidebarMenuSubButton`. |
| `sidebar::menu_action(content)` | `SidebarMenuAction` | Trailing action chip (typically a small icon button) on a menu row. Mirrors shadcn's `SidebarMenuAction`. |
| `sidebar::menu_badge(content)` | `Component` | Trailing badge (count, status indicator) on a menu row. Mirrors shadcn's `SidebarMenuBadge`. |
| `sidebar::menu_skeleton()` | `Container` | Loading placeholder for a menu row. Mirrors shadcn's `SidebarMenuSkeleton`. |

### Layout helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::trigger(on_press)` | `SidebarTrigger` | Collapse / expand toggle button (icon-only). Mirrors shadcn's `SidebarTrigger`. |
| `sidebar::rail(on_press)` | `SidebarRail` | Edge-grab handle for resize / toggle. Mirrors shadcn's `SidebarRail`. |
| `sidebar::main(content)` | `Container` | The main content area paired with a fixed/floating sidebar. Mirrors shadcn's `SidebarInset` for the flush variants. |
| `sidebar::main_styled(content)` | `Container` | The main content area paired with `sidebar::inset` — carries the rounded/border/shadow chrome that the inset sidebar drops. |

### State

| Type / const | Description |
| --- | --- |
| `sidebar::State` | `{ open: bool, side: Side, collapsible: Collapsible }`. Caller-owned. `.toggle()`, `.is_collapsed()`, `.width()` helpers. |
| `sidebar::Side` | `Left` / `Right`. Drives the chevron direction and the trigger orientation. |
| `sidebar::Collapsible` | `Offcanvas` (default — fully hides), `Icon` (collapses to `WIDTH_ICON`), `None`. |
| `sidebar::WIDTH` | `256.0` — default expanded width. |
| `sidebar::WIDTH_MOBILE` | `288.0` — mobile expanded width. |
| `sidebar::WIDTH_ICON` | `48.0` — collapsed icon-only width. |
| `sidebar::RAIL_WIDTH` | Width of the edge-grab `rail`. |

Shadcn's `SidebarProvider` context and `useSidebar` hook collapse into caller-owned `sidebar::State` in iced — thread the open/collapsed bool down through the view function rather than reaching for a context. Restyle via `core::theme::sidebar::*` style functions or compose your own surface helper.
