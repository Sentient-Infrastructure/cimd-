# Contributing

Contributions are welcome. Please follow these guidelines:

## Commit Messages

Use conventional commits:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `test:` for tests
- `refactor:` for code refactoring

## Code Style

- Format code with `cargo fmt`
- Lint with `cargo clippy`
- Write tests for new functionality
- Ensure all tests pass: `cargo test`

## Pull Requests

1. Fork and create a feature branch
2. Make your changes
3. Run `cargo fmt` and `cargo clippy`
4. Ensure tests pass
5. Open a PR with a clear description

## Testing

Run the full test suite:

```bash
cargo test --workspace
```

With coverage:

```bash
cargo tarpaulin --out Html
```
