Understanding the PXYZ Architecture: A Beginner's Guide

Introduction: The Assembly Line for Business Logic

Welcome! This guide is designed to introduce you to the PXYZ system. At its heart, PXYZ answers a critical question: how do you let powerful AI agents do dangerous things safely?

To understand how it works, we'll use a central metaphor: think of PXYZ as a sophisticated assembly line. This assembly line takes a simple, human-readable blueprint for a task and transforms it, station by station, into a secure and verifiable set of instructions that a machine can follow without error.

This guide will walk you through each station on this assembly line, explaining what each component does and why it's a critical part of the process.

--------------------------------------------------------------------------------

1. The Blueprint: The XML File

Every process in the PXYZ system begins with a simple XML file. This file is the starting point—the human-readable plan for the business logic you want to execute. It's designed to be clear and easy for a person to read and audit.

The XML file is like the rulebook for a board game. It doesn't play the game itself, but it clearly defines every possible move, every condition, and every outcome before the first piece is ever moved.

Within this blueprint, there are three fundamental building blocks that you use to define any workflow:

* <node>: A single step or action in the workflow, like an individual space on the game board.
* <edge>: A path connecting two nodes, representing the flow from one step to the next.
* <predicate>: A simple yes/no question that decides if an edge can be taken, like a locked door that only opens if you have the right key.

This blueprint provides the clear, human-auditable starting point for our entire secure process.

--------------------------------------------------------------------------------

2. The Factory: The Rust Compiler

Once the blueprint is written, it enters our assembly line's main factory: the Rust Compiler. The compiler's job is to take the human-readable XML file and forge it into a secure, efficient, and machine-ready format.

Crucially, its most important job is not just to translate the blueprint, but to enforce a comprehensive set of safety rules before the process can even be deployed. It acts as the system's chief safety inspector.

The compiler has three primary responsibilities:

1. Parse the Blueprint The first step is simply to read the XML file and understand its structure—the nodes, edges, and predicates. This is like a chef reading a recipe from start to finish before they begin cooking to make sure all the ingredients and steps make sense.
2. Validate for Safety (The Critical Step) This is the system's "quality control" station. The compiler runs hundreds of checks against the blueprint to find and reject dangerous or illogical patterns. These checks are organized into three distinct levels of increasing sophistication:

Check Level Purpose Example
Syntactic Does the blueprint make structural sense? SYN001: Checks that every edge points to a real, existing node.
Semantic Does the blueprint make logical sense? SEM004: Checks that there are no infinite loops in the workflow.
Pragmatic Does the blueprint follow our safety rules? PRAG003: Checks that irreversible actions (like sending an email) require a human in the loop.

1. Emit the Final Product Only after the blueprint has passed every single validation check does the compiler perform its final task. It emits the graph.bin file—a compact, secure, and optimized binary representation of the workflow. This is the final product of our factory.

With the blueprint validated and transformed, we now have a trustworthy artifact ready for execution.

--------------------------------------------------------------------------------

3. The Y-Constraint Layer: One Engine, Many Uses

Before we look at the final output, there's a powerful concept to understand: the **Y-constraint layer**. This is one of PXYZ's most elegant features.

In the blueprint, you define **predicates**—simple yes/no questions like "Is this user the owner?" or "Is this email valid?" These predicates are compiled into tiny programs that run on a bounded virtual machine (the Predicate VM).

Here's the key insight: **the same predicate engine is used everywhere**.

| Application Point | Question Being Asked | Same Engine? |
|-------------------|---------------------|--------------|
| Edge traversal | "Can we take this path?" | ✅ |
| Auth nodes | "Is the user authorized?" | ✅ |
| Input validation | "Is this data valid?" | ✅ |
| CRDT merge policies | "Which version wins in a conflict?" | ✅ |
| Projection filters | "Should this record appear in this view?" | ✅ |

This is like having one universal lock mechanism that works on doors, safes, and gates. You learn it once, and it works everywhere. The only difference is *what* you're checking and *when*.

