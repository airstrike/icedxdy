```rust
use icedxdy::widget::{badge, spinner};

badge![spinner(), "Deleting"].style(badge::Destructive)

badge!["Generating", spinner()].style(badge::Secondary)
```
