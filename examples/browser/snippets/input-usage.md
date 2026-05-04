## Usage

```rust
use icedxdy::widget::input;
```

```rust
input("Enter text", &self.value)
    .on_input(Message::Changed)
```

Pair with `text::small(...).medium()` for a label and `text::muted(...)` for a help description, all stacked in a `column!` with `.spacing(8)` to land the shadcn `Field` layout. Drop `.on_input(...)` to render the input as disabled.
