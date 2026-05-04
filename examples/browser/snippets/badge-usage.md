## Usage

```rust
use icedxdy::widget::badge;
```

```rust
badge("Badge")
badge::secondary("Secondary")
badge::destructive("Destructive")
badge::outline("Outline")
badge::ghost("Ghost")
```

Pass an icon or other inline content via the macro form, which wraps the children in a row with `gap-1` (4px) automatically. Use `.on_press(...)` to make a badge behave as a link (mirrors shadcn's `asChild` + `<a>` pattern).
