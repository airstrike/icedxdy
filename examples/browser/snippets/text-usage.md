## Usage

```rust
use icedxdy::widget::text;
```

Each typography level has its own constructor:

```rust
text::h1("Page Title")        // 36px, extrabold, tracking-tight
text::h2("Section Title")     // 30px, semibold
text::h3("Subsection")        // 24px, semibold
text::h4("Small Heading")     // 20px, semibold

text::p("Body paragraph")     // 16px, normal
text::lead("Intro paragraph") // 20px, muted
text::large("Large text")     // 18px, semibold
text::small("Small text")     // 14px, medium
text::muted("Secondary text") // 14px, muted color

text::blockquote("Quote text")
text::inline_code("@radix-ui/react-alert-dialog")
```

Chain `.medium()`, `.semibold()`, `.bold()`, or `.italic()` to override the default weight/style. Shadcn's underlined-heading cascade (`border-b pb-2` on `h2`) composes with our primitives — wrap the heading in a `column![text::h2(...), rule::horizontal(1)].spacing(8)` to land the underline.
