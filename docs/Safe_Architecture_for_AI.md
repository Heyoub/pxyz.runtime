PXYZ: A Provably Safe Architecture for Auditable AI Agent Workflows

1. Introduction: The Imperative for Safe AI Agency

As artificial intelligence moves from analytical tasks to active participation in business processes, a new and critical challenge emerges. We increasingly require AI agents to perform high-stakes, "dangerous" operations on our behalf—sending emails, modifying customer data, deleting records, and calling external webhooks. This leap in capability introduces a commensurate leap in risk. Traditional software development paradigms, built on complex codebases and opaque runtime environments, are fundamentally ill-equipped to manage the inherent unpredictability and potential for harm in autonomous AI workflows.

The PXYZ architecture is a direct response to this challenge. It is a solution founded on the principles of absolute auditability, provable safety, and explicit, verifiable intent. This whitepaper articulates the PXYZ design philosophy and technical implementation for an audience of system architects, security professionals, and compliance auditors. We will demonstrate how PXYZ re-frames the problem of AI safety by shifting guarantees from runtime hope to compile-time proof.

To achieve this, the architecture is built on seven core tenets that will be explored in detail:

* Treating all business logic as data, not code.
* Enforcing bounded computation for all logical operations.
* Unifying all constraints through a single evaluation engine (the Y-Layer).
* Making every input/output (I/O) operation explicit and traceable.
* Applying a rigorous, multi-layered constraint system at compile time.
* Requiring human gates for irreversible actions.
* Maintaining an auditable, minimal runtime.

Together, these tenets form a coherent architectural philosophy for building AI systems that are not just powerful, but also fundamentally trustworthy.

2. The PXYZ Philosophy: A Foundation of Verifiable Safety

Before detailing the system's components, it is crucial to understand the foundational philosophy that underpins its safety and auditability guarantees. PXYZ is not merely a collection of technologies; it is a principles-first approach to system design. Each technical decision is a direct implementation of one of the following seven core principles, which collectively ensure that the system's behavior is always finite, verifiable, and constrained.

1. Business logic is data, not code.
This is the central insight of the PXYZ architecture. Instead of writing executable code that does things, a developer defines a workflow that is compiled into a finite, traversable Directed Acyclic Graph (DAG). This binary artifact, graph.bin, is effectively a "map" of every possible action the system can take. Unlike traditional code, which is opaque and can hide complex, unbounded behavior, this data-centric representation allows for complete static analysis. The system's entire potential behavior can be audited and verified before a single operation is ever executed, transforming the program from a black box into an open book.
2. Predicates are bounded.
All conditional logic in a PXYZ workflow—the decisions that direct traversal through the graph—is executed within a purpose-built, strictly sandboxed stack machine. This virtual machine is intentionally and severely limited. It has a maximum execution limit of 256 steps and a maximum stack depth of 16 values. These hard constraints make it provably impossible to introduce common vulnerabilities like infinite loops (while(true)) or stack overflows, which are frequent sources of security exploits and denial-of-service attacks in conventional systems. By strictly bounding computation, PXYZ eliminates entire classes of bugs by design.

3. Constraints are unified (The Y-Layer).
A common architectural mistake is to implement separate systems for edge predicates, authorization checks, input validation, conflict resolution, and query filtering. PXYZ takes a radically different approach: **one constraint evaluation engine applied at five different points**. The predicate VM (~25 opcodes, ~250 lines of WebAssembly) is called from edge traversal, auth nodes, transform validation, CRDT merge policies, and projection views. This unification provides several critical benefits: a single security surface to audit, one language to learn, predicates that can be reused across all contexts, and safety bounds that apply universally. The only difference between application points is the context variables available and what happens with the result.
4. All IO is explicit.
In PXYZ, there are no hidden side effects. Every interaction with the outside world—from a database write to a network call—must be invoked via an explicit, predefined operation code (op code). This creates an unambiguous, verifiable audit trail for every external action. For example, an auditor can simply grep the compiled graph for the hexadecimal code 0x0340 to find every single location where an email can be sent. This principle eliminates the possibility of hidden network calls or unexpected file system access, ensuring that the system's interactions with its environment are fully transparent and cataloged.
5. Safety at compile time.
The PXYZ compiler acts as a vigilant gatekeeper, enforcing safety as a prerequisite for deployment. It applies a series of pragmatic constraints to identify and block dangerous workflow patterns before they can ever reach production. For instance, the PRAG001 constraint makes it a compile-time error for the output of a probabilistic system like a Large Language Model (LLM) to directly trigger an irreversible action like sending an email without an explicit approval step. This shifts safety from a reactive, runtime concern to a proactive, compile-time guarantee.
6. Human gates for irreversible actions.
Building on the principle of compile-time safety, the PXYZ compiler enforces critical safety policies as architectural law. You literally cannot compile a graph that violates PRAG003, a rule requiring that any path leading to an action like deleting data must pass through a node that requires explicit human interaction. This critical safety check is not a guideline or a best practice; it is a physical law of the architecture, hard-coded into the compiler itself.
7. Auditable runtime.
The ultimate guarantee of security lies in the production runtime environment that executes the workflow. The PXYZ runtime is the "crown jewel" of the architecture: a minimal, ~500-line WebAssembly Text format (WAT) module. Its small size and human-readable text format are critical features, not accidents. This minimalism allows a senior security auditor to read and understand the entire runtime in a single afternoon, and a formal methods expert to mathematically verify its properties within a week. This deliberate design choice necessitates a separate, more developer-friendly environment for testing and debugging, a strategy detailed later in the architecture.

