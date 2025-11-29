# OMAR Quick Reference: Feature → PXYZ Mapping

> **For AI Agents**: When given a feature request, use this guide to map it to PXYZ operations.

---

## The Translation Process

### Step 1: Identify the Entity (P-Axis)

**Question**: What is the user asking about?

| User says... | P-Axis Entity |
|-------------|---------------|
| "create/view/search contacts" | `P=contact` |
| "add/complete/assign tasks" | `P=task` |
| "start/track workflows" | `P=workflow` |
| "send/read/reply emails" | `P=email` |
| "write/save notes" | `P=note` |
| "create/share documents" | `P=document` |
| "generate/send invoices" | `P=invoice` |
| "upload/attach files" | `P=file` |
| "client portal actions" | `P=portal` |

### Step 2: Identify the Operation (X-Axis)

**Question**: What does the user want to DO with the entity?

| User wants to... | X-Axis Operation | Op Code |
|-----------------|------------------|---------|
| **CREATE** new entity | `X=create` | `0xNN00` |
| **VIEW/READ** existing | `X=read` | `0xNN01` |
| **UPDATE** existing | `X=update` | `0xNN02` |
| **DELETE/ARCHIVE** | `X=delete` | `0xNN03` |
| **LIST ALL** | `X=list` | `0xNN04` |
| **SEARCH/FILTER** | `X=search` | `0xNN05` |
| **TRANSFORM** state | `X=state_transition` | `0xNN10+` |
| **AI ENHANCE** | `X=ai_*` | `0xNN30+` |
| **CONVERT** to another type | `X=to_*` | `0xNN20+` |

Where `NN` = entity prefix:
- Contact: `01`
- Task: `02`
- Email: `05`
- Note: `07`
- Document: `06`
- Invoice: `08`
- Workflow: `04`
- File: `10`
- Portal: `09`

### Step 3: Identify Constraints (Y-Axis)

**Question**: What rules/permissions/filters apply?

| Constraint Type | Predicate Pattern | Example |
|----------------|-------------------|---------|
| **Access Control** | `can_view_*`, `can_edit_*` | User must own contact OR be admin |
| **Lifecycle Rules** | `can_promote_to_*`, `is_valid_*` | Contact can only become customer if engaged |
| **Data Quality** | `is_duplicate`, `needs_enrichment` | Detect similar contacts before creating |
| **Health Checks** | `is_overdue`, `is_at_risk` | Task is overdue if due_date < now |
| **Business Logic** | `should_*`, `requires_*` | Workflow requires approval before proceeding |

### Step 4: Identify Events (Z-Axis)

**Question**: What truth do we append to the log?

| User Action | Event Appended |
|------------|----------------|
| Creates contact | `contact.created` |
| Updates task state | `task.state_changed` |
| Sends email | `email.sent` |
| Completes workflow | `workflow.completed` |
| Uploads file | `file.uploaded` |
| Approves document | `document.approved` |

---

## Common Feature Patterns

### Pattern 1: Basic CRUD

**Request**: "I want to create a new contact"

```
P = contact
X = create (0x0100)
Y = { name, email, company } ← input data
Z = now

Workflow:
  1. validate_input (transform)
  2. check_duplicates (auth: is_duplicate)
  3. append_event (external: 0x0910)
     → event: contact.created
  4. enrich_from_signature (external: 0x0111)
  5. render_confirmation (render)
  6. success (terminal)
```

**Request**: "Show me all my contacts"

```
P = contact
X = list (0x0104)
Y = { workspace_id: $token.workspace_id }
Z = now

Workflow:
  1. auth_check (auth: is_authenticated)
  2. query_contacts (external: 0x0104)
  3. filter_by_access (auth: can_view_contact)
  4. sort_by_momentum (transform)
  5. render_list (render)
  6. success (terminal)
```

### Pattern 2: Search with AI

**Request**: "Search for contacts related to Q4 project"

