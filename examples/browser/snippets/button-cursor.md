## Cursor

Tailwind v4 ships `cursor: default` on every `<button>` by default — shadcn's TSX explicitly does nothing here because the framework already gets it right. icedxdy's `button` follows iced's convention: an interactive button with `.on_press(...)` shows the default arrow cursor, NOT a pointer.

Override per call site via `.interaction(...)` when you genuinely want the pointer cursor (links, sidebar nav entries, etc.):

```rust
use iced::mouse::Interaction;
use icedxdy::widget::button;

button::ghost("Docs")
    .interaction(Interaction::Pointer)
    .on_press(Message::OpenDocs)
```

The same builder also gates per-button cursors like `Interaction::NotAllowed` for soft-disabled states. See `button::Button::interaction` in the API Reference below.
