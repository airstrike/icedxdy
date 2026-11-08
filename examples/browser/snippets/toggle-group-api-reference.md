## API Reference

### `toggle_group()`

`toggle_group(|| items)` owns the pressed set. Its items come from a closure because the group rebuilds them on every view. At spacing `0` the items join into one segmented control.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.mode(mode)` | `toggle_group::Mode` | `Single` | `Single` (at most one pressed; pressing it again releases it) or `Multiple`. Radix's `type`. |
| `.default_value(values)` | `impl IntoIterator<Item = impl Into<String>>` | empty | The pressed set an uncontrolled group starts with. |
| `.value(values)` | `impl IntoIterator<Item = impl Into<String>>` | - | Control the group: `values` is always the pressed set. |
| `.on_value_change(f)` | `Fn(Vec<String>) -> Message` | `None` | The message published with the complete new pressed set. |
| `.disabled(disabled)` | `bool` | `false` | Disable every item. A disabled ancestor disables them too. |
| `.style(style)` | `impl Into<button::Spec>` | `toggle_group::Default` | `toggle_group::Outline`, or a tweaked `Spec`. |
| `.size(size)` | `impl Into<Size>` | `Default` | The size of every item. |
| `.orientation(orientation)` | `impl Into<Orientation>` | `Horizontal` | Lay the items out in a row or a column. |
| `.spacing(spacing)` | `impl Into<Pixels>` | `0` | The gap between items. |

### `toggle_group::item(value, content)`

One item. `.disabled(true)` disables it; `.style(..)` overrides the group's style.
