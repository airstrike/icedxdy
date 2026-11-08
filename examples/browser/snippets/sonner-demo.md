```rust
use icedxdy::widget::{button, toast, toaster};

// Once, around the app's view:
toaster(view)

// Anywhere inside it:
toast::trigger(
    button("Show Toast").style(button::Outline),
    toast::toast("Event has been created")
        .description("Sunday, December 03, 2023 at 9:00 AM")
        .action("Undo", Message::Undo),
)
```
