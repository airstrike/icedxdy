## API Reference

### `context_menu(trigger, content)`

An iced `Component`: a right press inside the area `trigger` builds opens the menu `content` builds, 2px to the right of the pointer with its top level with it, flipping and shifting to stay in the window. Build the menu from the same `menu::` parts as the Dropdown Menu; selecting an item closes it. The menu is modal: any press outside it, and `Escape`, close it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.width(px)` | `f32` | fits its items, min 144 | The menu's width (`className="w-48"`). |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Receives the open state. |

No long-press on touch, no keyboard navigation; a right press while the menu is open closes it rather than moving it.