These principles are not independent ideas but a tightly integrated system, realized through a distinct, multi-component architecture designed for maximum clarity and verifiability.

3. System Architecture: Separation of Concerns for Maximum Auditability

The PXYZ system architecture rigorously enforces a separation of concerns to ensure that each component has a single, well-defined, and auditable purpose. A useful metaphor is that of a board game. The workflow definition (XML) is the rulebook. The Rust Compiler is a factory that reads the rulebook and prints the physical game board (graph.bin). The WebAssembly (WAT) Runtime is the physics engine that enforces how pieces can move on the board. Finally, the JavaScript Host is the player's hands, which actually move the pieces and interact with the world outside the game. The physics engine doesn't know what "Monopoly" is; it just knows "if a piece is here and the dice says 3, move it there." The meaning comes from the board layout (the graph) and the hands (the I/O).

A core element of this design is the "Two-Runtime" strategy, which provides distinct environments for development and for production security audits.

Attribute Rust Runtime (Development) WASM (WAT) Runtime (Production)
Language Rust WebAssembly Text (WAT)
Purpose Development, testing, and debugging with rich abstractions. Minimal, auditable, and formally verifiable production execution.
Approximate Lines of Code ~1500 ~500

The rationale for this dual-runtime approach is a pragmatic acknowledgment of the different needs of developers and security auditors:

Security auditors can read 500 lines of WAT. They cannot read 1500 lines of Rust with its abstractions. The Rust runtime is for you. The WASM runtime is for them.

Crucially, both runtimes execute the exact same graph.bin artifact. A workflow is compiled once and can be run on either environment, ensuring perfect consistency between the feature-rich development environment and the minimalist, high-assurance production environment. This guarantees that what you test is exactly what you deploy, while still providing auditors with a component simple enough to be fully understood.

This architecture's strength comes from the compilation process, which transforms human-readable intent into a verifiable binary artifact.

4. The Compilation Pipeline: From Intent to a Verifiable Artifact

The compilation pipeline is the "assembly line" that transforms human-defined business logic into the compact, secure, and portable graph.bin artifact. Its primary function is to impose structure, apply validation, and produce a binary that is safe by construction.

Workflows are initially defined in a Domain-Specific Language (DSL) based on XML. The choice of a declarative DSL over a Turing-complete programming language is a deliberate architectural decision that supports the "Business logic is data" principle, ensuring that all potential behavior is analyzable and finite by construction. This format provides a structured way for developers to declare nodes, the edges that connect them, and the conditions (predicates) that govern transitions, without writing any general-purpose, unauditable code.

The final output of this pipeline is the graph.bin file, a self-contained binary representation of the workflow. Its structure is publicly specified to enable third-party auditing and tooling.

--------------------------------------------------------------------------------

Table 1: Header (96 bytes)

Offset Size Field
0x00 4 Magic: 0x504E5958 ("PXYZ")
0x04 2 Version major
0x06 2 Version minor
0x08 4 Node count
0x0C 4 Edge count
0x10 4 Predicate count
0x14 4 String pool size
0x18 4 Entry count
0x20 32 Source hash (SHA-256)
0x40 4 Nodes offset
0x44 4 Edges offset
0x48 4 Predicates offset
0x4C 4 Strings offset
0x50 4 Entries offset

--------------------------------------------------------------------------------

Table 2: Node Entry (16 bytes)

Offset Size Field
0x00 4 Node ID
0x04 1 Kind (0-6)
0x05 1 Flags
0x06 2 Op code
0x08 4 Data offset (string pool)
0x0C 2 Edge start index
0x0E 2 Edge count

--------------------------------------------------------------------------------

