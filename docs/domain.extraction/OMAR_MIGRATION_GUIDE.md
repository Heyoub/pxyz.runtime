# MIGRATION GUIDE: TypeScript Effects Programs → PXYZ/OMAR

> **Purpose**: Map old TypeScript architecture to new PXYZ/OMAR architecture.  
> **For**: Understanding what stays, what goes, and what transforms.

---

## EXECUTIVE SUMMARY

**What We Had**: 11,573 lines of TypeScript across 10 domain files using Effect-TS, branded types, and imperative service patterns.

**What We're Building**: ~700 lines of auditable PXYZ/OMAR runtime + XML workflows + IO adapters.

**Core Transformation**: 
- TypeScript service methods → XML workflow graphs
- Effect-TS composition → Graph traversal with predicates  
- Imperative code → Declarative coordinates
- 250KB TypeScript + dependencies → 15KB WASM + graph + IO

---

## ARCHITECTURAL MAPPING

### OLD ARCHITECTURE

```
┌─────────────────────────────────────────────────┐
│  TypeScript Effects Programs                    │
│  ────────────────────────────                   │
│  - AnalyticsEffectsProgram.ts (597 lines)       │
│  - ApprovalsEffectsProgram.ts (546 lines)       │
│  - BusinessEffectsProgram.ts (613 lines)        │
│  - CalendarEffectsProgram.ts (1703 lines)       │
│  - CommsEffectsProgram.ts (1628 lines)          │
│  - ContactsEffectsProgram.ts (1744 lines)       │
│  - DealsEffectsProgram.ts (683 lines)           │
│  - DocumentsEffectsProgram.ts (999 lines)       │
│  - EmailEffectsProgram.ts (1725 lines)          │
│  - EffectsProgramHelpers.ts (1335 lines)        │
├─────────────────────────────────────────────────┤
│  Effect-TS Runtime                              │
│  ────────────────                               │
│  - Effect composition                           │
│  - Service injection                            │
│  - Error handling                               │
│  - Async coordination                           │
├─────────────────────────────────────────────────┤
│  TypeScript Type System                         │
│  ───────────────────────                        │
│  - Branded types (P.ts)                         │
│  - Operation enums (X.ts)                       │
│  - Context types (Y.ts)                         │
│  - Event types (Z.ts)                           │
│  - Discriminated unions                         │
├─────────────────────────────────────────────────┤
│  Database Abstraction                           │
│  ──────────────────────                         │
│  - FileDatabase implementation                  │
│  - CRUD operations                              │
│  - Query engine                                 │
│  - Event sourcing                               │
└─────────────────────────────────────────────────┘
```

### NEW ARCHITECTURE

```
┌─────────────────────────────────────────────────┐
│  LAYER 1: XML DSL                               │
│  ────────────────                               │
│  - workflow.xml (human-editable)                │
│  - Business logic as declarative graph          │
│  - Excel-compatible editing                     │
├─────────────────────────────────────────────────┤
│  LAYER 2: Binary Graph                          │
│  ──────────────────────                         │
│  - graph.bin (compiled from XML)                │
│  - Content-addressed (SHA-256)                  │
│  - Hot-reloadable                               │
├─────────────────────────────────────────────────┤
│  LAYER 3: WASM Runtime                          │
│  ────────────────────────                       │
│  - pxyz.wat (~500 lines)                        │
│  - Graph traversal engine                       │
│  - Predicate VM                                 │
│  - Zero dependencies                            │
├─────────────────────────────────────────────────┤
│  LAYER 4: IO Adapter                            │
│  ──────────────────────                         │
│  - io-browser.ts (~200 lines per platform)     │
│  - All external calls (DB, APIs, etc.)         │
│  - Platform-specific implementations            │
└─────────────────────────────────────────────────┘
```

---

## WHAT STAYS

### 1. Business Logic (100%)
**ALL business requirements stay exactly the same.**

From `AnalyticsEffectsProgram.ts`:
```typescript
// OLD: TypeScript function
aggregateEventsIntoBuckets(events, buckets, aggregationFunction)
```

```xml
<!-- NEW: Workflow node -->
<node id="aggregate" kind="external" op="0x0701">
  <input ref="events"/>
  <input ref="buckets"/>
  <input ref="aggregation_function"/>
</node>
```

**The WHAT stays. The HOW changes.**

