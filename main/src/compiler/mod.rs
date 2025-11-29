//! Compiler Module
//!
//! All compile-time logic for transforming XML → graph.bin:
//! - IR types and lowering
//! - Predicate bytecode compilation
//! - Constraint validation (syntactic, semantic, pragmatic)
//! - Optimization passes

pub mod ir;
pub mod lower;
pub mod bytecode;
pub mod optimize;
pub mod syntactic;
pub mod semantic;
pub mod pragmatic;

use crate::{Diagnostic, CompileError};
use crate::dsl::ast::{OmarDocument, PredicateExpr};

// Re-export commonly used items
pub use ir::{GraphIR, GNode, GEdge, GEntry, CompiledPredicate, StringPool};
pub use lower::lower;
pub use optimize::optimize;

/// Compile all predicates in the IR to bytecode
pub fn compile_predicates(mut ir: GraphIR, doc: &OmarDocument) -> Result<GraphIR, CompileError> {
    // First, compile named predicates from the document
    for pred_def in &doc.predicates {
        // Find the corresponding CompiledPredicate in IR
        if let Some(pred) = ir.predicates.iter_mut().find(|p| p.name == pred_def.id) {
            let mut compiler = bytecode::PredicateCompiler::new(&mut ir.strings);
            pred.bytecode = compiler.compile(&pred_def.expr)?;
        }
    }
    
    // Then, compile inline predicates from edges
    for workflow in &doc.workflows {
        for edge in &workflow.edges {
            if let Some(ref expr) = edge.predicate {
                // Find the inline predicate by looking for _inline_ or _fail_ names
                compile_inline_predicate(&mut ir, expr)?;
            }
        }
    }
    
    Ok(ir)
}

/// Compile an inline predicate expression
fn compile_inline_predicate(ir: &mut GraphIR, expr: &PredicateExpr) -> Result<(), CompileError> {
    // Skip "always" predicates - they use ID 0 and don't need bytecode
    if matches!(expr, PredicateExpr::Always) {
        return Ok(());
    }
    
    // Find uncompiled inline predicates
    for pred in &mut ir.predicates {
        if pred.bytecode.is_empty() && (pred.name.starts_with("_inline_") || pred.name.starts_with("_fail_")) {
            // Clone strings for borrow checker
            let mut strings = ir.strings.clone();
            let mut compiler = bytecode::PredicateCompiler::new(&mut strings);
            pred.bytecode = compiler.compile(expr)?;
            ir.strings = strings;
            return Ok(());
        }
    }
    
    Ok(())
}

/// Run syntactic validation checks
pub fn check_syntactic(ir: &GraphIR) -> Vec<Diagnostic> {
    syntactic::check(ir)
}

/// Run semantic validation checks
pub fn check_semantic(ir: &GraphIR) -> Vec<Diagnostic> {
    semantic::check(ir)
}

/// Run pragmatic validation checks
pub fn check_pragmatic(ir: &GraphIR) -> Vec<Diagnostic> {
    pragmatic::check(ir)
}

/// Run all validation checks
pub fn validate(ir: &GraphIR) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    diags.extend(check_syntactic(ir));
    diags.extend(check_semantic(ir));
    diags.extend(check_pragmatic(ir));
    diags
}

/// Compile predicates and validate in one step
pub fn compile_and_validate(
    mut ir: GraphIR,
    doc: &OmarDocument,
) -> Result<(GraphIR, Vec<Diagnostic>), CompileError> {
    ir = compile_predicates(ir, doc)?;
    let diags = validate(&ir);
    Ok((ir, diags))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::ast::*;
    use crate::NodeKind;
    
    fn make_test_doc() -> OmarDocument {
        OmarDocument {
            version: "1.0".into(),
            predicates: vec![
                PredicateDef {
                    id: "is_admin".into(),
                    expr: PredicateExpr::Eq {
                        left: "$token.role".into(),
                        right: Value::Str("admin".into()),
                    },
                },
            ],
            merge_policies: vec![],
            workflows: vec![Workflow {
                description: None,
                id: "test".into(),
                entry: EntryPoint {
                    p: "test".into(),
                    x: "run".into(),
                    node: "start".into(),
                },
                nodes: vec![
                    Node {
                        id: "start".into(),
                        kind: "transform".into(),
                        ..Default::default()
                    },
                    Node {
                        id: "end".into(),
                        kind: "terminal".into(),
                        status: Some(200),
                        ..Default::default()
                    },
                ],
                edges: vec![Edge {
                    from: "start".into(),
                    to: "end".into(),
                    predicate: Some(PredicateExpr::Always),
                    ..Default::default()
                }],
            }],
            templates: vec![],
            schemas: vec![],
        }
    }
    
    #[test]
    fn test_compile_predicates() {
        let doc = make_test_doc();
        let ir = lower(&doc).unwrap();
        
        let ir = compile_predicates(ir, &doc).unwrap();
        
        // Named predicate should be compiled
        let is_admin = ir.predicates.iter().find(|p| p.name == "is_admin");
        assert!(is_admin.is_some());
        assert!(!is_admin.unwrap().bytecode.is_empty());
    }
    
    #[test]
    fn test_validate_all() {
        let doc = make_test_doc();
        let ir = lower(&doc).unwrap();
        
        let diags = validate(&ir);
        
        // Should pass all checks
        let errors: Vec<_> = diags.iter()
            .filter(|d| d.severity == crate::Severity::Error)
            .collect();
        assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    }
    
    #[test]
    fn test_compile_and_validate() {
        let doc = make_test_doc();
        let ir = lower(&doc).unwrap();
        
        let (ir, diags) = compile_and_validate(ir, &doc).unwrap();
        
        // IR should have compiled predicates
        assert!(ir.predicates.iter().any(|p| !p.bytecode.is_empty()));
        
        // No errors
        let errors: Vec<_> = diags.iter()
            .filter(|d| d.severity == crate::Severity::Error)
            .collect();
        assert!(errors.is_empty());
    }
}