## API Reference

### `switch()`

Constructor for an off, uncontrolled `Switch`. Radix state: the value lives in the widget tree unless the app controls it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_checked(bool)` | `bool` | `false` | Initial state of an uncontrolled switch (shadcn `defaultChecked`). |
| `.checked(bool)` | `bool` | uncontrolled | Control the switch: it always shows the given state (shadcn `checked`). |
| `.on_checked_change(f)` | `Fn(bool) -> Message` | `None` | Receives the new state on every toggle, in either mode (shadcn `onCheckedChange`). |
| `.disabled(bool)` | `bool` | `false` | Disable the switch. A disabled ancestor disables it too. |
| `.label(label)` | `impl IntoFragment` | `None` | Optional inline label. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. |
| `.style(style)` | `impl Into<switch::StyleFn>` | `theme::switch::Default` | Variant unit structs (`Default`, `Invalid`, `Disabled`) or a hand-rolled `Spec`. |

### Animated thumb slide

The thumb's position is animated via the vendored `Toggler`'s tree state — no caller bookkeeping needed.

### Labels

Inside `field(switch()).label("...")`, pressing the label toggles the switch, like `<FieldLabel htmlFor>`.

### Mapping from shadcn

| Shadcn prop | icedxdy equivalent |
| --- | --- |
| `Switch.defaultChecked` | `.default_checked(bool)`. |
| `Switch.checked` | `.checked(bool)`. |
| `Switch.onCheckedChange` | `.on_checked_change(f)`. |
| `Switch.disabled` | `.disabled(true)`. |
| `Switch.aria-invalid` | `.style(theme::switch::Invalid)`. |
| `Switch.size` (`"default" \| "sm"`) | not yet exposed. |
