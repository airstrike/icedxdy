## Usage

```rust
use icedxdy::Size;
use icedxdy::widget::{alert, button};

alert()
    .icon(icon::info())
    .title("Heads up!")
    .description("You can add components and dependencies to your app using the cli.")
    .action(button("Enable").size(Size::Xs).on_press(Message::Enable))
    .max_width(448)
```

Drop the `.icon(...)` call to render a title-and-description-only alert. Drop `.action(...)` for the simple callout. Use `.style(alert::Destructive)` for the destructive variant.
