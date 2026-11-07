```rust
use icedxdy::card;
use icedxdy::widget::{accordion, column, text};

let items = [
    (
        "plans",
        "What subscription plans do you offer?",
        "We offer three subscription tiers: Starter ($9/month), Professional \
         ($29/month), and Enterprise ($99/month). Each plan includes \
         increasing storage limits, API access, priority support, and team \
         collaboration features.",
    ),
    (
        "billing",
        "How does billing work?",
        "Billing occurs automatically at the start of each billing cycle. \
         We accept all major credit cards, PayPal, and ACH transfers for \
         enterprise customers. You'll receive an invoice via email after \
         each payment.",
    ),
    (
        "cancel",
        "How do I cancel my subscription?",
        "You can cancel your subscription anytime from your account \
         settings. There are no cancellation fees or penalties. Your access \
         will continue until the end of your current billing period.",
    ),
];

let inner = accordion(items.into_iter().map(|(id, trigger, body)| {
    accordion::item(id, text::small(trigger).medium())
        .content(text::small(body))
}))
.mode(accordion::Mode::Single { collapsible: true })
.default_value(["plans"]);

let header = card::header("Subscription & Billing")
    .description("Common questions about your account, plans, payments and cancellations.");

card(column![header, card::content(inner)])
    .max_width(384)
```
