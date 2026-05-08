## API Reference

### `text::label()`

Typography helper that returns a `text::Text<'a>` styled with shadcn's `<Label>` cascade — `text-sm font-medium leading-none` in foreground color, full-width by default.

`leading-none` (`line_height(1.0)`) strips the invisible line-height padding that otherwise throws off vertical alignment math in horizontal Field layouts — mirroring shadcn's `<Label>` cascade exactly.

```rust
text::label("Your email")
```

Returns the standard `text::Text<'a>` builder, so all the Text builder methods chain (`.size(...)`, `.weight(...)`, `.style(...)`, `.italic()`, etc.). For the click-to-focus / click-to-toggle pairing with an input or checkbox, wrap it in a `field(...)` — the `field(...).label(...)` builder auto-styles plain strings as `text::label` and wires `mouse_area` to the inner content's toggle target when applicable.

### Mapping from shadcn

| Shadcn prop | icedxdy equivalent |
| --- | --- |
| `<Label>` | `text::label("...")`. |
| `<Label htmlFor="...">` | n/a directly — wrap the label + input in `field(input(...)).label("...")`. The Field builder wires `mouse_area` to the input's toggle_target so clicking the label fires the linked message. For checkboxes / switches the toggle_target is auto-derived; for plain inputs (no toggle), the click target is just visual. |
| `<Label className="...">` | restyle via `.style(...)` on the returned Text — variant unit structs (`theme::text::Foreground`, `Muted`, `Destructive`) or a hand-rolled closure. |

### Why no separate `Label` widget?

shadcn's `<Label>` is a `<label>` HTML element styled with one Tailwind cascade and tied to an input via `htmlFor`. iced's primitive surface is text + mouse_area + element ids — no DOM `<label>` semantic exists, and the visual treatment is just Text styling. So `label` lives as a typography helper alongside `text::muted`, `text::error`, etc., and the `htmlFor` semantic is folded into `field(...).label(...)` where the click target is already known at build time.
