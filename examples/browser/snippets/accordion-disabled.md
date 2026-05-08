```rust
use icedxdy::widget::{accordion, text};

accordion::accordion([
    accordion::item("item-1", text::small("Can I access my account history?").medium())
        .content(text::small(
            "Yes, you can view your complete account history including all \
             transactions, plan changes, and support tickets in the Account \
             History section of your dashboard.",
        )),
    accordion::item("item-2", text::small("Premium feature information").medium())
        .content(text::small(
            "This section contains information about premium features. \
             Upgrade your plan to access this content.",
        ))
        .disabled(true),
    accordion::item("item-3", text::small("How do I update my email address?").medium())
        .content(text::small(
            "You can update your email address in your account settings. \
             You'll receive a verification email at your new address to \
             confirm the change.",
        )),
])
.mode(accordion::Mode::Single { collapsible: true })
.on_toggle(Message::Toggle)
```
