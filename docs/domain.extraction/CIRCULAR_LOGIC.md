---

## ADR-001 – ForgeStack Graph & Circular Evolution Architecture

**Title:**
Adopting a Bounded Graph Execution Model for Circular Business Logic in ForgeStack

**Status:**
Proposed – draft for ratification

**Date:**
2025-11-29

---

## 1. What this thing *is* in plain terms

From these files, the cohesive picture is:

> A CRM core where **any piece of content (note, email, task, workflow, etc.) can evolve into any other**, and the system **suggests those evolutions based on what you write and what you’ve done before**, then executes them through strongly-typed Effects programs and PXYZ coordinates.

The moving parts:

* **Tasks Engine** – canonical place where actions live (creation, status, dependencies, analytics, Kanban/List/Timeline views, recurring tasks, time tracking, team metrics). 
* **Wizard Engine** – guided flows (onboarding, project setup, automation setup) with templates, sessions, analytics, and a “smart wizard” that pre-fills from existing contacts/workflows. 
* **Workflow Engine** – multi-step automations with executions, health scoring, analytics, and accordion inbox views for ops/pipeline/work orders/marketing. 
* **Circular Evolution Engine** – maps how entities can transform into each other (note→task, note→contact, note→workflow, task→invoice, email→task, deal→workflow, etc.). 
* **Context Evolution Engine** – React hooks + heuristics + backend command that look at content and say, “hey, this looks like a task/contact/workflow, want me to convert it?” 

That *is* circular business logic + intent inference.

---

## 2. Circular CRM: the entity graph, not a pipeline

### 2.1 Entities as states on a loop

Your circular evolution map explicitly encodes **allowed transformations**:

* `note → task` (`extractActionItems`)
* `note → contact` (`extractPerson`)
* `note → document` (`formalizeContent`)
* `note → workflow` (`createProcess`)
* `task → invoice` (`billCompletedWork`)
* `task → note` (`captureOutcome`)
* `email → task` (`extractCommitments`)
* `email → note` (`saveReference`)
* `workflow → template` (`extractPattern`)
* `deal → workflow` (`createSalesProcess`) 

This is your **formal circular business logic**: entities are not stuck in one lifecycle; they move around a graph as reality unfolds.

Options on each evolution give you policy knobs:

* `preserveOriginal` – keep source intact
* `linkToOriginal` – create bidirectional links (graph history)
* `autoAssignOwner` – attach current user
* `extractDates` – pull due dates/timestamps out of text 

### 2.2 Simple vs complex evolution

* **Simple evolution** → one “bridge action” like `evolve_note_to_task`, sent through the `CommandDispatcher`, with a rich payload (`operation`, `entityId`, `options`, timestamp) and a new ID returned. 
* **Complex evolution** → orchestrated batch of `{extract, transform, create}` steps, each executed as a structured operation with metadata, then returning a `complex-evolved-…` ID and full audit trail of what happened. 

**Gap I’m filling:**
You should treat these complex ops as **first-class workflows** (literally create corresponding entries in `workflow` + `workflowexecution`) so circular evolution and workflows share the same analytics and health model instead of being parallel universes.

---

## 3. Intent inference: how the system “reads” what the user meant

### 3.1 Content heuristics (client-side, real-time)

`useContentAnalysis(content, entityType)` runs fast regex-driven heuristics and emits `ContextualEvolution` suggestions: 

* **Action items → tasks**

  * Detects phrases like “need to / should / must / have to,” “action item/todo/follow up,” and `due/by <date>`
  * Suggestion: `evolutionType: "Task Extraction"`, target `tasks`, options: `extractActionItems`, `assignToCurrentUser`, `extractDueDates`, etc.

* **People → contacts**

  * Detects full names, emails, phone numbers
  * Suggestion: `evolutionType: "Contact Creation"`, target `contacts`, options: `extractPeople`, `extractEmails`, `extractPhones`

* **Processes → workflows**

  * Detects words like “step/process/workflow/procedure” or textual outlines (`first, second, then…`, numbered lists)
  * Suggestion: `evolutionType: "Workflow Creation"`, target `workflows`, options: `extractSteps`, `preserveOrder`, `createProcess`

