## Cursor

iced's stock `Button` shows a pointer (hand) cursor on hover when `on_press` is set. Tailwind v4 switched buttons from `cursor: pointer` to `cursor: default`, so shadcn's TSX inherits the arrow. icedxdy's `button` matches shadcn — the default cursor is the arrow, not a pointer — by overriding iced's `mouse_interaction` and falling back to `Interaction::default()`.

If you want the pointer cursor at a specific call site (links, sidebar nav entries, etc.), opt in via `.interaction(Interaction::Pointer)`:

```rust
use iced::mouse::Interaction;
use icedxdy::widget::button;

button::ghost("Docs")
    .interaction(Interaction::Pointer)
    .on_press(Message::OpenDocs)
```

The same builder also gates per-button cursors like `Interaction::NotAllowed` for soft-disabled states. See `button::Button::interaction` in the API Reference below.
