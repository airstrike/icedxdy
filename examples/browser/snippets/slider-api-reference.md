## API Reference

### `slider()`

`slider(range)` creates an uncontrolled slider over `range`, starting at its start.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `T` | range start | The initial value of an uncontrolled slider. |
| `.value(value)` | `T` | - | Control the slider: it always shows `value`. |
| `.on_value_change(f)` | `Fn(T) -> Message` | `None` | The message published with the new value while dragging. |
| `.on_release(message)` | `Message` | `None` | Published when the thumb is released. |
| `.step(step)` | `impl AsPrimitive<f64>` | `1` | The step between values. |
| `.shift_step(step)` | `impl AsPrimitive<f64>` | `None` | The step while Shift is held. |
| `.disabled(disabled)` | `bool` | `false` | Ignore input and dim to half opacity. A disabled ancestor disables it too. |
| `.width(width)` | `impl Into<Length>` | `Fill` | The slider's width. |

Radix's slider takes an array of values for several thumbs; icedxdy's has one thumb and is horizontal only.
