//! JavaScript execution context
//!
//! The Context is the main entry point for the JavaScript engine.
//! It owns all memory and provides the API for evaluating JavaScript code.

use crate::gc::Heap;
use crate::parser::compiler::{CompileError, Compiler};
use crate::runtime::FunctionBytecode;
use crate::value::Value;
use crate::vm::Interpreter;

/// JavaScript execution context
///
/// The Context owns all memory used by the JavaScript engine.
/// Memory layout: [JSContext | Heap (grows up) | ... free ... | Stack (grows down)]
pub struct Context {
    /// The memory heap for GC-managed objects
    heap: Heap,

    /// Bytecode interpreter
    interpreter: Interpreter,

    /// Current exception (if any)
    current_exception: Value,

    /// Whether we're in the process of handling out-of-memory
    in_out_of_memory: bool,
}

/// Error from JavaScript evaluation
#[derive(Debug)]
pub enum EvalError {
    /// Compilation error
    CompileError(CompileError),
    /// Runtime error
    RuntimeError(String),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::CompileError(e) => write!(f, "Compile error: {}", e),
            EvalError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for EvalError {}

impl From<CompileError> for EvalError {
    fn from(e: CompileError) -> Self {
        EvalError::CompileError(e)
    }
}

/// Memory usage statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct MemoryStats {
    /// Total memory size
    pub heap_size: usize,
    /// Currently used heap memory
    pub used: usize,
    /// Currently used stack memory
    pub stack_used: usize,
    /// Free memory available
    pub free: usize,
    /// Number of runtime strings
    pub runtime_strings: usize,
    /// Number of arrays
    pub arrays: usize,
    /// Number of objects
    pub objects: usize,
    /// Number of closures
    pub closures: usize,
    /// Number of error objects
    pub error_objects: usize,
    /// Number of regex objects
    pub regex_objects: usize,
    /// Number of typed arrays
    pub typed_arrays: usize,
}

impl Context {
    /// Create a new JavaScript context with the given memory size
    ///
    /// # Arguments
    /// * `mem_size` - Total memory available for the JS engine in bytes
    ///
    /// # Panics
    /// Panics if mem_size is too small (minimum ~4KB recommended)
    pub fn new(mem_size: usize) -> Self {
        const MIN_MEM_SIZE: usize = 4096;
        assert!(
            mem_size >= MIN_MEM_SIZE,
            "Memory size must be at least {} bytes",
            MIN_MEM_SIZE
        );

        Context {
            heap: Heap::new(mem_size),
            interpreter: Interpreter::new(),
            current_exception: Value::undefined(),
            in_out_of_memory: false,
        }
    }

    /// Evaluate JavaScript source code
    ///
    /// # Arguments
    /// * `source` - JavaScript source code as a string
    ///
    /// # Returns
    /// The result of evaluating the code, or an error
    pub fn eval(&mut self, source: &str) -> Result<Value, EvalError> {
        // Compile the source code
        let compiled = Compiler::new(source).compile()?;

        // Convert to FunctionBytecode for the interpreter
        let bytecode = Self::compiled_to_bytecode(compiled);

        // Execute the bytecode
        self.interpreter
            .execute(&bytecode)
            .map_err(|e| EvalError::RuntimeError(e.to_string()))
    }

    /// Convert CompiledFunction to FunctionBytecode (recursive for inner functions)
    fn compiled_to_bytecode(
        compiled: crate::parser::compiler::CompiledFunction,
    ) -> FunctionBytecode {
        use crate::runtime::CaptureInfo;

        let inner_functions = compiled
            .functions
            .into_iter()
            .map(Self::compiled_to_bytecode)
            .collect();

        // Convert compiler's CaptureInfo to runtime's CaptureInfo
        let captures = compiled
            .captures
            .into_iter()
            .map(|c| CaptureInfo {
                outer_index: c.outer_index,
                is_local: c.is_local,
            })
            .collect();

        FunctionBytecode {
            name: None,
            arg_count: compiled.arg_count as u16,
            local_count: compiled.local_count as u16,
            stack_size: 64, // Default stack size
            has_arguments: false,
            bytecode: compiled.bytecode,
            constants: compiled.constants,
            string_constants: compiled.string_constants,
            source_file: None,
            line_numbers: Vec::new(),
            inner_functions,
            captures,
        }
    }

    /// Compile JavaScript source code without executing
    ///
    /// Returns the compiled bytecode for inspection or later execution.
    pub fn compile(&self, source: &str) -> Result<FunctionBytecode, CompileError> {
        let compiled = Compiler::new(source).compile()?;
        Ok(Self::compiled_to_bytecode(compiled))
    }

    /// Execute pre-compiled bytecode
    pub fn execute(&mut self, bytecode: &FunctionBytecode) -> Result<Value, EvalError> {
        self.interpreter
            .execute(bytecode)
            .map_err(|e| EvalError::RuntimeError(e.to_string()))
    }

    /// Run the garbage collector
    pub fn gc(&mut self) {
        self.heap.collect();
    }

    /// Get memory usage statistics
    pub fn memory_stats(&self) -> MemoryStats {
        let interp_stats = self.interpreter.get_stats();
        MemoryStats {
            heap_size: self.heap.total_size,
            used: self.heap.heap_used(),
            stack_used: self.heap.stack_used(),
            free: self.heap.free_space(),
            runtime_strings: interp_stats.runtime_strings,
            arrays: interp_stats.arrays,
            objects: interp_stats.objects,
            closures: interp_stats.closures,
            error_objects: interp_stats.error_objects,
            regex_objects: interp_stats.regex_objects,
            typed_arrays: interp_stats.typed_arrays,
        }
    }

    /// Get the current exception (if any)
    pub fn get_exception(&self) -> Value {
        self.current_exception
    }

    /// Clear the current exception
    pub fn clear_exception(&mut self) {
        self.current_exception = Value::undefined();
    }
}
