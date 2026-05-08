## API Reference

### `field()`

Constructor for a `Field` builder. Wraps any `impl Into<Component>` (typically an `input(...)`, `checkbox(...)`, `switch(...)`, etc.) with optional label / description / error / card chrome.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.label(label)` | `impl IntoLabel` | `None` | Plain strings auto-style as `text::label` (`text-sm font-medium`); pre-styled `Text` / `Component` pass through. Wrapped in `mouse_area` when the inner content widget exposes a `toggle_target` (matches shadcn's `<FieldLabel htmlFor>` click-target semantic). |
| `.description(description)` | `impl IntoDescription` | `None` | Plain strings auto-style as `text::muted` (`text-sm text-muted-foreground`); pre-styled values pass through. |
| `.error(error)` | `impl IntoError` | `None` | Plain strings auto-style as `text::error` (`text-sm text-destructive`); pre-styled values pass through. Note: the error message does NOT auto-style the inner content widget as invalid — wire `input(...).style(theme::text_input::Invalid)` (or the equivalent) at the call site. |
| `.orientation(orientation)` | `field::Orientation` | `Vertical` | Layout axis. Variants are reachable at module scope (`field::Vertical`, `field::Horizontal`). |
| `.card(is_active)` | `bool` | `None` (no card) | Wrap the entire field in a bordered card container. `true` → primary-tinted border + bg (matches shadcn's `has-data-checked:border-primary/30 has-data-checked:bg-primary/5`); `false` → 1px `border-border`, transparent bg. Adds 10px padding (`p-2.5`) on all sides and `rounded-lg` corners. Caller drives the active/inactive choice. |
| `.on_press(message)` | `Message` | `None` | Click-anywhere handler. The inner content widget keeps its own `.on_toggle(...)`; iced's `mouse_area` propagates the press up without consuming it from children, so clicking the checkbox itself fires the toggle, clicking the surrounding card area fires `on_press`. Requires `Message: Clone`. Use idempotent state-set messages (set `true` / `false`) rather than flip-flop toggles when both handlers fire on the same click. |
| `.width(width)` | `impl Into<Length>` | `Length::Shrink` | Outer width. Applied to the final rendered element so callers don't need to wrap a single Field in a `column![field].width(N)`. |

### `field::Orientation`

| Variant | Description |
| --- | --- |
| `Vertical` | Label → content → description → error stacked in a Column with 8px gap. Default — matches shadcn's vertical field shape used by Input, Textarea, etc. |
| `Horizontal` | Content on the left, FieldContent column (label / description / error at 2px gap) on the right, in a Row with 8px gap. Matches shadcn's `orientation="horizontal"` — used by Checkbox, Switch, and other compact controls. Alignment is `Center` for label-only and `Start` when description or error is present (mirrors shadcn's `has-[>[data-slot=field-content]]:items-start`). |

### Conversion traits

The `.label(...)`, `.description(...)`, `.error(...)` builders all accept any of:

- `&str` / `String` — auto-styled via `text::label(...)`, `text::muted(...)`, `text::error(...)` respectively.
- `text::Text<'a>` — pre-styled, passes through.
- `Component<'a, Message>` — passes through.

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

The TSX `className` prop is n/a in icedxdy — restyle via `.style(...)` on the inner widgets or compose your own surface helper. The TSX `data-checked` / `data-disabled` cascade is reproduced manually: pass the active-state bool to `.card(is_active)` and tint the label with `text::muted(...)` / `theme::text::Destructive` at the call site.