* **Meetings → structured notes**

  * Detects “meeting/call/conference/discussion/agenda”
  * Suggestion: `evolutionType: "Meeting Note Creation"`, target `notes`, options: `applyMeetingTemplate`, `extractParticipants`, `extractDecisions`

Plus a tiny confidence tweak by `entityType` so some sources (e.g. `document`, `note`) are “trusted” more than others. 

### 3.2 Backend analysis (tool-driven, heavier)

`useContextEvolution` sends a command like:

```ts
dispatch({
  commandId: `analyze-evolution-${Date.now()}`,
  type: 'analyze-context-evolution',
  pxyz: {
    P: 'context-evolution',
    X: 'analyze',
    Y: entityType,
    Z: timestamp,
  },
  userId,
  data: { entityType, entityId, content },
})
```

Backend responds with richer, possibly LLM-generated `suggestions`. 

This is your **“LLM as business logic reflex”**: a tool that inspects content + history and proposes transformations, but the *allowed* actions are still constrained by the circular evolution map and EffectsPrograms.

### 3.3 Context hooks (memory + preferences)

You have passive listeners that give context to those suggestions:

* `useEntityData` – subscribe to `<entityType>.loaded/updated` events to keep the UI in sync.
* `useUserPreferences` – subscribe to `preferences.loaded/updated` to adapt aggressiveness or defaults by user.
* `useEvolutionHistory` – subscribe to `evolution.history.*` events to see what transformations actually happened and use that as training signal. 

**Gap I’m filling:**
You should explicitly:

* Persist a **“suggestion accepted / rejected” log** per user + pattern.
* Feed those into backend analysis so the system **learns user-specific intent** (e.g., Eassa almost always converts “call John” to a task → raise confidence threshold and auto-create with a soft-undo instead of only suggesting).

---

## 4. Domain engines: where the inferred intent lands

### 4.1 Tasks Engine – the action sink

Capabilities (from `TasksEffectsProgram`): 

* CRUD + PXYZ-encoded operations (`createTask`, `updateTaskStatus`, `getTasks`, etc.)
* Task templates + “create from template”
* Dependencies + circular dependency detection & earliest start time calculation
* Multi-view: Kanban, Timeline/Gantt, List
* Subtasks (hierarchy, tree, descendants)
* Time tracking & time entries; roll-up into `actualHours`
* Recurring tasks and generating next occurrence (based on recurrence pattern)
* Analytics: completion rate, overdue counts, estimated vs actual hours, “productivityScore”
* Team metrics: by assignee, with completion rate + overdue counts

These are the **hard edges** where inferred intent has to become real state with strong invariants and audit (via PXYZ + typed errors).

### 4.2 Wizard Engine – guided intent shaping

Capabilities (from `WizardEffectsProgram`): 

* Wizard definitions with steps, statuses (`draft/active/paused/completed/archived`)

* Wizard sessions (`in_progress/completed/abandoned/paused`) with start/complete timestamps

* Wizard templates (categories: `onboarding/setup/configuration/migration/other`)

* CRUD for wizards, templates, sessions

* Analytics: counts, completion/abandonment rates, average completion time

* **Smart Wizard**:

  * `createSmartWizard({ wizardType, context })`
  * Pulls data from `contact` and `workflow` to pre-fill fields and reduce cognitive load
  * Uses `generateStepsForType` to build minimal, context-aware 3–5 step flows for things like `client_onboarding`, `project_setup`, `automation_setup`

* `resumeWizardSession` – resume from where you left off, with progress percentage

* `validateWizardStep` – checks required fields and returns structured validation errors

This is the **“conversation into structure”** bridge. Wizard is where vague intent gets turned into structured entities (contact, workflow, tasks) with minimal questioning.

### 4.3 Workflow Engine – the automation spine

Capabilities (from `WorkflowsEffectsProgram`): 

* Workflow definitions: trigger type (`manual/scheduled/event/webhook`), steps, tags
* Executions: `workflowexecution` rows with status, input/output, errors
* Templates: reusable automation blueprints
* Execution via `FlowExecutor` → turns steps into nodes + connections (`nodes[]`, `connections[]`) and runs them
* Analytics: total/active workflows, total/completed/failed executions, success rate
* Filtering + search by status, triggerType, tags
* Health: compute health score + status (`healthy/warning/critical`), issues, and “detailed workflow health” with blocked/overdue steps and risk factors
* UX features like **accordion inbox** grouping workflows by category (operations/pipeline/workOrder/marketing) for mental clarity

