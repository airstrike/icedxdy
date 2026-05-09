## API Reference

### `input()`

Constructor for an `Input` builder. Takes a placeholder string and the current value as a `&str` borrow — values are owned by the parent state.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | `Size::Default` | Size variant: `Sm`, `Default`, `Lg`. Drives both height and text size. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Input width. Set `iced::Fill` to fill the parent (matches the shadcn `Field` layout). |
| `.on_input(callback)` | `Fn(String) -> Message` | `None` | Per-keystroke change handler. Omitting this renders the input as disabled. |
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

Password and file inputs are not yet exposed at this builder. Identifier hooks live at the iced `Id` layer; disabled state is implicit (no `.on_input(...)` ⇒ disabled). Compose with `field(input(...))` to land the labeled-input shape — `field::field` handles label / description / error rendering.
