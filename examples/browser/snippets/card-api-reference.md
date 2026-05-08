## API Reference

Card is a family of helper constructors, not a single component. Each helper returns a builder (or styled `Container`) you compose at the call site to assemble the canonical `<Card><CardHeader><CardContent><CardFooter>` shape.

### `card()`

Wraps any content with shadcn's card surface (`bg-card` + ring + rounded corners). Returns an `iced_widget::Container`, so all the standard container builders chain (`.padding(...)`, `.width(...)`, `.max_width(...)`, etc.). Prefer a fixed `.width(384.0)` over `.max_width(384)` when the card sits inside a `Length::Shrink` ancestor — iced's compression-flag semantics collapse a Shrink + `max_width` chain to the intrinsic content width.

```rust
card(column![
    card::header("Login to your account")
        .description("Enter your email below to login to your account")
        .action(card::action(button::link("Sign Up").on_press(Message::SignUp))),
    card::content(my_form),
    card::footer_row([
        button("Login").on_press(Message::Submit).into(),
        button::outline("Login with Google").into(),
    ]),
])
.width(384.0)
```

### `card::header()`

Builder for a card header. Title is supplied at construction; description and action are independently optional.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.description(content)` | `impl IntoDescription` | `None` | Plain strings auto-style as muted-foreground (`text-sm`); pre-styled `Text` / `Component` pass through. |
| `.action(content)` | `impl Into<Component>` | `None` | Top-right action — typically a link, button, or badge. The Header pushes it flush-right with `space::horizontal` inside a row (the iced analog of shadcn's `col-start-2 ... justify-self-end` grid pin). |

### Section helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `card::title(content)` | `Text<'a>` | `<CardTitle>`-shaped Text — `text-base font-medium leading-snug`, foreground color. |
| `card::description(content)` | `Text<'a>` | `<CardDescription>`-shaped Text — equivalent to `text::muted` but exposed under `card::` so the slot vocabulary mirrors shadcn. |
| `card::action(content)` | `Component<'a>` | `<CardAction>`-shaped passthrough — identity today, reserved for any future action-specific styling. |
| `card::content(child)` | `Container<'a>` | Card content section (`px-4` only). |
| `card::footer(child)` | `Container<'a>` | Card footer (`px-4` only). Caller composes the inner shape. |
| `card::footer_row(children)` | `Container<'a>` | Card footer as a row of children with 8px (`gap-2`) spacing. |
| `card::header_simple(title)` | `Header<'a>` | Header with title only (no description, no action) — equivalent to `card::header(title)`. |

### `IntoTitle` / `IntoDescription`

The header's `.description(...)` builder and the constructor's `title` argument both accept any of:

- `&str` / `String` — auto-styled via `card::title(...)` / `card::description(...)`.
- `text::Text<'a>` — pre-styled, passes through unchanged.
- `Component<'a, Message>` — passes through unchanged.

The TSX `Card`/`CardHeader`/`CardTitle`/`CardDescription`/`CardAction`/`CardContent`/`CardFooter` quintet folds into the helpers above. The TSX `className` prop is n/a in icedxdy — restyle via the wrapping `Container`'s `.style(...)` builder or compose your own surface helper.

### Scope notes

- `Card.size = "default" | "sm"` (shadcn live docs) is not yet exposed at the helper level. The `card-small` example renders without a true `size="sm"` driver — visual delta vs the default size is small (text-base → text-sm on title, `gap-3 py-3 px-3` instead of `gap-4 py-4 px-4`).
- `card-image` examples use a muted-bg placeholder rectangle in lieu of a raster asset (no checked-in image, no URL fetcher in iced).
