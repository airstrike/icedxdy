## API Reference

### `separator()`

A 1px `bg-border` line. Horizontal lines span the width of their parent; vertical lines span the height of their row.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.orientation(orientation)` | `impl Into<Orientation>` | `Horizontal` | `Horizontal` or `Vertical`. |

shadcn's `decorative` prop has no iced counterpart: iced has no accessibility tree to hide the separator from.
