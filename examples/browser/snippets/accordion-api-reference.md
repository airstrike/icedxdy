## API Reference

### `accordion()`

Constructor for an `Accordion` from any `IntoIterator<Item = accordion::Item>`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.mode(mode)` | `accordion::Mode` | `Single { collapsible: true }` | Disclosure mode — matches shadcn's `type="single"` (with optional `collapsible`) or `type="multiple"`. |
| `.open(open)` | `IntoIterator<Item = impl Into<String>>` | `[]` | Pass the current open-set as ids. Items not in the set render closed; items in the set render open. |
| `.open_one(open)` | `Option<impl Into<String>>` | `None` | Convenience for `Mode::Single`. `None` means no item is open (only valid with `collapsible: true`). |
| `.open_set(set)` | `&HashSet<String>` | `&{}` | Sync the open-set against a `HashSet<String>`. |
| `.on_toggle(callback)` | `Fn(String) -> Message` | `None` | Click handler — receives the toggled item id. The parent app is expected to mutate its open-set in response. |
| `.style(style)` | `impl Into<accordion::StyleFn>` | `theme::accordion::Default` | Variant unit structs (`Default`, `Card`) or a hand-rolled `Spec`. |
| `.padding(padding)` | `impl Into<Padding>` | `Padding::ZERO` | Outer padding. The wrapping container (e.g. card) is normally responsible for breathing room — surfaced as a knob for the `borders` example's `px-4`. |

### `accordion::item()`

Constructor for one disclosure item. Takes a stable `id` (matched against the open-set) and trigger label content.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.content(body)` | `impl Into<Component>` | `None` | Body revealed when the item is open. Required for the item to render anything useful below the trigger. |
| `.chevron_closed(icon)` | `impl Into<Component>` | text glyph `>` | Closed-state chevron icon. Browser callers wire this to `icon::chevron_down()`. |
| `.chevron_open(icon)` | `impl Into<Component>` | text glyph `v` | Open-state chevron icon. Browser callers wire this to `icon::chevron_up()`. |
| `.disabled(bool)` | `bool` | `false` | Disabled triggers don't toggle; trigger row + body render at 50% opacity per nova's `disabled:opacity-50`. |

### `accordion::Mode`

| Variant | Description |
| --- | --- |
| `Mode::Single { collapsible: bool }` | At most one item open at a time. `collapsible = true` allows zero open. Mirrors shadcn's `type="single" collapsible`. |
| `Mode::Multiple` | Any subset open. Mirrors shadcn's `type="multiple"`. |

### State model

Open-state lives in the parent app, mirroring `switch`. The widget's own tree state only tracks per-item animation transitions (`Animation<bool>`, `last_is_open`, cached natural body height). The parent passes the current open-set in via `.open(...)` and receives `on_toggle(id)` callbacks when the user clicks a trigger; the parent is responsible for mutating its open-set and re-rendering. This keeps the single-vs-multiple branching, persistence, and any custom rules (e.g. "keep first item open by default") at the layer that already has the message context.

The TSX `<Accordion>`/`<AccordionItem>`/`<AccordionTrigger>`/`<AccordionContent>` quad folds into the `accordion(...)` + `accordion::item(id, trigger).content(...)` builders. The TSX `value`/`defaultValue` props map to the `.open(...)` builder. The TSX `className` prop is n/a in icedxdy — restyle via `.style(...)` with a `theme::accordion::*` variant or a hand-rolled `Spec`.
