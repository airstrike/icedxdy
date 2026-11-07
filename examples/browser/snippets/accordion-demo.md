```rust
use icedxdy::widget::{accordion, text};

accordion([
    accordion::item("shipping", text::small("What are your shipping options?").medium())
        .content(text::small(
            "We offer standard (5-7 days), express (2-3 days), and overnight \
             shipping. Free shipping on international orders.",
        )),
    accordion::item("returns", text::small("What is your return policy?").medium())
        .content(text::small(
            "Returns accepted within 30 days. Items must be unused and in \
             original packaging. Refunds processed within 5-7 business days.",
        )),
    accordion::item("support", text::small("How can I contact customer support?").medium())
        .content(text::small(
            "Reach us via email, live chat, or phone. We respond within 24 \
             hours during business days.",
        )),
])
.mode(accordion::Mode::Single { collapsible: true })
.default_value(["shipping"])
```
