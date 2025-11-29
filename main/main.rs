//! PXYZ CLI
//!
//! Compile workflow.xml → graph.bin

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pxyz")]
#[command(about = "PXYZ workflow compiler")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compile workflow.xml to graph.bin
    Compile {
        /// Input XML file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output binary file
        #[arg(short, long)]
        output: PathBuf,
        
        /// Generate audit.json alongside binary
        #[arg(long, default_value = "true")]
        audit: bool,
        
        /// Treat warnings as errors
        #[arg(long)]
        strict: bool,
    },
    
    /// Inspect graph.bin
    Inspect {
        /// Input binary file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output format: text, json, mermaid
        #[arg(long, default_value = "text")]
        format: String,
    },
    
    /// Validate without compiling
    Check {
        /// Input XML file
        #[arg(short, long)]
        input: PathBuf,
    },
    
    /// Create new project
    Init {
        /// Project name
        name: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Command::Compile { input, output, audit, strict } => {
            cmd_compile(&input, &output, audit, strict)
        }
        Command::Inspect { input, format } => {
            cmd_inspect(&input, &format)
        }
        Command::Check { input } => {
            cmd_check(&input)
        }
        Command::Init { name } => {
            cmd_init(&name)
        }
    }
}

fn cmd_compile(input: &PathBuf, output: &PathBuf, emit_audit: bool, strict: bool) -> anyhow::Result<()> {
    println!("{} {}", "Compiling".cyan(), input.display());
    
    let xml = std::fs::read_to_string(input)?;
    
    let options = pxyz::CompileOptions {
        optimize: true,
        strict,
        emit_audit,
    };
    
    match pxyz::compile(&xml, &options) {
        Ok(result) => {
            // Write binary
            std::fs::write(output, &result.binary)?;
            println!("{} {} ({} bytes)", "✓".green(), output.display(), result.binary.len());
            
            // Write audit
            if let Some(audit) = result.audit {
                let audit_path = output.with_extension("audit.json");
                let json = serde_json::to_string_pretty(&audit)?;
                std::fs::write(&audit_path, json)?;
                println!("{} {}", "✓".green(), audit_path.display());
            }
            
            // Print diagnostics
            for diag in &result.diagnostics {
                print_diagnostic(diag);
            }
            
            Ok(())
        }
        Err(pxyz::CompileError::ValidationFailed { diagnostics }) => {
            for diag in &diagnostics {
                print_diagnostic(diag);
            }
            let error_count = diagnostics.iter()
                .filter(|d| d.severity == pxyz::Severity::Error)
                .count();
            anyhow::bail!("Compilation failed with {} error(s)", error_count);
        }
        Err(e) => anyhow::bail!("{}", e),
    }
}

fn cmd_inspect(input: &PathBuf, format: &str) -> anyhow::Result<()> {
    let data = std::fs::read(input)?;
    let info = pxyz::inspect(&data).map_err(|e| anyhow::anyhow!(e))?;
    
    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                "version": info.version,
                "nodes": info.node_count,
                "edges": info.edge_count,
                "predicates": info.predicate_count,
                "strings": info.string_pool_size,
                "entries": info.entry_count,
                "size": info.binary_size,
            }))?);
        }
        "mermaid" => {
            // Would need to parse nodes/edges from binary for full mermaid
            // For now, just show summary
            println!("graph TD");
            println!("    subgraph \"{}\"", input.display());
            println!("    N[{} nodes]", info.node_count);
            println!("    E[{} edges]", info.edge_count);
            println!("    P[{} predicates]", info.predicate_count);
            println!("    end");
        }
        _ => {
            // text format
            println!("PXYZ Graph Binary");
            println!("─────────────────");
            println!("Version:     {}", info.version);
            println!("Nodes:       {}", info.node_count);
            println!("Edges:       {}", info.edge_count);
            println!("Predicates:  {}", info.predicate_count);
            println!("Strings:     {} bytes", info.string_pool_size);
            println!("Entries:     {}", info.entry_count);
            println!("Total size:  {} bytes", info.binary_size);
        }
    }
    
    Ok(())
}

fn cmd_check(input: &PathBuf) -> anyhow::Result<()> {
    println!("{} {}", "Checking".cyan(), input.display());
    
    let xml = std::fs::read_to_string(input)?;
    let diagnostics = pxyz::validate(&xml);
    
    for diag in &diagnostics {
        print_diagnostic(diag);
    }
    
    let errors = diagnostics.iter().filter(|d| d.severity == pxyz::Severity::Error).count();
    let warnings = diagnostics.iter().filter(|d| d.severity == pxyz::Severity::Warn).count();
    
    println!();
    if errors > 0 {
        println!("{} {} error(s), {} warning(s)", "✗".red(), errors, warnings);
        std::process::exit(1);
    } else if warnings > 0 {
        println!("{} {} warning(s)", "⚠".yellow(), warnings);
    } else {
        println!("{} No issues found", "✓".green());
    }
    
    Ok(())
}

fn cmd_init(name: &str) -> anyhow::Result<()> {
    std::fs::create_dir_all(name)?;
    
    let workflow = r#"<?xml version="1.0" encoding="UTF-8"?>
<omar version="1.0.0">
  <predicates>
    <predicate id="is_authenticated">
      <neq left="$token.sub" right=""/>
    </predicate>
  </predicates>
  
  <workflow id="hello">
    <entry p="hello" x="world" node="start"/>
    
    <nodes>
      <node id="start" kind="transform"/>
      <node id="done" kind="terminal" status="200"/>
    </nodes>
    
    <edges>
      <edge from="start" to="done">
        <when><always/></when>
      </edge>
    </edges>
  </workflow>
</omar>
"#;
    
    std::fs::write(format!("{}/workflow.xml", name), workflow)?;
    println!("{} Created {}/workflow.xml", "✓".green(), name);
    
    // Create minimal build script
    let build_sh = r#"#!/bin/bash
set -e

# Compile workflow
pxyz compile -i workflow.xml -o graph.bin

# Convert WAT to WASM (requires wabt or wasm-tools)
# wat2wasm pxyz.wat -o pxyz.wasm

echo "Done. Deploy graph.bin + pxyz.wasm + io.js"
"#;
    
    std::fs::write(format!("{}/build.sh", name), build_sh)?;
    println!("{} Created {}/build.sh", "✓".green(), name);
    
    Ok(())
}

fn print_diagnostic(diag: &pxyz::Diagnostic) {
    let (prefix, color) = match diag.severity {
        pxyz::Severity::Error => ("ERROR", "red"),
        pxyz::Severity::Warn => ("WARN ", "yellow"),
        pxyz::Severity::Info => ("INFO ", "blue"),
    };
    
    let prefix_colored = match color {
        "red" => prefix.red(),
        "yellow" => prefix.yellow(),
        _ => prefix.blue(),
    };
    
    println!("{} [{}] {}", prefix_colored, diag.code.dimmed(), diag.message);
    
    if let Some(hint) = &diag.hint {
        println!("      {} {}", "hint:".dimmed(), hint);
    }
    
    if let Some(loc) = &diag.location {
        let mut parts = Vec::new();
        if let Some(w) = &loc.workflow_id { parts.push(format!("workflow:{}", w)); }
        if let Some(n) = &loc.node_id { parts.push(format!("node:{}", n)); }
        if let Some(e) = &loc.edge_id { parts.push(format!("edge:{}", e)); }
        if !parts.is_empty() {
            println!("      {} {}", "at:".dimmed(), parts.join(" "));
        }
    }
}