```
P = contact
X = search (0x0105)
Y = { query: "Q4 project", limit: 20 }
Z = now

Workflow:
  1. validate_search (transform)
  2. auth_check (auth: is_authenticated)
  3. semantic_search (external: 0x0700 Qdrant)
  4. structured_search (external: 0x0105)
  5. merge_results (transform)
  6. filter_by_access (auth: can_view_contact)
  7. compute_momentum (external: 0x0122)
  8. render_results (render)
  9. success (terminal)
```

### Pattern 3: State Transition

**Request**: "Mark this task as done"

```
P = task
X = state_transition (0x0210)
Y = { task_id, new_state: "done" }
Z = now

Workflow:
  1. load_task (external: 0x0201)
  2. check_can_edit (auth: can_edit_task)
  3. validate_transition (auth: can_mark_done)
  4. append_state_event (external: 0x0910)
     → event: task.state_changed
  5. check_billable (predicate: is_billable)
  6. offer_invoice (external: 0x0800 LLM)
     ↓ if user accepts
  7. create_invoice (external: 0x0822)
  8. notify_watchers (external: 0x0300)
  9. success (terminal)
```

### Pattern 4: Circular Evolution

**Request**: "This task is too complex, turn it into a workflow"

```
P = task
X = to_workflow (0x0221)
Y = { task_id }
Z = now

Workflow:
  1. load_task (external: 0x0201)
  2. check_complexity (auth: should_promote_to_workflow)
  3. llm_breakdown (external: 0x0800)
     → LLM generates phases and steps
  4. create_workflow (external: 0x0400)
     → event: workflow.created
  5. create_phases (external: 0x0420)
  6. create_steps (external: 0x0431)
  7. link_to_task (external: 0x0202)
     → event: task.updated
  8. render_preview (render)
  9. success (terminal)
```

### Pattern 5: AI-Enhanced Operation

**Request**: "Draft a follow-up email to this contact"

```
P = email
X = draft_reply (0x0533)
Y = { contact_id, context: "follow up" }
Z = now

Workflow:
  1. load_contact (external: 0x0101)
  2. load_email_history (external: 0x0120 timeline)
  3. load_preferences (transform)
     → preferred_tone, best_contact_time
  4. llm_draft (external: 0x0800)
     Prompt:
       Draft follow-up email to {{contact.name}}
       Context: {{email_history}}
       Tone: {{preferred_tone}}
  5. check_tone (external: 0x0801 classify)
  6. render_draft (render)
  7. success (terminal)
```

### Pattern 6: Wizard-Generated Structure

**Request**: "Set up a client onboarding workflow"

```
P = workflow
X = create (0x0400)
Y = { wizard: true }
Z = now

Workflow:
  1. wizard_questions (transform)
     Q1: Who's the client? → contact_id
     Q2: What's the timeline? → due_date
     Q3: Who's the team? → participants
     Q4: Use template? → template_id
  
  2. load_template (external: 0x0401)
     OR
  3. llm_generate (external: 0x0800)
     Prompt:
       Generate client onboarding workflow for:
       Client: {{contact.name}}
       Timeline: {{due_date}}
       Return JSON phases/steps
  
  4. create_workflow (external: 0x0400)
     → event: workflow.created
  5. create_phases (loop: external 0x0420)
  6. create_steps (loop: external 0x0431)
  7. assign_participants (loop: external 0x0402)
  8. render_preview (render)
  9. offer_start (signal)
  10. success (terminal)
```

### Pattern 7: Health Monitoring

**Request**: "Check the health of this workflow"

```
P = workflow
X = health_check (0x0440)
Y = { workflow_id }
Z = now

Workflow:
  1. load_workflow (external: 0x0401)
  2. load_all_steps (external: 0x0911 event query)
  3. calculate_overdue (transform)
     → count steps where due_date < now
  4. calculate_blocked (transform)
     → count steps where status = blocked
  5. calculate_inactivity (transform)
     → days_since_last_event
  6. compute_health_score (transform)
     → score = progress - penalties
  7. llm_recommendations (external: 0x0800)
     IF health_score < 70
  8. notify_owner (external: 0x0300)
     IF health_score < 50
  9. return_health (terminal)
```

---

## Decision Tree: Feature → PXYZ

