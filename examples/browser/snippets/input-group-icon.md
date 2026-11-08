```rust
use icedxdy::widget::{input, input_group, row};

fn end<'a>(content: impl Into<Slot<'a, Message>>) -> input_group::Addon<'a, Message> {
    input_group::addon(content).align(input_group::Align::InlineEnd)
}

column![
    input_group(input("Search...")).addon(input_group::addon(icon::search())),
    input_group(input("Enter your email")).addon(input_group::addon(icon::mail())),
    input_group(input("Card number"))
        .addon(input_group::addon(icon::credit_card()))
        .addon(end(icon::check())),
    input_group(input("Card number")).addon(end(row![icon::star(), icon::info()].spacing(8))),
]
.spacing(24)
```
