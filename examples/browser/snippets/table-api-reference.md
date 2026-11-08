## API Reference

### `table::table()`

`table::table(children)` stacks its header, body, footer and caption.

| Function | Description |
| --- | --- |
| `table::header(rows)` | `TableHeader`: rows with a bottom border. |
| `table::body(rows)` | `TableBody`: rows separated by borders. |
| `table::footer(rows)` | `TableFooter`: `bg-muted/50` rows below a border. |
| `table::row(cells)` | `TableRow`. `.border(false)` drops its bottom border. |
| `table::head(content)` | `TableHead`: a 40px medium header cell. |
| `table::cell(content)` | `TableCell`: a `p-2` cell. |
| `table::caption(content)` | `TableCaption`: muted text below the table. |

Cells take `.width(..)`, `.height(..)`, `.padding(..)`, `.align_x(..)` and `.align_y(..)`. Columns line up when matching cells share a width; a cell cannot span columns (`colSpan`).