```
User Feature Request
        ↓
    [Is it about creating/viewing/editing data?]
        ↓ YES
    Identify Entity (P)
        ↓
    Map to CRUD operation (X)
        │
        ├→ Create: 0xNN00
        ├→ Read: 0xNN01
        ├→ Update: 0xNN02
        ├→ Delete: 0xNN03
        ├→ List: 0xNN04
        └→ Search: 0xNN05
        ↓
    Define constraints (Y predicates)
        ↓
    Append event (Z)
        ↓
    DONE: Build workflow XML
    
    [Is it about changing state/status?]
        ↓ YES
    Identify Entity (P)
        ↓
    State transition operation (X = 0xNN10+)
        ↓
    Define lifecycle predicates (Y)
        ↓
    Append state_changed event (Z)
        ↓
    Check for circular evolution opportunities
        ↓
    DONE: Build workflow XML
    
    [Is it about AI enhancement?]
        ↓ YES
    Identify Entity (P)
        ↓
    AI operation (X = 0xNN30+ or 0x0800 LLM)
        ↓
    Define input context (Y)
        ↓
    No event needed (unless AI creates entity)
        ↓
    DONE: Build workflow XML
    
    [Is it about integrating systems?]
        ↓ YES
    Identify source Entity (P)
        ↓
    External operation (X = external, op=0xNNNN)
        ↓
    Define IO adapter handler
        ↓
    Append integration event (Z)
        ↓
    DONE: Build workflow + IO handler
```

---

## XML Workflow Template

Use this template for ANY feature:

```xml
<workflow id="FEATURE_NAME">
  <entry p="ENTITY" x="OPERATION" node="start_node"/>
  
  <nodes>
    <!-- Step 1: Validate Input -->
    <node id="validate" kind="transform">
      <validate>
        <field name="FIELD" type="TYPE" required="BOOL"/>
      </validate>
    </node>
    
    <!-- Step 2: Check Authorization -->
    <node id="auth" kind="auth">
      <require predicate="PREDICATE_NAME"/>
    </node>
    
    <!-- Step 3: Execute Operation -->
    <node id="execute" kind="external" op="0xNNNN">
      <input>
        <map field="OUTPUT" from="$input.INPUT"/>
      </input>
    </node>
    
    <!-- Step 4: Optional AI Enhancement -->
    <node id="ai_enhance" kind="external" op="0x0800">
      <when>
        <predicate ref="SHOULD_ENHANCE"/>
      </when>
      <llm_prompt>
        PROMPT TEXT HERE
      </llm_prompt>
    </node>
    
    <!-- Step 5: Append Event -->
    <node id="log_event" kind="external" op="0x0910">
      <event>
        <type>ENTITY.EVENT_TYPE</type>
        <data>
          <field name="FIELD" value="$VALUE"/>
        </data>
      </event>
    </node>
    
    <!-- Step 6: Render Response -->
    <node id="render" kind="render">
      <template ref="TEMPLATE_NAME"/>
    </node>
    
    <!-- Step 7: Success -->
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate" to="auth">
      <when><always/></when>
    </edge>
    <edge from="auth" to="execute">
      <when><always/></when>
    </edge>
    <edge from="execute" to="ai_enhance">
      <when><predicate ref="SHOULD_ENHANCE"/></when>
    </edge>
    <edge from="execute" to="log_event">
      <when><not><predicate ref="SHOULD_ENHANCE"/></not></when>
    </edge>
    <edge from="ai_enhance" to="log_event">
      <when><always/></when>
    </edge>
    <edge from="log_event" to="render">
      <when><always/></when>
    </edge>
    <edge from="render" to="success">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

---

## Anti-Patterns to Avoid

### ❌ DON'T: Write imperative code

```typescript
// WRONG - This is NOT PXYZ
function createContact(data) {
  const contact = {
    id: uuid(),
    name: data.name,
    email: data.email,
    created_at: new Date()
  };
  
  db.contacts.insert(contact);
  return contact;
}
```

### ✅ DO: Define as workflow

```xml
<!-- RIGHT - This is PXYZ -->
<workflow id="contact_create">
  <entry p="contact" x="create" node="validate"/>
  
  <nodes>
    <node id="validate" kind="transform">
      <validate>
        <field name="name" type="string" required="true"/>
        <field name="email" type="string" required="true"/>
      </validate>
    </node>
    
    <node id="append_event" kind="external" op="0x0910">
      <event>
        <type>contact.created</type>
        <data>
          <field name="id" value="$uuid()"/>
          <field name="name" value="$input.name"/>
          <field name="email" value="$input.email"/>
        </data>
      </event>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate" to="append_event"><when><always/></when></edge>
    <edge from="append_event" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### ❌ DON'T: Store computed state