### 2. Domain Concepts
- Entity types (Contact, Deal, Event, etc.)
- Operations (create, read, update, delete, query, etc.)
- Business rules (approval flows, lifecycle stages, etc.)
- Validation rules (required fields, formats, ranges)
- Authorization logic (who can do what)

### 3. Data Structures
- Contact: {id, name, email, phone, ...}
- Deal: {id, name, value, stage, ...}
- Event: {id, title, startTime, endTime, ...}
- etc.

**Field names, types, and relationships stay the same.**

### 4. External Integrations
- Google Workspace (Contacts, Calendar, Drive, Gmail)
- Qdrant (vector search)
- LLM providers (Claude, GPT)
- Storage (IndexedDB, files)
- Event log

---

## WHAT GOES

### 1. TypeScript Service Classes (100%)
```typescript
// DELETE: All of this
export const createAnalyticsService = () =>
  Effect.gen(function* (_) {
    return {
      queryEvents: (userId, filters) => Effect.gen(...),
      aggregateTimeSeries: (metric, timeRange) => Effect.gen(...),
      generateSummary: (events) => Effect.sync(...),
      // ... hundreds of methods
    };
  });
```

**Why**: Business logic moves to declarative workflows. Services become XML graphs.

### 2. Effect-TS Composition
```typescript
// DELETE: Effect composition
Effect.gen(function* (_) {
  const service = yield* _(createAnalyticsService());
  const events = yield* _(service.queryEvents(userId, filters));
  const summary = yield* _(service.generateSummary(events));
  return summary;
});
```

**Why**: Graph traversal replaces Effect composition.

### 3. Explicit Type Definitions
```typescript
// DELETE: Verbose type definitions
export interface Contact {
  readonly id: UUID;
  readonly name: string;
  readonly email?: string;
  readonly phone?: string;
  readonly type?: ContactType;
  readonly status?: ContactStatus;
  // ... 20 more fields
  readonly pxyz: PXYZ;
  readonly createdAt: ISODateTime;
  readonly updatedAt: ISODateTime;
}
```

**Why**: Types become runtime schema validation in XML.

### 4. Discriminated Union Inputs
```typescript
// DELETE: Union type patterns
export type ContactsCreateInput =
  | { type: "contact"; data: {...} }
  | { type: "interaction"; data: {...} }
  | { type: "relationship"; data: {...} };

switch (input.type) {
  case "contact": return yield* _(service.createContact(input.data));
  case "interaction": return yield* _(service.logInteraction(input.data));
  // ...
}
```

**Why**: Workflows have explicit entry points, no switch statements needed.

### 5. Imperative Error Handling
```typescript
// DELETE: Try/catch, Effect.fail, tagged errors
if (!entity) {
  return yield* __(Effect.fail(new ContactNotFoundError({
    contactId: id,
    pxyz: buildPXYZ(...)
  })));
}
```

**Why**: Graph traversal handles errors via edge predicates.

### 6. Helper Functions (Mostly)
```typescript
// DELETE: Most helpers like logEffect, buildPXYZ
yield* __(logEffect(`[ContactsService] Creating contact`));
const pxyz = buildPXYZ("contact", "create", userId);
```

**Why**: Logging/tracing built into graph execution. PXYZ built by compiler.

---

## WHAT TRANSFORMS

### 1. Service Methods → Workflow Nodes

**OLD**:
```typescript
createContact: (args: {
  firstName: string;
  lastName: string;
  email?: string;
  // ...
}) => Effect.gen(function* (__) {
  // 1. Validate
  if (!args.email && !args.phone) {
    return yield* __(Effect.fail(new ValidationError({
      field: "contact",
      message: "Email or phone required"
    })));
  }
  
  // 2. Check duplicates
  const existing = yield* __(db.query(toEntityName("contact"), {
    email: args.email
  }));
  
  if (existing.length > 0) {
    return yield* __(Effect.fail(new DuplicateError({
      email: args.email,
      existingId: existing[0].id
    })));
  }
  
  // 3. Create
  const contact = yield* __(db.create(toEntityName("contact"), {
    name: `${args.firstName} ${args.lastName}`,
    email: args.email,
    phone: args.phone,
    // ...
  }));
  
  return contact;
})
```

