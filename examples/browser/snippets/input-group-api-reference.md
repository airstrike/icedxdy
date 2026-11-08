## API Reference

### `input_group()`

`input_group(control)` puts an `input(..)` or a `textarea(..)` and its addons in one bordered field. The group paints the Input's border, background and focus ring (the ring shows while the control is focused) and strips the control's own.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.addon(addon)` | `input_group::Addon` | - | Add an addon. |
| `.disabled(disabled)` | `bool` | `false` | Disable the control and dim the group. |
| `.style(style)` | `impl Into<text_input::StyleFn>` | `input::Default` | `input::Invalid` paints the invalid surface (shadcn sets `aria-invalid` on the control). |

### `input_group::addon(content)`

`InputGroupAddon`: muted `text-sm font-medium` content, icons at 16px. `.align(..)` places it: `InlineStart` (default), `InlineEnd`, `BlockStart` or `BlockEnd`.

### `input_group::text(..)` / `input_group::button(..)`

`InputGroupText` (muted `text-sm`) and `InputGroupButton` (a ghost `Xs` button; chain `.square()` for `icon-xs`).
