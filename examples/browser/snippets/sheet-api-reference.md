## API Reference

### `sheet(trigger, content)`

A `Dialog` pinned to a window edge: the same component, closures and `dialog::Handle` (`s.trigger`, `s.close`, `s.is_open()`). The panel spans the window along its side, `w-3/4 sm:max-w-sm` on the left and right, and pads its sections itself through `sheet::header` and `sheet::footer`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.side(side)` | `sheet::Side` | `Right` | The edge: `Top`, `Right`, `Bottom`, `Left`. |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled sheet starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the sheet. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Receives the new open state. |
| `.show_close_button(bool)` | `bool` | `true` | Show the corner close button. |
