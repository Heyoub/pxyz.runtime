//! PXYZ - Workflow Compiler
//!
//! Compiles XML workflow definitions to graph.bin binary format.
//! The binary is executed by pxyz.wasm (WAT runtime) + JS host.
//!
//! ```text
//! workflow.xml → [this crate] → graph.bin → [pxyz.wasm + io.js]
//! ```

#[path = "src/dsl/mod.rs"]
pub mod dsl;
#[path = "src/emit/mod.rs"]
pub mod emit;
#[path = "src/compiler/mod.rs"]
pub mod compiler;
#[path = "src/physics/mod.rs"]
pub mod physics;

// ═══════════════════════════════════════════════════════════════════════════
// SPEC: Binary Format Constants (must match WAT + ARCHITECTURE.md)
// ═══════════════════════════════════════════════════════════════════════════

/// Magic number: "PXYZ" in little-endian
pub const MAGIC: u32 = 0x504E5958;
pub const VERSION_MAJOR: u16 = 1;
pub const VERSION_MINOR: u16 = 0;
pub const HEADER_SIZE: usize = 96;

// ═══════════════════════════════════════════════════════════════════════════
// SPEC: Node Kinds (must match WAT globals)
// ═══════════════════════════════════════════════════════════════════════════

use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    Transform = 0,
    External = 1,
    Render = 2,
    Signal = 3,
    Auth = 4,
    Terminal = 5,
    Error = 6,
}

impl NodeKind {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(Self::Transform),
            1 => Some(Self::External),
            2 => Some(Self::Render),
            3 => Some(Self::Signal),
            4 => Some(Self::Auth),
            5 => Some(Self::Terminal),
            6 => Some(Self::Error),
            _ => None,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::Transform => "transform",
            Self::External => "external",
            Self::Render => "render",
            Self::Signal => "signal",
            Self::Auth => "auth",
            Self::Terminal => "terminal",
            Self::Error => "error",
        }
    }
}

impl std::str::FromStr for NodeKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "transform" => Ok(Self::Transform),
            "external" => Ok(Self::External),
            "render" => Ok(Self::Render),
            "signal" => Ok(Self::Signal),
            "auth" => Ok(Self::Auth),
            "terminal" => Ok(Self::Terminal),
            "error" => Ok(Self::Error),
            _ => Err(format!("Unknown node kind: {}", s)),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SPEC: Flags (must match WAT globals)
// ═══════════════════════════════════════════════════════════════════════════

pub mod node_flags {
    pub const ASYNC: u8 = 0x01;
    pub const REQUIRES_AUTH: u8 = 0x02;
    pub const HAS_SIDE_EFFECTS: u8 = 0x04;
    pub const IRREVERSIBLE: u8 = 0x08;
    pub const REQUIRES_HUMAN: u8 = 0x10;
    pub const CACHEABLE: u8 = 0x20;
}

pub mod edge_flags {
    pub const PARALLEL: u16 = 0x0001;
    pub const FALLBACK: u16 = 0x0002;
    pub const ERROR_EDGE: u16 = 0x0004;
}

// ═══════════════════════════════════════════════════════════════════════════
// SPEC: Safety Limits (must match WAT globals)
// ═══════════════════════════════════════════════════════════════════════════

pub mod limits {
    pub const MAX_VISITED_NODES: usize = 1000;
    pub const MAX_PREDICATE_STEPS: usize = 256;
    pub const MAX_STACK_DEPTH: usize = 16;
    pub const MAX_CALL_DEPTH: usize = 4;
    pub const MAX_PREDICATE_BYTECODE: usize = 256;
}

// ═══════════════════════════════════════════════════════════════════════════
// SPEC: Predicate VM Opcodes (must match WAT)
// ═══════════════════════════════════════════════════════════════════════════

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Stack operations
    Noop = 0x00,
    PushInt = 0x01,    // + 4 bytes (i32)
    PushStr = 0x02,    // + 4 bytes (string pool offset)
    LoadVar = 0x03,    // + 4 bytes (var path offset)
    LoadField = 0x04,  // + 4 bytes (field name offset)
    
    // Comparison
    Eq = 0x10,
    Neq = 0x11,
    Gt = 0x12,
    Gte = 0x13,
    Lt = 0x14,
    Lte = 0x15,
    
    // Logical
    And = 0x20,
    Or = 0x21,
    Not = 0x22,
    
    // String/Collection
    Contains = 0x30,
    Matches = 0x31,
    StartsWith = 0x32,
    EndsWith = 0x33,
    
    // Utility
    Len = 0x40,
    Get = 0x41,
    IsNull = 0x42,
    IsDefined = 0x43,
    IsConfirmed = 0x44,

    // Merge/CRDT operations (Y-constraint application for conflict resolution)
    // These opcodes enable predicates to be used as merge policies
    Timestamp = 0x50,      // pop value ref, push i64 timestamp (for LWW)
    IsFlagged = 0x51,      // pop value ref, push 1 if flagged for review
    Origin = 0x52,         // pop value ref, push origin/author string offset
    VClockGt = 0x53,       // pop 2 value refs, push 1 if first vclock dominates second
    MergeField = 0x54,     // + 1 byte (0=a, 1=b, 2=candidate), load merge context field

    // Control
    CallPred = 0xF0,   // + 2 bytes (predicate ID)
    Ret = 0xFF,
}

