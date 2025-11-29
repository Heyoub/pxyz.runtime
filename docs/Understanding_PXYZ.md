# Understanding the PXYZ System: A Conceptual Overview

At its core, the PXYZ system is designed to provide a definitive answer to a single, critical question facing the world of artificial intelligence:

How do you let AI agents do dangerous things safely?

Instead of writing complex code that directly performs actions, the PXYZ system asks developers to create a map of what can possibly be done. A separate, incredibly simple program, called the walker, then follows that map to execute the workflow.

For a student new to building safe systems, this separation is the key takeaway. It makes the entire system predictable, verifiable, and easy to audit. These qualities are essential when an AI agent is given the power to perform core business capabilities like sending emails, deleting user data, and calling webhooks on behalf of users.

This document will walk you through this map-and-walker architecture, explaining how each piece contributes to a system that is safe by design.

## 1. The Map: Defining What's Possible

The first major shift in thinking PXYZ introduces is that a program's logic isn't code; it's data. This data structure, which defines every possible action and decision, is called "the map."

The map has several key characteristics:

* A Directed Acyclic Graph (DAG): In simple terms, this means the map is a flowchart with a clear, one-way direction and no loops. This structure guarantees that any process will always move forward and eventually reach a defined end point. It can't get stuck in an infinite loop.
* Defined in XML: Developers create the map using a structured, human-readable XML file. This file acts as a blueprint for the entire program, clearly outlining all nodes (steps) and edges (connections).
* Compiled to Binary: This XML blueprint is then fed into a compiler. If the XML is a "rulebook," the compiler is a "factory" that produces a compact, efficient "game board" from those rules. This final binary file is called graph.bin.

This graph.bin file is a universal artifact. It can be executed by a fast, feature-rich Rust runtime for local development and testing, or by the minimal, formally auditable WebAssembly runtime in production, without any changes.

The significance of this approach is profound: because all business logic is contained within a finite, traversable map, the system's complete set of possible behaviors is defined and inspectable before it ever runs. It simply cannot perform an action that isn't explicitly drawn on the map.

But a map is just a blueprint. To bring it to life, the system uses a second, equally important component: the walker.

## 2. The Walker: A Simple and Auditable Executor

The static map is brought to life by the "walker," the runtime engine that executes the logic defined in graph.bin.

The walker's most important feature is its radical simplicity. The entire production runtime is a tiny WebAssembly Text (.wat) file of approximately 500-600 lines, designed to be small enough for a security expert to read and formally verify in a single sitting. This is a revolutionary concept for security, as it dramatically reduces the surface area that needs to be audited.

Traditional System PXYZ System
A complex, 20,000+ line application where business logic and execution are mixed. A tiny, ~600-line "walker" that only knows how to follow instructions.
Auditing requires analyzing the entire codebase, making it nearly impossible to guarantee that all behaviors have been found. An auditor can read and verify the entire runtime in an afternoon.

Following the "board game" analogy, the walker is like the game's physics engine. It doesn't know the rules of Monopoly (the business logic). It only knows how to read the board (the map) and move a game piece from one square to the next. All the complexity is in the board's layout, not in the engine that enforces the moves.

This strict separation of a complex, auditable map from a simple, verifiable walker isn't just an elegant design; it is the foundation for a series of non-negotiable safety guarantees we will explore next.

## 3. The Y-Constraint Layer: One Engine, Five Uses

One of PXYZ's most powerful architectural decisions is the **unified constraint engine**. Rather than building separate systems for different types of checks, PXYZ uses the same small predicate VM (~25 opcodes) for everything.

| Where It's Used | What Question It Answers |
|-----------------|-------------------------|
| **Edge Traversal** | "Can we take this path?" |
| **Auth Nodes** | "Is this user authorized?" |
| **Input Validation** | "Is this data valid?" |
| **CRDT Merge** | "Which version wins in a conflict?" |
| **Projection Views** | "Should this record appear in this query?" |

**Why This Matters:**

* **One language to learn.** Define a predicate once, use it everywhere.
* **One security surface.** Auditors analyze one 250-line VM, not five different systems.
* **Reusable logic.** A predicate like "is_owner" works for auth, merge policies, and view filters.
* **Uniform safety bounds.** All constraint evaluation is bounded to 256 steps.

For example, consider a predicate `is_owner`:
```xml
<predicate id="is_owner">
  <eq left="$entity.owner_id" right="$token.sub"/>
</predicate>
```

