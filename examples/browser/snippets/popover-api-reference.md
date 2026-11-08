## API Reference

### `popover(trigger, content)`

An iced `Component` that owns its open state, over iced's `anchored` overlay. Both closures receive a `popover::Handle` and build in your own message type: `trigger` what sits in place, `content` the panel shown next to it while open. The panel is built only while open, so its widgets mount fresh each time.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled popover starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the popover: open exactly when the app says so. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Receives the new open state when the popover opens or is dismissed. |
| `.side(side)` | `popover::Side` | `Bottom` | The side of the trigger the panel prefers; it flips when there is no room. |
| `.align(align)` | `popover::Align` | `Center` | `Start`, `Center` or `End` along the trigger. |
| `.side_offset(px)` | `f32` | `4.0` | The gap between the trigger and the panel. |
| `.align_offset(px)` | `f32` | `0.0` | Moves a start- or end-aligned panel inward. |
| `.width(px)` | `f32` | `288.0` | The panel's width (`w-72`). |

### `popover::Handle`

| Method | Description |
| --- | --- |
| `p.trigger(child)` | `<PopoverTrigger asChild>`: pressing `child` opens or closes the popover. |
| `p.close(child)` | `<PopoverClose asChild>`: pressing `child` closes it. The child's own `on_press` still fires. |
| `p.is_open()` | Whether the popover is open. |

A press outside the panel (which still reaches the page beneath) and `Escape` dismiss it. `popover::header(title, description)` lays out `PopoverHeader`, `PopoverTitle` and `PopoverDescription`. There is no `PopoverAnchor`: the panel anchors to the widget the trigger closure returns.
