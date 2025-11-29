# /PXYZ Ontology

> **The Coordinate System for Business Reality**
>
> Everything is events or constraints. Graph traversal is logic. State is a lie.

---

## The Core Axiom

```shell
State = View(Events, Constraints, Time)
```

If you can't rebuild it from events (Z) filtered by constraints (Y), it doesn't exist. There is no privileged "current state" - only observations at coordinates.

---

## The Four Dimensions

| Dimension | What It Is | Runtime Role | Example |
|-----------|------------|--------------|---------|
| **P** | Primitives (entities) | Node IDs in graph | `contact`, `deal`, `task` |
| **X** | Operations (transforms) | Edge traversals | `create`, `update`, `transition` |
| **Y** | Constraints (rules) | Predicate evaluation | `is_owner`, `valid_amount`, `has_role` |
| **Z** | Frames (time) | Event log position | `2025-01-20T10:30:00Z` |

```shell
┌─────────────────────────────────────────────────────────────┐
│                    PXYZ Coordinate Space                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   P (What)          X (How)           Y (Rules)             │
│   ─────────         ──────            ─────────             │
│   Contact           create            is_owner              │
│   Deal              update            can_edit              │
│   Task              delete            valid_deal            │
│   User              transition        has_permission        │
│                                                             │
│                         Z (When)                            │
│                         ────────                            │
│                    Event Log Timeline                       │
│   ──────────────────────────────────────────────────────►   │
│   E₁    E₂    E₃    E₄    E₅    E₆    ...    Eₙ            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## The Three Levels of Understanding

### Street Level (For Humans)

- **Things** - what exists (contacts, deals)
- **Changes** - how things transform (create, update)
- **Rules** - what's allowed (ownership, tiers)
- **Moments** - when you look (timestamps)

### Engineer Level (For Builders)

- **Primitives** - typed entities addressed by P
- **Operations** - graph edges traversed via X
- **Constraints** - predicates evaluated from Y
- **Frames** - temporal coordinates in Z

### Mathematical Level (For The Reveal)

- **Objects** - things in a category
- **Morphisms** - arrows between objects
- **Functors** - structure-preserving maps
- **Natural Transformations** - morphisms between functors

---

## The Architecture Stack

```shell
┌─────────────────────────────────────────────────────────────┐
│  LAYER 1: XML DSL (workflow.xml)                            │
│  ─────────────────────────────────                          │
│  Human-editable. Excel-compatible. Where P, X, Y defined.   │
├─────────────────────────────────────────────────────────────┤
│  LAYER 2: Binary Graph (graph.bin)                          │
│  ──────────────────────────────────                         │
│  Compiled from XML. Content-addressed. Hot-reloadable.      │
├─────────────────────────────────────────────────────────────┤
│  LAYER 3: WAT Runtime (pxyz.wat → pxyz.wasm)               │
│  ─────────────────────────────────────────────              │
│  ~500 lines. Graph traversal + predicate VM. THE SYSTEM.    │
├─────────────────────────────────────────────────────────────┤
│  LAYER 4: JS IO Adapter (io.js)                             │
│  ────────────────────────────────                           │
│  ~200 lines. All side effects. Google, Qdrant, LLM, events. │
├─────────────────────────────────────────────────────────────┤
│  LAYER 5: Datastar UI                                       │
│  ────────────────────────                                   │
│  ~10KB. HTML + data-* attributes. Signals = Y-context.      │
└─────────────────────────────────────────────────────────────┘

Total attack surface: ~700 lines auditable code
Dependencies: 0
```

---

## P: Primitive Dimension

**What exists in your domain.**

A Primitive is a named kind of fact that can be observed at coordinates.

```typescript
type PrimitiveId = 
  | "contact" 
  | "deal" 
  | "task" 
  | "user"
  | "organization";
```

### P in the Runtime

P-coordinates map to **node IDs** in graph.bin:

```xml
<schemas>
  <schema id="contact">
    <field name="id" type="uuid" required="true"/>
    <field name="name" type="string" required="true"/>
    <field name="email" type="string" required="true"/>
  </schema>
</schemas>
```

### P in Capabilities

P encodes the **subject** of a capability token:

```shell
P:contact → UCAN.cmd = "/contact/*"
P:deal    → UCAN.cmd = "/deal/*"
```

---

## X: Operation Dimension

**How things transform.**

An Operation is a named transformation between primitives.

```typescript
type OperationId =
  | "contact.create"
  | "contact.update"
  | "contact.delete"
  | "deal.stage_transition"
  | "task.complete";
