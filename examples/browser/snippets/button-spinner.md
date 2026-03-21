```rust
use icedxdy::widget::{button, row};

row![
    button::outline![icon::loader_circle(), "Generating"],
    button::secondary!["Downloading", icon::loader_circle()],
]
```
