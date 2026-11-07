## API Reference

### `badge()`

Constructor for a `Badge`. The single-arg constructor takes any `impl Into<Slot>`; the `badge!` macro wraps multiple children in a row with 4px spacing for the icon + label idiom.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.style(style)` | `impl Into<container::StyleFn>` | `badge::Default` | The variant: `badge::Secondary`, `badge::Destructive`, `badge::Outline`, `badge::Ghost`, or a tweaked `Spec`. |
| `.on_press(message)` | `Message` | `None` | Make the badge clickable (pointer cursor + click semantics) — the icedxdy equivalent of shadcn rendering the badge as a link. |

### `badge!`

Wraps multiple children in a `Row` with 4px spacing — the iced equivalent of shadcn's inline-start / inline-end icon slots. The variant is set after the macro.

| Macro | Description |
| --- | --- |
| `badge!["text", icon::foo()]` | Icon on the right. |
| `badge![icon::foo(), "text"].style(badge::Secondary)` | Icon on the left, secondary variant. |

### Variants

| Style | Description |
| --- | --- |
| `badge::Default` | Primary-foreground pill. |
| `badge::Secondary` | Muted background. |
| `badge::Destructive` | Destructive border + foreground. |
| `badge::Outline` | Border-only, transparent background. |
| `badge::Ghost` | No border, no background — text-only pill. |

Restyle via the `core::theme::badge::*` style functions (or compose your own).
