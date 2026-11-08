## API Reference

### `input()`

Constructor for an empty, uncontrolled `Input` with the given placeholder. React state: the input keeps what the user types unless the app controls it. Placeholder and values take owned or borrowed text, so an input can be built inside an iced `Component`'s view.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | `Size::Default` | Size variant: `Sm`, `Default`, `Lg`. Drives both height and text size. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Input width. Set `iced::Fill` to fill the parent (matches the shadcn `Field` layout). |
| `.default_value(value)` | `impl IntoFragment` | `""` | Initial value of an uncontrolled input (`defaultValue`). |
| `.value(value)` | `impl IntoFragment` | uncontrolled | Control the input: it always shows `value` (`value`). Without `.on_input`, a controlled input is read-only. |
| `.on_input(callback)` | `Fn(String) -> Message` | `None` | Receives the new value on every edit, in either mode. |
| `.disabled(bool)` | `bool` | `false` | Disable the input. A disabled ancestor (a disabled field) disables it too. |
| `.on_submit(message)` | `Message` | `None` | Emit `message` when the user presses Enter. |
| `.on_focus(message)` | `Message` | `None` | Emit `message` when the input gains focus. |
| `.on_blur(message)` | `Message` | `None` | Emit `message` when the input loses focus. |
| `.style(style)` | `impl Into<text_input::StyleFn>` | `theme::text_input::Default` | Override the style — variant unit structs (`Default`, `Invalid`) or a hand-rolled `Spec`. |

### `Size`

| Variant | Height | Notes |
| --- | --- | --- |
| `Size::Sm` | 28px | Compact form rows. |
| `Size::Default` | 32px | Mirrors shadcn `h-8`. |
| `Size::Lg` | 36px | Larger touch targets. |

### Validation

Apply `theme::text_input::Invalid` via `.style(...)` to land the destructive border + destructive focus ring (mirrors `aria-invalid` in shadcn).

Password and file inputs are not yet exposed at this builder. Identifier hooks live at the iced `Id` layer. Compose with `field(input(...))` to land the labeled-input shape — pressing the field's label focuses the input.
