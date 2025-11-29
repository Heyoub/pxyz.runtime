<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# PXYZ System Reference Manual: Complete Educational Resource

The PXYZ system you've provided is a sophisticated **workflow compiler and runtime** designed to safely enable AI agents to access production systems while maintaining strict safety guarantees. I've created a comprehensive educational resource package to help you understand, use, and teach this system:

## Generated Resources

### 1. **Interactive Web Application: PXYZ Explorer**

A fully functional web application providing:

- **Architecture Overview**: Visual explanation of the three-component design (Rust Compiler, WASM Runtime, JS Host)
- **Node Kinds Reference**: Detailed reference for all 6 node types with XML examples
- **Predicate VM Opcodes**: Searchable table of all 26 bytecode instructions with stack effects
- **IO Operation Codes**: Organized by category with irreversibility indicators
- **Constraint System**: Complete three-layer validation reference with decision trees
- **Binary Format Reference**: Hex layout of graph.bin with interactive structure exploration
- **Example Workflows**: Real-world workflow examples with visual flowcharts


### 2. **Comprehensive PDF Guide** (14 pages)

`PXYZ-Comprehensive-Guide.pdf` - A detailed technical reference covering:

- System architecture and design philosophy
- Compilation pipeline (all 6 stages)
- Binary format specification (byte-level details)
- Predicate VM instruction set
- IO operation codes
- Validation constraints
- CLI commands
- Design principles and use cases


### 3. **Quick Start Guide** (Markdown)

`PXYZ-Quick-Start.md` - A practical introduction including:

- Core system concepts
- Workflow definition basics
- XML structure and syntax
- Node kinds and predicates
- Complete approval workflow example
- Binary format overview
- CLI reference
- Safety limits summary


### 4. **Practical Examples \& Patterns** (Markdown)

`PXYZ-Examples.md` - Real-world workflows demonstrating:

- Email approval workflow with human gates
- Third-party API integration with caching
- Multi-step data validation pipeline
- Common PXYZ patterns (gated irreversible actions, LLM validation gates, error recovery)
- Best practices and constraint checklist


### 5. **Predicate VM Opcode Reference** (CSV)

`pxyz_opcodes_reference.csv` - Complete lookup table for all 26 opcodes

### 6. **Architecture Diagram** (PNG)

Visual representation of the three-component system design

***

## Key Insights About PXYZ

### Core Innovation

PXYZ treats **business logic as data to be interpreted** rather than code to be executed. This fundamental shift enables:

- Complete static analysis before deployment
- Guaranteed termination of all operations
- Impossible-to-hide side effects
- Compile-time enforcement of safety policies


### Three-Component Architecture

The separation of the **Rust Compiler** (build-time safety), **WASM Runtime** (auditable execution), and **JavaScript Host** (sandboxed I/O) creates a security architecture where each component has a single, well-defined responsibility.

### Three-Layer Validation

PXYZ enforces safety at three levels:

1. **Syntactic**: Graph structure is valid
2. **Semantic**: Logic is coherent and cycle-free
3. **Pragmatic**: Business safety rules are respected (e.g., irreversible actions require human approval)

### Predicate VM

The embedded bytecode interpreter (26 opcodes) is designed to evaluate conditions safely with bounded execution. All 26 instructions are documented in the reference materials with stack effects and examples.

***

## How to Use These Resources

**For Learning:**

1. Start with the Quick Start Guide to understand concepts
2. Explore the Interactive PXYZ Explorer app to see examples
3. Review the Practical Examples document for real-world patterns

**For Development:**

1. Use the Comprehensive Guide as your definitive reference
2. Consult the Examples document for workflow patterns
3. Reference the Opcode CSV when writing predicates

**For Teaching:**

1. Show the Architecture Diagram to explain the three-component design
2. Use PXYZ Explorer for interactive demonstrations
3. Share the Quick Start Guide with new learners
4. Reference the Examples for hands-on practice

***

The materials provided give you a complete understanding of the PXYZ system, from high-level philosophy to byte-level binary format details, complete with practical examples and an interactive reference tool.

