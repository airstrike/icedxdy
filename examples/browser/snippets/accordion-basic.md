```rust
use icedxdy::widget::{accordion, text};

let items = [
    (
        "item-1",
        "How do I reset my password?",
        "Click on 'Forgot Password' on the login page, enter your email \
         address, and we'll send you a link to reset your password. The link \
         will expire in 24 hours.",
    ),
    (
        "item-2",
        "Can I change my subscription plan?",
        "Yes, you can upgrade or downgrade your plan at any time from your \
         account settings. Changes will be reflected in your next billing cycle.",
    ),
    (
        "item-3",
        "What payment methods do you accept?",
        "We accept all major credit cards, PayPal, and bank transfers. All \
         payments are processed securely through our payment partners.",
    ),
];

accordion(items.into_iter().map(|(id, trigger, body)| {
    accordion::item(id, text::small(trigger).medium())
        .content(text::small(body))
}))
.mode(accordion::Mode::Single { collapsible: true })
.open(["item-1"])
.on_toggle(Message::Toggle)
```