impl Opcode {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x00 => Some(Self::Noop),
            0x01 => Some(Self::PushInt),
            0x02 => Some(Self::PushStr),
            0x03 => Some(Self::LoadVar),
            0x04 => Some(Self::LoadField),
            0x10 => Some(Self::Eq),
            0x11 => Some(Self::Neq),
            0x12 => Some(Self::Gt),
            0x13 => Some(Self::Gte),
            0x14 => Some(Self::Lt),
            0x15 => Some(Self::Lte),
            0x20 => Some(Self::And),
            0x21 => Some(Self::Or),
            0x22 => Some(Self::Not),
            0x30 => Some(Self::Contains),
            0x31 => Some(Self::Matches),
            0x32 => Some(Self::StartsWith),
            0x33 => Some(Self::EndsWith),
            0x40 => Some(Self::Len),
            0x41 => Some(Self::Get),
            0x42 => Some(Self::IsNull),
            0x43 => Some(Self::IsDefined),
            0x44 => Some(Self::IsConfirmed),
            0x50 => Some(Self::Timestamp),
            0x51 => Some(Self::IsFlagged),
            0x52 => Some(Self::Origin),
            0x53 => Some(Self::VClockGt),
            0x54 => Some(Self::MergeField),
            0xF0 => Some(Self::CallPred),
            0xFF => Some(Self::Ret),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SPEC: Validation Metadata (compile-time only, NOT in binary)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActorKind {
    #[default]
    Agent,
    Human,
}

impl std::str::FromStr for ActorKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "agent" | "automated" | "bot" => Ok(Self::Agent),
            "human" | "user" | "manual" => Ok(Self::Human),
            _ => Err(format!("Unknown actor kind: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SideEffects {
    #[default]
    None,
    ReadOnly,
    Write,
    Irreversible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfirmationStatus {
    #[default]
    Confirmed,
    Suggested,
    Quarantined,
}

impl std::str::FromStr for ConfirmationStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "confirmed" | "verified" => Ok(Self::Confirmed),
            "suggested" | "pending" | "unconfirmed" => Ok(Self::Suggested),
            "quarantined" | "blocked" | "flagged" => Ok(Self::Quarantined),
            _ => Err(format!("Unknown confirmation status: {}", s)),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMPILER API
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub hint: Option<String>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub workflow_id: Option<String>,
    pub node_id: Option<String>,
    pub edge_id: Option<String>,
    pub predicate_id: Option<String>,
}

#[derive(Debug)]
pub enum CompileError {
    Parse(String),
    Lower(String),
    Predicate(String),
    Emit(String),
    Io(std::io::Error),
    Validation { diagnostics: Vec<Diagnostic> },
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "Parse error: {}", s),
            Self::Lower(s) => write!(f, "Lower error: {}", s),
            Self::Predicate(s) => write!(f, "Predicate error: {}", s),
            Self::Emit(s) => write!(f, "Emit error: {}", s),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Validation { diagnostics } => {
                write!(f, "Validation failed: {} error(s)", 
                    diagnostics.iter().filter(|d| d.severity == Severity::Error).count())
            }
        }
    }
}

impl std::error::Error for CompileError {}

impl From<std::io::Error> for CompileError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    pub optimize: bool,
    pub strict: bool,
    pub emit_audit: bool,
}