This is where **repeated evolutions solidify into automations**: if you frequently do “note → task → workflow → invoice”, you eventually canonize that as a workflow template.

---

## 5. End-to-end: how “business logic infers itself” in your CRM

Putting it together as a loop:

1. **User expresses intent in the wild**

   * Types a note, saves an email, edits a document, updates a workflow description.

2. **Fast, local heuristics fire**

   * `useContentAnalysis` detects action items / people / processes / meetings and generates `ContextualEvolution` suggestions (e.g., “Convert to task”, “Create contact”). 

3. **Optional deep analysis runs**

   * `useContextEvolution` sends a PXYZ-annotated command (`context-evolution/analyze/<entityType>`) to backend for LLM-based pattern recognition across content + evolution history. 

4. **Suggestions surface in UI**

   * User sees small, context-aware prompts: “Turn this into a task?”, “Add John Doe as a contact?”, “Extract this into a workflow?”

5. **User accepts → CircularEvolutionEngine executes**

   * For simple cases: a single “bridge” command `evolve_note_to_task` executed via CommandDispatcher, honoring options (preserve/link/assign owner/extract dates). 
   * For complex cases: orchestrated `{extract, transform, create}` batch that may fan out into multiple entities and link them.

6. **Domain engines persist & analyze**

   * Tasks/Wizards/Workflows EffectsPrograms run, embedding PXYZ coordinates, updating analytics (productivity, workflow health, wizard completion rates).

7. **Patterns become templates**

   * Repeated similar workflows → `workflow → template` (`extractPattern`) and PRD features like “createWorkflowFromTemplate” turn live flows into reusable blueprints.
   * Wizard templates for common onboarding/automation flows become more refined over time. 

8. **History + preferences tighten the loop** *(gap I’d formalize)*

   * Evolution history + preference signals modulate thresholds and defaults, making the system feel more “telepathic” over time while staying constrained by your graph + PXYZ guardrails.

That’s the **circular, intent-driven business logic**: not a static pipeline, but a living graph where intent → suggestion → evolution → analytics → templates → sharper suggestions.

---

## 6. What I’d explicitly add (filling the gaps)

Stuff the code strongly implies but doesn’t yet formalize:

1. **Evolution Preference Model**

   * Table like `evolution_preferences`: `{ userId, sourceType, targetType, patternSignature, acceptRate, lastUsed }`
   * Every suggestion gets logged as accepted/rejected; backend uses it to:

     * auto-accept high-confidence, high-acceptRate suggestions
     * suppress low-value suggestions per user/team

2. **Evolution Policy Settings per workspace**

   * Modes: `"suggest-only" | "auto-with-undo" | "auto-for-high-confidence"`
   * Tied into `useUserPreferences` so teams can choose how aggressive the system is.

3. **Template Mining from History**

   * Background process that:

     * scans workflows & circular evolutions for frequent patterns
     * proposes wizard/workflow templates (“We’ve seen this 12 times; want to make it a template?”)

4. **Unified Evolution Timeline**

   * Single view that merges:

     * entity change history
     * wizard sessions
     * workflow executions
     * evolution operations (note→task, task→invoice, etc.)
   * This turns the circular logic into an explorable “story” per contact/deal/project (big for audit + cognition).

5. **“Intent Surface” in ForgeStack UI**

   * A dedicated dock/panel that always shows:

     * top suggested evolutions
     * associated confidence and rationale
     * 1-click actions (“Do it”, “Never suggest this again”, “Show steps”)

---

### 1. Context

ForgeStack is a neurodivergent-friendly CRM/OS with:

* **Circular business logic**: any entity (note, task, contact, workflow, deal, doc) can evolve into another (note→task, email→task, deal→workflow, etc.).
* **EffectsPrograms** per domain (Tasks, Deals, Notes, Workflows, Wizard, Business) that encapsulate side effects and analytics.
* **Evolution & Wizard engines** that:

  * infer intent from content and behavior,
  * suggest transformations or automations,
  * and execute them via typed commands.

