## API Reference

### `dropdown_menu(trigger, content)`

A `Popover` holding a menu: as wide as its trigger (at least 128px) unless given a `.width(..)`, with `p-1`, and modal, so the page ignores the pointer while it is open and a press anywhere outside closes it. The closures receive a `dropdown_menu::Handle` (`m.trigger(child)`, `m.close(child)`, `m.is_open()`).

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `false` | Whether an uncontrolled menu starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the menu. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Receives the new open state. |
| `.side(side)` | `Side` | `Bottom` | The side of the trigger the menu prefers. |
| `.align(align)` | `Align` | `Center` | `Start`, `Center` or `End`. |
| `.side_offset(px)` | `f32` | `4.0` | The gap between the trigger and the menu. |
| `.width(px)` | `f32` | trigger width | A fixed menu width. |

### `menu`

| Part | Description |
| --- | --- |
| `menu::item(content)` | `DropdownMenuItem`: `.on_select(message)`, `.shortcut(str)`, `.disabled(bool)`, `.inset(bool)`, `.style(menu::Destructive)`. Selecting it closes the menu. |
| `menu::checkbox_item(content, checked, on_checked_change)` | `DropdownMenuCheckboxItem`, with a trailing check. |
| `menu::radio_item(content, value, selected, on_select)` | `DropdownMenuRadioItem`, checked while `selected` holds `value`. |
| `menu::label(text)` | `DropdownMenuLabel`. |
| `menu::separator()` | `DropdownMenuSeparator`. |
| `menu::group(children)` | `DropdownMenuGroup`. |
| `menu::sub(content, items)` | `DropdownMenuSub`: opens `items` (like `group`'s) to the right while hovered; selecting inside closes the whole menu. |

Give a top-level `column![..]` `.width(Fill)` so every row spans the menu (`menu::group` already does). No keyboard navigation between items and no typeahead yet.
