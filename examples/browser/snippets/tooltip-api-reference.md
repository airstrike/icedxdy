## API Reference

### `tooltip()`

`tooltip(trigger, content)` shows `content` in `text-xs` on a `bg-foreground` panel with an arrow toward `trigger` while the pointer rests on the trigger, without a delay by default (shadcn's `delayDuration = 0`). It flips and shifts to stay in the window, and `Escape` hides it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.side(side)` | `tooltip::Side` | `Top` | `Top`, `Right`, `Bottom` or `Left` of the trigger. |
| `.align(align)` | `tooltip::Align` | `Center` | `Start`, `Center` or `End` along the trigger. |
| `.side_offset(px)` | `f32` | `10.0` | The gap to the trigger, the arrow included. |
| `.delay(duration)` | `Duration` | `0` | `delayDuration`: how long the pointer rests before it shows. |
| `.open(bool)` | `bool` | uncontrolled | Control the tooltip; hovering then only publishes. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Receives the open state hover asks for. |

Keyboard focus does not open the tooltip.
