## API Reference

### `combobox(items)`

An iced `Component` that owns the typed text, the open state and, uncontrolled, the value. Focusing or typing in the input opens a list as wide as the input, showing the items that contain the text (ignoring case); choosing one fills the input and closes the list. `Escape` and a press outside close it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.placeholder(text)` | `String` | `""` | The input's placeholder. |
| `.empty(text)` | `String` | `"No items found."` | `ComboboxEmpty`, shown when nothing matches. |
| `.default_value(item)` | `String` | `None` | The item an uncontrolled combobox starts with. |
| `.value(item)` | `String` | uncontrolled | Control the combobox. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | Receives the chosen item. |
| `.disabled(bool)` | `bool` | `false` | Disables the input. |
| `.width(length)` | `Length` | `256` | The input's width. |
| `.icon(f)` | `Fn() -> Text` | text glyph | The chevron at the input's end. |

Single choice only: chips (`multiple`), the clear button, groups and keyboard highlighting are not ported.
