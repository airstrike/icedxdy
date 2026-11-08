## API Reference

### `toaster()`

`toaster(content)` (Sonner's `<Toaster />`) shows the toasts raised inside `content`, usually the whole app, over the window. Each toast leaves by itself after its duration: the toaster asks the runtime for a redraw at the next expiry, so no subscription is needed.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.position(position)` | `toast::Position` | `BottomRight` | Where toasts stack. |
| `.duration(duration)` | `Duration` | 4s | How long each toast shows. |
| `.icons(f)` | `Fn(toast::Kind) -> Option<Slot>` | Unicode glyphs | The icons of the toast kinds, typically lucide's `circle-check`, `info`, `triangle-alert` and `octagon-x`. |

### `toast::toast(title)`

| Builder | Type | Description |
| --- | --- | --- |
| `.description(text)` | `impl Into<String>` | A muted line below the title. |
| `.kind(kind)` | `toast::Kind` | `Default`, `Success`, `Info`, `Warning` or `Error` (Sonner's `toast.success(..)` and friends): picks the icon. |
| `.action(label, message)` | `impl Into<String>`, `Message` | An action button: publishes `message` and dismisses the toast. |
| `.position(position)` | `toast::Position` | Stack this toast elsewhere than the toaster's position. |
| `.duration(duration)` | `Duration` | Show this toast for longer or shorter. |

### Raising a toast

- `toast::trigger(child, toast)`: pressing `child` raises `toast`.
- `toast::show(toast)`: a `Task` raising `toast`, for `update` (after a save completes, say).

`toast.promise(..)`, swipe-to-dismiss and the collapsed stack that expands on hover are not ported.
