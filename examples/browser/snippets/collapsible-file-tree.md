```rust
use icedxdy::widget::{button, collapsible, column, container};

fn entry(entry: &'static Entry) -> Slot<'static, Message> {
    match entry {
        Entry::File(name) => button![icon::file(), *name]
            .size(Size::Sm)
            .style(button::Link)
            .into(),
        Entry::Folder(name, children) => collapsible(move |c| {
            column![
                c.trigger(
                    button![icon::chevron_right(), icon::folder(), *name]
                        .size(Size::Sm)
                        .style(button::Ghost),
                ),
                c.content(column(children.iter().map(entry)).spacing(4)),
            ]
            .into()
        })
        .into(),
    }
}
```