```typescript
// WRONG - State is a lie
interface Contact {
  id: string;
  name: string;
  health_score: number; // ❌ Don't store this
  stage: string; // ❌ Don't store this
}
```

### ✅ DO: Compute from events

```xml
<!-- RIGHT - Compute on demand -->
<workflow id="contact_health_score">
  <entry p="contact" x="health" node="load_events"/>
  
  <nodes>
    <node id="load_events" kind="external" op="0x0911">
      <query>
        <filter field="entity_id" value="$input.contact_id"/>
        <filter field="type" in="email.sent,email.received,meeting.held"/>
        <filter field="timestamp" gte="$now-90days"/>
      </query>
    </node>
    
    <node id="calculate" kind="transform">
      <compute>
        <var name="score" value="calculate_health($events)"/>
      </compute>
    </node>
    
    <node id="return" kind="terminal" status="200">
      <return field="score"/>
    </node>
  </nodes>
</workflow>
```

---

## Quick Reference Tables

### Entity Prefixes
| Entity | Prefix |
|--------|--------|
| Contact | `01` |
| Task | `02` |
| Workflow | `04` |
| Email | `05` |
| Document | `06` |
| Note | `07` |
| Invoice | `08` |
| Portal | `09` |
| File | `10` |

### Common Operation Suffixes
| Suffix | Meaning |
|--------|---------|
| `00` | create |
| `01` | read |
| `02` | update |
| `03` | delete |
| `04` | list |
| `05` | search |
| `10-19` | state transitions |
| `20-29` | conversions |
| `30-39` | AI enhancements |
| `40-49` | health/analytics |

### Node Kinds
| Kind | Code | Purpose |
|------|------|---------|
| `transform` | 0 | Data validation, computation |
| `external` | 1 | Call IO adapter (DB, API, etc) |
| `render` | 2 | Generate HTML response |
| `signal` | 3 | Update Datastar signals |
| `auth` | 4 | Check predicate permissions |
| `terminal` | 5 | End traversal, return status |

---

## Examples by Domain

### Contact Examples
- Create contact: `P=contact, X=create (0x0100)`
- Search contacts: `P=contact, X=search (0x0105)`
- Promote to customer: `P=contact, X=stage_transition (0x0113)`
- Calculate health: `P=contact, X=health (0x0121)`

### Task Examples
- Create task: `P=task, X=create (0x0200)`
- Change state: `P=task, X=state_transition (0x0210)`
- Generate subtasks: `P=task, X=add_subtask (0x0212)`
- Convert to workflow: `P=task, X=to_workflow (0x0221)`

### Email Examples
- Send email: `P=email, X=send (0x0502)`
- Draft reply: `P=email, X=draft_reply (0x0533)`
- Summarize thread: `P=email, X=summarize (0x0530)`
- Create task from email: `P=email, X=to_task (0x0520)`

### Workflow Examples
- Create from wizard: `P=workflow, X=create (0x0400)`
- Start workflow: `P=workflow, X=start (0x0410)`
- Complete step: `P=workflow, X=step_complete (0x0422)`
- Health check: `P=workflow, X=health_check (0x0440)`

---

## The Golden Rule

**Every feature request becomes a coordinate query:**

```
/pxyz/{entity}/{operation}?{constraints}&z={timestamp}
```

**There is no other way to build features in OMAR.**

If you can't map it to PXYZ, you don't understand the requirement yet.