**NEW**:
```xml
<workflow id="contact_create">
  <entry p="contact" x="create" node="validate"/>
  
  <nodes>
    <!-- 1. Validate -->
    <node id="validate" kind="transform">
      <schema>
        <required field="firstName"/>
        <required field="lastName"/>
        <either fields="email,phone"/>
      </schema>
    </node>
    
    <!-- 2. Check duplicates -->
    <node id="check_dupe" kind="external" op="0x0105">
      <!-- ENTITY_SEARCH -->
    </node>
    
    <!-- 3. Create if no dupe -->
    <node id="create" kind="external" op="0x0100">
      <!-- ENTITY_CREATE -->
    </node>
    
    <node id="done" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="validate" to="check_dupe">
      <when><always/></when>
    </edge>
    <edge from="check_dupe" to="create">
      <when>
        <eq left="$result.count" right="0"/>
      </when>
    </edge>
    <edge from="check_dupe" to="done">
      <when>
        <gt left="$result.count" right="0"/>
      </when>
    </edge>
    <edge from="create" to="done">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

### 2. Pure Functions → Predicate Bytecode

**OLD**:
```typescript
export const eventInBucket = (
  event: EventCoordinate,
  bucket: TimeSeriesBucket
): boolean => {
  const timestamp = event.timestamp.getTime();
  const [start, end] = bucket.timeSpan;
  return timestamp >= start.getTime() && timestamp < end.getTime();
};
```

**NEW**:
```xml
<predicate id="event_in_bucket">
  <and>
    <ge left="$event.timestamp" right="$bucket.start"/>
    <lt left="$event.timestamp" right="$bucket.end"/>
  </and>
</predicate>
```

**Compiled to bytecode**:
```
LOAD_VAR event.timestamp    ; Push event timestamp
LOAD_VAR bucket.start        ; Push bucket start
GE                           ; timestamp >= start?
LOAD_VAR event.timestamp     ; Push event timestamp
LOAD_VAR bucket.end          ; Push bucket end
LT                           ; timestamp < end?
AND                          ; Both conditions true?
RET                          ; Return boolean
```

### 3. Database Calls → IO Operation Codes

**OLD**:
```typescript
const contact = yield* __(db.create(toEntityName("contact"), contactData));
const contacts = yield* __(db.query(toEntityName("contact"), { userId }));
const contact = yield* __(db.find(toEntityName("contact"), contactId));
const updated = yield* __(db.update(toEntityName("contact"), id, changes));
```

**NEW**:
```xml
<!-- Create -->
<node kind="external" op="0x0100">
  <param key="entity" value="contact"/>
  <param key="data" ref="$contactData"/>
</node>

<!-- Query -->
<node kind="external" op="0x0104">
  <param key="entity" value="contact"/>
  <param key="filter" ref="$filter"/>
</node>

<!-- Read -->
<node kind="external" op="0x0101">
  <param key="entity" value="contact"/>
  <param key="id" ref="$contactId"/>
</node>

<!-- Update -->
<node kind="external" op="0x0102">
  <param key="entity" value="contact"/>
  <param key="id" ref="$id"/>
  <param key="changes" ref="$changes"/>
</node>
```

### 4. Effect Streams → Event Log Queries

**OLD**:
```typescript
const stream = yield* __(PXYZStream);
const events = yield* __(stream.query(
  {
    P: toEntityName("contact"),
    X: "*",
    Y: "*",
    Z: "*"
  },
  {
    start: startDate,
    end: endDate
  }
));
```

**NEW**:
```xml
<node kind="external" op="0x0911">
  <!-- EVENT_LOG_QUERY -->
  <param key="p" value="contact"/>
  <param key="x" value="*"/>
  <param key="y" value="*"/>
  <param key="z_start" ref="$startDate"/>
  <param key="z_end" ref="$endDate"/>
</node>
```

### 5. Workflow Execution → Graph Traversal

**OLD**:
```typescript
const analysisWorkflow: WorkflowDefinition = {
  id: createUUID(),
  name: "time-series-analysis",
  nodes: [
    { id: "start", type: "start", ... },
    { id: "analyze", type: "agent", ... },
    { id: "end", type: "end", ... }
  ],
  connections: [...]
};

