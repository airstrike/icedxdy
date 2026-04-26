; Macro-body fallbacks (NOT vendored from Helix).
;
; tree-sitter-rust treats the contents of `foo!(...)` / `foo![...]` /
; `foo!{...}` as opaque `token_tree` nodes — there is no
; `scoped_identifier` or `call_expression` to match against, just a flat
; sequence of `(identifier)`, `"::"`, `(token_tree)` etc.
;
; Helix's upstream queries make no attempt to colour macro contents
; because they're guaranteed to be lossy (no semantic info). For our
; component browser, though, almost every example body is a macro
; invocation (`row![…]`, `column![…]`), so without these patterns the
; bulk of the rendered code reads as plain identifiers.
;
; These run *after* the Helix highlights and only fire inside
; `token_tree` nodes, so they can't affect normal code. The trade-off:
; an identifier that looks like a path / call inside a macro body will
; be styled as such even when the surrounding macro semantically uses
; it differently — acceptable for our examples, may surprise users of
; macros that re-purpose those token shapes.

; `Foo::bar` inside a macro — colour `Foo` as a namespace path component.
(token_tree
  (identifier) @namespace
  .
  "::")

; `foo(...)` inside a macro — colour `foo` as a function call.
(token_tree
  (identifier) @function
  .
  (token_tree))

; Nested `foo!(...)` / `foo![...]` / `foo!{...}` inside a macro body —
; tree-sitter-rust only recognises the outermost call as a
; `macro_invocation` node; nested ones are flat `identifier`, `!`,
; `token_tree` triples. Tag them the same way Helix does for the outer
; case so syntax stays consistent across nesting levels.
(token_tree
  (identifier) @function.macro
  .
  "!" @function.macro
  .
  (token_tree))

; `Foo {...}` constructor pattern (rare in macros but harmless).
(token_tree
  ((identifier) @constructor
    (#match? @constructor "^[A-Z]"))
  .
  (token_tree))

; Items inside a `use {...}` list — Helix tags these as @namespace, but
; Tokyo Night colours them as fg (no specific TM rule, falls back to
; `entity.name`). Override to @variable (higher CAPTURE_NAMES index =
; wins over @namespace) which maps to foreground in our derivation.
; PascalCase items (`Row`) are already handled by the existing
; `(#match? @type "^[A-Z]")` rule on `(identifier)`, so this only
; affects lowercase items (functions, modules).
(use_list
  (identifier) @variable)
