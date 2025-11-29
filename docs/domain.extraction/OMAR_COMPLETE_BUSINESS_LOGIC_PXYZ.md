# OMAR Complete Business Logic - PXYZ Coordinate Specification

> **Purpose**: Definitive tech-stack agnostic specification of all business logic as PXYZ coordinate operations.
> **Audience**: AI agents building the OMAR system.
> **Rule**: Everything is a coordinate query. No imperative code. Only events, predicates, and graph traversals.

---

## Core Axiom Reminder

```
PXYZ(P, X, Y, Z) where:
  P = Entity Type (what)
  X = Operation (how)
  Y = Constraints/Context (rules)
  Z = Temporal/Events (when)

Every business operation is:  /pxyz/{P}/{X}?{Y}&z={Z}
```

**State Does Not Exist**:
```
State = Project(EventLog, Predicates, CurrentTime)
```

All "data" you think you see is a VIEW computed from events + constraints.

---

## PART 1: CONTACTS DOMAIN AS PXYZ

### P-Axis: Contact Entities

```xml
<schemas>
  <schema id="contact">
    <field name="id" type="uuid" required="true"/>
    <field name="name" type="string" required="true"/>
    <field name="email" type="string" required="true"/>
    <field name="phone" type="string"/>
    <field name="company_id" type="uuid"/>
    <field name="stage" type="enum" values="lead,prospect,customer,partner,dormant"/>
    <field name="owner_id" type="uuid"/>
    <field name="created_at" type="timestamp"/>
    <field name="momentum_score" type="computed"/>
  </schema>
  
  <schema id="organization">
    <field name="id" type="uuid" required="true"/>
    <field name="name" type="string" required="true"/>
    <field name="domain" type="string"/>
    <field name="industry" type="string"/>
    <field name="size" type="string"/>
  </schema>
</schemas>
```

### X-Axis: Contact Operations

Every contact feature maps to operation codes:

```yaml
# Entity CRUD Operations
contact_create: 0x0100    # Create new contact from any source
contact_read: 0x0101      # Retrieve contact projection
contact_update: 0x0102    # Append event to contact history
contact_delete: 0x0103    # Mark contact as deleted (soft)
contact_list: 0x0104      # Query contacts with filters
contact_search: 0x0105    # Full-text search contacts

# Relationship Operations
contact_link: 0x0110      # Link contact to another entity
contact_enrich: 0x0111    # Enrich from email signature/external
contact_merge: 0x0112     # Merge duplicate contacts
contact_stage_transition: 0x0113  # Move through lifecycle

# Interaction Operations
contact_timeline: 0x0120  # Compute interaction timeline
contact_health: 0x0121    # Calculate relationship health score
contact_momentum: 0x0122  # Compute momentum indicator
contact_stakeholders: 0x0123  # Map organizational relationships

# Communication Operations
contact_email: 0x0130     # Send email to contact
contact_task: 0x0131      # Create task linked to contact
contact_workflow: 0x0132  # Enroll in workflow
```

### Y-Axis: Contact Predicates

All contact constraints as predicate bytecode:

```xml
<predicates>
  <!-- Access Control -->
  <predicate id="can_view_contact">
    <or>
      <eq left="$token.sub" right="$contact.owner_id"/>
      <contains left="$token.roles" right="admin"/>
      <eq left="$contact.visibility" right="team"/>
    </or>
  </predicate>
  
  <predicate id="can_edit_contact">
    <or>
      <eq left="$token.sub" right="$contact.owner_id"/>
      <contains left="$token.roles" right="admin"/>
    </or>
  </predicate>
  
  <!-- Lifecycle Rules -->
  <predicate id="can_promote_to_customer">
    <and>
      <eq left="$contact.stage" right="prospect"/>
      <gt left="$contact.engagement_count" right="3"/>
      <predicate ref="has_active_deal"/>
    </and>
  </predicate>
  
  <predicate id="is_cooling">
    <and>
      <gt left="$days_since_last_contact" right="30"/>
      <lt left="$days_since_last_contact" right="90"/>
      <eq left="$contact.stage" right="customer"/>
    </and>
  </predicate>
  
  <predicate id="is_dormant">
    <gt left="$days_since_last_contact" right="90"/>
  </predicate>
  
  <!-- Data Quality -->
  <predicate id="is_duplicate_contact">
    <or>
      <contains left="$existing_emails" right="$new_contact.email"/>
      <similar left="$existing_names" right="$new_contact.name" threshold="0.85"/>
    </or>
  </predicate>
  
  <predicate id="needs_enrichment">
    <or>
      <null left="$contact.company_id"/>
      <null left="$contact.phone"/>
      <null left="$contact.title"/>
    </or>
  </predicate>
  
  <!-- Health Scoring -->
  <predicate id="has_healthy_relationship">
    <and>
      <gt left="$contact.health_score" right="70"/>
      <lt left="$days_since_last_contact" right="14"/>
    </and>
  </predicate>
</predicates>
```

### Z-Axis: Contact Events

All state changes are events appended to log:

```typescript
// Event Schema
type ContactEvent = {
  id: UUID;
  contact_id: UUID;
  type: ContactEventType;
  timestamp: ISO8601;
  actor_id: UUID;
  data: Record<string, unknown>;
  pxyz: PXYZ;
}

// Event Types
enum ContactEventType {
  CREATED = "contact.created",
  UPDATED = "contact.updated",
  STAGE_CHANGED = "contact.stage_changed",
  EMAIL_SENT = "contact.email_sent",
  EMAIL_RECEIVED = "contact.email_received",
  MEETING_HELD = "contact.meeting_held",
  TASK_CREATED = "contact.task_created",
  TASK_COMPLETED = "contact.task_completed",
  WORKFLOW_ENROLLED = "contact.workflow_enrolled",
  LINKED_TO_ORG = "contact.linked_to_org",
  ENRICHED = "contact.enriched",
  MERGED = "contact.merged",
  ARCHIVED = "contact.archived"
}
```

### Workflow Example: Contact Search