const result = yield* __(executeWorkflow(analysisWorkflow, { metric, timeRange }));
```

**NEW**:
```xml
<!-- Workflow IS the graph, no runtime construction needed -->
<workflow id="time_series_analysis">
  <entry p="analytics_timeseries" x="analyze" node="start"/>
  
  <nodes>
    <node id="start" kind="transform">
      <input ref="metric"/>
      <input ref="timeRange"/>
    </node>
    
    <node id="analyze" kind="external" op="0x0800">
      <!-- LLM_COMPLETE -->
      <param key="model" value="claude-3.5-sonnet"/>
      <param key="prompt" value="Analyze time series..."/>
    </node>
    
    <node id="done" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="start" to="analyze"><when><always/></when></edge>
    <edge from="analyze" to="done"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## FILE-BY-FILE MIGRATION

### AnalyticsEffectsProgram.ts (597 lines)

**What Moves to XML**:
- `queryEvents` → workflow `analytics_query_events`
- `aggregateTimeSeries` → workflow `analytics_aggregate`
- `generateSummary` → workflow `analytics_summary`
- `calculateTrends` → predicate or workflow node
- `groupByDimension` → workflow node
- `buildReport` → workflow `analytics_build_report`

**What Moves to IO Adapter**:
- PXYZStream.query() → op code 0x0911 (EVENT_LOG_QUERY)
- LLM calls → op code 0x0800 (LLM_COMPLETE)

**Pure Functions** (keep for reference, implement in predicates):
- `createTimeBuckets` → logic in external node or predicate
- `eventInBucket` → predicate
- `aggregateEventsIntoBuckets` → external node logic
- `calculateMultiGranularityBucketing` → workflow

### ApprovalsEffectsProgram.ts (546 lines)

**What Moves to XML**:
- `requestApproval` → workflow `approval_request`
- `respondToApproval` → workflow `approval_respond`
- `listPendingApprovals` → workflow `approval_list_pending`
- `sendApprovalReminder` → workflow `approval_remind`
- `initiateESignature` → workflow `esignature_initiate`
- `getApprovalAnalytics` → workflow `approval_analytics`

**Authorization Predicates**:
- `is_approver` → `<contains left="$approval.approvers" right="$userId"/>`
- `is_approved` → `<eq left="$approval.status" right="approved"/>`
- `all_approved` → predicate checking approval chain

### BusinessEffectsProgram.ts (613 lines)

**What Moves to XML**:
- `registerQuickTool` → workflow `quick_tool_create`
- `getQuickTools` → workflow `quick_tool_query`
- `updateToolUsage` → workflow `quick_tool_track_usage`
- `createToolChain` → workflow `tool_chain_create`
- `executeToolChain` → workflow `tool_chain_execute`
- `registerAgent` → workflow `agent_register`
- `getToolRecommendations` → workflow `quick_tool_recommend`

**Shapes API** (delete entirely):
- All `shapes.quickTool.*`, `shapes.toolChain.*`, etc.
- Replaced by generic entity operations with P="quick_tool"

### CalendarEffectsProgram.ts (1703 lines)

**What Moves to XML**:
- `createEvent` → workflow `calendar_event_create`
- `getEvents` → workflow `calendar_event_query`
- `checkConflicts` → workflow `calendar_check_conflicts`
- `findOptimalMeetingTimes` → workflow `calendar_find_slots`
- `scheduleFocusBlock` → workflow `calendar_schedule_focus`
- `createCalendar` → workflow `calendar_create`
- `shareCalendar` → workflow `calendar_share`
- `analyzeCalendarHealth` → workflow `calendar_analyze_health`

**Scheduling Constraints** → Predicates:
- `DEFAULT_SCHEDULING_CONSTRAINTS` → predicate config
- `determineThreadActivityStatus` → predicate
- `eventInBucket` → predicate

**Pure Functions** (implement in predicates or external nodes):
- `createTimeBuckets` → external node
- `detectOverlaps` → predicate
- `resolveConflicts` → workflow logic

### CommsEffectsProgram.ts (1628 lines)

**What Moves to XML**:
- `sendMessage` → workflow `message_send`
- `getMessages` → workflow `message_query`
- `createChannel` → workflow `channel_create`
- `getThreadActivity` → workflow `message_analyze_threads`
- `findUrgentUnread` → workflow `message_find_urgent`
- `createTemplate` → workflow `message_template_create`

**Thread Analysis** → Predicates/Workflows:
- `calculateThreadActivitySpan` → external node
- `calculateUnreadWindow` → external node
- `determineThreadActivityStatus` → predicate
- `filterThreadsByActivity` → workflow
- `findUrgentUnreadThreads` → workflow
- `aggregateThreadActivity` → workflow

