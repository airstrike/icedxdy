```rust
use icedxdy::widget::{input, input_group, textarea};

column![
    input_group(input("0.00"))
        .addon(input_group::addon(input_group::text("$")))
        .addon(end(input_group::text("USD"))),
    input_group(input("example.com"))
        .addon(input_group::addon(input_group::text("https://")))
        .addon(end(input_group::text(".com"))),
    input_group(input("Enter your username")).addon(end(input_group::text("@company.com"))),
    input_group(textarea("Enter your message")).addon(
        input_group::addon(input_group::text("120 characters left").size(12))
            .align(input_group::Align::BlockEnd),
    ),
]
.spacing(24)
```