```xml
<workflow id="contact_search">
  <entry p="contact" x="search" node="validate_search"/>
  
  <nodes>
    <!-- Validation Node -->
    <node id="validate_search" kind="transform">
      <validate>
        <field name="query" type="string" required="false"/>
        <field name="filters" type="object" required="false"/>
        <field name="limit" type="integer" default="20" max="100"/>
        <field name="offset" type="integer" default="0"/>
      </validate>
    </node>
    
    <!-- Authorization Node -->
    <node id="auth_search" kind="auth">
      <require predicate="is_authenticated"/>
    </node>
    
    <!-- External Search Operation -->
    <node id="execute_search" kind="external" op="0x0105">
      <input>
        <map field="query" from="$input.query"/>
        <map field="filters" from="$input.filters"/>
        <map field="workspace_id" from="$token.workspace_id"/>
      </input>
    </node>
    
    <!-- Apply Access Control Predicate -->
    <node id="filter_results" kind="transform">
      <filter predicate="can_view_contact" on="$search_results"/>
    </node>
    
    <!-- Compute Momentum Scores -->
    <node id="compute_momentum" kind="external" op="0x0122">
      <for_each item="$filtered_results"/>
    </node>
    
    <!-- Render Results -->
    <node id="render_results" kind="render">
      <template ref="contact_search_results"/>
    </node>
    
    <!-- Terminal Success -->
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_search" to="auth_search">
      <when><always/></when>
    </edge>
    <edge from="auth_search" to="execute_search">
      <when><always/></when>
    </edge>
    <edge from="execute_search" to="filter_results">
      <when><always/></when>
    </edge>
    <edge from="filter_results" to="compute_momentum">
      <when><always/></when>
    </edge>
    <edge from="compute_momentum" to="render_results">
      <when><always/></when>
    </edge>
    <edge from="render_results" to="success">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

### Workflow Example: Contact Stage Transition

```xml
<workflow id="contact_promote_to_customer">
  <entry p="contact" x="stage_transition" node="load_contact"/>
  
  <nodes>
    <node id="load_contact" kind="external" op="0x0101">
      <input>
        <map field="id" from="$input.contact_id"/>
      </input>
    </node>
    
    <node id="check_can_promote" kind="auth">
      <require predicate="can_promote_to_customer"/>
    </node>
    
    <node id="append_stage_event" kind="external" op="0x0910">
      <event>
        <type>contact.stage_changed</type>
        <data>
          <field name="old_stage" value="$contact.stage"/>
          <field name="new_stage" value="customer"/>
          <field name="reason" value="$input.reason"/>
        </data>
      </event>
    </node>
    
    <node id="trigger_onboarding" kind="external" op="0x0132">
      <workflow ref="customer_onboarding"/>
      <input>
        <map field="contact_id" from="$contact.id"/>
      </input>
    </node>
    
    <node id="notify_owner" kind="external" op="0x0800">
      <llm_prompt>
        Draft a notification to {{contact.owner.name}} that {{contact.name}} 
        has been promoted to customer. Suggest next steps for onboarding.
      </llm_prompt>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_contact" to="check_can_promote">
      <when><always/></when>
    </edge>
    <edge from="check_can_promote" to="append_stage_event">
      <when><always/></when>
    </edge>
    <edge from="append_stage_event" to="trigger_onboarding">
      <when><always/></when>
    </edge>
    <edge from="trigger_onboarding" to="notify_owner">
      <when><always/></when>
    </edge>
    <edge from="notify_owner" to="success">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

### Workflow Example: Relationship Health Calculation

```xml
<workflow id="contact_health_score">
  <entry p="contact" x="health" node="load_events"/>
  
  <nodes>
    <node id="load_events" kind="external" op="0x0911">
      <query>
        <filter field="contact_id" value="$input.contact_id"/>
        <filter field="type" in="email_sent,email_received,meeting_held,task_completed"/>
        <filter field="timestamp" gte="$now-90days"/>
      </query>
    </node>
    
    <node id="calculate_recency" kind="transform">
      <compute>
        <var name="last_interaction" value="max($events.timestamp)"/>
        <var name="days_since" value="$now - $last_interaction"/>
        <var name="recency_score" value="max(0, 100 - $days_since)"/>
      </compute>
    </node>
    
    <node id="calculate_frequency" kind="transform">
      <compute>
        <var name="interaction_count" value="count($events)"/>
        <var name="frequency_score" value="min(100, $interaction_count * 5)"/>
      </compute>
    </node>
    
    <node id="calculate_sentiment" kind="external" op="0x0801">
      <classify>
        <input from="$events.content"/>
        <categories>positive,neutral,negative</categories>
      </classify>
    </node>
    
    <node id="compute_final_score" kind="transform">
      <compute>
        <var name="health_score" value="(
          $recency_score * 0.4 +
          $frequency_score * 0.3 +
          $sentiment_score * 0.3
        )"/>
      </compute>
    </node>
    
    <node id="return_score" kind="terminal" status="200">
      <return field="health_score"/>
    </node>
  </nodes>
  
  <edges>
    <edge from="load_events" to="calculate_recency"><when><always/></when></edge>
    <edge from="calculate_recency" to="calculate_frequency"><when><always/></when></edge>
    <edge from="calculate_frequency" to="calculate_sentiment"><when><always/></when></edge>
    <edge from="calculate_sentiment" to="compute_final_score"><when><always/></when></edge>
    <edge from="compute_final_score" to="return_score"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 2: TASKS DOMAIN AS PXYZ

### P-Axis: Task Entities

```xml
<schema id="task">
  <field name="id" type="uuid" required="true"/>
  <field name="title" type="string" required="true"/>
  <field name="description" type="string"/>
  <field name="owner_id" type="uuid" required="true"/>
  <field name="state" type="enum" values="idea,planned,in_progress,blocked,delivered,follow_up,done,cancelled"/>
  <field name="priority" type="enum" values="low,medium,high,urgent"/>
  <field name="due_date" type="timestamp"/>
  <field name="start_date" type="timestamp"/>
  <field name="project_id" type="uuid"/>
  <field name="parent_task_id" type="uuid"/>
  <field name="estimated_hours" type="float"/>
  <field name="actual_hours" type="computed"/>
</schema>
```

### X-Axis: Task Operations

```yaml
task_create: 0x0200
task_read: 0x0201
task_update: 0x0202
task_delete: 0x0203
task_list: 0x0204
task_search: 0x0205
task_state_transition: 0x0210
task_assign: 0x0211
task_add_subtask: 0x0212
task_add_dependency: 0x0213
task_time_track: 0x0220
task_convert_to_workflow: 0x0221
task_convert_to_invoice: 0x0222
```

### Y-Axis: Task Predicates

```xml
<predicates>
  <predicate id="can_edit_task">
    <or>
      <eq left="$token.sub" right="$task.owner_id"/>
      <contains left="$token.roles" right="admin"/>
      <contains left="$task.collaborators" right="$token.sub"/>
    </or>
  </predicate>
  
  <predicate id="is_overdue">
    <and>
      <not_null left="$task.due_date"/>
      <lt left="$task.due_date" right="$now"/>
      <not_in left="$task.state" values="done,cancelled"/>
    </and>
  </predicate>
  
  <predicate id="is_blocked_valid">
    <and>
      <eq left="$task.state" right="blocked"/>
      <not_null left="$task.blocker_description"/>
    </and>
  </predicate>
  
  <predicate id="can_mark_done">
    <and>
      <predicate ref="can_edit_task"/>
      <eq left="$task.state" right="in_progress"/>
      <or>
        <null left="$task.parent_task_id"/>
        <all_complete left="$task.subtasks"/>
      </or>
    </and>
  </predicate>
  
  <predicate id="should_promote_to_workflow">
    <and>
      <gt left="count($task.subtasks)" right="5"/>
      <gt left="count($task.dependencies)" right="3"/>
      <null left="$task.workflow_id"/>
    </and>
  </predicate>