For example, a predicate like "is the current user the owner of this entity?" can be:
- An edge condition (only proceed if owner)
- An auth check (block non-owners)
- A merge policy (prefer owner's version in conflicts)
- A view filter (only show entities you own)

One definition, five uses, zero duplication.

--------------------------------------------------------------------------------

4. Merge Policies: Resolving Conflicts with Predicates

When data can be edited from multiple places (like a contact being updated on two devices simultaneously), conflicts can occur. PXYZ handles this elegantly using the same predicate engine.

**Merge policies** tell the system how to resolve conflicts:

| Policy | What It Does |
|--------|-------------|
| `lww` (Last Writer Wins) | The most recent change wins |
| `fww` (First Writer Wins) | The original value is preserved |
| `max` / `min` | Higher or lower value wins |
| `union` | Combine sets/arrays together |
| `human-review` | Flag for human decision |

In the XML blueprint:

```xml
<merge>
  <entity name="Contact" default="lww">
    <field name="email" policy="fww"/>      <!-- Email is immutable -->
    <field name="tags" policy="union"/>      <!-- Combine all tags -->
    <field name="notes" policy="human-review"/> <!-- Human decides -->
  </entity>
</merge>
```

The beauty: these policies compile to the same predicate bytecode. The merge engine calls the Predicate VM with the conflicting values, and the predicate decides the winner.

--------------------------------------------------------------------------------

5. The Game Board: The graph.bin File

The graph.bin file is the output of the Rust Compiler and the sole input for the execution runtimes. It is a self-contained, unchangeable artifact that represents the approved business logic.

Continuing our metaphor, if the XML was the rulebook, the graph.bin file is the final, printed game board. It’s a physical object that cannot be altered once it leaves the factory. The "players" (the runtimes) can only move on the spaces and paths printed on this board.

This approach provides two significant benefits:

* Portable: Because graph.bin is a simple, well-defined binary format, the exact same file can be executed anywhere. You can run it on a server using the native Rust runtime or safely within a user's web browser using the WASM runtime.
* Secure & Auditable: The file's structure contains all the logic, including a cryptographic hash (specifically, a SHA-256 hash) of the original source XML. This means the file can be stored, hashed, and inspected at any time to prove exactly which version of the business logic was approved and executed.

This single, verifiable file is the key to PXYZ's "compile once, run anywhere securely" philosophy.

--------------------------------------------------------------------------------

6. Playing the Game: The Two Runtimes

The final component is the runtime—the engine that actually executes the logic contained within the graph.bin file. The runtime is the "player" that moves the pieces across the game board according to the predefined rules.

A core design feature of PXYZ is that it has two distinct runtimes, and the reason is central to the system's philosophy. Security auditors can read 600 lines of WebAssembly Text, but they cannot read thousands of lines of complex Rust. The Rust runtime is for you, the developer. The WASM runtime is for them, the auditors.

Here is a breakdown of their distinct roles:

Runtime Purpose & Audience Key Characteristic
Rust Runtime For developers during building and testing. Full-featured and easy to debug.
WASM Runtime For production and security auditors. Minimal (between 500-700 lines of WebAssembly Text), highly auditable, and sandboxed for safety.

The most crucial insight is that both of these runtimes execute the exact same graph.bin file. This guarantees that the logic you test during development is identical to the logic that runs in production.

This two-runtime strategy provides both developer-friendly tools and production-grade security without compromise.

--------------------------------------------------------------------------------

Conclusion: From Blueprint to Secure Action

The PXYZ architecture is a deliberate, multi-stage process designed for safety and auditability. By walking through our assembly line metaphor one last time, we can see the complete flow from idea to secure execution.

1. A developer defines a workflow in a human-readable XML file (The Blueprint).
2. The Rust Compiler validates this blueprint against strict safety rules and compiles it (The Factory).
3. The Y-Constraint Layer provides one evaluation engine for all constraint needs (One Engine, Many Uses).
4. Merge policies handle data conflicts using the same predicate system (Resolving Conflicts).
5. The output is a single, secure graph.bin file (The Game Board).
6. The ultra-auditable WASM Runtime executes this file in production, guaranteeing safety (Playing the Game).

This step-by-step process is what makes the PXYZ system so robust. By strictly separating the definition of business logic from its execution—and by performing exhaustive safety validation at compile time—the system ensures that business logic is treated as auditable data, not code, providing a powerful guarantee: only safe, pre-approved logic can ever be run.
