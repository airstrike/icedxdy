```rust
use icedxdy::widget::{alert, text};

alert::alert()
    .icon(icon::terminal().size(16))
    .title(text::small("Heads up!").semibold().letter_spacing(-0.025))
    .description(text::small("You can add components to your app using the cli."))
```