</predicates>
```

### Z-Axis: Task Events

```typescript
enum TaskEventType {
  CREATED = "task.created",
  STATE_CHANGED = "task.state_changed",
  ASSIGNED = "task.assigned",
  DUE_DATE_CHANGED = "task.due_date_changed",
  BLOCKED = "task.blocked",
  UNBLOCKED = "task.unblocked",
  COMPLETED = "task.completed",
  CANCELLED = "task.cancelled",
  TIME_LOGGED = "task.time_logged",
  SUBTASK_ADDED = "task.subtask_added",
  DEPENDENCY_ADDED = "task.dependency_added",
  COMMENT_ADDED = "task.comment_added"
}
```

### Workflow Example: Task State Transition (Kanban)

```xml
<workflow id="task_state_transition">
  <entry p="task" x="state_transition" node="load_task"/>
  
  <nodes>
    <node id="load_task" kind="external" op="0x0201"/>
    
    <node id="check_can_edit" kind="auth">
      <require predicate="can_edit_task"/>
    </node>
    
    <node id="validate_transition" kind="transform">
      <validate>
        <field name="new_state" type="enum" values="idea,planned,in_progress,blocked,delivered,follow_up,done,cancelled"/>
        <constraint predicate="is_valid_transition">
          <param name="from" value="$task.state"/>
          <param name="to" value="$input.new_state"/>
        </constraint>
      </validate>
    </node>
    
    <node id="check_blocking_required" kind="auth">
      <when>
        <eq left="$input.new_state" right="blocked"/>
      </when>
      <require predicate="has_blocker_description"/>
    </node>
    
    <node id="append_state_event" kind="external" op="0x0910">
      <event>
        <type>task.state_changed</type>
        <data>
          <field name="old_state" value="$task.state"/>
          <field name="new_state" value="$input.new_state"/>
          <field name="reason" value="$input.reason"/>
        </data>
      </event>
    </node>
    
    <node id="check_completion" kind="transform">
      <when>
        <eq left="$input.new_state" right="done"/>
      </when>
    </node>
    
    <node id="offer_invoice_conversion" kind="external" op="0x0800">
      <when>
        <and>
          <eq left="$input.new_state" right="done"/>
          <eq left="$task.billable" right="true"/>
          <gt left="$task.actual_hours" right="0"/>
        </and>
      </when>
      <llm_prompt>
        Task completed with {{task.actual_hours}} hours tracked. 
        Generate invoice line item?
      </llm_prompt>
    </node>
    
    <node id="update_parent_progress" kind="external" op="0x0202">
      <when>
        <not_null left="$task.parent_task_id"/>
      </when>
      <compute>
        <var name="parent_progress" value="calculate_parent_progress($task.parent_task_id)"/>
      </compute>
    </node>
    
    <node id="notify_watchers" kind="external" op="0x0300">
      <notification>
        <recipients from="$task.watchers"/>
        <template ref="task_state_changed"/>
      </notification>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_task" to="check_can_edit"><when><always/></when></edge>
    <edge from="check_can_edit" to="validate_transition"><when><always/></when></edge>
    <edge from="validate_transition" to="check_blocking_required">
      <when>
        <eq left="$input.new_state" right="blocked"/>
      </when>
    </edge>
    <edge from="validate_transition" to="append_state_event">
      <when>
        <ne left="$input.new_state" right="blocked"/>
      </when>
    </edge>
    <edge from="check_blocking_required" to="append_state_event"><when><always/></when></edge>
    <edge from="append_state_event" to="check_completion"><when><always/></when></edge>
    <edge from="check_completion" to="offer_invoice_conversion">
      <when>
        <and>
          <eq left="$input.new_state" right="done"/>
          <eq left="$task.billable" right="true"/>
        </and>
      </when>
    </edge>
    <edge from="check_completion" to="update_parent_progress">
      <when>
        <not_null left="$task.parent_task_id"/>
      </when>
    </edge>
    <edge from="offer_invoice_conversion" to="notify_watchers"><when><always/></when></edge>
    <edge from="update_parent_progress" to="notify_watchers"><when><always/></when></edge>
    <edge from="notify_watchers" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: AI-Powered Task Breakdown

```xml
<workflow id="task_generate_subtasks">
  <entry p="task" x="generate_subtasks" node="load_task"/>
  
  <nodes>
    <node id="load_task" kind="external" op="0x0201"/>
    
    <node id="check_complexity" kind="auth">
      <require predicate="is_complex_task">
        <param name="min_description_length" value="100"/>
      </require>
    </node>
    
    <node id="llm_breakdown" kind="external" op="0x0800">
      <llm_prompt>
        Break down this task into logical subtasks:
        Title: {{task.title}}
        Description: {{task.description}}
        Project: {{task.project.name}}
        
        Return JSON array of subtasks with:
        - title
        - estimated_hours
        - dependencies (array of indices)
        
        DO NOT OUTPUT ANYTHING OTHER THAN VALID JSON.
      </llm_prompt>
    </node>
    
    <node id="parse_subtasks" kind="transform">
      <parse format="json" from="$llm_response"/>
      <validate schema="subtask_array"/>
    </node>
    
    <node id="create_subtasks" kind="external" op="0x0212">
      <for_each item="$parsed_subtasks">
        <create_subtask parent_id="$task.id"/>
      </for_each>
    </node>
    
    <node id="render_preview" kind="render">
      <template ref="subtask_preview"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_task" to="check_complexity"><when><always/></when></edge>
    <edge from="check_complexity" to="llm_breakdown"><when><always/></when></edge>
    <edge from="llm_breakdown" to="parse_subtasks"><when><always/></when></edge>
    <edge from="parse_subtasks" to="create_subtasks"><when><always/></when></edge>
    <edge from="create_subtasks" to="render_preview"><when><always/></when></edge>
    <edge from="render_preview" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 3: WORKFLOWS DOMAIN AS PXYZ

### P-Axis: Workflow Entities

```xml
<schema id="workflow">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="description" type="string"/>
  <field name="category" type="enum" values="operations,pipeline,work_order,marketing"/>
  <field name="owner_id" type="uuid" required="true"/>
  <field name="status" type="enum" values="not_started,in_progress,paused,complete,cancelled"/>
  <field name="health_score" type="computed"/>
  <field name="progress_pct" type="computed"/>
  <field name="template_id" type="uuid"/>
</schema>

<schema id="workflow_phase">
  <field name="id" type="uuid" required="true"/>
  <field name="workflow_id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="sequence_order" type="integer"/>
  <field name="duration_estimate_hours" type="float"/>
  <field name="status" type="enum" values="not_started,in_progress,complete"/>
</schema>

<schema id="workflow_step">
  <field name="id" type="uuid" required="true"/>
  <field name="phase_id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="type" type="enum" values="manual,automated,approval_gate"/>
  <field name="owner_id" type="uuid"/>
  <field name="status" type="enum" values="not_started,ready,in_progress,blocked,review,complete,skipped"/>
  <field name="dependencies" type="array"/>
</schema>
```

### X-Axis: Workflow Operations

```yaml
workflow_create: 0x0400
workflow_read: 0x0401
workflow_update: 0x0402
workflow_delete: 0x0403
workflow_list: 0x0404
workflow_search: 0x0405
workflow_start: 0x0410
workflow_pause: 0x0411
workflow_resume: 0x0412
workflow_complete: 0x0413
workflow_cancel: 0x0414
workflow_phase_transition: 0x0420
workflow_step_start: 0x0421
workflow_step_complete: 0x0422
workflow_add_phase: 0x0430
workflow_add_step: 0x0431
workflow_health_check: 0x0440
workflow_save_as_template: 0x0441
```

### Y-Axis: Workflow Predicates

```xml
<predicates>
  <predicate id="can_start_workflow">
    <and>
      <eq left="$workflow.status" right="not_started"/>
      <predicate ref="can_edit_workflow"/>
      <all_satisfied>
        <check left="$workflow.prerequisites"/>
      </all_satisfied>
    </and>
  </predicate>
  
  <predicate id="can_complete_step">
    <and>
      <eq left="$step.status" right="in_progress"/>
      <or>
        <eq left="$token.sub" right="$step.owner_id"/>
        <contains left="$token.roles" right="admin"/>
      </or>
      <all_complete left="$step.dependencies"/>
    </and>
  </predicate>
  
  <predicate id="phase_can_advance">
    <and>
      <all_complete left="$phase.steps"/>
      <or>
        <null left="$phase.approval_gate"/>
        <approved left="$phase.approval_gate"/>
      </or>
    </and>
  </predicate>
  
  <predicate id="workflow_at_risk">
    <or>
      <gt left="$workflow.overdue_steps_count" right="2"/>
      <gt left="$workflow.blocked_steps_count" right="1"/>
      <gt left="$workflow.days_since_activity" right="7"/>
    </or>
  </predicate>
  
  <predicate id="is_valid_workflow_structure">
    <and>
      <gt left="count($workflow.phases)" right="0"/>
      <not_empty left="$workflow.owner_id"/>
      <no_circular_dependencies left="$workflow.steps"/>
    </and>
  </predicate>
