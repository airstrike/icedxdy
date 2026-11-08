## API Reference

### `checkbox()`

Constructor for an unchecked, uncontrolled `Checkbox`. Radix state: the value lives in the widget tree unless the app controls it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_checked(state)` | `impl Into<State>` | `Unchecked` | Initial state of an uncontrolled checkbox (shadcn `defaultChecked`). |
| `.checked(state)` | `impl Into<State>` | uncontrolled | Control the checkbox: it always shows `state` (shadcn `checked`). |
| `.on_checked_change(f)` | `Fn(bool) -> Message` | `None` | Receives the new checked state on every toggle, in either mode (shadcn `onCheckedChange`). |
| `.disabled(bool)` | `bool` | `false` | Disable the checkbox. A disabled ancestor (a disabled field) disables it too. |
| `.label(label)` | `impl IntoFragment` | `None` | Optional inline label rendered next to the box. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. |
| `.style(style)` | `impl Into<checkbox::StyleFn>` | `theme::checkbox::Default` | Variant unit structs (`Default`, `Invalid`, `Disabled`) or a hand-rolled `Spec`. The focus ring is preserved end-to-end via the vendored widget bridge. |

### `checkbox::State`

| Variant | Description |
| --- | --- |
| `State::Checked` | Renders the check glyph. |
| `State::Unchecked` | Renders an empty box. |
| `State::Indeterminate` | Renders a horizontal stroke. A click commits it to checked. |

`From<bool> for State` maps `true` to `Checked` and `false` to `Unchecked`.

### Labels

Inside `field(checkbox()).label("...")`, pressing the label toggles the checkbox, like `<FieldLabel htmlFor>`: the field runs the `activate` operation on its control. With `.card()` the whole card is the label, and its tint follows the checkbox.

### Validation

Apply `theme::checkbox::Invalid` via `.style(...)` to land the destructive border + destructive focus ring (mirrors `aria-invalid` in shadcn).

### Mapping from shadcn

| Shadcn prop | icedxdy equivalent |
| --- | --- |
| `Checkbox.defaultChecked` | `.default_checked(state)`. |
| `Checkbox.checked` | `.checked(state)`. |
| `Checkbox.onCheckedChange` | `.on_checked_change(f)`. |
| `Checkbox.disabled` | `.disabled(true)`. |
| `Checkbox.aria-invalid` | `.style(theme::checkbox::Invalid)`. |