```

### X in the Runtime

X-coordinates map to **graph edges** and **external node op-codes**:

```xml
<workflow id="contact_search">
  <entry p="contact" x="search" node="validate"/>
  
  <nodes>
    <node id="validate" kind="transform"/>
    <node id="auth" kind="auth"/>
    <node id="search" kind="external" op="0x0300"/>  <!-- X = search -->
    <node id="render" kind="render"/>
  </nodes>
  
  <edges>
    <edge from="validate" to="auth"/>
    <edge from="auth" to="search"/>
    <edge from="search" to="render"/>
  </edges>
</workflow>
```

### X in Capabilities

X encodes the **verb** of a capability token:

```shell
X:read   → UCAN.cmd = "/*/read"
X:update → UCAN.cmd = "/*/update"
X:delete → UCAN.cmd = "/*/delete"
```

### Graph Traversal = Logic

**There is no separate "compile" step.**

```shell
Finding path through graph = compilation
Traversing path = execution
```

The route through coordinate space IS the program.

---

## Y: Constraint Dimension

**What's allowed.**

### The Key Insight: Y = Primitives Applied at Config Points

Y is NOT five different things. Y is **one thing** (constraint primitives) applied at **five different points**.

### Y Primitives (The Atoms)

```shell
Comparison: eq, neq, gt, lt, gte, lte
Logical:    and, or, not
Set:        contains, intersects, subset
Temporal:   before, after, within
Identity:   is_owner, has_role, is_self
Existence:  exists, empty, null
```

That's ~15-20 opcodes in the predicate VM. Done.

### Y Application Points (Config Says Where)

| Application Point | What It's Called | Example |
|-------------------|------------------|---------|
| Input validation | Syntactic Y | `exists($entity.email)` |
| Business rules | Semantic Y | `gt($deal.value, 0)` |
| Auth checks | Pragmatic Y | `eq($token.sub, $entity.owner_id)` |
| CRDT merge | Merge policy | `gt($a.timestamp, $b.timestamp)` |
| View filters | Projection Y | `eq($entity.status, "active")` |

### Y in XML

```xml
<predicates>
  <!-- Same primitives, reused everywhere -->
  <predicate id="is_owner">
    <eq left="$token.sub" right="$entity.owner_id"/>
  </predicate>
  
  <predicate id="valid_deal">
    <and>
      <gt left="$entity.value" right="0"/>
      <exists path="$entity.contact_id"/>
    </and>
  </predicate>
  
  <predicate id="can_access">
    <or>
      <ref predicate="is_owner"/>
      <contains left="$token.roles" right="admin"/>
    </or>
  </predicate>
</predicates>
```

### Y in the Runtime

The predicate VM doesn't care *why* you're evaluating. It just evaluates:

```shell
┌─────────────────────────────────────────┐
│         Predicate VM (~100 lines)       │
├─────────────────────────────────────────┤
│ Input: predicate bytecode + Y-context   │
│ Output: boolean                         │
│ Limits: 256 steps, 16 stack, 4 depth    │
│ Side effects: NONE                      │
└─────────────────────────────────────────┘
```

### Y in Capabilities

Y encodes the **constraints** of a capability token:

```javascript
{
  "cmd": "/contact/update",           // P + X
  "args": { 
    "scope": "own-records",           // Y constraint
    "filter": { "owner": "$INVOKER" } // Y constraint  
  },
  "nbf": 1732924800,                  // Z: not before
  "exp": 1735516800                   // Z: expires
}
```

### Y as Geometry (Continuous, Not Boolean)

Traditional validation loses information:

```typescript
// Boolean (information lost)
creditScore >= 580  →  true/false

// Geometric (information preserved)
creditScore = 720  →  {
  fits: true,
  score: 0.92,           // 92% deep inside region
  distance_to_edge: +140 // 140 points above minimum
}
```

---

## Z: Temporal Dimension

**When you observe.**

A Frame is a temporal coordinate where observations are made.

```typescript
type Frame = string; // ISO 8601
// "2025-01-15T10:30:00Z"
```

### Z and Event Sourcing

**Events are the only thing stored. Everything else is computed.**

```shell
┌─────────────────────────────────────────────────────────────┐
│                    Event Log (Z-Space)                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  E₁: contact.created     Z: 2025-01-15T10:00:00Z           │
│  E₂: contact.updated     Z: 2025-01-16T14:30:00Z           │
│  E₃: contact.deleted     Z: 2025-01-20T09:00:00Z           │
│                                                             │
│  ──────────────────────────────────────────────────────►    │
│                                                             │
│  Observer A at Z₁: sees contact as "active"                 │
│  Observer B at Z₂: sees contact as "updated"                │
│  Observer C at Z₃: sees contact as "deleted"                │
│                                                             │
│  ALL THREE ARE CORRECT for their temporal coordinate.       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### The Formula

