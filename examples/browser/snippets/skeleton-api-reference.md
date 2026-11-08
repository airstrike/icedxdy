## API Reference

### `skeleton()`

A `bg-muted` block with `rounded-md` corners. Size it like the content it stands in for.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.width(width)` | `impl Into<Length>` | `Shrink` | The block's width. |
| `.height(height)` | `impl Into<Length>` | `Shrink` | The block's height. |
| `.radius(radius)` | `impl Into<Radius>` | `radius::MD` | Corner radius; `u32::MAX as f32` for `rounded-full`. |

shadcn's `animate-pulse` is not ported: the block does not pulse.
