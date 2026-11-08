## API Reference

### `alert()`

Constructor for an `Alert` builder with the default style.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.icon(icon)` | `impl Into<Slot>` | `None` | Leading icon (rendered at 16px). |
| `.title(title)` | `impl IntoFragment` | `None` | Title text, `text-sm`, medium weight. |
| `.description(description)` | `impl IntoFragment` | `None` | Body text, `text-sm`, in the style's description color. |
| `.action(action)` | `impl Into<Slot>` | `None` | Top-right action element (typically a `button(...)`). |
| `.style(style)` | `impl Into<alert::Spec>` | `alert::Default` | The variant: `alert::Destructive`, or a tweaked `Spec`. |
| `.max_width(width)` | `impl Into<Pixels>` | `f32::INFINITY` | Cap the alert's outer width. Mirrors shadcn's `max-w-md` (448px) usage. |

### Variants

| Style | Description |
| --- | --- |
| `alert::Default` | Card background, muted description. |
| `alert::Destructive` | Destructive foreground; description at `destructive/90`. |

Shadcn's `AlertTitle`, `AlertDescription`, and `AlertAction` sub-components map to the `.title(...)`, `.description(...)`, and `.action(...)` builders on the `Alert` itself; there are no separate widget types in icedxdy.