</predicates>
```

### Z-Axis: Workflow Events

```typescript
enum WorkflowEventType {
  CREATED = "workflow.created",
  STARTED = "workflow.started",
  PAUSED = "workflow.paused",
  RESUMED = "workflow.resumed",
  COMPLETED = "workflow.completed",
  CANCELLED = "workflow.cancelled",
  PHASE_STARTED = "workflow.phase_started",
  PHASE_COMPLETED = "workflow.phase_completed",
  STEP_STARTED = "workflow.step_started",
  STEP_COMPLETED = "workflow.step_completed",
  STEP_BLOCKED = "workflow.step_blocked",
  STEP_UNBLOCKED = "workflow.step_unblocked",
  STEP_SKIPPED = "workflow.step_skipped",
  GATE_APPROVED = "workflow.gate_approved",
  GATE_REJECTED = "workflow.gate_rejected",
  PARTICIPANT_ADDED = "workflow.participant_added",
  SCOPE_CHANGED = "workflow.scope_changed"
}
```

### Workflow Example: Wizard-Generated Workflow

```xml
<workflow id="workflow_create_from_wizard">
  <entry p="workflow" x="create" node="gather_inputs"/>
  
  <nodes>
    <node id="gather_inputs" kind="transform">
      <wizard steps="4">
        <step id="1" question="What's the outcome?">
          <input name="outcome" type="text"/>
        </step>
        <step id="2" question="Who's involved?">
          <input name="participants" type="multiselect"/>
        </step>
        <step id="3" question="What's the timeline?">
          <input name="due_date" type="date"/>
          <input name="time_boxed" type="boolean"/>
        </step>
        <step id="4" question="Use existing template?">
          <input name="template_id" type="select" optional="true"/>
        </step>
      </wizard>
    </node>
    
    <node id="llm_generate_structure" kind="external" op="0x0800">
      <when>
        <null left="$input.template_id"/>
      </when>
      <llm_prompt>
        Generate workflow structure for:
        Outcome: {{input.outcome}}
        Participants: {{input.participants}}
        Timeline: {{input.due_date}}
        
        Return JSON:
        {
          "phases": [
            {
              "name": "string",
              "duration_hours": number,
              "steps": [
                {
                  "name": "string",
                  "owner": "string",
                  "type": "manual|automated|approval_gate",
                  "duration_hours": number
                }
              ]
            }
          ]
        }
        
        DO NOT OUTPUT ANYTHING OTHER THAN VALID JSON.
      </llm_prompt>
    </node>
    
    <node id="load_template" kind="external" op="0x0401">
      <when>
        <not_null left="$input.template_id"/>
      </when>
    </node>
    
    <node id="create_workflow_entity" kind="external" op="0x0400">
      <event>
        <type>workflow.created</type>
        <data>
          <field name="name" value="$input.outcome"/>
          <field name="owner_id" value="$token.sub"/>
          <field name="due_date" value="$input.due_date"/>
        </data>
      </event>
    </node>
    
    <node id="create_phases" kind="external" op="0x0420">
      <for_each item="$workflow_structure.phases">
        <create_phase workflow_id="$workflow.id"/>
      </for_each>
    </node>
    
    <node id="create_steps" kind="external" op="0x0431">
      <for_each item="$workflow_structure.phases.steps">
        <create_step phase_id="$phase.id"/>
      </for_each>
    </node>
    
    <node id="assign_participants" kind="external" op="0x0402">
      <for_each item="$input.participants">
        <add_participant workflow_id="$workflow.id"/>
      </for_each>
    </node>
    
    <node id="render_preview" kind="render">
      <template ref="workflow_created_summary"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="gather_inputs" to="llm_generate_structure">
      <when><null left="$input.template_id"/></when>
    </edge>
    <edge from="gather_inputs" to="load_template">
      <when><not_null left="$input.template_id"/></when>
    </edge>
    <edge from="llm_generate_structure" to="create_workflow_entity"><when><always/></when></edge>
    <edge from="load_template" to="create_workflow_entity"><when><always/></when></edge>
    <edge from="create_workflow_entity" to="create_phases"><when><always/></when></edge>
    <edge from="create_phases" to="create_steps"><when><always/></when></edge>
    <edge from="create_steps" to="assign_participants"><when><always/></when></edge>
    <edge from="assign_participants" to="render_preview"><when><always/></when></edge>
    <edge from="render_preview" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: Health Monitoring

```xml
<workflow id="workflow_health_check">
  <entry p="workflow" x="health_check" node="load_workflow"/>
  
  <nodes>
    <node id="load_workflow" kind="external" op="0x0401"/>
    
    <node id="load_all_steps" kind="external" op="0x0911">
      <query>
        <filter field="workflow_id" value="$input.workflow_id"/>
        <filter field="type" value="workflow_step"/>
      </query>
    </node>
    
    <node id="calculate_overdue" kind="transform">
      <compute>
        <var name="overdue_steps" value="filter($steps, is_overdue)"/>
        <var name="overdue_count" value="count($overdue_steps)"/>
      </compute>
    </node>
    
    <node id="calculate_blocked" kind="transform">
      <compute>
        <var name="blocked_steps" value="filter($steps, {status: 'blocked'})"/>
        <var name="blocked_count" value="count($blocked_steps)"/>
      </compute>
    </node>
    
    <node id="calculate_inactivity" kind="transform">
      <compute>
        <var name="last_activity" value="max($workflow.events.timestamp)"/>
        <var name="days_since_activity" value="$now - $last_activity"/>
      </compute>
    </node>
    
    <node id="compute_health_score" kind="transform">
      <compute>
        <var name="progress_score" value="$workflow.progress_pct"/>
        <var name="overdue_penalty" value="$overdue_count * 10"/>
        <var name="blocked_penalty" value="$blocked_count * 15"/>
        <var name="inactivity_penalty" value="min(30, $days_since_activity * 3)"/>
        <var name="health_score" value="max(0, $progress_score - $overdue_penalty - $blocked_penalty - $inactivity_penalty)"/>
      </compute>
    </node>
    
    <node id="generate_recommendations" kind="external" op="0x0800">
      <when>
        <lt left="$health_score" right="70"/>
      </when>
      <llm_prompt>
        Workflow health issues detected:
        - Overdue steps: {{overdue_count}}
        - Blocked steps: {{blocked_count}}
        - Days inactive: {{days_since_activity}}
        - Health score: {{health_score}}/100
        
        Suggest 3 specific actions to improve workflow health.
      </llm_prompt>
    </node>
    
    <node id="notify_owner" kind="external" op="0x0300">
      <when>
        <lt left="$health_score" right="50"/>
      </when>
      <notification>
        <recipient value="$workflow.owner_id"/>
        <template ref="workflow_at_risk"/>
        <data>
          <field name="health_score" value="$health_score"/>
          <field name="recommendations" value="$llm_recommendations"/>
        </data>
      </notification>
    </node>
    
    <node id="return_health" kind="terminal" status="200">
      <return>
        <field name="health_score" value="$health_score"/>
        <field name="overdue_count" value="$overdue_count"/>
        <field name="blocked_count" value="$blocked_count"/>
        <field name="days_since_activity" value="$days_since_activity"/>
        <field name="recommendations" value="$llm_recommendations"/>
      </return>
    </node>
  </nodes>
  
  <edges>
    <edge from="load_workflow" to="load_all_steps"><when><always/></when></edge>
    <edge from="load_all_steps" to="calculate_overdue"><when><always/></when></edge>
    <edge from="calculate_overdue" to="calculate_blocked"><when><always/></when></edge>
    <edge from="calculate_blocked" to="calculate_inactivity"><when><always/></when></edge>
    <edge from="calculate_inactivity" to="compute_health_score"><when><always/></when></edge>
    <edge from="compute_health_score" to="generate_recommendations">
      <when><lt left="$health_score" right="70"/></when>
    </edge>
    <edge from="compute_health_score" to="return_health">
      <when><gte left="$health_score" right="70"/></when>
    </edge>
    <edge from="generate_recommendations" to="notify_owner">
      <when><lt left="$health_score" right="50"/></when>
    </edge>
    <edge from="generate_recommendations" to="return_health">
      <when><gte left="$health_score" right="50"/></when>
    </edge>
    <edge from="notify_owner" to="return_health"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 4: EMAIL DOMAIN AS PXYZ

### P-Axis: Email Entities

```xml
<schema id="email_thread">
  <field name="id" type="uuid" required="true"/>
  <field name="subject" type="string" required="true"/>
  <field name="participants" type="array"/>
  <field name="account_id" type="uuid"/>
  <field name="message_count" type="integer"/>
  <field name="last_message_at" type="timestamp"/>
  <field name="status" type="enum" values="unread,read,starred,snoozed,archived"/>
