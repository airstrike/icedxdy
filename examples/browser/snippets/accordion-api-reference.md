## API Reference

### `accordion()`

Constructor for an `Accordion` from any `IntoIterator<Item = accordion::Item>`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.mode(mode)` | `accordion::Mode` | `Single { collapsible: true }` | Disclosure mode — single-open (with optional collapsible) or multiple-open. |
| `.default_value(ids)` | `IntoIterator<Item = impl Into<String>>` | `[]` | Items an uncontrolled accordion starts with open. The open set then lives in the widget tree. |
| `.value(ids)` | `IntoIterator<Item = impl Into<String>>` | uncontrolled | Control the accordion: the given items are always the open ones. |
| `.on_value_change(f)` | `Fn(Vec<String>) -> Message` | `None` | Receives the complete new open set on every accepted trigger click, after the mode rules are applied. |
| `.style(style)` | `impl Into<accordion::StyleFn>` | `theme::accordion::Default` | Variant unit structs (`Default`, `Card`) or a hand-rolled `Spec`. |
| `.padding(padding)` | `impl Into<Padding>` | `Padding::ZERO` | Outer padding. The wrapping container (e.g. card) is normally responsible for breathing room — surfaced as a knob for the `borders` example's `px-4`. |

### `accordion::item()`

Constructor for one disclosure item. Takes a stable `id` (matched against the open-set) and trigger label content.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.content(body)` | `impl Into<Slot>` | `None` | Body revealed when the item is open. Required for the item to render anything useful below the trigger. |
| `.chevron_closed(icon)` | `impl Into<Slot>` | text glyph `>` | Closed-state chevron icon. Browser callers wire this to `icon::chevron_down()`. |
| `.chevron_open(icon)` | `impl Into<Slot>` | text glyph `v` | Open-state chevron icon. Browser callers wire this to `icon::chevron_up()`. |
| `.disabled(bool)` | `bool` | `false` | Disabled triggers don't toggle; trigger row + body render at 50% opacity per nova's `disabled:opacity-50`. |

### `accordion::Mode`

| Variant | Description |
| --- | --- |
| `Mode::Single { collapsible: bool }` | At most one item open at a time. `collapsible = true` allows zero open. |
| `Mode::Multiple` | Any subset open. |

### State model

Radix state, like every icedxdy control. Uncontrolled by default: the open set lives in the widget tree, seeded by `.default_value(...)`. Controlled with `.value(...)` plus `.on_value_change(...)` when the app needs the open set (the palette's notifications inbox counts it). Either way the widget applies the single/multiple rules itself and publishes the complete new set, never just the clicked id.

Shadcn's `Accordion`/`AccordionItem`/`AccordionTrigger`/`AccordionContent` quad folds into the `accordion(...)` + `accordion::item(id, trigger).content(...)` builders. Restyle via `.style(...)` with a `theme::accordion::*` variant or a hand-rolled `Spec`.
