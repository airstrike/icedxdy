## API Reference

### `kbd()`

A keyboard key: `text-xs font-medium` content on a 20px `bg-muted` cap.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.style(style)` | `impl Into<container::Spec>` | `kbd::Default` | `kbd::Tooltip` for keys inside tooltip content, or a tweaked `Spec`. |

### `kbd_group!` / `kbd::group()`

Lays keys (and any other children) out in a row with a 4px gap, like `KbdGroup`.

shadcn restyles a `Kbd` inside `TooltipContent` through a CSS context selector; iced cascades no such context, so the call site picks `kbd::Tooltip`.
