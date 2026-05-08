## API Reference

### `breadcrumb()`

Constructor for a `Breadcrumb` from any `IntoIterator<Item = breadcrumb::Item>`. The separator is auto-interspersed between adjacent items.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.separator(factory)` | `Fn() -> Component` | muted `›` glyph | Override the default separator. The closure is called once per gap between adjacent items (Component values are not `Clone`, so a fresh instance is built for each insertion). Pass `\|\| icon::chevron_right().size(14).into()` for lucide-icon parity. |

### Item constructors

| Constructor | Returns | Description |
| --- | --- | --- |
| `breadcrumb::link(content, on_press)` | `Item::Link` | Clickable crumb. Cursor changes to `Pointer`; emits `on_press` on left-button release. Mirrors `<BreadcrumbLink>`. |
| `breadcrumb::page(content)` | `Item::Page` | Current page — renders content as-is, no click affordance. Mirrors `<BreadcrumbPage>`. |
| `breadcrumb::ellipsis(Some(message))` | `Item::Ellipsis` | Clickable `…` indicator (e.g. to open a dropdown of elided crumbs). Mirrors `<BreadcrumbEllipsis>`. |
| `breadcrumb::ellipsis(None)` | `Item::Ellipsis` | Static visual `…` placeholder. |

### Color cascade — caller's responsibility

Shadcn's `<BreadcrumbList>` applies `text-muted-foreground` once and `<BreadcrumbPage>` overrides with `text-foreground`. There's no breadcrumb `Catalog` yet, so the muted/foreground distinction is the **caller's** responsibility — wrap link content in `text::muted(...)` for the canonical visual, and pass the page content unstyled (default foreground).

### Animation gap

Shadcn's `<BreadcrumbLink>` carries `transition-colors hover:text-foreground`, so a link's color fades from muted to foreground on hover. Because color is caller-driven (no Catalog yet), our impl snaps — there is no hover-color animation. A future Catalog pass would surface link state and restore the fade.

### Mapping from shadcn

| Shadcn primitive | icedxdy equivalent |
| --- | --- |
| `<Breadcrumb>` + `<BreadcrumbList>` | `breadcrumb([...])` (the JSX nesting collapses into a flat list of `Item` values). |
| `<BreadcrumbItem>` | implicit — each `Item` IS the slot. |
| `<BreadcrumbLink>` | `breadcrumb::link(content, on_press)` |
| `<BreadcrumbPage>` | `breadcrumb::page(content)` |
| `<BreadcrumbSeparator>` | `.separator(\|\| ...)` builder. |
| `<BreadcrumbEllipsis>` | `breadcrumb::ellipsis(on_press)` |

The TSX `className` prop is n/a in icedxdy — content is `Component`, so style at the call site (`text::muted(...)`, `.size(...)`).