```shell
State(T) = fold(initialState, events[0..T], applyEvent)
```

This gives you for free:

- **Audit trail** - the event log IS compliance documentation
- **Time travel** - replay to any Z coordinate
- **Bug recovery** - fix projection logic, replay, state corrects
- **Retroactive corrections** - issue correction event, recompute

### Z in Capabilities

Z encodes the **temporal bounds** of a capability:

```javascript
{
  "nbf": 1732924800,  // not before (Unix timestamp)
  "exp": 1735516800   // expires (Unix timestamp)
}
```

---

## The Core Objects

### Event (The Only Thing Stored)

```typescript
interface Event {
  id: EventId;
  P: PrimitiveId;      // what it's about
  X: OperationId;      // what happened
  Y?: RegionId;        // under which rules
  Z: Frame;            // when it happened
  payload: unknown;    // the actual data
  proof: {
    hash: string;      // SHA-256
    prevHash: string;  // chain link
  };
}
```

### Observation (What You See)

```typescript
interface Observation<T> {
  P: PrimitiveId;
  Z: Frame;
  Y?: RegionId;
  value: T;           // reconstructed by replaying events
  proof: EventId[];   // which events justify this
}
```

### Path (How You Got There)

```typescript
interface Path {
  from: PrimitiveId;
  to: PrimitiveId;
  steps: Array<{
    node: NodeId;
    X: OperationId;
  }>;
}
```

### Fitness (How Well It Fits)

```typescript
interface Fitness {
  fits: boolean;
  score: number;                    // 0..1, depth inside region
  distance: Record<string, number>; // per-constraint distances
  violations?: Violation[];         // if outside, what failed
}
```

---

## The Four Verbs

Every query in PXYZ answers one of four questions:

| Verb | Question | Navigates | Returns |
|------|----------|-----------|---------|
| `at` | What was X at that moment? | Z | Observation |
| `trace` | How did we get from A to B? | X | Path |
| `check` | Does this fit these rules? | Y | Fitness |
| `track` | How did X change over time? | Z interval | Timeline |

### Verb Implementations

```typescript
// What was the contact's email when the deal closed?
const obs = pxyz.at({
  P: "contact.email",
  Z: deal.closed_at,
  Y: "workspace_ABC"
});

// How do we calculate deal score?
const path = pxyz.trace({
  from: "deal.attributes",
  to: "deal.score"
});

// Does this org fit the startup tier?
const fitness = pxyz.check({
  item: organization,
  region: "startup_tier"
});

// Show me contact status changes this month
const timeline = pxyz.track({
  P: "contact.status",
  start: "2025-01-01",
  end: "2025-01-31"
});
```

---

## Capability Tokens as Coordinates

A PXYZ coordinate maps directly to a UCAN capability:

```shell
P:contact/X:update/Y:own-records/Z:next-30-days
                    ↓
{
  "cmd": "/contact/update",
  "args": { "scope": "own-records", "owner": "$INVOKER" },
  "nbf": <now>,
  "exp": <now + 30 days>
}
```

### What This Gives You

| Capability Feature | PXYZ Mapping |
|--------------------|--------------|
| Self-documenting permissions | P + X = the path IS the permission |
| Attenuation | Narrow coordinates = reduced access |
| Delegation without coordination | Issue sub-capabilities locally |
| Offline authorization | Token contains complete proof |
| Zero-trust API | Every request self-verifies |

---

## Datastar Integration

Datastar signals = Y-context for UI. SSE patches = Z-dimension made visible.

```html
<div data-signals="{ query: '', contacts: [], loading: false }">
  <input 
    data-bind:query
    data-on:input.debounce_300ms="$pxyz('contact', 'search')"
  />
  <div id="content">
    <!-- Graph render nodes emit HTML, Datastar merges it -->
  </div>
</div>
```

### Why LLMs Can Compose UI

1. HTML is self-describing
2. `data-*` attributes are declarative
3. No JSON schema contracts to learn
4. Browser already knows how to render `<table>`, `<button>`, etc.

