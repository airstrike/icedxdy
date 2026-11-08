## API Reference

### `radio_group()`

`radio_group(|g| ..)` owns the checked value; its closure receives a handle and lays out the items. A `field` around an item links its label: pressing the label checks the item.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_value(value)` | `impl Into<String>` | `""` | The item an uncontrolled group starts on. |
| `.value(value)` | `impl Into<String>` | - | Control the group: `value` is always the checked item. |
| `.on_value_change(f)` | `Fn(String) -> Message` | `None` | The message published with the newly chosen value. |
| `.disabled(disabled)` | `bool` | `false` | Disable every item. A disabled ancestor disables them too. |

### `g.item(value)`

A `RadioGroupItem`: a 16px circle, filled with `bg-primary` while checked. `.disabled(true)` disables one item.

### `radio(..)`

The standalone radio, `radio(label, value, selected, on_click)`, stays for app-owned values; it takes `.disabled(..)` too.

Arrow-key navigation between items is not ported.
