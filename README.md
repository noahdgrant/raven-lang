# Raven-lang

## Debug Info

```bash
cargo build --features "debug_trace_execution"
cargo run --features "debug_trace_execution"
```

## Developer Workflow

```bash
cargo clippy
cargo fmt
```

## Language Definition

### Reserved Words

Control flow:
- `while`
- `for`
- `if`
- `else if`
- `else`
- `in`
- `match`

Declaration:
- `let`

State/Scope:
- `public`
- `mutable`
- `volatile`

Booleans:
- `true`
- `false`

Self-reference:
- `self`

Functions:
- `function`
- `return`

Logical operators:
- `and`
- `or`
- `not`

Structures:
- `struct`

Imports:
- `import`
