## API Reference

### `alert_dialog()`

`alert_dialog(trigger, content)` is a dialog without a corner close button that a press outside does not dismiss: its cancel and action buttons and `Escape` close it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | `Default` | `Sm`: a 320px panel, centered header, footer buttons in equal halves. |
| `.default_open(open)` | `bool` | `false` | Whether an uncontrolled dialog starts open. |
| `.open(open)` | `bool` | - | Control the dialog. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | The message published with the new open state. |

### The handle

| Method | Description |
| --- | --- |
| `a.trigger(child)` | `AlertDialogTrigger`: pressing `child` opens the dialog. |
| `a.header(title, description)` | `AlertDialogHeader` with its title and description; `.media(icon)` adds an `AlertDialogMedia` tile, `.media_style(alert_dialog::Media + ..)` restyles it. |
| `a.footer(buttons)` | `AlertDialogFooter`: a `bg-muted/50` band below a border. Give the column holding the header and footer `.width(Fill)` so the band spans the panel. |
| `a.cancel(button)` | `AlertDialogCancel`: the button closes the dialog. |
| `a.action(button)` | `AlertDialogAction`: the button closes the dialog and still publishes its `on_press`. |
