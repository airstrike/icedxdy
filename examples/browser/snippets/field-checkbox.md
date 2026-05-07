```rust
use icedxdy::widget::{checkbox, field};

field(checkbox(self.is_checked).on_toggle(Message::Toggled))
    .label("Sync Desktop & Documents folders")
    .description(
        "Your Desktop & Documents folders are being synced with iCloud Drive.",
    )
```
