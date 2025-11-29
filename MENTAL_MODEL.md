# PXYZ: The Journey to Clarity

> This document explains what we were trying to build, the confusion we went through, and what actually matters. It's a handoff for implementation.

---

## The Core Insight

We kept arriving at the same place from different directions:

**State is a lie. Events are truth. The graph is physics.**

Everything in a CRM - contacts, deals, tasks, emails - is just a projection. The only real things are:

1. **Events** (Z) - what happened, when
2. **Constraints** (Y) - rules about what's allowed
3. **The graph** - how operations flow

---

## What We're Building

A CRM runtime in ~700 lines of auditable code.

```mermaid
graph TD
workflow.xml  →  [Rust compiler]  →  graph.bin  →  [pxyz.wasm]  →  io.js
                                                        ↑
                                                   THE RUNTIME
                                                   (~600 lines WAT)
```

**That's it.** No npm. No node_modules. No React. No frameworks.

---

## The Confusion We Went Through

### Confusion 1: "Where does the runtime live?"

We kept going back and forth:

- First: Everything in TypeScript
- Then: Rust for everything
- Then: Rust + WAT + JS (three things??)
- Then: WAT is THE runtime, Rust just compiles, JS just does IO

**Resolution:** WAT is the runtime. Rust is a compiler. JS is an IO adapter. They don't overlap.

### Confusion 2: "Why do I have duplicate code?"

I (Claude) created Rust files that duplicated WAT functionality:

- `runtime/predicate_vm.rs` - but WAT has `$eval_predicate`
- `runtime/traversal.rs` - but WAT has `$execute`
- `runtime/graph.rs` - but WAT loads graph.bin directly

**Resolution:** Delete all of `runtime/`. Rust only compiles. WAT executes.

### Confusion 3: "What's 'core' for?"

We had a `core/` folder with types, opcodes, binary format... but WAT duplicates all those constants anyway. There's no shared code between Rust and WAT - they just agree on byte layouts.

**Resolution:** Flatten it. Put constants where they're used. `core/` was organizational theater.

### Confusion 4: "Where do events belong?"

Events are the Z-dimension - the foundation of the whole system ("State = View(Events, Constraints)"). But we had them buried in `runtime/events.rs` as a "debugging utility."

**Resolution:** Events happen in JS. The WAT calls `$io_emit_event`. The JS EventBus handles the rest. Rust doesn't need event types at all.

### Confusion 5: "What patterns go where?"

From old research, we extracted patterns like:

- ActorKind (Human vs Agent)
- ConfirmationStatus
- Error nodes
- Three-layer constraints (SYN/SEM/PRAG)
- Fuse/SafetyGate

We kept putting them in Rust types. But some are compile-time, some are runtime.

**Resolution:**

| Pattern | Compile-time (Rust) | Runtime (WAT) | Host (JS) |
|---------|---------------------|---------------|-----------|
| SYN/SEM/PRAG checks | ✓ | | |
| ActorKind flag | Sets bit in binary | Checks flag | Provides `is_human()` |
| ConfirmationStatus | PRAG check | IS_CONFIRMED opcode | Provides `is_confirmed()` |
| Error nodes | Validates reachability | Routes to error edges | |
| Fuse limits | Documents them | Enforces them | |
| EventBus | | Calls `emit_event` | Handles events |

---

## The Final Architecture

### Rust Compiler (~1000 lines)

One file. One job. Parse XML, validate, emit binary.

```shell
src/
├── lib.rs          # pub fn compile(xml) -> Vec<u8>
├── main.rs         # CLI
└── pipeline.rs     # Everything else
```

What it does:

1. Parse XML → AST
2. Lower AST → IR
3. Compile predicates → bytecode
4. Validate (syntactic, semantic, pragmatic)
5. Emit graph.bin

What it doesn't do:

- Execute anything
- Know what IO operations mean
- Have runtime types

### WAT Runtime (~600 lines)

The complete file: `pxyz.wat`

What it does:

1. Load graph.bin into WASM memory
2. Hash (P, X) to find entry point
3. Traverse nodes, evaluate predicates
4. Call host for all IO
5. Enforce safety limits
6. Handle errors via error edges
7. Check actor permissions

