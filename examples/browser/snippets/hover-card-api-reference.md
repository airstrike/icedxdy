## API Reference

### `hover_card(trigger, content)`

Shows `content` in a `w-64` card next to `trigger` once the pointer has rested on the trigger for the open delay. The pointer can cross the gap into the card; the card closes once the pointer has left both for the close delay, or on `Escape`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.open_delay(duration)` | `Duration` | `700ms` | Radix's `openDelay`. |
| `.close_delay(duration)` | `Duration` | `300ms` | Radix's `closeDelay`. |
| `.side(side)` | `hover_card::Side` | `Bottom` | The side of the trigger the card prefers; it flips when there is no room. |
| `.align(align)` | `hover_card::Align` | `Center` | `Start`, `Center` or `End`. |
| `.side_offset(px)` | `f32` | `4.0` | The gap between the trigger and the card. |
| `.width(px)` | `f32` | `256.0` | The card's width. |
| `.open(bool)` | `bool` | uncontrolled | Control the card; hovering then only publishes. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Receives the open state hover intent asks for. |

Hover only: keyboard focus does not open the card.