Until now, orchestration has been a mixture of:

* ad-hoc command handlers,
* hard-coded flows in TypeScript,
* UI-driven wizards.

We now have a **PXYZ-style workflow system**: graphs as data, compiled to a bounded, auditable runtime; opcodes for external effects; and a self-learning loop (EvolutionEffectsProgram / WizardEffectsProgram / WorkflowsEffectsProgram).

We need a single decision:

> How do we fuse the PXYZ graph model with ForgeStack’s circular business logic so that **all non-trivial automation and evolution runs through a single, safe, inspectable graph system** instead of scattered imperative code?

---

### 2. Decision

We will:

1. **Adopt a bounded graph execution model as the orchestration core for ForgeStack**, inspired by PXYZ:

   * Business logic is represented as **graphs (nodes + edges)**, not imperative code.
   * Graphs are validated and compiled to a binary/IR; a small runtime just walks the graph.
   * All I/O goes through typed **EffectsPrograms** and/or a thin `io_call`-style host.

2. **Model circular business logic as a first-class graph layer**, not just helper functions:

   * Every allowed entity evolution (note→task, email→task, deal→workflow, etc.) is represented as:

     * **Node types**: `EvolutionCandidate`, `EvolutionDecision`, `EvolutionApply`, `EvolutionLink`, `EvolutionAudit`.
     * **Edges** that encode allowed transformations with constraints (who, when, under what conditions).
   * Simple evolutions (`note→task`) are **single-step graphs**; complex ones (note→tasks+workflow+invoice) are **small workflows** with multiple nodes.

3. **Use a PXYZ-like coordinate system and “graph as capability” model**:

   * For orchestration, commands get coordinates like:

     * `P = 'evolution' | 'tasks' | 'wizard' | 'workflows' | 'business'`
     * `X = 'apply' | 'analyze' | 'route' | 'schedule' | 'summarize'`
     * `Y = entity/context (‘note’, ‘task’, ‘deal’, etc.)`
     * `Z = timeline / version / environment`
   * A given graph + coordinate pair is the **capability**. LLMs / tools can select a graph, but **cannot extend capability** without going through compilation + approval.

4. **Route all higher-order behavior through the graph runtime; EffectsPrograms become op-nodes, not orchestrators**:

   * Tasks/Deals/Notes/Workflows/Wizard/Business EffectsPrograms:

     * remain the **only place** where writes & integration logic live (“host ops”),
     * but no longer orchestrate multi-step sequences directly.
   * Orchestration steps (e.g., “extract actions from note → create tasks → link back → update workflow health”) are encoded as graphs that:

     * call EffectsProgram operations,
     * branch via predicates,
     * emit events and analytics.

5. **Standardize “batteries included” on top of the graph model**:

   * **CircularEvolutionEngine** – defines allowed transitions and exposes:

     * simple evolution graphs (one-hop transforms),
     * complex evolution templates (multi-step flows).
   * **ContextEvolutionEngine** – provides:

     * content heuristics (local, cheap),
     * backend/LLM analysis (deep, history-aware),
     * and emits **suggested evolution graphs** or updates to existing graphs.
   * **Learning loop**:

     * event log → pattern mining (EvolutionEffectsProgram) → candidate rules → wizard approval (WizardEffectsProgram) → compiled graphs (WorkflowsEffectsProgram) → execution → telemetry → back to log.

6. **Enforce safety by construction**:

   * Graphs must be **acyclic**, with bounded:

     * steps, predicate evaluations, call depth, stack depth.
   * Irreversible actions (sending email/SMS, deleting, billing) are:

     * executed only from nodes flagged `IRREVERSIBLE`,
     * reachable only through nodes with `REQUIRES_HUMAN` / `IS_CONFIRMED` constraints.

---

### 3. Alternatives Considered

1. **Keep imperative orchestration in TypeScript + EffectsPrograms**

   * Pros: familiar, flexible, no compiler/runtime work.
   * Cons:

     * harder to audit and visualize flows;
     * more surface for AI agents to generate dangerous code;
     * behavior scattered in many files; difficult to enforce global safety rules.