</schema>

<schema id="email_message">
  <field name="id" type="uuid" required="true"/>
  <field name="thread_id" type="uuid" required="true"/>
  <field name="from" type="string" required="true"/>
  <field name="to" type="array" required="true"/>
  <field name="cc" type="array"/>
  <field name="bcc" type="array"/>
  <field name="subject" type="string"/>
  <field name="body_html" type="string"/>
  <field name="body_text" type="string"/>
  <field name="attachments" type="array"/>
  <field name="sent_at" type="timestamp"/>
</schema>
```

### X-Axis: Email Operations

```yaml
email_sync: 0x0500
email_read: 0x0501
email_send: 0x0502
email_reply: 0x0503
email_forward: 0x0504
email_search: 0x0505
email_create_thread: 0x0506
email_archive: 0x0510
email_snooze: 0x0511
email_star: 0x0512
email_tag: 0x0513
email_to_task: 0x0520
email_to_note: 0x0521
email_summarize: 0x0530
email_extract_commitments: 0x0531
email_sentiment_analysis: 0x0532
email_draft_reply: 0x0533
```

### Y-Axis: Email Predicates

```xml
<predicates>
  <predicate id="needs_reply">
    <and>
      <eq left="$thread.last_sender" right="external"/>
      <gt left="$days_since_last_message" right="2"/>
      <not_in left="$thread.status" values="archived,snoozed"/>
    </and>
  </predicate>
  
  <predicate id="is_urgent">
    <or>
      <contains left="$message.subject" right="urgent" case_insensitive="true"/>
      <contains left="$message.body" right="asap" case_insensitive="true"/>
      <has_deadline left="$message.body" within_days="2"/>
    </or>
  </predicate>
  
  <predicate id="has_commitment">
    <contains left="$message.body" patterns="i will,i'll,by friday,by eod,will send"/>
  </predicate>
  
  <predicate id="can_send_as_account">
    <or>
      <eq left="$token.sub" right="$account.owner_id"/>
      <contains left="$account.delegates" right="$token.sub"/>
    </or>
  </predicate>
  
  <predicate id="should_create_contact">
    <and>
      <not_exists left="$sender.email" in="contacts"/>
      <external_sender left="$sender.email"/>
      <gt left="$thread.message_count" right="2"/>
    </and>
  </predicate>
</predicates>
```

### Workflow Example: Email Send with AI Draft

```xml
<workflow id="email_compose_and_send">
  <entry p="email" x="send" node="validate_input"/>
  
  <nodes>
    <node id="validate_input" kind="transform">
      <validate>
        <field name="to" type="array" required="true" min_length="1"/>
        <field name="subject" type="string" required="true"/>
        <field name="body" type="string"/>
        <field name="account_id" type="uuid" required="true"/>
        <field name="draft_from_prompt" type="string"/>
      </validate>
    </node>
    
    <node id="check_account_access" kind="auth">
      <require predicate="can_send_as_account"/>
    </node>
    
    <node id="llm_draft_body" kind="external" op="0x0533">
      <when>
        <and>
          <not_null left="$input.draft_from_prompt"/>
          <or>
            <null left="$input.body"/>
            <empty left="$input.body"/>
          </or>
        </and>
      </when>
      <llm_prompt>
        Draft email to {{input.to}} about:
        {{input.draft_from_prompt}}
        
        Context:
        - Recipient name: {{recipient.name}}
        - Past communication tone: {{recipient.preferred_tone}}
        - Current relationship: {{recipient.stage}}
        
        Subject: {{input.subject}}
        
        Write professional email body. Keep it concise.
      </llm_prompt>
    </node>
    
    <node id="spam_check" kind="external" op="0x0534">
      <check_spam_score message="$message"/>
    </node>
    
    <node id="warn_spam_risk" kind="signal">
      <when>
        <gt left="$spam_score" right="5"/>
      </when>
      <signal>spam_risk_detected</signal>
    </node>
    
    <node id="create_message_event" kind="external" op="0x0910">
      <event>
        <type>email.sent</type>
        <data>
          <field name="to" value="$input.to"/>
          <field name="subject" value="$input.subject"/>
          <field name="body" value="$composed_body"/>
          <field name="account_id" value="$input.account_id"/>
        </data>
      </event>
    </node>
    
    <node id="send_via_smtp" kind="external" op="0x0502">
      <smtp>
        <from account_id="$input.account_id"/>
        <to value="$input.to"/>
        <cc value="$input.cc"/>
        <bcc value="$input.bcc"/>
        <subject value="$input.subject"/>
        <body value="$composed_body"/>
        <attachments value="$input.attachments"/>
      </smtp>
    </node>
    
    <node id="update_contact_timeline" kind="external" op="0x0120">
      <for_each item="$input.to">
        <append_timeline_event contact_email="$item"/>
      </for_each>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_input" to="check_account_access"><when><always/></when></edge>
    <edge from="check_account_access" to="llm_draft_body">
      <when>
        <and>
          <not_null left="$input.draft_from_prompt"/>
          <or><null left="$input.body"/><empty left="$input.body"/></or>
        </and>
      </when>
    </edge>
    <edge from="check_account_access" to="spam_check">
      <when>
        <or>
          <null left="$input.draft_from_prompt"/>
          <not_empty left="$input.body"/>
        </or>
      </when>
    </edge>
    <edge from="llm_draft_body" to="spam_check"><when><always/></when></edge>
    <edge from="spam_check" to="warn_spam_risk">
      <when><gt left="$spam_score" right="5"/></when>
    </edge>
    <edge from="spam_check" to="create_message_event">
      <when><lte left="$spam_score" right="5"/></when>
    </edge>
    <edge from="warn_spam_risk" to="create_message_event"><when><always/></when></edge>
    <edge from="create_message_event" to="send_via_smtp"><when><always/></when></edge>
    <edge from="send_via_smtp" to="update_contact_timeline"><when><always/></when></edge>
    <edge from="update_contact_timeline" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: Email Thread Summarization

