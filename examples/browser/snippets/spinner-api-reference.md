## API Reference

### `spinner()`

A 16px (`size-4`) loading indicator that turns continuously.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `f32` | `16.0` | The spinner's side in logical pixels. |
| `.bar_height(height)` | `f32` | `2.0` | The thickness of the spinner's arc. |
| `.rotation_duration(duration)` | `Duration` | 1.2s | How long one full turn takes. |
| `.style(style)` | `impl Into<spinner::StyleFn>` | `spinner::default` | Override the style; the default paints in the current text color. |
