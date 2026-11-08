## API Reference

### `item::item()`

`item::item(children)` lays its children out in a row (`gap-2.5 px-3 py-2.5`) inside a `rounded-lg` box.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.style(style)` | `impl Into<container::Spec>` | `item::Default` | `item::Outline`, `item::Muted`, or a tweaked `Spec`. |
| `.size(size)` | `item::Size` | `Default` | `Default`, `Sm` or `Xs`. |
| `.orientation(orientation)` | `impl Into<Orientation>` | `Horizontal` | Lay the children out in a row or a column. |

### Slots

| Function | Description |
| --- | --- |
| `item::media(content)` | Unframed media; `.style(item::Icon)` or `.style(item::Image)` (a 40px clipped tile). |
| `item::content(children)` | A filling column; `.width(Shrink)` for `flex-none`. |
| `item::title(text)` | `ItemTitle` typography. |
| `item::description(text)` | `ItemDescription` typography. |
| `item::actions(children)` | A row of trailing actions. |
| `item::header(children)` / `item::footer(children)` | Full-width rows above and below. |
