// MQuickJS-RS error handling and edge case tests
//
// Three parts:
//   PART 1: Working features
//   PART 2: Design tradeoffs (expected behavior of integer-only engine)
//   PART 3: Implementation bugs (fixable)

// ============================================================
// PART 1: Working features
// ============================================================

print("=== throw / catch ===");
try { throw new Error("basic error"); } catch (e) { print("OK: " + e.name + ": " + e.message); }
try { throw new TypeError("type error"); } catch (e) { print("OK: " + e.name + ": " + e.message); }
try { throw new RangeError("range error"); } catch (e) { print("OK: " + e.name + ": " + e.message); }

print("\n=== Throw primitives ===");
try { throw 42; } catch (e) { print("OK number: " + e); }
try { throw "a string error"; } catch (e) { print("OK string: " + e); }
try { throw null; } catch (e) { print("OK null: " + e); }
try { throw undefined; } catch (e) { print("OK undefined: " + e); }
try { throw true; } catch (e) { print("OK boolean: " + e); }

print("\n=== Re-throw ===");
try {
    try { throw new Error("original"); } catch (e) { throw e; }
} catch (e) {
    print("OK re-caught: " + e.message);
}

print("\n=== Finally ===");
try { throw new Error("boom"); } catch (e) { print("catch: " + e.message); } finally { print("OK finally executed"); }
try {
    try { throw new Error("inner"); } finally { print("OK inner finally"); }
} catch (e) {
    print("OK outer catch: " + e.message);
}

print("\n=== String concat coercion ===");
print("'5' + 3 = " + ("5" + 3));
print("'hello' + true = " + ("hello" + true));
print("'x' + null = " + ("x" + null));

print("\n=== Comparison ===");
print("1 < 2: " + (1 < 2));
print("2 >= 2: " + (2 >= 2));
print("3 !== 4: " + (3 !== 4));

print("\n=== String methods ===");
print("indexOf('ll') = " + "hello".indexOf("ll"));
print("slice(-2) = " + "hello".slice(-2));
print("slice(2,-1) = " + "hello".slice(2, -1));
print("toUpperCase = " + "hello".toUpperCase());
print("trim = '" + " abc ".trim() + "'");

print("\n=== Array methods ===");
var a1 = [3, 1, 4, 1, 5];
print("indexOf(4) = " + a1.indexOf(4));
print("indexOf(9) = " + a1.indexOf(9));
print("includes(4) = " + a1.includes(4));

// ============================================================
// PART 2: Design tradeoffs (integer-only engine, expected)
// ============================================================

print("\n=== [DESIGN] Integer-only arithmetic ===");
// NaN is internally 0, Infinity is i32::MAX
print("NaN === NaN: " + (NaN === NaN) + " (std: false, here: true, NaN=0)");
print("isNaN(NaN): " + isNaN(NaN) + " (std: true, here: false, NaN=0)");
print("parseFloat('3.14') = " + parseFloat("3.14") + " (std: 3.14, here: 3)");

print("\n=== [DESIGN] Strict types - no implicit coercion ===");
// These throw TypeError in MQuickJS (standard JS would coerce):
// null + 1 -> TypeError (std: 1)
// true + true -> TypeError (std: 2)
// "5" - 3 -> TypeError (std: 2)
// 1 / 0 -> DivisionByZero (std: Infinity)
print("null+1, true+true, '5'-3, 1/0 all throw errors");

print("\n=== [DESIGN] Compile-time variable resolution ===");
// Undeclared variables cause compile error, not catchable by try/catch
print("undeclaredVar -> compile-time ReferenceError (not runtime)");

// ============================================================
// PART 3: Fixes and remaining issues
// ============================================================

print("\n=== [FIXED] Runtime errors caught by try/catch ===");
try { var x = 42; x(); } catch (e) { print("OK: call-non-function caught: " + e.name); }
try { var r = 1/0; } catch (e) { print("OK: division-by-zero caught: " + e.name); }
try { var bad = null + 1; } catch (e) { print("OK: type-error caught: " + e.name); }
try { function rec() { return rec(); } rec(); } catch (e) { print("OK: stack-overflow caught: " + e.name); }

print("\n=== [BUG] parseInt lacks hex and partial parse ===");
print("parseInt('') = " + parseInt("") + " (expected NaN, got 0)");
print("parseInt('0x1f') = " + parseInt("0x1f") + " (expected 31, got 0)");
print("parseInt('123abc') = " + parseInt("123abc") + " (expected 123, got 0)");
print("parseInt('  42  ') = " + parseInt("  42  ") + " (expected 42, got 0)");

print("\n=== [BUG] String.split('') ===");
print("'abc'.split('') = " + "abc".split("").join(",") + " (expected a,b,c)");

print("\n=== [BUG] Array.indexOf fromIndex ===");
var a2 = [3, 1, 4, 1, 5];
print("indexOf(1, 2) = " + a2.indexOf(1, 2) + " (expected 3, fromIndex ignored)");

print("\n=== All tests complete ===");
