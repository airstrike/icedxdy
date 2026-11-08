## API Reference

### `empty::empty()`

A centered empty state (`p-6 gap-4`). shadcn's `EmptyHeader`, `EmptyTitle`, `EmptyDescription` and `EmptyContent` are builder slots.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.media(media)` | `impl Into<Slot>` | `None` | The media above the title, usually `empty::media(..)`. |
| `.title(title)` | `&str`, `Text` or `Slot` | `None` | Strings are styled as `EmptyTitle`. |
| `.description(description)` | `&str`, `Text` or `Slot` | `None` | Strings are styled as `EmptyDescription`. |
| `.content(content)` | `impl Into<Slot>` | `None` | Actions below the header, at most 384px wide. |
| `.padding(padding)` | `impl Into<Padding>` | `24` | Override the root padding. |

### `empty::media()`

Unframed by default. `.style(empty::Icon)` puts the content on a 32px muted tile and sizes icons at 16px.
