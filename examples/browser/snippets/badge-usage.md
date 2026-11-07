## Usage

```rust
use icedxdy::widget::badge;
```

```rust
badge("Badge")
badge("Secondary").style(badge::Secondary)
badge("Destructive").style(badge::Destructive)
badge("Outline").style(badge::Outline)
badge("Ghost").style(badge::Ghost)
```

Pass an icon or other inline content via the macro form, which wraps the children in a row with `gap-1` (4px) automatically. Use `.on_press(...)` to make a badge behave as a link.
