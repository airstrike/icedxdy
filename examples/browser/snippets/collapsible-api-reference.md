## API Reference

### `collapsible(body)`

An iced `Component` that owns its open state. `body` is a closure that receives a `collapsible::Handle` and builds the content in your own message type; it runs again whenever the collapsible opens or closes.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled collapsible starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the collapsible: open exactly when the app says so. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Receives the new open state when a trigger is pressed. |
| `.width(width)` | `impl Into<Length>` | `Shrink` | Width of the collapsible. |

### `collapsible::Handle`

| Method | Description |
| --- | --- |
| `c.trigger(child)` | Make `child` toggle the collapsible when pressed, like `<CollapsibleTrigger asChild>`. The child keeps its own `on_press`. A disabled child never toggles. |
| `c.content(child)` | Show `child` only while open, like `<CollapsibleContent>`. |
| `c.is_open()` | Whether the collapsible is open, for content that changes with it (a flipping chevron, a tinted background). |

Cascading props the collapsible inherits (size, weight, disabled) reach the content the closure returns.
