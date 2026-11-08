## API Reference

### `toggle()`

`toggle(content)` creates an off, uncontrolled toggle. `toggle![icon, "Label"]` lays several children out with a 4px gap.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_pressed(pressed)` | `bool` | `false` | The initial value of an uncontrolled toggle. |
| `.pressed(pressed)` | `bool` | - | Control the toggle: it always shows `pressed`. |
| `.on_pressed_change(f)` | `Fn(bool) -> Message` | `None` | The message published with the new value on every press. |
| `.disabled(disabled)` | `bool` | `false` | Ignore input and dim to half opacity. A disabled ancestor disables it too. |
| `.size(size)` | `impl Into<Size>` | `Default` | `Sm` (28px), `Default` (32px) or `Lg` (36px). |
| `.style(style)` | `impl Into<button::Spec>` | `toggle::Default` | `toggle::Outline`, or a tweaked `Spec`. |

A field label linked to a toggle presses it, like a checkbox.
