//! Audit JSON Generation

use crate::compiler::ir::*;
use crate::{Diagnostic, Severity};
use serde::Serialize;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize)]
pub struct GraphAudit {
    pub version: String,
    pub compiled_at: String,
    pub source_hash: String,
    pub graph_hash: String,
    pub stats: AuditStats,
    pub checks: AuditChecks,
    pub entries: Vec<AuditEntry>,
    pub warnings: Vec<AuditWarning>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub predicate_count: usize,
    pub entry_count: usize,
    pub string_pool_size: usize,
    pub binary_size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditChecks {
    pub syntactic: CheckResult,
    pub semantic: CheckResult,
    pub pragmatic: CheckResult,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckResult {
    pub passed: bool,
    pub error_count: usize,
    pub warning_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditEntry {
    pub p: String,
    pub x: String,
    pub node_id: u32,
    pub node_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuditWarning {
    pub code: String,
    pub message: String,
    pub hint: Option<String>,
}

pub fn generate(ir: &GraphIR, source_xml: &str, binary: &[u8], diagnostics: &[Diagnostic]) -> GraphAudit {
    let now = chrono::Utc::now().to_rfc3339();
    
    // Source hash
    let mut hasher = Sha256::new();
    hasher.update(source_xml.as_bytes());
    let source_hash = format!("{:x}", hasher.finalize());
    
    // Binary hash
    let mut hasher = Sha256::new();
    hasher.update(binary);
    let graph_hash = format!("{:x}", hasher.finalize());
    
    // Categorize diagnostics
    let syntactic: Vec<_> = diagnostics.iter().filter(|d| d.code.starts_with("SYN")).collect();
    let semantic: Vec<_> = diagnostics.iter().filter(|d| d.code.starts_with("SEM")).collect();
    let pragmatic: Vec<_> = diagnostics.iter().filter(|d| d.code.starts_with("PRAG")).collect();
    
    fn count_errors_warnings(diags: &[&Diagnostic]) -> (usize, usize) {
        let errors = diags.iter().filter(|d| d.severity == Severity::Error).count();
        let warnings = diags.iter().filter(|d| d.severity == Severity::Warn).count();
        (errors, warnings)
    }
    
    let (syn_err, syn_warn) = count_errors_warnings(&syntactic);
    let (sem_err, sem_warn) = count_errors_warnings(&semantic);
    let (prag_err, prag_warn) = count_errors_warnings(&pragmatic);
    
    let warnings: Vec<_> = diagnostics.iter()
        .filter(|d| d.severity == Severity::Warn)
        .map(|d| AuditWarning {
            code: d.code.clone(),
            message: d.message.clone(),
            hint: d.hint.clone(),
        })
        .collect();
    
    let entries: Vec<_> = ir.entries.iter()
        .map(|e| {
            let node_name = ir.nodes.iter()
                .find(|n| n.id == e.node_id)
                .map(|n| n.name.clone())
                .unwrap_or_default();
            AuditEntry {
                p: e.p.clone(),
                x: e.x.clone(),
                node_id: e.node_id,
                node_name,
            }
        })
        .collect();
    
    GraphAudit {
        version: "1.0.0".into(),
        compiled_at: now,
        source_hash,
        graph_hash,
        stats: AuditStats {
            node_count: ir.nodes.len(),
            edge_count: ir.edges.len(),
            predicate_count: ir.predicates.len(),
            entry_count: ir.entries.len(),
            string_pool_size: ir.strings.data.len(),
            binary_size: binary.len(),
        },
        checks: AuditChecks {
            syntactic: CheckResult { 
                passed: syn_err == 0, 
                error_count: syn_err, 
                warning_count: syn_warn 
            },
            semantic: CheckResult { 
                passed: sem_err == 0, 
                error_count: sem_err, 
                warning_count: sem_warn 
            },
            pragmatic: CheckResult { 
                passed: prag_err == 0, 
                error_count: prag_err, 
                warning_count: prag_warn 
            },
        },
        entries,
        warnings,
    }
}