```xml
<workflow id="email_thread_summarize">
  <entry p="email" x="summarize" node="load_thread"/>
  
  <nodes>
    <node id="load_thread" kind="external" op="0x0501">
      <include field="messages" nested="true"/>
    </node>
    
    <node id="extract_messages" kind="transform">
      <compute>
        <var name="messages_text" value="map($thread.messages, extract_body)"/>
        <var name="message_count" value="count($thread.messages)"/>
      </compute>
    </node>
    
    <node id="llm_summarize" kind="external" op="0x0800">
      <llm_prompt>
        Summarize this email thread in 2-3 sentences:
        
        {{#each thread.messages}}
        From: {{from}}
        Date: {{sent_at}}
        {{body_text}}
        ---
        {{/each}}
        
        Extract:
        1. Main topic
        2. Key decisions made
        3. Action items
        4. Overall sentiment
        
        Return as JSON.
      </llm_prompt>
    </node>
    
    <node id="extract_commitments" kind="external" op="0x0531">
      <for_each item="$thread.messages">
        <detect_commitments text="$item.body_text"/>
      </for_each>
    </node>
    
    <node id="sentiment_analysis" kind="external" op="0x0532">
      <classify messages="$thread.messages"/>
    </node>
    
    <node id="compose_summary" kind="transform">
      <compute>
        <var name="summary" value="{
          text: $llm_summary,
          commitments: $extracted_commitments,
          sentiment: $sentiment_analysis,
          message_count: $message_count
        }"/>
      </compute>
    </node>
    
    <node id="return_summary" kind="terminal" status="200">
      <return field="summary"/>
    </node>
  </nodes>
  
  <edges>
    <edge from="load_thread" to="extract_messages"><when><always/></when></edge>
    <edge from="extract_messages" to="llm_summarize"><when><always/></when></edge>
    <edge from="llm_summarize" to="extract_commitments"><when><always/></when></edge>
    <edge from="extract_commitments" to="sentiment_analysis"><when><always/></when></edge>
    <edge from="sentiment_analysis" to="compose_summary"><when><always/></when></edge>
    <edge from="compose_summary" to="return_summary"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 5: DOCUMENTS, NOTES, INVOICES, FILES, WIZARDS, PORTAL

(Due to length constraints, I'll provide the comprehensive extraction for remaining domains in conceptual form with key workflows)

### DOCUMENTS DOMAIN - KEY PXYZ OPERATIONS

```yaml
# P-Axis
document_entities: [document, document_block, document_version, document_comment, document_template]

# X-Axis Operations
document_create: 0x0600
document_read: 0x0601
document_update: 0x0602
document_delete: 0x0603
document_search: 0x0605
document_render: 0x0610
document_export_pdf: 0x0611
document_collaborate: 0x0620
document_version_create: 0x0621
document_comment_add: 0x0622
document_approve: 0x0630
document_sign_request: 0x0631
document_from_template: 0x0640
document_ai_draft: 0x0641
document_ai_improve: 0x0642

# Y-Axis Key Predicates
can_edit_document: owner OR collaborator
can_approve_document: approver role
needs_approval: workflow_gate requirement
is_stale_data: linked_data not refreshed >7 days

# Z-Axis Events
document.created
document.updated
document.shared
document.approved
document.version_created
document.comment_added
document.exported
```

### NOTES DOMAIN - KEY PXYZ OPERATIONS

```yaml
# P-Axis
note_entities: [note, note_version, note_tag, note_template]

# X-Axis Operations
note_create: 0x0700
note_read: 0x0701
note_update: 0x0702
note_delete: 0x0703
note_search: 0x0705
note_tag: 0x0710
note_to_task: 0x0720
note_to_document: 0x0721
note_to_contact: 0x0722
note_ai_extract: 0x0730
note_ai_summarize: 0x0731
note_ai_suggest_tags: 0x0732
note_voice_transcribe: 0x0740

# Y-Axis Key Predicates
can_view_note: owner OR shared_with
is_orphaned: no_links AND >90_days_old
has_action_items: contains_checkbox OR contains_todo_keywords
should_promote: length >500 AND has_structure

# Z-Axis Events
note.created
note.updated
note.tagged
note.evolved_to_task
note.evolved_to_document
note.voice_captured
```

### INVOICES DOMAIN - KEY PXYZ OPERATIONS

```yaml
# P-Axis
invoice_entities: [invoice, invoice_line_item, payment, tax_config]

# X-Axis Operations
invoice_create: 0x0800
invoice_read: 0x0801
invoice_update: 0x0802
invoice_send: 0x0810
invoice_pay: 0x0811
invoice_from_tasks: 0x0820
invoice_from_workflow: 0x0821
invoice_recurring_create: 0x0830
invoice_reminder_send: 0x0831
invoice_ai_line_items: 0x0840
invoice_export_pdf: 0x0850

# Y-Axis Key Predicates
can_send_invoice: invoice_status=draft AND has_line_items
is_overdue: due_date < now AND status=sent
needs_reminder: days_overdue IN [7,14,30]
can_accept_payment: status=sent AND payment_method_configured

# Z-Axis Events
invoice.created
invoice.sent
invoice.viewed
invoice.paid
invoice.overdue
invoice.reminder_sent
invoice.voided
```

### CLIENT PORTAL DOMAIN - KEY PXYZ OPERATIONS

```yaml
# P-Axis
portal_entities: [portal_user, portal_message, portal_upload, portal_approval]

# X-Axis Operations
portal_login: 0x0900
portal_view_project: 0x0901
portal_view_documents: 0x0902
portal_upload_file: 0x0910
portal_approve_deliverable: 0x0920
portal_message_send: 0x0930
portal_view_invoice: 0x0940
portal_pay_invoice: 0x0941
portal_activity_log: 0x0950

# Y-Axis Key Predicates
is_portal_client: role=client
can_approve: has_approval_permission
can_view_project: project.client_id = user.client_id
is_secure_action: requires_2fa OR high_value_transaction

# Z-Axis Events
portal.login
portal.file_uploaded
portal.deliverable_approved
portal.message_sent
portal.invoice_viewed
portal.invoice_paid
```

---

## PART 6: CRITICAL PATTERNS - HOW EVERYTHING CONNECTS

### Pattern 1: Circular Evolution (Email → Task → Workflow → Invoice)

```xml
<workflow id="circular_evolution_example">
  <!-- Email arrives -->
  <entry p="email" x="receive" node="email_received"/>
  
  <nodes>
    <!-- Step 1: Email arrives -->
    <node id="email_received" kind="external" op="0x0500">
      <event type="email.received"/>
    </node>
    
    <!-- Step 2: AI detects action item -->
    <node id="detect_action" kind="external" op="0x0801">
      <classify>
        <input from="$email.body"/>
        <check for="action_item"/>
      </classify>
    </node>
    
    <!-- Step 3: Offer task conversion -->
    <node id="offer_task_conversion" kind="signal">
      <when>
        <eq left="$classification" right="action_item"/>
      </when>
      <signal>suggest_task_creation</signal>
    </node>
    
    <!-- Step 4: User confirms → create task -->
    <node id="create_task" kind="external" op="0x0200">
      <when>
        <eq left="$user_action" right="accept"/>
      </when>
      <event type="task.created">
        <data>
          <field name="title" value="$extracted_action"/>
          <field name="source_email_id" value="$email.id"/>
        </data>
      </event>
    </node>
    
    <!-- Step 5: Task grows complex → promote to workflow -->
    <node id="check_complexity" kind="auth">
      <require predicate="should_promote_to_workflow"/>
    </node>
    
    <node id="create_workflow" kind="external" op="0x0400">
      <event type="workflow.created">
        <data>
          <field name="source_task_id" value="$task.id"/>
        </data>
      </event>
    </node>
    
    <!-- Step 6: Workflow completes → generate invoice -->
    <node id="workflow_complete_check" kind="auth">
      <require predicate="workflow_is_billable"/>
    </node>
    
    <node id="generate_invoice" kind="external" op="0x0821">
      <event type="invoice.created">
        <data>
          <field name="source_workflow_id" value="$workflow.id"/>
          <field name="line_items" value="$workflow.completed_tasks"/>
        </data>
      </event>
    </node>
  </nodes>
