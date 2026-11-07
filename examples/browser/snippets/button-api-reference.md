## API Reference

### `button()`

Constructor for a `Button`. The single-arg constructor takes any `impl Into<Slot>`; the `button!` macro wraps multiple children in a row. The variant is a style: `.style(button::Outline)`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | `Size::Default` | Size variant: `Xs`, `Sm`, `Default`, `Lg`. Drives both the button height and the propagated text/icon sizes. |
| `.square()` | — | — | Make the button square (icon-only). Width matches the size variant's height. |
| `.rounded()` | — | — | Pill-shaped (`rounded-full`). Combine with `.square()` for a circular icon button. |
| `.style(style)` | `impl Into<button::StyleFn>` | `button::Default` | Override the button's style — accepts variant unit structs (`Outline`, `Secondary`, `Ghost`, `Destructive`, `Link`) or a hand-rolled `Spec`. |
| `.on_press(message)` | `Message` | `None` | Click handler. A button without one stays enabled (a trigger marker can still observe it). |
| `.disabled(bool)` | `bool` | `false` | Disable the button (`disabled:opacity-50`, no input, skipped by focus). A disabled ancestor disables it too. |
| `.weight(weight)` | `impl Into<Weight>` | propagated | Override the inherited font weight. |
| `.translate(bool)` | `bool` | `true` | Gates the 1px press-down effect (matches shadcn's `active:translate-y-px`). |
| `.interaction(interaction)` | `iced::mouse::Interaction` | size-dependent default | Per-button cursor override (Pointer for navigation, NotAllowed for soft-disabled, etc.). |
| `.inset(padding)` | `impl Into<Padding>` | `Padding::ZERO` | Transparent margin between the button's hit-test bounds and painted bg/border. Lets a parent column keep `spacing(0)` so adjacent buttons hover continuously while the visible fills sit apart by `inset.top + inset.bottom` (likewise horizontally). Used by sidebar items. |

### Variants

shadcn's `variant` prop is a style. The unit structs live at `button::*`, take tweaks (`button::Outline + Bg(...)`) and work with the macro too.

| Style | Variant |
| --- | --- |
| `button(...)` | `Default` (primary). |
| `.style(button::Secondary)` | `Secondary`. |
| `.style(button::Outline)` | `Outline`. |
| `.style(button::Ghost)` | `Ghost`. |
| `.style(button::Destructive)` | `Destructive`. |
| `.style(button::Link)` | `Link` (text-only, primary color). |
| `.square()` | shadcn's `size="icon"`: width matches the size's height. |

### `button!`

Wraps multiple children in a `Row` so labels and icons land side by side without manual layout. The leading-ident form rewrites `name` to `icon::name()` at the call site (so `use crate::icon;` must be in scope).

| Macro | Description |
| --- | --- |
| `button!["Submit"]` | Single child. |
| `button![icon::git_branch(), "New Branch"]` | Icon on the left. |
| `button![git_branch, "Branch"]` | Leading-ident sugar — equivalent to the previous form. |
| `button![git_branch, "Branch"].style(button::Outline)` | Any variant, set after the macro. |

### `Size`

| Variant | Height | Notes |
| --- | --- | --- |
| `Size::Xs` | 24px | Mirrors shadcn's `xs` size variant. |
| `Size::Sm` | 28px | Mirrors shadcn's `sm` size variant. |
| `Size::Default` | 32px | Default. Matches shadcn's `h-8`. |
| `Size::Lg` | 36px | Mirrors shadcn's `lg` size variant. |

Combine `.size(Sm)` + `.square()` to reproduce shadcn's `icon-sm` size, etc.

Restyle via the variant styles above or the `core::theme::button::*` style functions. shadcn's `asChild` triggers (`CollapsibleTrigger`, `DialogClose`) wrap a button through the owner's handle: `c.trigger(button(...))`.
