//! AST Types for XML DSL

use serde::{Deserialize, Serialize};

/// Root document
#[derive(Debug, Clone, Default)]
pub struct OmarDocument {
    pub version: String,
    pub schemas: Vec<Schema>,
    pub predicates: Vec<PredicateDef>,
    pub workflows: Vec<Workflow>,
    pub templates: Vec<Template>,
    /// Merge policies for CRDT conflict resolution (Y-constraint application)
    pub merge_policies: Vec<EntityMerge>,
}

/// Schema definition
#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

/// Field definition
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub default: Option<String>,
    pub pattern: Option<String>,
}

/// Predicate definition
#[derive(Debug, Clone)]
pub struct PredicateDef {
    pub id: String,
    pub expr: PredicateExpr,
}

/// Predicate expression
#[derive(Debug, Clone)]
pub enum PredicateExpr {
    Always,
    Fail,
    Eq { left: String, right: Value },
    Neq { left: String, right: Value },
    Gt { left: String, right: Value },
    Gte { left: String, right: Value },
    Lt { left: String, right: Value },
    Lte { left: String, right: Value },
    Contains { left: String, right: String },
    Matches { left: String, pattern: String },
    StartsWith { left: String, prefix: String },
    EndsWith { left: String, suffix: String },
    And { conditions: Vec<PredicateExpr> },
    Or { conditions: Vec<PredicateExpr> },
    Not { condition: Box<PredicateExpr> },
    Ref { predicate: String },
    Fn { name: String, arg: String },
}

/// Value in predicate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Var(String),
}

impl Value {
    pub fn from_str_guess(s: &str) -> Self {
        if s.starts_with('$') {
            return Value::Var(s.to_string());
        }
        if let Ok(i) = s.parse::<i64>() {
            return Value::Int(i);
        }
        if let Ok(f) = s.parse::<f64>() {
            return Value::Float(f);
        }
        if s == "true" {
            return Value::Bool(true);
        }
        if s == "false" {
            return Value::Bool(false);
        }
        Value::Str(s.to_string())
    }
}

/// Workflow definition
#[derive(Debug, Clone)]
pub struct Workflow {
    pub id: String,
    pub description: Option<String>,
    pub entry: EntryPoint,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

/// Entry point
#[derive(Debug, Clone)]
pub struct EntryPoint {
    pub p: String,
    pub x: String,
    pub node: String,
}

/// Node definition
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub kind: String,
    pub op: Option<String>,
    pub template: Option<String>,
    pub schema: Option<String>,
    pub predicate: Option<String>,
    pub selector: Option<String>,
    pub status: Option<u16>,
    pub message: Option<String>,
    pub signals: Vec<(String, String)>,
    pub actor: Option<String>,
    pub confirmation: Option<String>,
    pub async_node: bool,
    pub cacheable: bool,
    pub data: std::collections::HashMap<String, String>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: String::new(),
            kind: "transform".into(),
            op: None,
            template: None,
            schema: None,
            predicate: None,
            selector: None,
            status: None,
            message: None,
            signals: Vec::new(),
            actor: None,
            confirmation: None,
            async_node: false,
            cacheable: false,
            data: std::collections::HashMap::new(),
        }
    }
}

/// Edge definition
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub predicate: Option<PredicateExpr>,
    pub predicate_ref: Option<String>,
    pub weight: Option<u16>,
    pub parallel: bool,
    pub fallback: bool,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            from: String::new(),
            to: String::new(),
            predicate: None,
            predicate_ref: None,
            weight: None,
            parallel: false,
            fallback: false,
        }
    }
}

/// Template definition
#[derive(Debug, Clone)]
pub struct Template {
    pub id: String,
    pub content: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// MERGE POLICIES (Y-constraint application for CRDT conflict resolution)
// ═══════════════════════════════════════════════════════════════════════════

/// Merge policy determines how CRDT conflicts are resolved.
///
/// The predicate VM evaluates these policies with a merge context containing:
/// - `$a` - The first conflicting value
/// - `$b` - The second conflicting value
/// - `$candidate` - The proposed merged value (for validation)
///
/// Built-in strategies compile to bytecode; custom predicates use the full VM.
#[derive(Debug, Clone)]
pub enum MergePolicy {
    /// Last-Writer-Wins: Compare timestamps, higher wins
    /// Compiles to: timestamp($a) > timestamp($b) ? $a : $b
    LWW,

    /// First-Writer-Wins: Compare timestamps, lower wins (immutable-ish)
    /// Compiles to: timestamp($a) < timestamp($b) ? $a : $b
    FWW,

    /// Vector clock dominance: True partial ordering
    /// Compiles to: vclock_gt($a, $b) ? $a : (vclock_gt($b, $a) ? $b : CONFLICT)
    VClock,

    /// Maximum value wins (for counters, versions)
    /// Compiles to: $a > $b ? $a : $b
    Max,

    /// Minimum value wins
    /// Compiles to: $a < $b ? $a : $b
    Min,

    /// Union of sets/arrays
    /// Compiles to: union($a, $b)
    Union,

    /// Intersection of sets/arrays
    /// Compiles to: intersect($a, $b)
    Intersect,

    /// Flag for human review - never auto-resolve
    /// Compiles to: flag_for_review($a, $b); return PENDING
    HumanReview,

    /// Origin-based: Prefer values from specific actor
    /// Compiles to: origin($a) == preferred ? $a : $b
    PreferOrigin { actor: String },

    /// Custom predicate: Full predicate expression for complex logic
    /// The predicate must return: 0 = use $a, 1 = use $b, 2 = use $candidate
    Custom { predicate: String },
}

impl Default for MergePolicy {
    fn default() -> Self {
        MergePolicy::LWW
    }
}

/// Field-level merge configuration
#[derive(Debug, Clone)]
pub struct FieldMerge {
    /// Field name/path
    pub field: String,
    /// Merge policy for this field
    pub policy: MergePolicy,
    /// Optional validation predicate for merged result
    pub validate: Option<String>,
}

/// Entity-level merge configuration
#[derive(Debug, Clone)]
pub struct EntityMerge {
    /// Entity/schema name this applies to
    pub entity: String,
    /// Default policy for fields without explicit config
    pub default_policy: MergePolicy,
    /// Field-specific overrides
    pub fields: Vec<FieldMerge>,
    /// Predicate that must pass for any merge to proceed
    pub pre_condition: Option<String>,
    /// Predicate to validate final merged entity
    pub post_validate: Option<String>,
}