Table 3: Edge Entry (12 bytes)

Offset Size Field
0x00 4 Target node ID
0x04 2 Predicate ID (0 = always)
0x06 2 (reserved)
0x08 2 Weight
0x0A 2 Flags

--------------------------------------------------------------------------------

This binary structure relies on a small, well-defined set of fundamental building blocks, or Node Kinds, which provide architects and auditors with a clear vocabulary for understanding any workflow's capabilities.

Value Kind Purpose
0 Transform Perform synchronous, pure-function data manipulation or validation.
1 External Invoke a host-provided side effect via an op code.
2 Render Generate HTML or other user-facing output.
3 Signal Emit signals to the user interface.
4 Auth Guard a workflow path with a predicate; traversal fails if false.
5 Terminal Successfully end the workflow traversal.
6 Error Represent a designated error-handling state.

Once this verifiable artifact is created, it is handed off to an execution environment specifically engineered for safety and transparency.

5. The Execution Model: A Bounded and Sandboxed Runtime

The PXYZ execution model is the heart of its security guarantees. Unlike general-purpose runtimes that offer broad capabilities, the PXYZ runtime is a minimalist, sandboxed environment designed for a single task: to safely "walk the map" defined by the graph.bin artifact.

The Auditable WASM Runtime

The "crown jewel" of the architecture is the pxyz.wat runtime. Written in WebAssembly Text format, its source code is approximately 500 lines long and is designed to be read and understood by human auditors. The use of WebAssembly (WASM) provides a critical security feature: a formally specified, memory-isolated sandbox. The runtime cannot access memory or system resources outside of its designated sandbox, eliminating a vast array of potential vulnerabilities by default.

Host Imports

A key design decision for ensuring auditability is that the WASM runtime makes zero direct system calls. It cannot open a file, make a network request, or interact with the host operating system in any way. All interaction with the outside world is delegated through a minimal set of imported functions provided by a JavaScript host environment. Functions like io_call act as a single, auditable choke point for all side effects, ensuring that no operation can occur without passing through this explicit and controllable boundary.

The Bounded Predicate VM

To decide which path to take at each step of the workflow, the runtime uses a purpose-built, strictly sandboxed stack machine to evaluate edge predicates. This VM's sole purpose is to execute tiny bytecode programs that answer a simple yes/no question, directing the flow of the graph traversal. Its design is governed by a non-negotiable set of safety limits that provably prevent unbounded computation.

Limit Value Purpose
MAX_PREDICATE_STEPS 256 Prevents infinite loops in predicate logic.
MAX_STACK_DEPTH 16 Prevents stack overflow vulnerabilities.
MAX_CALL_DEPTH 4 Prevents infinite recursion in nested predicate calls.
MAX_PREDICATE_BYTECODE 256 bytes Limits the compiled size and complexity of any single predicate.
MAX_VISITED_NODES 1000 Ensures that the overall graph traversal must terminate.

These limits are not configurable; they are fundamental properties of the runtime that guarantee all workflow logic will terminate in a finite and predictable amount of time.

Explicit I/O and the Irreversibility Spectrum

PXYZ does not treat all external I/O operations as equal. Instead, it categorizes them along a spectrum of risk, enabling the compiler to apply proportionally strict safety rules. All external actions, or side effects, are invoked by External nodes using specific IO Operation Codes. These codes form an explicit catalog of every possible interaction the system can have with the outside world, allowing an auditor to comprehensively trace every potential side effect. Critically, these operations are classified by their potential for harm, with some explicitly marked as irreversible.

A representative list of op codes includes:

* 0x0102 ENTITY_UPDATE
* 0x0400 HTTP_GET
* 0x0800 LLM_COMPLETE
* 0x0340 EMAIL_SEND ⚠️ IRREVERSIBLE
* 0x0360 WEBHOOK_CALL ⚠️ IRREVERSIBLE

This explicit classification is the mechanism that enables the compile-time pragmatic validation layer to enforce stringent safety policies on the most dangerous operations.

CRDT Merge Policies: Conflict Resolution via Unified Constraints

A critical capability for distributed and offline-first systems is conflict resolution when the same data is modified concurrently. PXYZ addresses this through **merge policies** that leverage the same unified predicate VM used for all other constraint evaluation.

When conflicts occur, the system invokes the predicate VM with a merge context containing three values: `$a` (first version), `$b` (second version), and `$candidate` (the proposed merge result). The predicate determines how to resolve the conflict.

Built-in merge policies include:

