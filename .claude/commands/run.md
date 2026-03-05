Run a JavaScript file or expression with mqjs. Usage: /run <file.js or expression>

If the argument ends with `.js`, run it as a file:
```
cargo run --bin mqjs -- $ARGUMENTS
```

If the argument does not end with `.js`, evaluate it as an expression:
```
cargo run --bin mqjs -- -e "$ARGUMENTS"
```