### ContactsEffectsProgram.ts (1744 lines)

**What Moves to XML**:
- `createContact` → workflow `contact_create`
- `getContacts` → workflow `contact_query`
- `findDuplicates` → workflow `contact_find_duplicates`
- `mergeContacts` → workflow `contact_merge`
- `updateLifecycle` → workflow `contact_update_lifecycle`
- `logInteraction` → workflow `interaction_create`
- `getInteractions` → workflow `interaction_query`
- `createRelationship` → workflow `relationship_create`
- `getRelationships` → workflow `relationship_query`
- `enrichContact` → workflow `contact_enrich`

**Validation** → Schema validation in XML:
```xml
<node kind="transform">
  <schema>
    <required field="firstName"/>
    <required field="lastName"/>
    <either fields="email,phone"/>
    <format field="email" pattern="email"/>
    <format field="phone" pattern="phone"/>
  </schema>
</node>
```

### DealsEffectsProgram.ts (683 lines)

**What Moves to XML**:
- `createDeal` → workflow `deal_create`
- `moveDealToStage` → workflow `deal_update_stage`
- `logActivity` → workflow `deal_activity_log`
- `getPipeline` → workflow `deal_query_pipeline`
- `forecastRevenue` → workflow `deal_forecast`
- `scoreDeal` → workflow `deal_score`
- `recordLoss` → workflow `deal_record_loss`

**Probability Calculation** → Predicate or External Node:
```xml
<node id="calculate_probability" kind="transform">
  <mapping>
    <case when="$deal.stage == 'lead'" then="10"/>
    <case when="$deal.stage == 'qualified'" then="25"/>
    <case when="$deal.stage == 'proposal'" then="50"/>
    <case when="$deal.stage == 'negotiation'" then="75"/>
    <case when="$deal.stage == 'closed_won'" then="100"/>
    <case when="$deal.stage == 'closed_lost'" then="0"/>
  </mapping>
</node>
```

### DocumentsEffectsProgram.ts (999 lines)

**What Moves to XML**:
- `createDocument` → workflow `document_create`
- `createVersion` → workflow `document_create_version`
- `shareDocument` → workflow `document_share`
- `addComment` → workflow `document_comment_create`
- `searchDocuments` → workflow `document_search`
- `createTemplate` → workflow `document_template_create`
- `generateFromTemplate` → workflow `document_generate`
- `processOCR` → workflow `document_extract_text`

**File Operations** → IO Adapter:
- File upload → op code (custom)
- OCR processing → op code 0x08xx (AI processing)
- Preview generation → op code (custom)

### EmailEffectsProgram.ts (1725 lines)

**What Moves to XML**:
- `sendEmail` → workflow `email_send`
- `getEmails` → workflow `email_query`
- `createRule` → workflow `email_rule_create`
- `createTemplate` → workflow `email_template_create`
- `trackEmail` → workflow `email_track`

**Email Processing** → External Nodes:
- SMTP send → op code (custom)
- IMAP fetch → op code (custom)
- Thread detection → op code or workflow
- Categorization → op code 0x0801 (LLM_CLASSIFY)

### EffectsProgramHelpers.ts (1335 lines)

**What Stays** (move to IO adapter or delete):
- `buildPXYZ` → Delete (compiler generates PXYZ)
- `logEffect` → Delete (tracing built into runtime)
- `searchRecords` → Generic search in IO adapter
- `filterByFieldIn` → Predicate
- `sortRecords` → Sorting in IO adapter
- `groupRecords` → Grouping in IO adapter

**Constants** (move to config):
```typescript
export const SEARCH_LIMITS = {
  MAX_RESULTS: 1000,
  DEFAULT_LIMIT: 100
};
```

**NEW** (config file or XML constants):
```xml
<constants>
  <constant name="SEARCH_MAX_RESULTS" value="1000"/>
  <constant name="SEARCH_DEFAULT_LIMIT" value="100"/>
</constants>
```

---

## MIGRATION WORKFLOW

### Step 1: Extract Business Logic ✅
**Status**: COMPLETE (this document)

Created:
- `OMAR_BUSINESS_LOGIC_REFERENCE.md` - Technology-agnostic requirements
- `OMAR_PXYZ_COORDINATE_MAPPING.md` - PXYZ coordinate addressing

### Step 2: Design XML Workflows
**Action**: Create `workflow.xml` for each domain

