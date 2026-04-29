```rust
use icedxdy::widget::{column, text};

column![
    text::h1("Taxing Laughter: The Joke Tax Chronicles"),
    text::lead(
        "Once upon a time, in a far-off land, there was a very lazy king who \
         spent all day lounging on his throne. One day, his advisors came to him \
         with a problem: the kingdom was running out of money."
    ),
    text::h2("The King's Plan"),
    text::p(
        "The king thought long and hard, and finally came up with a brilliant \
         plan: he would tax the jokes in the kingdom."
    ),
    text::blockquote(
        "\"After all,\" he said, \"everyone enjoys a good joke, so it's only \
         fair that they should pay for the privilege.\""
    ),
    text::h3("The Joke Tax"),
    text::p(
        "The king's subjects were not amused. They grumbled and complained, \
         but the king was firm:"
    ),
    column![
        text::list("1st level of puns: 5 gold coins"),
        text::list("2nd level of jokes: 10 gold coins"),
        text::list("3rd level of one-liners : 20 gold coins"),
    ]
    .spacing(8),
    text::p(
        "The king, seeing how much happier his subjects were, realized the error \
         of his ways and repealed the joke tax."
    ),
]
.spacing(16)
```