</workflow>
```

### Pattern 2: AI-Enhanced Predicate Evaluation

```xml
<predicates>
  <!-- Traditional Predicate -->
  <predicate id="is_high_value_client">
    <gt left="$client.lifetime_value" right="10000"/>
  </predicate>
  
  <!-- AI-Enhanced Predicate -->
  <predicate id="is_at_risk_client">
    <llm_evaluate>
      <context>
        - Last contact: {{client.days_since_last_contact}} days ago
        - Sentiment trend: {{client.sentiment_trend}}
        - Engagement frequency: {{client.engagement_frequency}}
        - Recent complaints: {{client.recent_complaints}}
        - Payment history: {{client.payment_timeliness}}
      </context>
      <question>
        Is this client at risk of churning? Return boolean.
      </question>
      <confidence_threshold>0.8</confidence_threshold>
    </llm_evaluate>
  </predicate>
  
  <!-- Composite Predicate -->
  <predicate id="needs_urgent_attention">
    <and>
      <predicate ref="is_high_value_client"/>
      <predicate ref="is_at_risk_client"/>
      <not>
        <predicate ref="has_recent_outreach"/>
      </not>
    </and>
  </predicate>
</predicates>
```

### Pattern 3: State as Event Projection

```typescript
// WRONG: Storing state directly
interface Contact {
  id: UUID;
  health_score: number; // ❌ This is a lie
  stage: "lead" | "customer"; // ❌ This is a lie
}

// RIGHT: State is computed from events
interface Contact {
  id: UUID;
  // NO OTHER FIELDS
}

function getContactState(contact_id: UUID, at_time?: Timestamp): ContactState {
  const events = eventLog.query({
    entity_id: contact_id,
    type: "contact.*",
    before: at_time ?? now()
  });
  
  return project(events, contactPredicates);
}

// The "database" is just an event log:
type EventLog = {
  append(event: Event): void;
  query(filter: EventFilter): Event[];
}

// All "state" is derived:
const contact = {
  id: "uuid-123",
  // Everything else computed:
  get stage() {
    return getContactState(this.id).stage;
  },
  get health_score() {
    return calculateHealthScore(this.id);
  },
  get timeline() {
    return getContactTimeline(this.id);
  }
}
```

### Pattern 4: Workflow as Graph Traversal

```
Input: PXYZ(contact, search, {query: "Acme"}, now)

Graph Traversal:
  1. Entry → validate_search
     - Predicate: always (no-op)
     - Node Type: transform
     - Operation: schema validation
     
  2. validate_search → auth_search
     - Predicate: always
     - Node Type: auth
     - Operation: check is_authenticated
     
  3. auth_search → execute_search
     - Predicate: always
     - Node Type: external
     - Operation: 0x0105 (contact_search)
     
  4. execute_search → filter_results
     - Predicate: always
     - Node Type: transform
     - Operation: apply can_view_contact to each result
     
  5. filter_results → render
     - Predicate: always
     - Node Type: render
     - Operation: apply template
     
  6. render → terminal
     - Predicate: always
     - Node Type: terminal
     - Operation: return 200

Output: HTML fragment via SSE
```

---

## PART 7: IMPLEMENTATION MAPPING

### How XML → Binary Graph

```rust
// Compiler transforms XML workflow to binary graph
pub struct GraphCompiler {
    fn compile(xml: &str) -> Result<Vec<u8>> {
        let workflow = parse_xml(xml)?;
        
        let mut graph = BinaryGraph::new();
        
        // 1. Write header
        graph.write_magic(0x504E5958); // "PXYZ"
        graph.write_version(1, 0);
        
        // 2. Compile nodes
        for node in workflow.nodes {
            graph.add_node(Node {
                id: node.id,
                kind: match node.kind {
                    "transform" => 0,
                    "external" => 1,
                    "render" => 2,
                    "signal" => 3,
                    "auth" => 4,
                    "terminal" => 5,
                },
                op_code: node.op.unwrap_or(0),
                data_offset: graph.add_string(node.data),
            });
        }
        
        // 3. Compile edges
        for edge in workflow.edges {
            let predicate_id = compile_predicate(&edge.predicate)?;
            graph.add_edge(Edge {
                from: edge.from,
                to: edge.to,
                predicate_id,
            });
        }
        
        // 4. Compile predicates to bytecode
        for pred in workflow.predicates {
            let bytecode = PredicateCompiler::compile(&pred)?;
            graph.add_predicate(bytecode);
        }
        
        graph.finalize()
    }
}
```

### How WAT Runtime Executes

```wat
;; PXYZ Runtime Core - Simplified

(module
  ;; Memory layout:
  ;; [0-96]: Header
  ;; [96-...]: Graph data
  (memory $graph 1)
  
  ;; Graph traversal state
  (global $current_node (mut i32) (i32.const 0))
  (global $visited_bitmap (mut i32) (i32.const 0))
  (global $traversal_depth (mut i32) (i32.const 0))
  
  ;; Main execution function
  (func $execute (param $entry_node i32) (param $context i32) (result i32)
    (local $node_kind i32)
    (local $next_node i32)
    
    ;; Set current node
    (global.set $current_node (local.get $entry_node))
    
    ;; Main traversal loop
    (block $exit
      (loop $traverse
        ;; Depth check
        (if (i32.gt_u (global.get $traversal_depth) (i32.const 1000))
          (return (i32.const -1)) ;; Max depth exceeded
        )
        
        ;; Cycle check
        (if (call $is_visited (global.get $current_node))
          (return (i32.const -2)) ;; Cycle detected
        )
        
        ;; Mark visited
        (call $mark_visited (global.get $current_node))
        
        ;; Get node kind
        (local.set $node_kind
          (i32.load8_u
            (i32.add
              (call $get_node_offset (global.get $current_node))
              (i32.const 4) ;; Offset to kind field
            )
          )
        )
        
        ;; Execute node based on kind
        (block $node_executed
          ;; Kind 0: Transform
          (if (i32.eqz (local.get $node_kind))
            (then
              (call $execute_transform (global.get $current_node) (local.get $context))
              (br $node_executed)
            )
          )
          
          ;; Kind 1: External (call host)
          (if (i32.eq (local.get $node_kind) (i32.const 1))
            (then
              (call $execute_external (global.get $current_node) (local.get $context))
              (br $node_executed)
            )
          )
          
          ;; Kind 5: Terminal
          (if (i32.eq (local.get $node_kind) (i32.const 5))
            (then
              (return (call $get_terminal_status (global.get $current_node)))
            )
          )
        )
        
        ;; Find next node via edge traversal
        (local.set $next_node
          (call $find_next_node (global.get $current_node) (local.get $context))
        )
        
        ;; If no next node, exit
        (if (i32.eq (local.get $next_node) (i32.const 0xFFFFFFFF))
          (br $exit)
        )
        
        ;; Move to next node
        (global.set $current_node (local.get $next_node))
        (global.set $traversal_depth
          (i32.add (global.get $traversal_depth) (i32.const 1))
        )
        
        (br $traverse)
      )
    )
    
    (i32.const 0) ;; Success
  )
  
  ;; Find next node by evaluating edge predicates
  (func $find_next_node (param $current_node i32) (param $context i32) (result i32)
    (local $edge_start i32)
    (local $edge_count i32)
    (local $i i32)
    (local $predicate_id i32)
    (local $target_node i32)
    
    ;; Get edge start and count from node
    (local.set $edge_start
      (i32.load16_u
        (i32.add (call $get_node_offset (local.get $current_node)) (i32.const 12))
      )
    )
    (local.set $edge_count
      (i32.load16_u
        (i32.add (call $get_node_offset (local.get $current_node)) (i32.const 14))
      )
    )
    
    ;; Iterate edges
    (local.set $i (i32.const 0))
    (block $found
      (loop $check_edge
        (if (i32.ge_u (local.get $i) (local.get $edge_count))
          (br $found)
        )
        
        ;; Get predicate ID for this edge
        (local.set $predicate_id
          (call $get_edge_predicate
            (i32.add (local.get $edge_start) (local.get $i))
          )
        )
        
        ;; Evaluate predicate
        (if (call $eval_predicate (local.get $predicate_id) (local.get $context))
          (then
            ;; Predicate true - return this edge's target
            (return (call $get_edge_target
              (i32.add (local.get $edge_start) (local.get $i))
            ))
          )
        )
        
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $check_edge)
      )
    )
    
    (i32.const 0xFFFFFFFF) ;; No edge matched
  )
  
  ;; Predicate VM
  (func $eval_predicate (param $pred_id i32) (param $context i32) (result i32)
    (local $bytecode_offset i32)
    (local $stack_ptr i32)
    (local $opcode i32)
    (local $steps i32)
    
    ;; Safety: max 256 steps
    (local.set $steps (i32.const 0))
    
    (block $pred_done
      (loop $pred_step
        ;; Step limit check
        (if (i32.gt_u (local.get $steps) (i32.const 256))
          (return (i32.const 0)) ;; Limit exceeded = false
        )
        
        ;; Read opcode
        (local.set $opcode
          (i32.load8_u (local.get $bytecode_offset))
        )
        
        ;; Execute based on opcode
        (block $op_executed
          ;; 0x10: EQ
          (if (i32.eq (local.get $opcode) (i32.const 0x10))
            (then
              (call $op_eq (local.get $stack_ptr))
              (br $op_executed)
            )
          )
          
          ;; 0xFF: RET
          (if (i32.eq (local.get $opcode) (i32.const 0xFF))
            (then
              (return (call $stack_pop (local.get $stack_ptr)))
            )
          )
        )
        
        (local.set $steps (i32.add (local.get $steps) (i32.const 1)))
        (local.set $bytecode_offset (i32.add (local.get $bytecode_offset) (i32.const 1)))
        (br $pred_step)
      )
    )
    
    (i32.const 0)
  )
  
  ;; External operation dispatch
  (func $execute_external (param $node_id i32) (param $context i32)
    (local $op_code i32)
    
    ;; Get operation code from node
    (local.set $op_code
      (i32.load16_u
        (i32.add (call $get_node_offset (local.get $node_id)) (i32.const 6))
      )
    )
    
    ;; Dispatch to host via import
    (call $host_execute_op (local.get $op_code) (local.get $context))
  )
  
  ;; Host imports
  (import "io" "execute_op" (func $host_execute_op (param i32 i32)))
  
  ;; Exports
  (export "execute" (func $execute))
  (export "memory" (memory $graph))
)
```

### How IO Adapter Works

```typescript
// io-browser.ts - Host IO implementation

