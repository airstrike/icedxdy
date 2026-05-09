## API Reference

### `checkbox()`

Constructor for a `Checkbox` builder. Takes a `State` (`Checked` / `Unchecked` / `Indeterminate`) or a plain `bool` via the `From<bool> for State` impl, so existing call sites like `checkbox(self.is_checked)` keep working.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.label(label)` | `impl IntoFragment` | `None` | Optional inline label rendered next to the box. Mirrors the label shadcn pairs `Checkbox` with — clicking the label toggles the checkbox via the embedded `toggle_target` semantic. |
| `.on_toggle(callback)` | `Fn(bool) -> Message` | `None` | Toggle callback. Without this call the checkbox is disabled (matches iced's no-callback-means-disabled convention). |
| `.on_toggle_maybe(callback)` | `Option<Fn(bool) -> Message>` | `None` | Conditional callback — `None` disables, `Some(f)` enables. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. |
| `.style(style)` | `impl Into<checkbox::StyleFn>` | `theme::checkbox::Default` | Override the style — variant unit structs (`Default`, `Invalid`, `Disabled`) or a hand-rolled `Spec`. The focus ring (Tailwind `ring-*`) is preserved end-to-end via the vendored widget bridge. |

### `checkbox::State`

Domain enum modelling the checked / unchecked / indeterminate triplet — promoted to a first-class type so `Indeterminate` lands without a sentinel-bool dance.

| Variant | Description |
| --- | --- |
| `State::Checked` | Renders the check glyph. |
| `State::Unchecked` | Renders an empty box. |
| `State::Indeterminate` | Renders a horizontal stroke (the `MinusIcon` shape shadcn uses for indeterminate). Styled identically to `Checked` (both show the primary surface). |

The `From<bool> for State` impl maps `true → Checked`, `false → Unchecked` so bool-shaped call sites flow through unchanged.

### Validation

Apply `theme::checkbox::Invalid` via `.style(...)` to land the destructive border + destructive focus ring (mirrors `aria-invalid` in shadcn). For the full Field-level cascade (label tinted destructive, etc.) wire `text::label("...").style(theme::text::Destructive)` at the call site — see `examples/browser/src/component/checkbox.rs::"invalid-state"`.

### Disabled state

Disabled state follows iced's convention: a checkbox with no `.on_toggle(...)` is disabled. Pair with `text::muted(label)` to approximate shadcn's `data-disabled:opacity-50` cascade on the FieldLabel (a color shift, not a true alpha overlay).

### Mapping from shadcn

| Shadcn prop | icedxdy equivalent |
| --- | --- |
| `Checkbox.checked` | constructor arg (`State::Checked` / `bool: true`). |
| `Checkbox.defaultChecked` | initial value of the parent's `bool` state. |
| `Checkbox.onCheckedChange` | `.on_toggle(callback)`. |
| `Checkbox.disabled` | omit `.on_toggle(...)`. |
| `Checkbox.aria-invalid` | `.style(theme::checkbox::Invalid)`. |

Identifier hooks live at the iced `Id` layer; restyling collapses into `.style(...)`. Shadcn's `Field`, `FieldLabel`, and `FieldDescription` sub-components compose with this widget via the `field(checkbox(...)).label(...).description(...)` builder.
