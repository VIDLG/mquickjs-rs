Guide me through adding a new built-in method to the engine. The argument should be in the format `Object.method`, e.g. `Array.from` or `String.raw`.

Steps:
1. Read the relevant builtin file in `src/builtins/` and the `get_*_property()` dispatcher in `src/vm/interpreter.rs`
2. Implement the method in the builtin file
3. Wire it up in the interpreter's property dispatcher
4. Add inline tests in the same builtin file's `#[cfg(test)]` module
5. Run `cargo test` to verify
6. Run `cargo clippy -- -D warnings` to check for lint issues

Target: $ARGUMENTS
