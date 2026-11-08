## API Reference

### `field()`

Constructor for a `Field` builder. Wraps any `impl Into<Slot>` (typically an `input(...)`, `checkbox(...)`, `switch(...)`, etc.) with optional label / description / error / card chrome.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.label(label)` | `impl IntoLabel` | `None` | Plain strings auto-style as `text::label` (`text-sm font-medium`); pre-styled `Text` / `Slot` pass through. Pressing the label activates the field's control, like `<FieldLabel htmlFor>`: a checkbox, switch or radio toggles, an input takes focus. |
| `.description(description)` | `impl IntoDescription` | `None` | Plain strings auto-style as `text::muted` (`text-sm text-muted-foreground`); pre-styled values pass through. |
| `.error(error)` | `impl IntoError` | `None` | Plain strings auto-style as `text::error` (`text-sm text-destructive`); pre-styled values pass through. Note: the error message does NOT auto-style the inner content widget as invalid — wire `input(...).style(theme::text_input::Invalid)` (or the equivalent) at the call site. |
| `.orientation(orientation)` | `field::Orientation` | `Vertical` | Layout axis. Variants are reachable at module scope (`field::Vertical`, `field::Horizontal`). |
| `.card()` | — | no card | Wrap the field in a bordered card, `rounded-lg` with 10px padding. The whole card is the label, and it takes the primary tint (`border-primary/30 bg-primary/5`) while its control is checked, read through the `checked` operation. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. Applied to the final rendered element so callers don't need to wrap a single Field in a `column![field].width(N)`. |

### `field::Orientation`

| Variant | Description |
| --- | --- |
| `Vertical` | Label → content → description → error stacked in a Column with 8px gap. Default — matches shadcn's vertical field shape used by Input, Textarea, etc. |
| `Horizontal` | Content on the left, FieldContent column (label / description / error at 2px gap) on the right, in a Row with 8px gap. Matches shadcn's horizontal field orientation — used by Checkbox, Switch, and other compact controls. Alignment is `Center` for label-only and `Start` when description or error is present (mirrors shadcn's items-start cascade when a content slot is present). |

### Conversion traits

The `.label(...)`, `.description(...)`, `.error(...)` builders all accept any of:

- `&str` / `String` — auto-styled via `text::label(...)`, `text::muted(...)`, `text::error(...)` respectively.
- `text::Text<'a>` — pre-styled, passes through.
- `Slot<'a, Message>` — passes through.

Implemented via `field::IntoLabel`, `field::IntoDescription`, `field::IntoError` traits.

### Mapping from shadcn

| Shadcn primitive | icedxdy equivalent |
| --- | --- |
| `<Field>` | `field(content)` |
| `<FieldSet>` | n/a — compose with `column![field(...), field(...)]`. |
| `<FieldLegend>` | n/a — use `text::h3(...)` or `text::small(...).medium()` as the row's heading. |
| `<FieldGroup>` | n/a — `column![field(...), field(...)]` with explicit spacing. |
| `<FieldLabel>` | `.label(content)` builder. |
| `<FieldDescription>` | `.description(content)` builder. |
| `<FieldError>` | `.error(content)` builder. |
| `<FieldSeparator>` | n/a — use `rule::horizontal(1)`. |
| `<FieldContent>` | implicit — the right-hand column in `Horizontal` mode is the FieldContent slot. |

Restyle via `.style(...)` on the inner widgets or compose your own surface helper. shadcn's `data-checked` card tint follows the control on its own; the label's `data-disabled` muting is still applied at the call site with `text::muted(...)`.
