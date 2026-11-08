```rust
use icedxdy::widget::{column, tabs};

tabs(|t| {
    column![
        t.list([
            t.trigger("overview", "Overview"),
            t.trigger("analytics", "Analytics"),
            t.trigger("reports", "Reports"),
            t.trigger("settings", "Settings"),
        ]),
        t.content("overview", panel("Overview", "View your key metrics...", "You have 12 active projects and 3 pending tasks.")),
        t.content("analytics", panel("Analytics", "Track performance...", "Page views are up 25% compared to last month.")),
        t.content("reports", panel("Reports", "Generate and download...", "You have 5 reports ready and available to export.")),
        t.content("settings", panel("Settings", "Manage your account...", "Configure notifications, security, and themes.")),
    ]
    .spacing(8)
    .into()
})
.default_value("overview")
.width(400)
```