```javascript
// LLM outputs UI by composing primitives
const ui = {
  card: (title, content) => `<div class="card"><h3>${title}</h3>${content}</div>`,
  button: (label, p, x, y) => `<button data-on:click="$pxyz('${p}','${x}',${JSON.stringify(y)})">${label}</button>`,
  field: (label, value) => `<div><span class="label">${label}</span>: ${value}</div>`,
};

// Agent composes at runtime
ui.card('Contact', ui.field('Name', data.name) + ui.button('Edit', 'contact', 'edit', {id: data.id}))
```

---

## The Runtime in Detail

### Graph Structure (graph.bin)

```shell
Header (96 bytes)
├── Magic: 0x504E5958 ("PXYZ")
├── Version
├── Counts (nodes, edges, predicates, strings, entries)
├── Source hash (SHA-256 of XML)
└── Section offsets

Nodes (16 bytes each)
├── Node ID
├── Kind (transform=0, external=1, render=2, signal=3, auth=4, terminal=5)
├── Op code (for external nodes)
├── Data offset (string pool)
└── Edge range

Edges (12 bytes each)
├── Target node ID
├── Predicate ID (0 = always true)
├── Weight
└── Flags

Predicates (variable)
└── Bytecode (max 256 bytes each)

String Pool
└── Null-terminated strings
```

### Traversal Algorithm

```shell
1. Load graph.bin into WASM memory
2. Look up entry point: (P, X) → start node
3. Loop:
   a. Execute current node by kind
   b. For each outgoing edge:
      - Evaluate predicate (Y-context)
      - If true, mark as candidate
   c. Follow highest-weight true edge
   d. If terminal node, return
   e. If max steps exceeded, abort
4. Return result + trace
```

### Safety Guarantees

| Guarantee | Mechanism |
|-----------|-----------|
| Traversal terminates | Max 1000 nodes, cycle detection via bitmap |
| Predicates terminate | Max 256 steps, max 4 call depth |
| Memory isolation | WASM sandbox, no host memory access |
| No side effects in logic | IO only through explicit imports |
| Deterministic | Same inputs → same outputs |

---

## The Deep Implications

### 1. State Is Derived, Not Stored

Traditional: Store state, hope it's correct.
PXYZ: Store events, compute state with proof.

### 2. Compilation Is Pathfinding

Traditional: Separate compile and execute phases.
PXYZ: Finding path = compilation, traversing = execution.

### 3. Constraints Are Geometry

Traditional: Boolean validation loses information.
PXYZ: Fitness scores preserve distance to boundaries.

### 4. The Pattern Is Universal

Anywhere you have:

- Things that exist (P)
- Transformations between things (X)
- Rules that constrain validity (Y)
- Observation that depends on time (Z)

You're navigating coordinate space. Git, SQL, React, finance, healthcare - all PXYZ.

---

## Domain Application Template

For any new domain:

```typescript
// 1. Identify Primitives (P)
type Primitives = "contact" | "deal" | "task";

// 2. Identify Operations (X)
type Operations = "create" | "update" | "delete" | "transition";

// 3. Identify Constraint Regions (Y)
type Regions = "free_tier" | "startup_tier" | "enterprise_tier";

// 4. Define Temporal Frames (Z)
type Frames = ISODateTime;

// 5. Map common queries
pxyz.at({ P, Z, Y? })         // What was it?
pxyz.trace({ from, to })      // How did it get there?
pxyz.check({ item, region })  // Does it fit?
pxyz.track({ P, start, end }) // How did it change?
```

---

## The Meta-Insight

**Hard problems are often coordinate problems.**

"Prove what the APR was at lock time" in state-coordinates → detective work.
"Prove what the APR was at lock time" in PXYZ → `pxyz.at({ P: "APR", Z: lockTime })`.

**The problem didn't get easier. We stopped fighting the coordinate system.**

---

## Summary

```shell
┌─────────────────────────────────────────────────────────────┐
│                         /PXYZ                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  P = What exists      → Nodes in graph, subjects in caps    │
│  X = How it changes   → Edges, op-codes, verbs in caps      │
│  Y = What's allowed   → Predicates (atoms applied at config)│
│  Z = When you look    → Event log position, temporal bounds │
│                                                             │
│  Graph traversal = Logic                                    │
│  Capability = Coordinate                                    │
│  State = View(Events, Constraints, Time)                    │
│                                                             │
│  ~700 lines total. Zero dependencies. Auditable in an hour. │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

*The graph is physics. The predicates are laws. The runtime is the universe.*
