# MQuickJS-RS JavaScript 特性规范

本文档记录 MQuickJS-RS 引擎当前实现层面的 JavaScript 特性集合。

对于 ESP32 LED 特效等产品脚本，请优先以 `docs/LED_PROFILE.md` 为准。`LED_PROFILE` 是产品约束，本文档是实现说明；两者不一致时，应以产品 Profile 驱动后续实现和修正。

## 核心设计约束

### 纯整数运算（无浮点数）

引擎只支持 **31 位有符号整数**（`-1073741824` ~ `1073741823`），**没有浮点数类型**。这是最核心的设计取舍，影响了大量语义行为：

- `NaN` 内部表示为 `0`，`Infinity` 表示为 `i32::MAX`
- `NaN === NaN` 返回 `true`（因为底层是 `0 === 0`）
- `isNaN(NaN)` 返回 `false`（因为 NaN 就是整数 0）
- `1/0` 抛出除零错误（而非返回 `Infinity`）
- `parseFloat("3.14")` 返回 `3`（截断为整数）
- `Math.PI` 等常量为整数近似值
- `Number.MAX_VALUE` / `MIN_VALUE` 为 31 位整数范围

### 严格类型模式（有限的隐式类型转换）

算术运算（`+`数值加法、`-`、`*`、`/`、`%`）和比较运算（`<`、`<=`、`>`、`>=`）只接受整数操作数，不像标准 JS 那样自动将 `null`、`boolean`、`string` 转换为数字：

| 表达式 | 标准 JS | MQuickJS | 说明 |
|--------|---------|----------|------|
| `null + 1` | `1` | TypeError | 不隐式转换 null |
| `true + true` | `2` | TypeError | 不隐式转换 boolean |
| `"5" - 3` | `2` | TypeError | 不隐式转换 string |
| `"5" + 3` | `"53"` | `"53"` | **字符串拼接正常** |
| `false + 1` | `1` | TypeError | 不隐式转换 boolean |

**注意**：`+` 运算符的字符串拼接路径支持丰富的类型转换——当任一操作数为字符串时，会将 number、boolean、null、undefined、object 转为字符串后拼接。`Number()` / `Boolean()` / `String()` 类型转换函数也正常工作（`Number(true)` → `1`）。仅算术和比较的隐式转换被禁用。

### 相等运算符

`==` 和 `===` 的行为相同，均为严格的位模式比较（`Value` 的 u64 值相等即为 true）。不实现标准 JS 中 `==` 的抽象比较算法（不做类型转换后比较）。

### 编译时变量解析

未声明的变量在编译阶段就会报错（ReferenceError），而非运行时。这意味着 `try { undeclaredVar } catch(e) {}` 无法捕获这类错误。

---

## 语言特性

### 支持

| 特性 | 说明 |
|------|------|
| `var` / `let` / `const` | 变量声明 |
| `function` 声明 / 表达式 | 包括递归 |
| 箭头函数 | `(x) => x + 1` |
| 闭包 | 值捕获语义 |
| `if` / `else` | 条件语句 |
| `while` / `for` / `for-in` / `for-of` | 循环 |
| `break` / `continue` | 循环控制 |
| `try` / `catch` / `finally` | 异常处理（显式 throw 的值） |
| `throw` | 支持抛出任意值（Error、string、number 等） |
| `new` | 构造函数（含隐式 this 返回） |
| `typeof` | 返回正确的类型字符串 |
| `instanceof` | 基于构造函数引用判断 |
| `delete` | 删除对象属性 |
| `in` | 属性存在检查 |
| 三元运算符 | `a ? b : c` |
| 短路逻辑 | `&&` / `||` |
| 位运算 | `&` `|` `^` `~` `<<` `>>` `>>>` |
| 赋值运算 | `=` `+=` `-=` `*=` `/=` `%=` `&=` `|=` `^=` `<<=` `>>=` `>>>=` |
| `++` / `--` | 前缀和后缀 |
| 幂运算 | `**` |
| 对象字面量 | `{key: value}` |
| 数组字面量 | `[1, 2, 3]`，支持尾逗号 |
| 属性访问 | `obj.prop` 和 `obj["prop"]` |
| 数组下标 | `arr[i]`，越界返回 undefined |
| 字符串字面量 | 含转义字符，支持拼接 |
| 正则表达式字面量 | `/pattern/flags` |
| 字符串拼接类型转换 | `"x" + null` → `"xnull"`，`"x" + 42` → `"x42"` 等 |

### 不支持

| 特性 | 说明 |
|------|------|
| 浮点数 | 核心设计——仅 31 位整数 |
| `class` | 无 ES6 class 语法 |
| Generator / `yield` | 不支持 |
| `async` / `await` / Promise | 不支持 |
| ES6 模块 `import` / `export` | 不支持 |
| 解构赋值 | `let {a, b} = obj` 不支持 |
| 展开运算符 | `...args` 不支持 |
| Symbol | 不支持 |
| Map / Set / WeakMap / WeakSet | 不支持 |
| Proxy / Reflect | 不支持 |
| BigInt | 不支持 |
| 可选链 | `?.` 不支持 |
| 空值合并 | `??` 不支持 |
| `with` 语句 | 不支持 |
| 标签语句 | `label: for(...)` 不支持 |
| `switch` | 不支持 |
| `do...while` | 不支持 |
| getter / setter | `get` / `set` 不支持 |
| 计算属性名 | `{[expr]: value}` 不支持 |
| 模板字符串 / 标签模板 | `` `${expr}` `` 不支持 |
| `eval()` | 不支持（编译时变量解析） |
| `arguments` 对象 | 不支持 |
| 严格模式 `"use strict"` | 默认即严格行为 |