Example structure:
```xml
<omar>
  <!-- Schemas -->
  <schemas>
    <schema id="contact">...</schema>
    <schema id="deal">...</schema>
    <!-- ... -->
  </schemas>
  
  <!-- Predicates -->
  <predicates>
    <predicate id="is_owner">...</predicate>
    <predicate id="can_write">...</predicate>
    <!-- ... -->
  </predicates>
  
  <!-- Workflows -->
  <workflow id="contact_create">...</workflow>
  <workflow id="deal_pipeline">...</workflow>
  <workflow id="approval_flow">...</workflow>
  <!-- ... -->
  
  <!-- Templates -->
  <templates>
    <template id="contact_list">...</template>
    <!-- ... -->
  </templates>
</omar>
```

### Step 3: Implement IO Adapter
**Action**: Create `io-browser.ts` with operation handlers

```typescript
// io-browser.ts
export const IOAdapter = {
  // Entity operations
  0x0100: async (params) => { /* ENTITY_CREATE */ },
  0x0101: async (params) => { /* ENTITY_READ */ },
  0x0102: async (params) => { /* ENTITY_UPDATE */ },
  0x0103: async (params) => { /* ENTITY_DELETE */ },
  0x0104: async (params) => { /* ENTITY_LIST */ },
  0x0105: async (params) => { /* ENTITY_SEARCH */ },
  
  // Google Workspace
  0x0300: async (params) => { /* GOOGLE_CONTACTS_SEARCH */ },
  0x0301: async (params) => { /* GOOGLE_CONTACTS_GET */ },
  // ...
  
  // Vector/RAG
  0x0700: async (params) => { /* QDRANT_SEARCH */ },
  0x0701: async (params) => { /* QDRANT_INDEX */ },
  // ...
  
  // AI
  0x0800: async (params) => { /* LLM_COMPLETE */ },
  0x0801: async (params) => { /* LLM_CLASSIFY */ },
  // ...
  
  // Storage
  0x0900: async (params) => { /* STORAGE_GET */ },
  0x0901: async (params) => { /* STORAGE_SET */ },
  0x0910: async (params) => { /* EVENT_LOG_APPEND */ },
  0x0911: async (params) => { /* EVENT_LOG_QUERY */ },
};
```

### Step 4: Build Compiler
**Action**: Implement XML → graph.bin compiler

Inputs: `workflow.xml`
Output: `graph.bin`

Validations:
- Schema validation (XSD)
- Reference validation (all node/predicate IDs exist)
- Cycle detection
- Type checking

### Step 5: Implement Runtime
**Action**: Write `pxyz.wat` (~500 lines)

Core functions:
- `execute(entry_id, y_ctx, z_time) -> result`
- `traverse_graph(node_id) -> next_node_id`
- `eval_predicate(pred_id, context) -> bool`
- `call_io(op_code, params) -> result`

### Step 6: Integration Testing
**Action**: Test each workflow end-to-end

For each operation in OMAR_BUSINESS_LOGIC_REFERENCE.md:
1. Create test inputs
2. Execute workflow
3. Verify outputs match expected behavior
4. Verify events emitted
5. Verify authorization enforced

### Step 7: Performance Testing
**Action**: Verify runtime performance

Targets:
- Graph load: <50ms
- Simple traversal (3-5 nodes): <10ms
- Complex workflow (10+ nodes): <100ms
- Predicate evaluation: <1ms
- IO call overhead: <5ms

### Step 8: Decommission TypeScript
**Action**: Remove old Effects Programs

Delete:
- All *EffectsProgram.ts files
- EffectsProgramHelpers.ts
- Effect-TS dependencies
- Branded type files (P.ts, X.ts, Y.ts, Z.ts) - replaced by XML schemas

Keep:
- Business logic documentation (this document)
- Test suites (port to graph-based tests)
- IO integrations (moved to io-adapter)

---

## COMPARISON: BEFORE vs AFTER

### Lines of Code

**BEFORE**:
```
TypeScript Effects Programs: 11,573 lines
Effect-TS runtime: ~5,000 lines (dependency)
Type definitions: ~2,000 lines
Total: ~18,573 lines of TypeScript
```

**AFTER**:
```
XML workflows: ~2,000 lines (human-readable)
WAT runtime: ~500 lines (auditable)
IO adapter: ~200 lines per platform
Binary graph: ~5KB (compiled, not source)
Total: ~700 auditable lines + declarative workflows
```