This single definition can:
- Guard an edge (only owners can proceed)
- Protect an auth node (only owners can access)
- Define a merge policy (prefer owner's version in conflicts)
- Filter a projection (only show owned records)

The walker doesn't care *why* it's evaluating the predicate—it just runs the same bytecode interpreter each time.

## 4. Merge Policies: Conflict Resolution

When data is edited simultaneously from multiple places (like offline edits syncing), PXYZ handles conflicts using the same predicate system.

**Built-in merge policies:**

| Policy | Behavior |
|--------|----------|
| `lww` | Last Writer Wins (higher timestamp) |
| `fww` | First Writer Wins (immutable fields) |
| `vclock` | Vector clock dominance |
| `union` | Combine sets/arrays |
| `human-review` | Flag for manual resolution |

**In the XML:**
```xml
<merge>
  <entity name="Contact" default="lww">
    <field name="email" policy="fww"/>
    <field name="tags" policy="union"/>
  </entity>
</merge>
```

The merge system calls the predicate VM with `$a`, `$b`, and `$candidate` context variables. Custom predicates can implement any resolution logic while remaining bounded and auditable.

## 5. The Core Safety Principles in Action

The map-and-walker design isn't just an architectural choice; it's the foundation for several powerful safety guarantees that are built into the very fabric of the system.

Bounded Execution (Fuses, Not Trust) PXYZ doesn't trust logic to behave correctly; it forces it to operate within strict, non-negotiable limits. This is most evident in the Predicate VM, a tiny virtual machine responsible for making all the "yes/no" decisions that guide the walker along the map's edges. It has hard-coded safety fuses:

* Max 256 Steps: Prevents infinite loops. A decision-making process is physically incapable of running forever.
* Max 16 Stack Depth: Prevents crashes that can result from overly complex logic.
* Max 4 Call Depth: Prevents infinite recursion between different decision-making rules.
* Max 256 Bytes Bytecode: Limits the complexity of any single predicate, ensuring it remains simple and analyzable.
* Max 1000 Visited Nodes: Acts as a final backstop against runaway graph traversals, even in a valid DAG.
* Explicit I/O (No Hidden Side Effects) Any action that affects the outside world—sending an email, calling an API, updating a database—is called an Input/Output (I/O) operation. In PXYZ, every single I/O operation must be declared with an explicit "op code." For example, an auditor can run a simple text search (e.g., grep) for the code 0x0340 across the map files to find every single place the system is capable of sending an email. There are no hidden network calls or unexpected side effects; every interaction with the world is cataloged and visible.
* Safety at Compile Time (Preventing Problems Before They Happen) The PXYZ compiler acts as a vigilant safety inspector before the program is ever built. It runs a series of "pragmatic" checks to find dangerous business logic patterns. The most powerful example is check PRAG001, which catches patterns like "an LLM output trying to send an email without an approval step." A program containing this dangerous flow cannot even be compiled, let alone deployed. The error is caught at the earliest possible moment, preventing any potential damage.
* Human Gates for Irreversible Actions The system recognizes that some actions, like deleting user data or sending an external email, are irreversible and builds a mandatory human approval gate directly into the architecture. The compiler enforces a strict, non-negotiable rule (PRAG003): a map is considered invalid and will not compile if it allows an irreversible action to occur without first passing through a node that explicitly requires a human actor. This builds a mandatory safety gate directly into the architecture for the most sensitive operations.

Together, these principles are enforced by a hierarchy of constraints built into the compiler. It first performs Syntactic checks to ensure the map is drawn correctly (e.g., all connections point to real nodes). It then runs Semantic checks to ensure the map makes logical sense (e.g., an authorization step actually has a rule to check). Finally, it applies Pragmatic checks like those above to enforce business-level safety rules. This layered approach ensures safety is not an afterthought but a fundamental, provable property of the system.

## 6. The PXYZ Philosophy Summarized

The design of the PXYZ system is guided by a clear and deliberate philosophy, best captured by a quote from its architectural documentation:

This architecture is not about being clever. It's about being auditable.

This philosophy is expressed through a set of core tenets that ensure every PXYZ program is safe, predictable, and verifiable.

* Logic is Data: The program is a map you can inspect, not complex code you have to run to understand.
* Decisions are Finite: Logic is physically incapable of getting stuck in infinite loops; it operates under strict, hard-coded limits on steps and complexity.
* Constraints are Unified: One evaluation engine handles auth, validation, merge, and filtering—learn once, apply everywhere.
* Actions are Obvious: Every interaction with the outside world is explicitly labeled and cataloged.
* Safety is a Prerequisite: Dangerous patterns are blocked by the compiler before the program can even be created.
* Humans are Required: For the most critical actions, the system architecturally requires human approval.
* The Runtime is Readable: The core engine is simple enough for a human to read and verify completely.

When AI agents are empowered to act on behalf of users, a system whose safety can be proven before a single line of code runs is not just an option—it's an architectural and ethical necessity.
