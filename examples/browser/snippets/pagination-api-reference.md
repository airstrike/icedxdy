## API Reference

### `pagination()`

`pagination(items)` lays its items out in a centered row with a 2px gap. It stands for shadcn's `Pagination`, `PaginationContent` and `PaginationItem` together.

### `pagination::link()`

A page number: a square ghost button, in outline while it is the current page.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.active(active)` | `bool` | `false` | Mark the link as the current page (`isActive`). |
| `.on_press(message)` | `Message` | `None` | Publish `message` when pressed. |

### `pagination::previous()` / `pagination::next()`

Ghost buttons labelled "Previous" and "Next" beside a chevron.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.label(label)` | `impl Into<Slot>` | `"Previous"` / `"Next"` | Replace the label (shadcn's `text` prop). |
| `.icon(icon)` | `impl Into<Slot>` | `‹` / `›` | Replace the chevron glyph with an icon. |
| `.on_press(message)` | `Message` | `None` | Publish `message` when pressed. |

### `pagination::ellipsis()`

A 32px "more pages" marker. `.icon(icon)` replaces its `…` glyph.