**Reduction**: 96% fewer lines of imperative code

### Bundle Size

**BEFORE**:
```
TypeScript + dependencies: ~250KB minified
Effect-TS: ~80KB
Total: ~330KB
```

**AFTER**:
```
WASM runtime: ~10KB
Graph binary: ~5KB
IO adapter: ~5KB
Total: ~20KB
```

**Reduction**: 94% smaller bundle

### Attack Surface

**BEFORE**:
```
Dependencies: npm packages (hundreds of transitive deps)
Runtime: TypeScript → JavaScript → Browser VM
Code paths: Thousands (complex Effect composition)
Audit effort: Weeks (too much code to audit)
```

**AFTER**:
```
Dependencies: 0
Runtime: WASM sandbox (memory isolation)
Code paths: Graph edges (explicit, finite)
Audit effort: Afternoon (700 lines + graph visualization)
```

### Maintainability

**BEFORE**:
```
To add feature:
1. Create new service method (50-100 lines TypeScript)
2. Add type definitions (20-30 lines)
3. Wire up Effect composition (10-20 lines)
4. Add discriminated union case (5-10 lines)
5. Test with TypeScript compiler + Effect runtime

Developer skill required: Advanced TypeScript + Effect-TS
```

**AFTER**:
```
To add feature:
1. Add workflow to XML (20-40 lines declarative)
2. Add schema if new entity (10-20 lines)
3. Add predicates if needed (5-10 lines)
4. Recompile graph (instant)
5. Test with graph runtime

Developer skill required: Basic XML + business domain knowledge
Non-developer skill: Can edit workflows in Excel
```

---

## MIGRATION CHECKLIST

### Phase 1: Foundation
- [x] Extract business logic to tech-agnostic docs
- [x] Map PXYZ coordinates
- [ ] Design XML schema (XSD)
- [ ] Implement compiler (XML → graph.bin)
- [ ] Implement runtime (pxyz.wat)
- [ ] Implement IO adapter (io-browser.ts)

### Phase 2: Core Workflows
- [ ] Analytics workflows
- [ ] Approvals workflows
- [ ] Business tools workflows
- [ ] Calendar workflows
- [ ] Communications workflows
- [ ] Contacts workflows
- [ ] Deals workflows
- [ ] Documents workflows
- [ ] Email workflows

### Phase 3: Integration
- [ ] Google Workspace integration
- [ ] Qdrant integration
- [ ] LLM integration
- [ ] Storage integration
- [ ] Event log implementation

### Phase 4: Testing
- [ ] Unit tests (per workflow)
- [ ] Integration tests (end-to-end)
- [ ] Performance tests
- [ ] Security audit

### Phase 5: Migration
- [ ] Feature parity check
- [ ] A/B testing
- [ ] Gradual rollout
- [ ] Decommission old code

---

## DECISION LOG

### Why Replace Effect-TS?

**Effect-TS Strengths**:
- ✅ Type-safe error handling
- ✅ Composable effects
- ✅ Service injection
- ✅ Async coordination

**Effect-TS Weaknesses**:
- ❌ Large runtime (~80KB)
- ❌ Complex learning curve
- ❌ Imperative composition (not declarative)
- ❌ Can't edit business logic without code changes
- ❌ Hard to audit (too much code)

**PXYZ/OMAR Advantages**:
- ✅ Zero runtime dependencies
- ✅ Declarative workflows (XML)
- ✅ Excel-editable business logic
- ✅ 700 lines auditable runtime
- ✅ Formally verifiable (graph properties)
- ✅ Hot-reloadable workflows
- ✅ Math-native (coordinates, not code)

### Why XML for Workflows?

**Alternatives Considered**:
1. JSON - Less expressive, no schema validation
2. YAML - Indentation-sensitive, no types
3. Custom DSL - More to learn, no tooling
4. TypeScript - Back to imperative, defeats purpose

**XML Chosen Because**:
- Industry-standard schema validation (XSD)
- Expressive (predicates, templates, etc.)
- Excel-compatible (can edit workflows in spreadsheet)
- Tooling exists (validators, editors)
- Self-documenting (tags describe purpose)

---

**END OF MIGRATION GUIDE**

This document maps the entire TypeScript architecture to the new PXYZ/OMAR architecture. Use this to understand what changes, what stays, and how to migrate systematically.
