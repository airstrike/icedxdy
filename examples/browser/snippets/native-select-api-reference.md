## API Reference

### `native_select()`

`native_select(items)` shows the chosen item, or the first item while nothing else is chosen, like an HTML `<select>`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `impl Into<String>` | first item | The value an uncontrolled select starts on. |
| `.value(value)` | `impl Into<String>` | - | Control the select: it always shows `value`. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | The message published with the newly chosen value. |
| `.disabled(disabled)` | `bool` | `false` | Disable the select. A disabled ancestor disables it too. |
| `.size(size)` | `impl Into<Size>` | `Default` | `Sm` (28px) or `Default` (32px). |
| `.style(style)` | `impl Into<pick_list::StyleFn>` | `native_select::Default` | `native_select::Invalid` for `aria-invalid`. |

### `native_select::item(value, label)`

A `NativeSelectOption`. `NativeSelectOptGroup` has no counterpart: iced's pick list has no option groups.