---

## 内置对象

### Object

| 方法/属性 | 支持 | 说明 |
|-----------|------|------|
| `Object.keys(obj)` | 是 | |
| `Object.values(obj)` | 是 | |
| `Object.entries(obj)` | 是 | |
| `Object.create(proto)` | 是 | |
| `Object.defineProperty()` | 是 | |
| `Object.getPrototypeOf()` | 是 | |
| `Object.setPrototypeOf()` | 是 | |
| `hasOwnProperty(prop)` | 是 | |
| `toString()` | 是 | |
| `Object.assign()` | 否 | |
| `Object.freeze()` / `seal()` | 否 | |

### Array

| 方法/属性 | 支持 | 说明 |
|-----------|------|------|
| `length` | 是 | |
| `push` / `pop` | 是 | |
| `shift` / `unshift` | 是 | |
| `indexOf(value)` | 是 | 注：`fromIndex` 参数未实现 |
| `lastIndexOf` | 是 | |
| `slice(start, end)` | 是 | 含负索引支持 |
| `splice` | 是 | |
| `join(sep)` | 是 | |
| `reverse()` | 是 | |
| `concat(...arrays)` | 是 | |
| `sort(compareFn?)` | 是 | |
| `map` / `filter` / `forEach` | 是 | |
| `reduce` / `reduceRight` | 是 | |
| `find` / `findIndex` | 是 | |
| `some` / `every` | 是 | |
| `includes(value)` | 是 | |
| `flat(depth?)` | 是 | |
| `fill(value, start?, end?)` | 是 | |
| `toString()` | 是 | |
| `Array.isArray(value)` | 是 | |
| `Array.from()` | 否 | |
| `Array.of()` | 否 | |
| `entries` / `keys` / `values` | 否 | 无迭代器协议 |
| `flatMap` | 否 | |

### String

| 方法/属性 | 支持 | 说明 |
|-----------|------|------|
| `length` | 是 | |
| `charAt` / `charCodeAt` / `codePointAt` | 是 | |
| `indexOf` / `lastIndexOf` | 是 | |
| `slice(start, end)` | 是 | 含负索引 |
| `substring(start, end)` | 是 | |
| `toUpperCase` / `toLowerCase` | 是 | |
| `trim` / `trimStart` / `trimEnd` | 是 | |
| `split(separator)` | 是 | 注：空字符串 `split("")` 有缺陷 |
| `concat` | 是 | |
| `repeat(count)` | 是 | |
| `startsWith` / `endsWith` / `includes` | 是 | |
| `padStart` / `padEnd` | 是 | |
| `replace` / `replaceAll` | 是 | |
| `match(regexp)` / `search(regexp)` | 是 | |
| `String.fromCharCode` / `fromCodePoint` | 是 | |
| `matchAll` | 否 | |
| `normalize` | 否 | |
| `[Symbol.iterator]` | 否 | 无 Symbol |

### Number

| 方法/属性 | 支持 | 说明 |
|-----------|------|------|
| `Number.isInteger(value)` | 是 | 所有值都是整数 |
| `Number.isNaN(value)` | 是 | 但 NaN 内部为 0 |
| `Number.isFinite(value)` | 是 | |
| `Number.parseInt(value)` | 是 | |
| `MAX_VALUE` / `MIN_VALUE` | 是 | 31 位整数范围 |
| `MAX_SAFE_INTEGER` / `MIN_SAFE_INTEGER` | 是 | 31 位整数范围 |
| `toString(radix?)` | 是 | |
| `toFixed(digits)` | 是 | |
| `toExponential(digits)` | 是 | |
| `toPrecision(digits)` | 是 | |
| `Number.parseFloat` | 否 | |
| `Number.EPSILON` | 否 | 无浮点 |
| `Number.POSITIVE_INFINITY` | 否 | 伪 Infinity |
| `Number.NEGATIVE_INFINITY` | 否 | |

### Math

| 方法 | 支持 | 说明 |
|------|------|------|
| `abs` `floor` `ceil` `round` `trunc` | 是 | 整数运算（floor/ceil 无实际效果） |
| `sqrt` `pow` | 是 | 整数版本 |
| `max` `min` `sign` | 是 | |
| `sin` `cos` `tan` `asin` `acos` `atan` `atan2` | 是 | 整数近似 |
| `exp` `log` `log2` `log10` | 是 | 整数近似 |
| `random` | 是 | |
| `imul` `clz32` `fround` | 是 | |
| `PI` `E` `LN2` `LN10` 等常量 | 是 | 整数截断值 |
| `hypot` `cbrt` `sinh` `cosh` `tanh` | 否 | |

