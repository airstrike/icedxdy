## API Reference

### `alert()`

Constructor for an `Alert` builder. Default variant.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.icon(icon)` | `impl Into<Component>` | `None` | Leading icon (rendered at 16px). |
| `.title(title)` | `impl IntoFragment` | `None` | Title text — `text-sm`, medium weight. |
| `.description(description)` | `impl IntoFragment` | `None` | Body text — `text-sm`, muted (default) or destructive at alpha 0.9 (destructive). |
| `.action(action)` | `impl Into<Component>` | `None` | Top-right action element (typically a `button(...)`). |
| `.variant(variant)` | `alert::Variant` | `Default` | `Default` or `Destructive`. |
| `.destructive()` | — | — | Shorthand for `.variant(Variant::Destructive)`. |
| `.max_width(width)` | `impl Into<Pixels>` | `f32::INFINITY` | Cap the alert's outer width. Mirrors shadcn's `max-w-md` (448px) usage. |

### `alert::default()`

Equivalent to `alert()`. Provided for symmetry with `alert::destructive()`.

### `alert::destructive()`

Constructor that pre-applies the destructive variant.

### `alert::Variant`

| Variant | Description |
| --- | --- |
| `Default` | Neutral background, muted description. |
| `Destructive` | Destructive border + foreground; description rendered at alpha 0.9. |

The TSX `AlertTitle`, `AlertDescription`, and `AlertAction` sub-components map to the `.title(...)`, `.description(...)`, and `.action(...)` builders on the `Alert` itself — there are no separate widget types in icedxdy. The TSX `className` prop is n/a in icedxdy; use the `core::theme::alert::*` style functions (or compose your own) when full restyling is needed.
