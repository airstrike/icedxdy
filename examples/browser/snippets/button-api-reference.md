## API Reference

### `button()`

Constructor for a default-variant `Button`. The single-arg constructor takes any `impl Into<Component>`; the `button!` macro wraps multiple children in a row.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | `Size::Default` | Size variant: `Xs`, `Sm`, `Default`, `Lg`. Drives both the button height and the propagated text/icon sizes. |
| `.square()` | — | — | Make the button square (icon-only). Width matches the size variant's height. |
| `.rounded()` | — | — | Pill-shaped (`rounded-full`). Combine with `.square()` for a circular icon button. |
| `.style(style)` | `impl Into<button::StyleFn>` | `theme::button::Default` | Override the button's style — accepts variant unit structs (`Outline`, `Secondary`, `Ghost`, `Destructive`, `Link`) or a hand-rolled `Spec`. |
| `.on_press(message)` | `Message` | `None` | Click handler. Omitting this disables the button. |
| `.weight(weight)` | `impl Into<Weight>` | propagated | Override the inherited font weight. |
| `.translate(bool)` | `bool` | `true` | Gates the 1px press-down effect (matches shadcn's `active:translate-y-px`). |
| `.interaction(interaction)` | `iced::mouse::Interaction` | size-dependent default | Per-button cursor override (Pointer for navigation, NotAllowed for soft-disabled, etc.). |

### Variant constructors

| Constructor | Variant |
| --- | --- |
| `button(...)` / `button::primary(...)` | `Default` (primary). |
| `button::secondary(...)` | `Secondary`. |
| `button::outline(...)` | `Outline`. |
| `button::ghost(...)` | `Ghost`. |
| `button::destructive(...)` | `Destructive`. |
| `button::icon(...)` | Square icon-only button (presets `.square()`). |
| `button::link(...)` | `Link` (text-only, primary color, underline on hover). |

### Variant macros

Each variant has a matching macro that wraps multiple children in a `Row` so labels and icons land side-by-side without manual layout. The leading-ident form rewrites `name` to `icon::name()` at the call site (so `use crate::icon;` must be in scope).

| Macro | Description |
| --- | --- |
| `button!["Submit"]` | Default variant, single child. |
| `button![icon::git_branch(), "New Branch"]` | Default variant, icon on the left. |
| `button![git_branch, "Branch"]` | Leading-ident sugar — equivalent to the previous form. |
| `button::outline!`, `button::secondary!`, `button::ghost!`, `button::destructive!` | Same arms as `button!`. |
| `button::icon![arrow_up]` | Icon-only button (bare ident). |
| `button::icon![icon::arrow_up]` | Icon-only button (path form). |

### `Size`

| Variant | Height | Notes |
| --- | --- | --- |
| `Size::Xs` | 24px | Mirrors shadcn `size="xs"`. |
| `Size::Sm` | 28px | Mirrors shadcn `size="sm"`. |
| `Size::Default` | 32px | Default. Matches shadcn's `h-8`. |
| `Size::Lg` | 36px | Mirrors shadcn `size="lg"`. |

Combine `.size(Sm)` + `.square()` to reproduce shadcn's `size="icon-sm"`, etc.

The TSX `className` prop is n/a in icedxdy; `asChild` is also n/a (use `mouse_area` or chain through your own composition). The TSX `variant` prop maps to the variant constructors above.
