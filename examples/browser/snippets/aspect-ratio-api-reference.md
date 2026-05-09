## API Reference

### `aspect_ratio()`

Constructor for an `AspectRatio` wrapper. Takes any `impl Into<Component>` plus a `width / height` ratio (e.g. `16.0 / 9.0` for widescreen, `9.0 / 16.0` for portrait).

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.width(width)` | `impl Into<Length>` | `Length::Fill` | Width policy. Use `Length::Fixed(n)` for an explicit pixel width or `FillPortion(n)` for proportional layouts. |
| `.max_width(max_width)` | `impl Into<Pixels>` | `f32::INFINITY` | Cap the resolved width — mirrors shadcn's `max-w-sm` / `max-w-[10rem]` parent-width idioms. Prefer this over wrapping in a `Shrink`-width container, which collapses the layout to zero (see widget docs for the width-compression footgun). |

The `ratio` argument follows the shadcn / CSS convention: `width / height`. Non-finite or non-positive ratios fall back to `1.0` (square box) — no panic, no NaN propagation.

The shadcn `ratio` prop maps to the constructor arg. Style the child (e.g. `bg-muted`, `rounded-lg`) at the call site, just as shadcn's examples do. There's no theme catalog: aspect-ratio is purely structural.