#[derive(Debug)]
pub struct CompileResult {
    pub binary: Vec<u8>,
    pub audit: Option<emit::audit::GraphAudit>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Compile workflow XML to graph.bin
pub fn compile(xml: &str, options: &CompileOptions) -> Result<CompileResult, CompileError> {
    // Parse XML → AST
    let ast = dsl::parse(xml).map_err(|e| CompileError::Parse(e.to_string()))?;
    
    // Lower AST → IR
    let mut ir = compiler::lower(&ast).map_err(|e| CompileError::Lower(e.to_string()))?;
    
    // Compile predicates to bytecode
    ir = compiler::compile_predicates(ir, &ast).map_err(|e| CompileError::Predicate(e.to_string()))?;
    
    // Optimize
    if options.optimize {
        ir = compiler::optimize(ir);
    }
    
    // Validate
    let mut diagnostics = Vec::new();
    diagnostics.extend(compiler::check_syntactic(&ir));
    diagnostics.extend(compiler::check_semantic(&ir));
    diagnostics.extend(compiler::check_pragmatic(&ir));
    
    // Check for errors
    let has_errors = diagnostics.iter().any(|d| d.severity == Severity::Error);
    let has_warnings = diagnostics.iter().any(|d| d.severity == Severity::Warn);
    
    if has_errors || (options.strict && has_warnings) {
        return Err(CompileError::Validation { diagnostics });
    }
    
    // Emit binary
    let binary = emit::binary::emit(&ir, xml).map_err(|e| CompileError::Emit(e.to_string()))?;
    
    // Generate audit
    let audit = if options.emit_audit {
        Some(emit::audit::generate(&ir, xml, &binary, &diagnostics))
    } else {
        None
    };
    
    Ok(CompileResult { binary, audit, diagnostics })
}

/// Validate without compiling
pub fn validate(xml: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    let ast = match dsl::parse(xml) {
        Ok(ast) => ast,
        Err(e) => {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "PARSE".into(),
                message: e.to_string(),
                hint: None,
                location: None,
            });
            return diagnostics;
        }
    };
    
    let ir = match compiler::lower(&ast) {
        Ok(ir) => ir,
        Err(e) => {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "LOWER".into(),
                message: e.to_string(),
                hint: None,
                location: None,
            });
            return diagnostics;
        }
    };
    
    diagnostics.extend(compiler::check_syntactic(&ir));
    diagnostics.extend(compiler::check_semantic(&ir));
    diagnostics.extend(compiler::check_pragmatic(&ir));
    
    diagnostics
}

/// Inspect a compiled graph.bin
pub fn inspect(data: &[u8]) -> Result<GraphInfo, String> {
    if data.len() < HEADER_SIZE {
        return Err("Binary too small".into());
    }
    
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    if magic != MAGIC {
        return Err(format!("Invalid magic: 0x{:08x}", magic));
    }
    
    Ok(GraphInfo {
        version: format!("{}.{}", 
            u16::from_le_bytes([data[4], data[5]]),
            u16::from_le_bytes([data[6], data[7]])),
        node_count: u32::from_le_bytes([data[8], data[9], data[10], data[11]]),
        edge_count: u32::from_le_bytes([data[12], data[13], data[14], data[15]]),
        predicate_count: u32::from_le_bytes([data[16], data[17], data[18], data[19]]),
        string_pool_size: u32::from_le_bytes([data[20], data[21], data[22], data[23]]),
        entry_count: u32::from_le_bytes([data[24], data[25], data[26], data[27]]),
        binary_size: data.len(),
    })
}

#[derive(Debug, Clone)]
pub struct GraphInfo {
    pub version: String,
    pub node_count: u32,
    pub edge_count: u32,
    pub predicate_count: u32,
    pub string_pool_size: u32,
    pub entry_count: u32,
    pub binary_size: usize,
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// FNV-1a hash for (P, X) entry lookup - must match WAT implementation
pub fn hash_px(p: &str, x: &str) -> u32 {
    const FNV_PRIME: u32 = 0x01000193;
    const FNV_OFFSET: u32 = 0x811c9dc5;
    
    let mut hash = FNV_OFFSET;
    for byte in p.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash ^= 0xFF; // separator
    hash = hash.wrapping_mul(FNV_PRIME);
    for byte in x.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Check if an IO opcode is irreversible (for PRAG003 validation)
/// NOTE: The actual dispatch happens in JS, this is just for compile-time checks
pub fn is_irreversible_op(code: u16) -> bool {
    matches!(code, 
        0x0332 |  // GoogleGmailSend
        0x0340 |  // EmailSend
        0x0350 |  // SmsSend
        0x0360    // WebhookCall
    )
}

/// Check if an IO opcode is an LLM call (for PRAG004 validation)
pub fn is_llm_op(code: u16) -> bool {
    (0x0800..=0x08FF).contains(&code)
}

/// Check if an IO opcode writes data
pub fn is_write_op(code: u16) -> bool {
    matches!(code, 
        0x0100 | 0x0102 | 0x0103 | // Entity CUD
        0x0302 |                    // Google create
        0x0401 | 0x0402 | 0x0403 | // HTTP POST/PUT/DELETE
        0x0701 |                    // Qdrant index
        0x0901 | 0x0910            // Storage write
    )
}