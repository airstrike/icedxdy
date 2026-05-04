## API Reference

Typography is a collection of helper constructors and two composed
widgets, not a single component. Each helper returns a `Text<'a>` (or
`Blockquote` / `List`) builder you can chain `.size(...)`, `.bold()`,
etc. on.

### Constructors

| Constructor | Returns | Description |
| --- | --- | --- |
| `text::h1(content)` | `Text<'a>` | `text-3xl` (30px), semibold, `-0.025em` letter-spacing, foreground. |
| `text::h2(content)` | `Text<'a>` | `text-xl` (20px), medium, `-0.025em` letter-spacing, foreground. |
| `text::h3(content)` | `Text<'a>` | `text-lg` (18px), medium, `-0.025em` letter-spacing, foreground. |
| `text::h4(content)` | `Text<'a>` | `text-base` (16px), medium, `-0.025em` letter-spacing, foreground. |
| `text::p(content)` | `Text<'a>` | Body paragraph: `text-base` (16px), foreground, `Fill` width. |
| `text::lead(content)` | `Text<'a>` | Lead paragraph: `text-xl` (20px), muted, `Fill` width. |
| `text::large(content)` | `Text<'a>` | `text-lg` (18px), semibold, foreground. |
| `text::small(content)` | `Text<'a>` | `text-sm` (14px), foreground, `Fill` width. |
| `text::muted(content)` | `Text<'a>` | `text-sm` (14px), muted, `Fill` width. |
| `text::xs(content)` | `Text<'a>` | `text-xs` (12px), foreground. |
| `text::inline_code(content)` | `Text<'a>` | `text-sm` (14px), semibold, foreground. Wrap in a `bg-muted` container at the call site for the full `<code>` chrome. |
| `text::blockquote(content)` | `Blockquote<'a>` | 2px left rule + 24px gap + italic body. |
| `text::list(items)` | `List<'a, Message>` | Bulleted list — 24px left indent, 8px row gap. |
| `text(content)` | `Text<'a>` | Crate-root shorthand for `text::small`. |

`content` is `impl IntoFragment<'a>` (`&str`, `String`, `Cow<'a, str>`),
so all constructors accept owned or borrowed strings.

### `Text<'a>` builder methods

| Builder | Type | Description |
| --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | Override the font size. Accepts `Size::Xs`/`Sm`/`Default`/`Lg`, an `f32` (px), or `Size::Custom(v)`. |
| `.weight(weight)` | `impl Into<Weight>` | Override the font weight. Accepts `Weight::Medium`/`Semibold`/`Bold`/etc. or a numeric `u16`. |
| `.medium()` | — | Shorthand for `.weight(Weight::Medium)`. |
| `.semibold()` | — | Shorthand for `.weight(Weight::Semibold)`. |
| `.bold()` | — | Shorthand for `.weight(Weight::Bold)`. |
| `.italic()` | — | Render the text in italic (folded into the active font). |
| `.letter_spacing(spacing)` | `impl Into<iced_core::Em>` | Adjust letter spacing in `em` units. |
| `.width(width)` | `impl Into<Length>` | Override the layout width. Paragraph helpers default to `Fill`; headings and `xs`/`large`/`inline_code` default to `Shrink`. |
| `.style(style)` | `impl Into<text::StyleFn>` | Override the text style — variant unit structs (`theme::text::Foreground`, `Muted`, `Link`, `Destructive`), a `Spec` (`Foreground + Alpha(0.5)`), or a hand-rolled closure. |

### `Blockquote<'a>`

Composed widget: 2px left rule + 24px gap + italic body in a single
horizontal row. Mirrors shadcn nova's `border-l-2 pl-6 italic` cascade.
Vertical spacing (`mt-6` in shadcn) is the caller's responsibility —
drive it from the enclosing column's `.spacing(...)`.

| Builder | Type | Description |
| --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | Override the body text size. Defaults to 16px (`size::BASE`). |
| `.style(style)` | `impl Into<text::StyleFn>` | Override the body text style. Defaults to `theme::text::Foreground`. |

### `List<'a, Message>`

Composed widget: 24px left indent + 8px row gap + a `\u{2022}` bullet
prefix per item. Mirrors shadcn nova's
`<ul class="ml-6 list-disc [&>li]:mt-2">` cascade. Vertical spacing
around the list (`my-6` in shadcn) is the caller's responsibility.

The `text::list(items)` constructor takes any
`IntoIterator<Item = impl IntoFragment<'a>>`. Empty input produces an
empty list (no panic). Items are rendered at body size + foreground
color and have no further builder methods today.

### `text::size` constants

Raw `f32` constants for the Tailwind/shadcn font-size scale, exposed
for call sites that want a literal pixel size (e.g. `Span::size`).