const OP_HANDLERS: Record<number, OpHandler> = {
  // Contact Operations
  0x0105: async (ctx) => {
    // contact_search
    const { query, filters, workspace_id } = ctx.input;
    
    // Query Qdrant for semantic search
    const vectors = await qdrant.search(workspace_id, query);
    
    // Query IndexedDB for structured filters
    const results = await db.contacts
      .where('workspace_id').equals(workspace_id)
      .and(contact => applyFilters(contact, filters))
      .toArray();
    
    // Merge and rank
    return mergeAndRank(vectors, results);
  },
  
  // Email Operations
  0x0502: async (ctx) => {
    // email_send
    const { to, subject, body, account_id } = ctx.input;
    
    // Get account credentials
    const account = await getAccount(account_id);
    
    // Send via SMTP
    await smtp.send({
      from: account.email,
      to,
      subject,
      html: body
    });
    
    // Append event
    await eventLog.append({
      type: 'email.sent',
      entity_id: generateThreadId(to, subject),
      data: { to, subject, account_id }
    });
  },
  
  // LLM Operations
  0x0800: async (ctx) => {
    // llm_complete
    const { prompt, model = 'claude-sonnet-4' } = ctx.input;
    
    // Call Anthropic API
    const response = await anthropic.messages.create({
      model,
      max_tokens: 1000,
      messages: [{ role: 'user', content: prompt }]
    });
    
    return response.content[0].text;
  }
};

// Runtime execution
export async function executePXYZ(
  p: string,
  x: string,
  y: Record<string, unknown>,
  z?: Timestamp
) {
  // 1. Load graph binary
  const graphBin = await loadGraph();
  
  // 2. Initialize WASM runtime
  const wasm = await WebAssembly.instantiate(graphBin, {
    io: {
      execute_op: (opCode: number, contextPtr: number) => {
        const handler = OP_HANDLERS[opCode];
        if (!handler) throw new Error(`Unknown op: 0x${opCode.toString(16)}`);
        
        const ctx = readContext(wasm.memory, contextPtr);
        const result = await handler(ctx);
        
        writeResult(wasm.memory, contextPtr, result);
      }
    }
  });
  
  // 3. Find entry point
  const entryNode = findEntry(graphBin, p, x);
  
  // 4. Execute
  const status = wasm.instance.exports.execute(entryNode, packContext(y));
  
  return status;
}
```

---

## PART 8: DEPLOYMENT ARCHITECTURE

### System Topology

```
User Browser
    |
    v
[Datastar Frontend] ← SSE events
    |
    v
[WASM Runtime (pxyz.wasm)]
    |
    ├─→ [graph.bin] (loaded into WASM memory)
    |
    └─→ [IO Adapter (io-browser.ts)]
            |
            ├─→ Google Workspace APIs
            ├─→ Qdrant Vector DB
            ├─→ IndexedDB (local)
            ├─→ Anthropic API (for LLM)
            └─→ Event Log (append-only)
```

### Data Flow

```
1. User action: Click "Search contacts"
   ↓
2. Datastar: data-on:click="$pxyz('contact','search',{query})"
   ↓
3. Fetch: /pxyz/contact/search?query=Acme
   ↓
4. WASM Runtime:
   - Load graph.bin
   - Find entry: P=contact, X=search
   - Traverse nodes
   - Evaluate predicates
   - Call external ops via IO adapter
   ↓
5. IO Adapter:
   - Execute 0x0105 (contact_search)
   - Query Qdrant + IndexedDB
   - Return results
   ↓
6. WASM Runtime:
   - Continue traversal
   - Apply access control predicates
   - Render template
   ↓
7. SSE Response:
   event: datastar-merge-fragments
   data: selector #results
   data: fragment <div>...results...</div>
   ↓
8. Datastar: Patch DOM
```

---

## CONCLUSION: THE COMPLETE PICTURE

Every business operation in OMAR is:

1. **A coordinate in PXYZ space**: `/pxyz/{entity}/{operation}?{constraints}&z={time}`
2. **A graph traversal**: Starting at entry node, following edges based on predicates
3. **An event source**: All state changes appended to event log
4. **A projection**: Current state = `Project(EventLog, Predicates, Now)`

**There is no imperative code.** There are only:
- **Graphs** (business logic as data)
- **Predicates** (constraints as bytecode)
- **Events** (truth as append-only log)
- **IO** (explicit boundary)

The entire system is:
- **~500 lines of WAT** (the runtime)
- **~600 lines of Rust** (the compiler)
- **~200 lines of TS** (IO adapter per platform)
- **XML workflows** (business logic)

Total attack surface: **~1,300 lines of auditable code.**

Everything else is **data**, not code.

This is OMAR.
