## API Reference

### `select(entries)`

An iced `Component` that owns its open state and, uncontrolled, its value. The trigger shows the chosen item's label or the placeholder; pressing it opens a modal list 4px below (flipping above when there is no room), at least as wide as the trigger and 144px, scrolling when taller than the window.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.placeholder(text)` | `String` | `""` | Shown while nothing is chosen. |
| `.default_value(value)` | `String` | `None` | The value an uncontrolled select starts with. |
| `.value(value)` | `String` | uncontrolled | Control the select. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | Receives the chosen value. |
| `.disabled(bool)` | `bool` | `false` | Dims the trigger; it does not open. |
| `.width(length)` | `Length` | `Shrink` | The trigger's width. |
| `.size(size)` | `Size` | `Default` | `Size::Sm` is `h-7`. |
| `.style(select::Invalid)` | variant | `Default` | A destructive border (`aria-invalid`). |
| `.icon(f)` | `Fn() -> Text` | text glyph | The trigger's chevron, e.g. `icon::chevron_down`. |

### Entries

`select::item(value, label)` (with `.disabled(bool)`), `select::label(text)`, `select::separator()` and `select::group(entries)`.

Radix's default `position="item-aligned"` (the chosen item laid over the trigger) is not ported: every select opens as `popper`. No keyboard navigation or typeahead yet.
