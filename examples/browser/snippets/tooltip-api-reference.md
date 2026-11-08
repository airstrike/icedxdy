## API Reference

### `tooltip()`

`tooltip(trigger, content)` shows `content` in `text-xs` on a `bg-foreground` panel while the pointer is over `trigger`, without a delay (shadcn's `delayDuration = 0`).

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.side(side)` | `tooltip::Side` | `Top` | `Top`, `Right`, `Bottom` or `Left` of the trigger. |

The tooltip opens on hover only: iced's tooltip has no keyboard-focus trigger, no controlled `open` state and no arrow.