2. **Use off-the-shelf workflow engine (e.g., temporal.io, n8n, etc.)**

   * Pros: mature tooling, built-in retries, UI builders.
   * Cons:

     * mismatch with ForgeStack’s **circular entity model** and neurodivergent UX constraints;
     * less control over bounded execution / custom predicates / PXYZ semantics;
     * more infrastructure weight than needed.

3. **“Pure LLM” approach for automation (toolformer-style)**

   * Pros: extremely flexible, minimal explicit modeling.
   * Cons:

     * no hard safety boundaries;
     * no deterministic graph to inspect or version;
     * very hard to prove behavior to compliance people.

We reject these for the core orchestration in favor of a **typed, bounded, inspectable graph layer** tailored to ForgeStack’s circular logic.

---

### 4. Consequences

#### Positive

* **Single orchestration brain**:
  All cross-entity flows (notes→tasks, deals→workflows, wizards→tasks, etc.) are visible as graphs.
* **Safer AI integration**:
  LLMs can propose graphs, but only approved, validated graphs run. The model never “owns the instruction pointer” directly.
* **Better UX for ND brains**:
  Circular business logic is explicit, visualizable, and can be surfaced as “this is how your work actually flows” diagrams.
* **Powerful “free batteries”**:

  * Templates;
  * Learning from event history;
  * Suggestion + approval loops;
  * Easy simulation/shadow runs;
  * Unified audit across all domains.

#### Negative / Risks

* **Increased up-front complexity**:
  Devs must learn the graph/IR mental model and write graph definitions, not just imperative functions.
* **Need for tooling**:
  We must build decent:

  * graph inspector,
  * debugger/simulator,
  * metrics panels,
  * and schema for graph storage.
* **Migration risk**:
  Existing imperative orchestration will need to be slowly migrated into graphs; partial duplication may exist during transition.

---

### 5. Implementation Plan (High-level)

**Phase A – Core Graph Substrate**

* Define ForgeStack graph schema (nodes, edges, predicates, metadata).
* Implement:

  * compiler/validator (syntactic, semantic, pragmatic),
  * small runtime with bounded execution.
* Add host adapter for EffectsPrograms (each domain op exposed as graph op-node).

**Phase B – Circular Evolution Integration**

* Port `CircularEvolutionEngine` operations into graph definitions:

  * simple evolutions → one-hop graphs;
  * complex evolutions → multi-node flows.
* Wire `ContextEvolutionEngine` to:

  * produce suggested evolution graphs or graph patches,
  * log suggestions & outcomes for learning.

**Phase C – Learning & Wizard Loop**

* Integrate event log with evolution/workflow execution events.
* Implement:

  * pattern mining → candidate graphs,
  * Wizard UI → approve/edit/reject,
  * graph compilation & hot reload.

**Phase D – Hardening & UX**

* Build:

  * graph inspector (Mermaid or similar),
  * unified evolution/automation timeline per entity,
  * safety dashboards (irreversible actions, health).

---

## PRD – ForgeStack Intent & Circular Evolution Engine

**Product Name (slice):**
Intent & Circular Evolution Engine (ICE)

**Scope:**
A vertical inside ForgeStack that turns **what users write and do** into **suggested evolutions and automations**, powered by the new graph system and circular business logic.

---

### 1. Overview

The Intent & Circular Evolution Engine (ICE) is the brain that:

* Watches **content** (notes, emails, documents, comments),
* Watches **behavior** (what tasks, workflows, and wizards users actually perform),
* Infers **what users meant**, and
* Converts that into:

  * subtle, low-friction suggestions (convert this to a task? contact? workflow?),
  * or full automations (when X happens, do Y and Z),
  * encoded as safe graphs executed by the ForgeStack graph runtime.

It’s the piece that makes ForgeStack feel like it “gets” your workflow without you being a workflow designer.

---

### 2. Goals / Non-Goals

#### 2.1 Goals

1. **Capture intent from natural behavior**

   * Infer “this is a task,” “this is a contact,” “this is a process,” from content and repeated patterns.
2. **Reduce cognitive load**

   * Replace “build a workflow” with “accept/reject this suggestion.”
3. **Centralize orchestration in graphs**

   * All non-trivial evolutions & automations go through the graph substrate using EffectsPrograms as op-nodes.
