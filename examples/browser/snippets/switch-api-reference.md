## API Reference

### `switch()`

Constructor for a `Switch` builder. Takes the current on/off state as a `bool`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.label(label)` | `impl IntoFragment` | `None` | Optional inline label. Wired to `mouse_area` via the `toggle_target` semantic when wrapped in a `field(...)` — clicking the label toggles the switch. |
| `.on_toggle(callback)` | `Fn(bool) -> Message` | `None` | Toggle callback. Without this call the switch is disabled (matches iced's no-callback-means-disabled convention). |
| `.on_toggle_maybe(callback)` | `Option<Fn(bool) -> Message>` | `None` | Conditional callback — `None` disables, `Some(f)` enables. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. |
| `.style(style)` | `impl Into<switch::StyleFn>` | `theme::switch::Default` | Override the style — variant unit structs (`Default`, `Invalid`, `Disabled`) or a hand-rolled `Spec`. The focus ring (Tailwind `ring-*`) is preserved end-to-end via the vendored widget bridge. |

### Animated thumb slide

The thumb's left/right position is animated via the vendored `Toggler`'s tree state — no caller bookkeeping needed. Disabled state freezes the position at the start frame.

### Validation

Apply `theme::switch::Invalid` via `.style(...)` to land the destructive border + destructive focus ring (mirrors `aria-invalid` in shadcn). For the full Field-level cascade (label tinted destructive, etc.) wire `text::label("...").style(theme::text::Destructive)` at the call site.

### Disabled state

Disabled state follows iced's convention: a switch with no `.on_toggle(...)` is disabled. Pair with `text::muted(label)` to approximate shadcn's `data-disabled:opacity-50` cascade (a color shift, not a true alpha overlay).

### Mapping from shadcn

| Shadcn prop | icedxdy equivalent |
| --- | --- |
| `Switch.checked` | constructor arg (`bool`). |
| `Switch.defaultChecked` | initial value of the parent's `bool` state. |
| `Switch.onCheckedChange` | `.on_toggle(callback)`. |
| `Switch.disabled` | omit `.on_toggle(...)`. |
| `Switch.aria-invalid` | `.style(theme::switch::Invalid)`. |
| `Switch.size` (`"default" \| "sm"`) | not yet exposed — out of scope for the MVP. Vendoring the full size matrix is its own design surface. |

Identifier hooks live at the iced `Id` layer; restyling collapses into `.style(...)`. Shadcn's `Field`, `FieldLabel`, and `FieldDescription` sub-components compose with this widget via the `field(switch(...)).label(...).description(...)` builder; click-to-toggle on the label is reproduced automatically via `toggle_target`.
