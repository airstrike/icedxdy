## API Reference

### `textarea()`

`textarea(placeholder)` is a multi-line field with the Input's look: at least 64px tall, growing with its text.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `impl Into<String>` | `""` | The initial text of an uncontrolled textarea. |
| `.value(value)` | `impl Into<String>` | - | Control the textarea: it always shows `value`. |
| `.on_input(f)` | `Fn(String) -> Message` | `None` | The message published with the complete new text on every edit. |
| `.disabled(disabled)` | `bool` | `false` | Ignore input and dim. A disabled ancestor disables it too. |
| `.min_height(height)` | `f32` | `64.0` | The least height (`min-h-16`). |
| `.style(style)` | `impl Into<text_editor::StyleFn>` | `textarea::Default` | `textarea::Invalid` for `aria-invalid`. |

The Input's 3px focus ring has no counterpart in iced's text editor style; focus shows as the `border-ring` color only.