4. **Learn and personalize**

   * Use evolution history and user preferences so suggestions become more accurate and less noisy over time.
5. **Stay provably safe**

   * Bounded graphs, hard constraints on irreversible actions, full audit of what was suggested, accepted, and executed.

#### 2.2 Non-Goals

* Building a generic flowchart editor for arbitrary business processes (focus is CRM-ish flows).
* Replacing human judgment in edge cases; ICE should ask for help, not guess dangerously.
* Building predictive scoring models from scratch (we can consume scores as fields; not our lane here).

---

### 3. User Journeys

#### 3.1 “Turn this into something for me” (micro-evolution)

* User types a note:
  “Call John on Tuesday about renewing his contract. Also follow up with Sarah on the onboarding docs.”
* ICE:

  * Client heuristics detect action-y phrases.
  * Suggestion bubble appears:

    * “Create 2 tasks?”
  * User clicks “Yes”.
* Graph executed:

  * Node: `ExtractActionsFromNote` → tasks engine → create 2 tasks.
  * Node: `LinkTasksToNote` → link tasks back.
* Result:

  * Tasks created, linked, scheduled.
  * Evolution recorded to evolution history and analytics.

#### 3.2 “Remember how we handle this?” (pattern → flow)

* New sales rep asks in chat:

  * “How do we handle a lead that’s over 50k from a referral?”
* ICE:

  * Queries event log + existing graphs.
  * Synthesizes answer:

    * “Typically we: 1) assign to senior rep, 2) create an intro call task, 3) move deal into ‘High Priority’ pipeline. Want me to package this as a reusable workflow?”
* User clicks “Yes”.
* Graph created:

  * Trigger: `LeadCreated` with conditions (`amount > 50k`, `source = referral`).
  * Actions: `AssignOwner(senior)`, `CreateTask(intro call)`, `UpdateStage(High Priority)`.
* Wizard shows a preview; user tweaks owners/timing and approves.

#### 3.3 “This keeps happening; stop making me do it manually” (learning loop)

* Over a month:

  * User often converts inbound emails from a certain address pattern into tasks.
* Evolution history shows:

  * 20+ `email→task` evolutions with similar conditions.
* ICE:

  * Learning job detects this pattern.
  * Suggestion in automations pane:

    * “Whenever we get an email from [support@client.com](mailto:support@client.com) that mentions ‘urgent’ in the subject, automatically create a task assigned to you. Automate this?”
* User:

  * Opens wizard → sees the graph and sample events → sets “only on weekdays” constraint → approves.
* From then on:

  * Emails matching pattern auto-create tasks with an undo (“Task created from this email. Undo?”).

---

### 4. Functional Requirements

#### 4.1 Content Analysis (Client-Side Heuristics)

* Detect action items, people, processes, and meetings from text.
* Emit `ContextualEvolution` suggestions:

  * `type`: `task_from_note`, `contact_from_text`, `workflow_from_outline`, etc.
  * `confidence`: low/medium/high.
  * `options`: `extractDates`, `preserveOriginal`, `linkToOriginal`, `autoAssignOwner`.
* Integrate with UI:

  * Small, non-intrusive suggestion bubbles in editors and reading views.
  * Respect user preferences for hint frequency / auto-create behavior.

#### 4.2 Backend Analysis (ContextEvolutionEngine)

* Command: `context-evolution/analyze` with:

  * `entityType`, `entityId`, `content`, `userId`, `recentEvolutionHistory`.
* Returns:

  * normalized suggestions (same shape as client, potentially with more context),
  * potential candidate graphs for multi-step flows.
* Must:

  * only propose actions that exist in the CircularEvolution map,
  * never emit “raw code”; all proposals expressed in graph terms.

#### 4.3 Circular Evolution Engine

* Maintain a map of allowed transitions:

  * `NOTE → TASK`, `NOTE → CONTACT`, `NOTE → WORKFLOW`, `EMAIL → TASK`, `TASK → INVOICE`, `DEAL → WORKFLOW`, etc.
* For each:

  * Provide a **simple evolution graph** (1–3 nodes) representing the canonical path.
  * Provide **complex evolution templates** where relevant with multiple entities created/linked.
