## API Reference

### `button_group()`

Constructor for a `ButtonGroup` from any `IntoIterator<Item = button_group::Item>`. Visually fuses sibling `Button`s along a single axis (collapses touching corners so the cluster reads as one pill-shaped group).

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.orientation(orientation)` | `button_group::Orientation` | `Horizontal` | Layout axis (horizontal or vertical). Variants are reachable at module scope (`button_group::Horizontal`, `button_group::Vertical`). |
| `.width(width)` | `impl Into<Length>` | `Shrink` | Outer width of the rendered group. |

### Item constructors

Children are typed — they must arrive as `Item` values, NOT as `iced::Element`. The group needs to mutate each button's resolved style (corner-radius collapse) at iteration time, which the typed enum permits but an erased element does not.

| Constructor | Returns | Description |
| --- | --- | --- |
| `button_group::button(b)` | `Item::Button` | Wrap any `Button` (or any `Into<Button>`) into an `Item`. Sugar for `Item::Button(b)`. |
| `button_group::separator()` | `Item::Separator` | A 1px divider perpendicular to the group's orientation. The group patches the orientation onto the separator at iteration time, so callers don't pass an axis. |
| `button(...).into()` | `Item::Button` | Direct `From<Button> for Item` impl — keeps `button_group([button::outline("Copy").into(), ...])` calls free of `Item::` ceremony. |

### `button_group::Orientation`

| Variant | Description |
| --- | --- |
| `Horizontal` | Buttons sit in a row (default). Corners collapse on the inside (right edge of first, both sides of middle, left edge of last). |
| `Vertical` | Buttons stack in a column. Corners collapse on the inside (bottom of first, top + bottom of middle, top of last). |

### Edge-radius collapse

Shadcn does this via Tailwind's child-selector cascade (`[&>*:not(:first-child)]:rounded-l-none`). We reproduce it by walking the items vec, computing each button's `Position` (`First`, `Middle`, `Last`, `Only`), and calling `Button::map_style` to zero the appropriate corners on the resolved style. Position counts every child slot (buttons AND separators) — a button immediately after a separator is *not* a "first" button by shadcn's cascade rules.

Shadcn's `ButtonGroup` / `ButtonGroupSeparator` pair folds into `button_group([...])` + `button_group::separator()`. Restyle children via their own `.style(...)` builders before they enter the group.
