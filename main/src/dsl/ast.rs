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