* Public API:

  * `proposeEvolution(sourceEntity, targetType, options) → graphDefinition`
  * `executeEvolution(graphId, sourceEntityId, options) → { newEntities[], links[] }`

#### 4.4 Learning & Suggestion Loop

* Log every evolution event:

  * `who`, `when`, `sourceType`, `targetType`, `patternSignature`, `options`, `accepted/rejected`.
* Periodically (or triggered by volume):

  * mine logs for patterns:

    * repeated `source→target` evolution under similar conditions,
    * repeated sequences of evolutions.
* Generate:

  * `CandidateAutomation` records with:

    * conditions, actions (in abstract form),
    * support/confidence,
    * example events,
    * proposed graph template.
* Show in “Suggested Automations” panel and optionally via chat prompts.

#### 4.5 Wizard Integration

* Wizard flows for:

  * reviewing & editing candidate automations,
  * configuring options (owners, timing, scope),
  * seeing example traces.
* Actions:

  * Approve → compile to graph + activate.
  * Edit → update conditions/actions before compile.
  * Reject → record as “don’t suggest again” for that pattern.

#### 4.6 Graph Execution

* All ICE-driven automations run via the graph runtime:

  * Graph definitions stored and versioned.
  * Compiler validates:

    * acyclicity,
    * bounded size/steps,
    * enforcement of irreversible action constraints.
  * Runtime:

    * traverses graphs,
    * calls EffectsPrograms for domain writes,
    * logs `GraphExecuted`, `NodeExecuted`, `Error` events.

---

### 5. Data Model (High Level)

You don’t need exact schemas here, just shape:

* `EvolutionSuggestion`

  * `id`, `userId`, `sourceEntityType`, `sourceEntityId`, `targetType`, `options`, `confidence`, `createdAt`, `source` (`client`, `backend`), `status` (`shown`, `accepted`, `dismissed`).

* `EvolutionEvent`

  * `id`, `userId`, `sourceType`, `targetType`, `sourceEntityId`, `targetEntityIds[]`, `graphId`, `createdAt`, `options`, `resultStatus`.

* `CandidateAutomation`

  * `id`, `patternSignature`, `conditions`, `actions`, `support`, `confidence`, `sampleEvents[]`, `graphTemplate`, `status`.

* `GraphDefinition`

  * `id`, `version`, `type` (`evolution`, `workflow`, `wizard_flow`), `nodes[]`, `edges[]`, `metadata` (PXYZ coordinates, labels), `status`.

* `UserEvolutionPreference`

  * `userId`, `patternSignature`, `preference` (`always_auto`, `ask`, `never`), `lastUpdated`.

---

### 6. Technical Constraints & Integration

* ICE must:

  * Use **EffectsPrograms** for all domain writes (Tasks, Deals, Notes, Workflows, Wizard, Business).
  * Respect the **graph runtime boundaries**:

    * no unbounded loops,
    * no direct DB access from orchestration code.
  * Integrate with **Dock/Crown/Pillar** UI structure:

    * suggestions surface mainly in Dock panels, editors, and relevant views.

* ICE **MUST NOT**:

  * run irreversible actions (email, SMS, deletion, billing) without:

    * human confirmation (for now),
    * or explicit workspace policies allowing scoped auto-actions with undo.

---

### 7. Open Questions / Risks

* How aggressive should auto-creation be by default?

  * Per-user vs per-workspace defaults; ND users often prefer consistency but low interruption.
* How do we expose graphs visually in a way that’s powerful but not cognitively overwhelming?

  * Likely: abbreviated “story views” instead of node-salad.
* How much of the learning loop is generic vs domain-specific?

  * Some patterns are CRM universal (followups), others are workspace-specific.
* Where exactly do we plug in LLMs?

  * Text analysis,
  * Suggestion explanation,
  * candidate graph synthesis,
  * answering “how do we handle X?” from history.

---

This gives you:

* An **ADR** that says: “Yes, we’re committing to graphs + circular business logic as the orchestration spine.”
* A **PRD** that scopes the Intent & Circular Evolution Engine as a first-class product slice inside ForgeStack, integrated with that graph substrate.

You can drop the ADR into `/docs/adr/ADR-001-graph-circular-evolution.md` and the PRD into `/docs/prd/PRD-ICE-forgestack.md` and iterate from there.