### JSON

| 方法 | 支持 | 说明 |
|------|------|------|
| `JSON.stringify(value)` | 是 | 支持 number/boolean/null/string/array/object |
| `JSON.parse(string)` | 是 | 含正确的转义处理 |
| `JSON.stringify(value, replacer, space)` | 否 | 无 replacer / space 参数 |
| `JSON.parse(text, reviver)` | 否 | 无 reviver 参数 |

### RegExp

| 方法/属性 | 支持 | 说明 |
|-----------|------|------|
| `new RegExp(pattern, flags)` | 是 | |
| `/pattern/flags` 字面量 | 是 | |
| `test(string)` | 是 | |
| `exec(string)` | 是 | 返回 match 数组或 null |
| `source` / `flags` / `lastIndex` | 是 | |
| `g` / `i` / `m` flags | 是 | |
| `u` / `s` / `y` flags | 否 | |
| 命名捕获组 | 否 | |
| `[Symbol.match]` 等 | 否 | 无 Symbol |

### Error

| 类型 | 支持 |
|------|------|
| `Error` | 是 |
| `TypeError` | 是 |
| `ReferenceError` | 是 |
| `SyntaxError` | 是 |
| `RangeError` | 是 |
| `EvalError` | 是 |
| `URIError` | 是 |
| `InternalError` | 是 |

所有错误类型都有 `name`、`message`、`stack`、`toString()` 属性。

### TypedArray

| 类型 | 支持 |
|------|------|
| `Int8Array` / `Uint8Array` / `Uint8ClampedArray` | 是 |
| `Int16Array` / `Uint16Array` | 是 |
| `Int32Array` / `Uint32Array` | 是 |
| `Float32Array` / `Float64Array` | 是 |

支持的操作：构造（from length / from array）、元素访问、`length`、`byteLength`、`BYTES_PER_ELEMENT`、`subarray(begin, end)`。

### Date

| 方法 | 支持 | 说明 |
|------|------|------|
| `Date.now()` | 是 | 受 31 位整数限制，约 12 天窗口 |
| `new Date()` | 否 | |
| 其他 Date 方法 | 否 | |

### Function

| 方法 | 支持 |
|------|------|
| `call(thisArg, ...args)` | 是 |
| `apply(thisArg, argsArray)` | 是 |
| `bind(thisArg, ...args)` | 是 |
| `toString()` | 是 |

---

## 全局函数与对象

| 函数/对象 | 支持 | 说明 |
|-----------|------|------|
| `parseInt(string)` | 部分 | 仅接受已为整数的值；不解析字符串、不支持 hex `0x`、不支持部分解析 |
| `parseFloat(string)` | 部分 | 能解析字符串中的整数部分，小数点后截断（无浮点） |
| `isNaN(value)` | 部分 | 对非整数类型一律返回 true（无真正的 NaN 值） |
| `isFinite(value)` | 是 | |
| `Boolean(value)` | 是 | 类型转换函数 |
| `Number(value)` | 是 | 类型转换函数 |
| `String(value)` | 是 | 类型转换函数 |
| `print(...)` | 是 | 输出到 stdout |
| `console.log` / `error` / `warn` | 是 | |
| `gc()` | 是 | 触发垃圾回收 |
| `load(filename)` | 是 | 加载并执行 JS 文件 |
| `setTimeout(cb, delay)` | 是 | 返回 timer ID |
| `clearTimeout(id)` | 是 | |
| `performance.now` | 是 | |
| `globalThis` | 是 | 含所有内置对象 |
| `encodeURIComponent` / `decodeURIComponent` | 否 | |
| `eval()` | 否 | |
| `Proxy` / `Reflect` | 否 | |

---

## 已知限制与偏差

### try/catch 错误处理

所有运行时错误（TypeError、RangeError、InternalError 等）均可被 try/catch 捕获，包括：

- 调用非函数值（`var x = 42; x()`）→ TypeError
- 栈溢出（无限递归）→ InternalError
- 除零错误（`1/0`）→ RangeError
- 类型错误（`null + 1`）→ TypeError

显式 `throw` 和引擎内部运行时错误走统一的异常处理路径（`exception_handlers` 栈）。

### parseInt / parseFloat 偏差

| 输入 | 标准 JS | MQuickJS |
|------|---------|----------|
| `parseInt("")` | `NaN` | `0` |
| `parseInt("0x1f")` | `31` | `0` |
| `parseInt("123abc")` | `123` | `0` |
| `parseInt("abc")` | `NaN` | `0` |
| `parseFloat("3.14")` | `3.14` | `3` |
| `parseFloat("")` | `NaN` | `0` |

### String/Array 方法偏差

- `"abc".split("")` 无法正确拆分为单字符数组（空分隔符未做特殊处理）
- `Array.prototype.indexOf(value, fromIndex)` 的 `fromIndex` 参数未实现，始终从索引 0 开始搜索
- `String.prototype.split(sep)` 的 `limit` 参数未实现

---

## 版本

本文档基于 MQuickJS-RS 当前代码库（373 个测试通过）。
