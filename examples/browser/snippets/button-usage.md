## Usage

```rust
use icedxdy::widget::button;
```

```rust
button::outline("Button").on_press(Message::Pressed)
```

Pair the bare-fn constructor with `.on_press(...)` for the click handler, `.size(Size::Sm)` to switch size variants, and the macro form (`button![icon::git_branch(), "New Branch"]`) when you need an inline icon next to a label. Omit `.on_press(...)` to render a disabled button.