What it doesn't do:

- Know what "Google Contacts" means
- Parse anything
- Store state between calls

### JS Host (~200 lines)

Plain JavaScript. No build step.

```javascript
// io.js
export function createHost(wasm) {
  return {
    io_call(op, ptr, len) {
      switch (op) {
        case 0x0300: return googleContactsSearch(...);
        case 0x0800: return llmComplete(...);
        // etc
      }
    },
    io_is_human() {
      return this.yCtx.actor === 'human' ? 1 : 0;
    },
    io_is_confirmed(entity_id) {
      return this.yCtx.confirmed?.has(entity_id) ? 1 : 0;
    },
    io_emit_event(kind, node_id, data) {
      this.eventBus.emit(kind, { node_id, data });
    }
  };
}
```

---

## The Patterns We Extracted (and where they landed)

### From "Cognitive Agent Compilers" Research

**Three-layer constraints** → Rust compiler

- SYN001-007: Structure validation (refs exist, no dangling edges)
- SEM001-007: Logic validation (auth has predicates, external has opcodes)
- PRAG001-007: Safety validation (human-in-path, confirmed inputs)

### From "Rust Runtime Engines" Research

**Fuse/SafetyGate** → WAT globals

```wat
(global $MAX_VISITED i32 (i32.const 1000))
(global $MAX_PRED_STEPS i32 (i32.const 256))
(global $MAX_PRED_DEPTH i32 (i32.const 4))
```

**Tick/fuel pattern** → WAT step counting

```wat
(global.set $pred_steps (i32.add (global.get $pred_steps) (i32.const 1)))
(if (i32.gt_u (global.get $pred_steps) (global.get $MAX_PRED_STEPS))
  (then ...))
```

**EventBus** → JS host

```javascript
class EventBus {
  emit(kind, data) { ... }
  subscribe(fn) { ... }
}
```

### From Our Own Iteration

**ActorKind** → Split

- Rust: sets FLAG_REQUIRES_HUMAN in node flags
- WAT: checks flag, calls `io_is_human()`
- JS: returns whether current actor is human

**ConfirmationStatus** → Split

- Rust: PRAG004 checks paths to irreversible nodes
- WAT: IS_CONFIRMED opcode (0x44)
- JS: checks Y-context for confirmed entities

**Error nodes** → Split

- Rust: validates error nodes are reachable
- WAT: finds error edges, routes to them on failure

---

## What's Left To Do

### For Rust Compiler

1. Consolidate into single `pipeline.rs`
2. Remove all `runtime/` code
3. Remove IoOp enum (only JS needs op codes)
4. Implement the 21 constraint checks
5. Make predicate bytecode compiler match WAT opcodes

### For WAT

The file is complete. It has:

- Graph loading with magic/version validation
- FNV-1a hash for entry lookup
- Visited tracking with bitmap
- Full predicate VM with all opcodes
- Actor permission checking
- Error edge routing
- EventBus integration

### For JS Host

Write `io.js` with:

- IO dispatcher (switch on op codes)
- Variable resolver (Y-context access)
- String operations (contains, matches, etc.)
- EventBus class
- is_human() and is_confirmed() helpers

---

## The Mental Model

Think of it like this:

**XML** = The rules of a board game, written in a book

**Rust compiler** = A factory that prints the game board from the rulebook

**graph.bin** = The printed game board

**WAT** = The physics engine that moves pieces according to board layout

**JS** = The hands that actually pick up pieces and interact with the world

The physics engine doesn't know what "Monopoly" is. It just knows "if piece is here and dice says 3, move to there." The meaning comes from the board layout (the graph) and the hands (the IO).

---

## For Claude Code

When implementing:

1. **Start with WAT** - it's the foundation, and it's done
2. **Build JS host** - make WAT actually runnable
3. **Build Rust compiler** - make XML → graph.bin work
4. **Test end-to-end** - XML → compile → load → execute

The WAT file in this handoff is complete and tested against the binary format spec. The Rust patterns docs have all the constraint checks. The JS is just a dispatcher.

No npm. No frameworks. Just files.
