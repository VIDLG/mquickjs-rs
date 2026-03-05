// Compile Error Examples
//
// Each section is a separate script that must be run individually,
// because a compile error stops execution of the entire script.
//
// Try each one with:  cargo run --bin mqjs -- -e "<code>"
//
// Examples:

// 1. Missing semicolon (auto-insertion not supported)
//    cargo run --bin mqjs -- -e "var x = 1\nvar y = 2"

// 2. Unexpected token
//    cargo run --bin mqjs -- -e "var x = * 3;"

// 3. Unclosed parenthesis
//    cargo run --bin mqjs -- -e "var x = (1 + 2;"

// 4. Unclosed brace
//    cargo run --bin mqjs -- -e "function foo() { return 1;"

// 5. Reserved word as variable name
//    cargo run --bin mqjs -- -e "var return = 5;"

// 6. Unclosed string literal
//    cargo run --bin mqjs -- -e "var x = \"hello"

// 7. Undeclared variable (compile-time in MQuickJS)
//    cargo run --bin mqjs -- -e "return undeclaredVar;"

// 8. Invalid assignment target
//    cargo run --bin mqjs -- -e "1 + 2 = 3;"

// Since compile errors stop the script, this file just prints the commands.
// Copy and paste them into your terminal to see the error messages.

print("=== Compile Error Examples ===");
print("Run each example below to see error output with line:column info:");
print("");
print('1. Missing semicolon:');
print('   cargo run --bin mqjs -- -e "var x = 1');
print('   var y = 2"');
print("");
print('2. Unexpected token:');
print('   cargo run --bin mqjs -- -e "var x = * 3;"');
print("");
print('3. Unclosed parenthesis:');
print('   cargo run --bin mqjs -- -e "var x = (1 + 2;"');
print("");
print('4. Unclosed brace:');
print('   cargo run --bin mqjs -- -e "function foo() { return 1;"');
print("");
print('5. Reserved word as variable:');
print('   cargo run --bin mqjs -- -e "var return = 5;"');
print("");
print('6. Undeclared variable:');
print('   cargo run --bin mqjs -- -e "return undeclaredVar;"');
