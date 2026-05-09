## API Reference

### `badge()`

Constructor for a default-variant `Badge`. The single-arg constructor takes any `impl Into<Component>`; the `badge!` macro wraps multiple children in a row with 4px spacing for the icon + label idiom.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.variant(variant)` | `badge::Variant` | `Default` | Switch between the five variants. Equivalent to calling the per-variant constructor (`badge::secondary(...)`, etc.). |
| `.on_press(message)` | `Message` | `None` | Make the badge clickable (pointer cursor + click semantics) — the icedxdy equivalent of shadcn rendering the badge as a link. |

### Variant constructors

| Constructor | Variant |
| --- | --- |
| `badge(...)` / `badge::default(...)` | `Default` |
| `badge::secondary(...)` | `Secondary` |
| `badge::destructive(...)` | `Destructive` |
| `badge::outline(...)` | `Outline` |
| `badge::ghost(...)` | `Ghost` |

### Variant macros

Each variant has a matching macro that wraps multiple children in a `Row` with 4px spacing — the iced equivalent of shadcn's inline-start / inline-end icon slots.

| Macro | Description |
| --- | --- |
| `badge!["text", icon::foo()]` | Default variant, icon on the right. |
| `badge::secondary![icon::foo(), "text"]` | Secondary variant, icon on the left. |
| `badge::outline!`, `badge::destructive!`, `badge::ghost!` | Same shape as `badge::secondary!`. |

### `badge::Variant`

| Variant | Description |
| --- | --- |
| `Default` | Primary-foreground pill. |
| `Secondary` | Muted background. |
| `Destructive` | Destructive border + foreground. |
| `Outline` | Border-only, transparent background. |
| `Ghost` | No border, no background — text-only pill. |

Restyle via the `core::theme::badge::*` style functions (or compose your own).
