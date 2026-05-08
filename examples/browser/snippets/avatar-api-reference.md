## API Reference

### `avatar()`

Constructor for an `Avatar` builder. Takes an `Option<image::Handle>` — `Some(handle)` renders the image (clipped to a circle); `None` always renders the fallback content.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.fallback(content)` | `impl Into<Component>` | `None` | Content rendered on the muted surface when no image is provided. Typically initials (`"CN"`, `"ER"`) but accepts any `Component` — an icon, a single character, etc. |
| `.size(size)` | `impl Into<Size>` | `Default` (32px) | Size variant: `avatar::Sm` (24px), `avatar::Default` (32px), `avatar::Lg` (40px). Drives diameter, font size, and badge chip dimensions. |

**Image-load failure caveat.** Shadcn's `<AvatarImage>` falls back to `<AvatarFallback>` on runtime image-load failure. Iced's `Image` widget exposes no equivalent event surface, so the fallback only triggers when the caller passes `None` — a `Some(handle)` whose file is broken renders iced's blank/broken-image affordance instead.

### `avatar::badge()`

Wrap an `Avatar` with a corner badge chip overlaid at the bottom-right. Mirrors shadcn's `<AvatarBadge>`.

```rust
// Presence dot.
avatar::badge(avatar(None).fallback("CN"), avatar::dot())

// Icon inside the badge.
avatar::badge(avatar(None).fallback("PP"), icon::plus())
```

The chip's diameter and inner-icon size derive from the parent avatar's geometry. On `Sm` the inner icon is hidden entirely (matches shadcn's `[&>svg]:hidden` cascade).

### `avatar::dot()`

A presence dot for use as the inner content of `avatar::badge` — renders nothing, leaving just the chip surface (matches shadcn's empty `<AvatarBadge className="bg-green-600" />`).

### `avatar::group()`

Compose N overlapping avatars into a horizontal group with negative-space overlap (`-8px` between siblings) plus an optional trailing count chip. Mirrors shadcn's `<AvatarGroup>`.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.count(content)` | `impl Into<Component>` | `None` | Trailing count chip after the avatars — pass a string like `"+3"` for the numeric form, or `icon::plus()` for the icon form. Mirrors shadcn's `<AvatarGroupCount>`. |

Every avatar in a group renders with a 2px outer ring colored to the page background so overlapping avatars read as distinct discs (matches shadcn's `*:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:ring-background` cascade). The count chip's geometry mirrors the first avatar's size variant.

### `avatar::Size`

| Variant | Diameter | Notes |
| --- | --- | --- |
| `avatar::Sm` | 24px | Mirrors shadcn `size="sm"`. Badge inner-icon hidden. |
| `avatar::Default` | 32px | Default. Mirrors shadcn `size="default"`. |
| `avatar::Lg` | 40px | Mirrors shadcn `size="lg"`. |

The TSX `src`/`alt` props on `<AvatarImage>` map to the `image::Handle` argument. The TSX `className` prop is n/a — restyle via the `core::theme::avatar::*` style functions when full restyling is needed. The TSX `<AvatarImage>` / `<AvatarFallback>` sub-components fold into the `avatar(image).fallback(content)` builder shape.
