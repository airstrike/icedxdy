```rust
use iced::Fill;
use icedxdy::widget::{container, text};

// nova `typography-h1.tsx` is `text-center` — wrap in a container that
// centers horizontally inside the parent.
container(text::h1("Taxing Laughter: The Joke Tax Chronicles"))
    .center_x(Fill)
```
