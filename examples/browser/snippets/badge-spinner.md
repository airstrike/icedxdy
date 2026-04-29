```rust
use icedxdy::widget::{badge, spinner};

badge::destructive(row![spinner(), "Deleting"])

badge::secondary(row!["Generating", spinner()])
```