| Constant | Value (px) |
| --- | --- |
| `text::size::XS` | 12 |
| `text::size::SM` | 14 |
| `text::size::BASE` | 16 |
| `text::size::LG` | 18 |
| `text::size::XL` | 20 |
| `text::size::XL2` | 24 |
| `text::size::XL3` | 30 |
| `text::size::XL4` | 36 |

### `Size`

| Variant | `text_size()` (px) | Notes |
| --- | --- | --- |
| `Size::Xs` | 12 | Mirrors shadcn `size="xs"`. |
| `Size::Sm` | 12.8 | Mirrors shadcn `size="sm"` (`text-[0.8rem]`). |
| `Size::Default` | 14 | Default body size. |
| `Size::Lg` | 14 | Same as `Default` — drives container height, not text size. |
| `Size::Custom(v)` | `v` | Arbitrary pixel size. `From<f32>` / `From<u16>` / `From<i32>` blanket impls let you pass numerics directly. |

### `Weight`

`Thin`, `ExtraLight`, `Light`, `Normal`, `Medium`, `Semibold`, `Bold`,
`ExtraBold`, `Black`, `Custom(u16)`. The numeric variant clamps to the
nearest CSS weight bucket (100–900).

### Rich text — `rich_text!` + `Span<'a>`

Build inline runs of differently-styled text without resorting to a
`row![text, text]` hack. Each `Span` carries an optional theme-aware
color `Token` resolved against the active palette at draw time, so the
same widget renders correctly in light and dark modes.

```rust
use icedxdy::rich_text;
use icedxdy::widget::{rich_text::span, text};
use icedxdy::theme::tweak::Token;

rich_text![
    span("Required Field ").size(text::size::SM).medium(),
    span("*").size(text::size::SM).color(Token::Destructive),
]
```

`rich_text![span(...), span(...)]` mirrors `row!` / `column!`. Bare
string literals work too — `From<&str>` and `From<String>` for `Span`
let you write `rich_text!["plain ", span("bold").bold()]`. The
`+ Token` operator overload (`span("hi") + Token::Destructive`) is
shorthand for `.color(...)`.

#### `Span<'a>` builder methods

| Builder | Type | Description |
| --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | Override the span's font size. |
| `.weight(weight)` | `impl Into<Weight>` | Override the span's font weight. |
| `.medium()` | — | Shorthand for `.weight(Weight::Medium)`. |
| `.semibold()` | — | Shorthand for `.weight(Weight::Semibold)`. |
| `.bold()` | — | Shorthand for `.weight(Weight::Bold)`. |
| `.italic()` | — | Render the span in italic. |
| `.line_height(line_height)` | `impl Into<text::LineHeight>` | Override the line height. |
| `.letter_spacing(letter_spacing)` | `impl Into<iced_core::Em>` | Adjust letter spacing. |
| `.color(token)` | `impl Into<Token>` | Theme-aware color — resolves against the palette at draw time. |

#### `RichText<'a>` builder methods

| Builder | Type | Description |
| --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | Default font size for spans that don't set their own. |
| `.line_height(line_height)` | `impl Into<text::LineHeight>` | Line height for the whole rich text. |
| `.width(width)` | `impl Into<Length>` | Layout width. Defaults to `Length::Shrink`. |
| `.height(height)` | `impl Into<Length>` | Layout height. Defaults to `Length::Shrink`. |
| `.align_x(alignment)` | `impl Into<text::Alignment>` | Horizontal alignment. |
| `.align_y(alignment)` | `impl Into<alignment::Vertical>` | Vertical alignment. |
| `.wrapping(wrapping)` | `text::Wrapping` | Wrapping strategy. |
| `.letter_spacing(letter_spacing)` | `impl Into<iced_core::Em>` | Letter spacing for the whole rich text. |
| `.style(style)` | `impl Into<text::StyleFn>` | Default catalog class for spans without their own `color`. |

### Style variants

Variant unit structs you pass to `.style(...)` on `Text`, `Blockquote`,
or `RichText`:

| Variant | Description |
| --- | --- |
| `theme::text::Default` | Inherit from parent — no override. |
| `theme::text::Foreground` | Body foreground color. |
| `theme::text::Muted` | Secondary / muted text color. |
| `theme::text::Link` | Primary brand color (link text). |
| `theme::text::Destructive` | Destructive / error color. |

Compose with tweak atoms via `+`: `Foreground + Alpha(0.5)`,
`Default + Fg(Token::Primary)`, etc. The TSX `className` prop is n/a
in icedxdy — restyle via `.style(...)` with a variant or a hand-rolled
closure instead.
