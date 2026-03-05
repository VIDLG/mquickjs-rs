// Runtime Error Examples
//
// Demonstrates runtime errors, try/catch handling, and error formatting.

print("=== try/catch catches runtime errors ===");

// Call non-function - caught by try/catch
try {
    var x = 42;
    x();
} catch (e) {
    print("Caught call-non-function: " + e.name + ": " + e.message);
}

// Stack overflow - caught by try/catch
try {
    function recurse() { return recurse(); }
    recurse();
} catch (e) {
    print("Caught stack overflow: " + e.name + ": " + e.message);
}

print("");
print("=== Throw and catch custom errors ===");

// Error with message
try {
    throw new Error("something went wrong");
} catch (e) {
    print("Error: " + e.message);
}

// TypeError
try {
    throw new TypeError("expected a number");
} catch (e) {
    print(e.name + ": " + e.message);
}

// RangeError
try {
    throw new RangeError("index out of bounds");
} catch (e) {
    print(e.name + ": " + e.message);
}

// Throw primitive values
try { throw 42; } catch (e) { print("Caught number: " + e); }
try { throw "oops"; } catch (e) { print("Caught string: " + e); }
try { throw null; } catch (e) { print("Caught null: " + e); }

print("");
print("=== Finally blocks ===");

try {
    throw new Error("boom");
} catch (e) {
    print("catch: " + e.message);
} finally {
    print("finally: always runs");
}

// Finally without catch - error propagates after finally
try {
    try {
        throw new Error("inner error");
    } finally {
        print("inner finally runs before propagation");
    }
} catch (e) {
    print("outer catch: " + e.message);
}

print("");
print("=== Re-throw ===");

try {
    try {
        throw new Error("original");
    } catch (e) {
        print("caught, re-throwing...");
        throw e;
    }
} catch (e) {
    print("re-caught: " + e.message);
}

print("");
print("=== Uncaught runtime errors (run each with -e) ===");
print("These terminate execution immediately:");
print('  cargo run --bin mqjs -- -e "var x = 42; x();"');
print('  cargo run --bin mqjs -- -e "return 1/0;"');
print('  cargo run --bin mqjs -- -e "return null + 1;"');
print('  cargo run --bin mqjs -- -e "throw new Error(\'uncaught!\');"');

print("");
print("All runtime error examples complete.");