| Policy | Semantics |
|--------|-----------|
| `lww` (Last Writer Wins) | Compare timestamps; higher wins |
| `fww` (First Writer Wins) | Compare timestamps; lower wins (immutability) |
| `vclock` | Vector clock dominance for true causal ordering |
| `max` / `min` | Numeric comparison for counters/versions |
| `union` / `intersect` | Set operations for collections |
| `human-review` | Flag for manual resolution |

Merge-specific opcodes extend the predicate VM:

| Opcode | Code | Purpose |
|--------|------|---------|
| TIMESTAMP | 0x50 | Get timestamp of a value |
| IS_FLAGGED | 0x51 | Check if flagged for human review |
| ORIGIN | 0x52 | Get author/origin of a value |
| VCLOCK_GT | 0x53 | Compare vector clocks |
| MERGE_FIELD | 0x54 | Access merge context fields |

This design ensures that conflict resolution benefits from the same safety bounds (256 steps, 16 stack depth) as all other predicate evaluations, and custom merge logic can reuse existing predicates.

6. Compile-Time Safety: The Three-Layer Constraint System

PXYZ's approach to security is proactive, not reactive. Safety is not left to chance at runtime but is rigorously enforced during compilation through a three-layer constraint system. This system validates the structure, logic, and policy compliance of every workflow before it can be deployed, catching entire classes of errors that would be catastrophic in a live environment.

Syntactic (SYN) Validation

This first layer checks for basic structural and grammatical integrity, ensuring the workflow definition is well-formed. It is analogous to a syntax checker in a traditional programming language.

* SYN001: Edge targets exist: Ensures that every edge points to a node that has been defined.
* SYN004: No duplicate node IDs: Verifies that every node within a workflow has a unique identifier.

Semantic (SEM) Validation

The second layer checks for logical coherence, ensuring that the workflow makes sense and follows the rules of the PXYZ model.

* SEM002: External nodes have op codes: Guarantees that any node intended to perform an I/O operation specifies which operation it will perform.
* SEM004: No cycles in graph: Enforces that all workflows are Directed Acyclic Graphs, which is essential for guaranteeing that execution will terminate.

Pragmatic (PRAG) Validation

This is the highest and most critical layer of validation. It enforces business rules and non-negotiable safety policies, particularly those related to the risks of AI agency.

Analysis of PRAG001: Preventing Unsupervised LLM Actions

The PRAG001: LLM → Irreversible requires validation gate constraint is a cornerstone of PXYZ's AI safety model. It makes it a compile-time error for a node that invokes a probabilistic system, like an LLM, to be directly connected to a node that performs a deterministic, irreversible action. To be valid, the path must first pass through a validation node, such as one that requires human approval or applies a strict data validation schema. This architecturally enforces a critical guardrail against LLM "hallucinations" or unexpected outputs causing real-world harm.

Analysis of PRAG003: Enforcing Human-in-the-Loop

The PRAG003: Irreversible actions require human in path constraint embeds a fundamental safety policy directly into the compiler. You literally cannot compile a graph that violates this rule. It is a compile-time error to create a workflow where a dangerous, irreversible action can be triggered without the workflow path first passing through a node explicitly designated as requiring a human actor. This elevates the concept of "human-in-the-loop" from a recommended practice to a mandatory, compiler-enforced architectural rule for all high-stakes operations.

This layered, pre-deployment validation system is what elevates PXYZ from merely a secure runtime to a provably safe architecture.

7. Conclusion: Deliberate Design for a Trustworthy Future

Enabling AI agents to safely and reliably perform high-stakes operations requires a fundamental shift away from conventional software architectures. The PXYZ architecture demonstrates that such a shift is possible by embracing a new paradigm centered on auditability, simplicity, and provability. It replaces the uncertainty of complex, general-purpose code with the clarity of a finite, data-driven "map" of all possible actions.

This promise is delivered through a set of mutually reinforcing architectural pillars:

* The logic-as-data paradigm, which makes all potential system behavior fully analyzable before execution.
* A minimal and auditable WebAssembly runtime of ~500 lines, providing a sandboxed environment simple enough for complete human review and formal verification.
* The strictly bounded predicate VM, which eliminates entire classes of runtime vulnerabilities by design.
* A multi-layered system of compile-time constraints, which shifts safety from a reactive runtime concern to a proactive, pre-deployment guarantee.

Ultimately, PXYZ is an argument for deliberate design in an era of rapid, often reckless, technological advancement. It asserts that when AI agents are empowered to act in the real world, our primary responsibility is to build systems that are not just capable, but verifiably safe.

"This architecture is not about being clever. It's about being auditable. When the AI can send emails, delete data, and call webhooks on behalf of users, "move fast and break things" is not an option. PXYZ moves deliberately and proves things."
