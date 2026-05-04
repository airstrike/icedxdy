## API Reference

### `progress()`

Constructor for a `Progress` builder. Takes the current value (clamped to `0.0..=100.0`).

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.value(value)` | `impl Into<f32>` | constructor arg | Update the value at builder time (clamped to `0..=100`). |
| `.width(width)` | `impl Into<Length>` | `iced::Fill` | Total width of the bar. Cap with a numeric value or wrap in a `container().max_width(...)` for shadcn's `w-[60%]` / `max-w-sm` patterns. |
| `.style(style)` | `impl Into<progress_bar::StyleFn>` | `theme::progress_bar::Default` | Override the style. |
| `.on_idle(callback)` | `Fn(f32) -> Message` | `None` | Fire `callback(value)` once the easing animation settles at its target — useful for gating follow-up actions on the bar reaching a specific value. |

The bar's height (`h-1` = 4px in nova) is fixed by the wrapper; restyle via `.style(...)` if you need a thicker bar.

The TSX `value`, `className`, and `id` props map to the constructor arg, the `core::theme::progress_bar::*` style functions, and the iced `Id` layer respectively. There is no `Indicator` sub-component in icedxdy — the indicator is drawn internally by the vendored progress bar widget.
