# OMAR Agent Core Services - PXYZ Extraction

> **Services**: AgentBuilder, AgentService, AgentOps, AuditQueryService  
> **Purpose**: Agent lifecycle management, CRM operations, audit trail

---

## PART 1: AGENT BUILDER SERVICE

### Purpose
**Control plane** for agent definitions - create, store, validate agents from configs/prompts/templates.

### P-Axis: Agent Builder Entities

```xml
<schema id="custom_agent">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="description" type="string"/>
  <field name="config" type="object" required="true"/>
  <field name="capabilities" type="array"/>
  <field name="status" type="enum" values="draft,active,archived"/>
  <field name="created_at" type="timestamp"/>
  <field name="updated_at" type="timestamp"/>
  <field name="performance" type="object"/>
</schema>

<schema id="agent_execution">
  <field name="id" type="uuid" required="true"/>
  <field name="agent_id" type="uuid" required="true"/>
  <field name="status" type="enum" values="pending,running,completed,failed"/>
  <field name="start_time" type="timestamp" required="true"/>
  <field name="end_time" type="timestamp"/>
  <field name="input" type="object"/>
  <field name="output" type="object"/>
  <field name="error" type="string"/>
  <field name="metadata" type="object"/>
</schema>

<schema id="agent_template">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="description" type="string"/>
  <field name="category" type="string" required="true"/>
  <field name="capabilities" type="array"/>
  <field name="config" type="object" required="true"/>
  <field name="tags" type="array"/>
  <field name="popularity" type="number"/>
  <field name="author" type="string"/>
  <field name="version" type="string"/>
</schema>

<schema id="flow_definition">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="steps" type="array" required="true"/>
  <field name="triggers" type="array"/>
  <field name="nodes" type="array"/>
  <field name="connections" type="array"/>
  <field name="metadata" type="object"/>
</schema>

<schema id="agent_capability">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="type" type="enum" values="tool,knowledge,workflow"/>
  <field name="permissions" type="array"/>
  <field name="input_types" type="array"/>
  <field name="output_types" type="array"/>
  <field name="confidence" type="number"/>
</schema>
```

### X-Axis: Agent Builder Operations

```yaml
# Agent Lifecycle
agent_create: 0x2100                # Create agent from config
agent_read: 0x2101                  # Read agent definition
agent_update: 0x2102                # Update agent config
agent_delete: 0x2103                # Delete agent
agent_list: 0x2104                  # List all agents
agent_search: 0x2105                # Search agents

# Agent From Flows
agent_create_from_flow: 0x2110      # Create from FlowDefinition
agent_create_from_prompt: 0x2111    # Create from markdown/POML
agent_create_from_template: 0x2112  # Create from template
agent_validate_flow: 0x2113         # Validate against KernelSchema
agent_enrich_flow: 0x2114           # Add schema defaults

# Agent Execution
agent_execute: 0x2120               # Execute agent
agent_execution_create: 0x2121      # Create execution record
agent_execution_update: 0x2122      # Update execution status
agent_execution_list: 0x2123        # List executions

# Template Management
template_create: 0x2130             # Create template
template_read: 0x2131               # Read template
template_update: 0x2132             # Update template
template_delete: 0x2133             # Delete template
template_list: 0x2134               # List templates
template_instantiate: 0x2135        # Create agent from template

# Flow Compilation
flow_parse_markdown: 0x2140         # Parse markdown → flow
flow_parse_latex: 0x2141            # Parse LaTeX → flow
flow_parse_poml: 0x2142             # Parse POML → flow
flow_compile: 0x2143                # Compile flow → executable
flow_validate: 0x2144               # Validate flow structure

# Performance Tracking
agent_performance_update: 0x2150    # Update performance metrics
agent_performance_read: 0x2151      # Read performance stats
```

### Y-Axis: Agent Builder Predicates

```xml
<predicates>
  <!-- Creation Validation -->
  <predicate id="has_valid_agent_config">
    <and>
      <not_null left="$config.name"/>
      <not_null left="$config.description"/>
      <gt left="length($config.capabilities)" right="0"/>
    </and>
  </predicate>
  
  <predicate id="is_unique_agent_name">
    <not>
      <exists left="$agents[$name]"/>
    </not>
  </predicate>
  
  <!-- Flow Validation -->
  <predicate id="has_valid_flow">
    <and>
      <gt left="count($flow.steps)" right="0"/>
      <gt left="count($flow.nodes)" right="0"/>
      <all_steps_have_valid_types left="$flow.steps"/>
    </and>
  </predicate>
  
  <predicate id="flow_passes_kernel_schema">
    <matches_kernel_schema left="$flow" schema="$kernel_schema"/>
  </predicate>
  
  <!-- Execution Validation -->
  <predicate id="can_execute_agent">
    <and>
      <eq left="$agent.status" right="active"/>
      <gt left="$agent.performance.success_rate" right="0.5"/>
    </and>
  </predicate>
  
  <predicate id="agent_not_overloaded">
    <lt left="count($agent.active_executions)" right="10"/>
  </predicate>
  
  <!-- Template Validation -->
  <predicate id="is_valid_template">
    <and>
      <not_null left="$template.name"/>
      <not_null left="$template.category"/>
      <has_valid_config left="$template.config"/>
    </and>
  </predicate>
</predicates>
```

### Z-Axis: Agent Builder Events

```typescript
enum AgentBuilderEventType {
  // Agent Lifecycle
  AGENT_CREATED = "agent.created",
  AGENT_UPDATED = "agent.updated",
  AGENT_DELETED = "agent.deleted",
  AGENT_ARCHIVED = "agent.archived",
  AGENT_ACTIVATED = "agent.activated",
  
  // Flow Compilation
  FLOW_PARSED = "agent.flow_parsed",
  FLOW_COMPILED = "agent.flow_compiled",
  FLOW_VALIDATED = "agent.flow_validated",
  FLOW_ENRICHED = "agent.flow_enriched",
  
  // Execution
  EXECUTION_STARTED = "agent.execution_started",
  EXECUTION_COMPLETED = "agent.execution_completed",
  EXECUTION_FAILED = "agent.execution_failed",
  
  // Template
  TEMPLATE_CREATED = "agent.template_created",
  TEMPLATE_INSTANTIATED = "agent.template_instantiated",
  
  // Performance
  PERFORMANCE_UPDATED = "agent.performance_updated"
}
```

### Workflow Example: Create Agent from Markdown Prompt

```xml
<workflow id="agent_create_from_markdown">
  <entry p="agent" x="create_from_prompt" node="parse_markdown"/>
  
  <nodes>
    <node id="parse_markdown" kind="external" op="0x2050">
      <parse content="$input.prompt_markdown"/>
      <event type="agent.flow_parsed"/>
    </node>
    
    <node id="validate_flow" kind="external" op="0x2144">
      <validate flow="$parsed.flow"/>
      <event type="agent.flow_validated"/>
    </node>
    
    <node id="enrich_flow" kind="external" op="0x2114">
      <load_schema config="default-kernel-config"/>
      <apply_defaults schema="$kernel_schema" flow="$validated_flow"/>
      <event type="agent.flow_enriched"/>
    </node>
    
    <node id="create_agent" kind="external" op="0x2100">
      <data>
        <field name="name" value="$input.agent_name"/>
        <field name="description" value="Generated from markdown prompt"/>
        <field name="config" value="$enriched_flow"/>
        <field name="status" value="draft"/>
      </data>
      <event type="agent.created"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="parse_markdown" to="validate_flow"><when><always/></when></edge>
    <edge from="validate_flow" to="enrich_flow"><when><always/></when></edge>
    <edge from="enrich_flow" to="create_agent"><when><always/></when></edge>
    <edge from="create_agent" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 2: AGENT SERVICE (CRM Operations)

### Purpose
Facade over AgentOps - converts intents to CRM operations via Business Council.

### P-Axis: Agent Service Entities

```xml
<schema id="agent_crm_operation">
  <field name="id" type="uuid" required="true"/>
  <field name="type" type="enum" values="create,update,delete,link,transition"/>
  <field name="entity_type" type="enum" values="contact,task,deal,workflow"/>
  <field name="entity_id" type="uuid"/>
  <field name="data" type="object"/>
  <field name="changes" type="object"/>
  <field name="confidence" type="number"/>
  <field name="pxyz" type="object" required="true"/>
</schema>
```

### X-Axis: Agent Service Operations

```yaml
# Intent Analysis
intent_analyze: 0x2200              # Analyze natural language intent
intent_to_operations: 0x2201        # Convert intent → CRM ops
operations_execute_batch: 0x2202    # Execute multiple operations
operation_execute_single: 0x2203    # Execute one operation

# CRM Operation Types (reuse from core)
# These delegate to domain operations
crm_contact_create: 0x0100          # Delegate to contact.create
crm_contact_update: 0x0102          # Delegate to contact.update
crm_task_create: 0x0200             # Delegate to task.create
crm_deal_create: 0x0400             # Delegate to deal.create
```

### Workflow Example: Intent to CRM Operation

```xml
<workflow id="agent_intent_to_crm">
  <entry p="agent" x="intent_to_operations" node="analyze_intent"/>
  
  <nodes>
    <node id="analyze_intent" kind="external" op="0x2010">
      <agent_id value="strategist"/>
      <context>
        <query value="$input.intent"/>
        <prompt>
          Analyze this CRM intent and identify operations:
          - Entity type (contact, task, deal)
          - Operation type (create, update, delete)
          - Data fields mentioned
          - Status transitions
          - Relationships to link
        </prompt>
      </context>
      <event type="agent.deliberation_completed"/>
    </node>
    
    <node id="parse_synthesis" kind="transform">
      <extract_operations from="$analyze_intent.statement">
        <pattern type="create" entity="contact">
          <regex>create.*contact.*name:\s*(.+)</regex>
        </pattern>
        <pattern type="update" entity="task">
          <regex>update.*task.*status:\s*(.+)</regex>
        </pattern>
      </extract_operations>
    </node>
    
    <node id="build_operation_plans" kind="transform">
      <for_each operation="$extracted_operations">
        <build_plan>
          <field name="operation_type" value="$operation.type"/>
          <field name="entity_type" value="$operation.entity"/>
          <field name="data" value="$operation.extracted_data"/>
        </build_plan>
      </for_each>
    </node>
    
    <node id="execute_operations" kind="external" op="0x2202">
      <for_each plan="$operation_plans">
        <execute plan="$plan"/>
        <event type="crm.operation.executed"/>
      </for_each>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="analyze_intent" to="parse_synthesis"><when><always/></when></edge>
    <edge from="parse_synthesis" to="build_operation_plans"><when><always/></when></edge>
    <edge from="build_operation_plans" to="execute_operations"><when><always/></when></edge>
    <edge from="execute_operations" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 3: AGENT OPS (Intent → Operation Mapper)

### Purpose
Thin adapter: natural language → structured CRM operations.

### X-Axis: Agent Ops Operations

```yaml
# Already defined above, but key ones:
intent_analyze: 0x2200              # Map intent to operation type
operation_build: 0x2201             # Build operation parameters
operation_execute_from_intent: 0x2202  # End-to-end execution
```

### Y-Axis: Agent Ops Predicates

```xml
<predicates>
  <predicate id="is_valid_intent">
    <and>
      <not_empty left="$intent"/>
      <matches_crm_pattern left="$intent"/>
    </and>
  </predicate>
  
  <predicate id="has_high_confidence">
    <gte left="$analysis.confidence" right="0.85"/>
  </predicate>
  
  <predicate id="requires_confirmation">
    <or>
      <eq left="$operation_type" right="delete"/>
      <lt left="$analysis.confidence" right="0.7"/>
    </or>
  </predicate>
</predicates>
```

### Z-Axis: Agent Ops Events

```typescript
enum AgentOpsEventType {
  INTENT_ANALYZED = "agent_ops.intent_analyzed",
  OPERATION_BUILT = "agent_ops.operation_built",
  OPERATION_EXECUTED = "agent_ops.operation_executed",
  OPERATION_FAILED = "agent_ops.operation_failed",
  CRM_OPERATION_EXECUTED = "crm.operation.executed"
}
```

### Intent Analysis Rules (from config)

```json
{
  "crmOperationMapping": {
    "create_contact": "CREATE",
    "update_contact": "UPDATE",
    "delete_contact": "DELETE",
    "create_task": "CREATE",
    "update_task": "UPDATE",
    "delete_task": "DELETE",
    "create_deal": "CREATE",
    "update_deal": "UPDATE",
    "link_contact_task": "LINK",
    "transition_contact_status": "TRANSITION",
    "transition_task_status": "TRANSITION"
  },
  "confidence": {
    "explicitIntent": 0.95,
    "implicitIntent": 0.85,
    "linkOperation": 0.9
  }
}
```

---

## PART 4: AUDIT QUERY SERVICE

### Purpose
Query EventBus for audit trail - no separate logging needed.

### P-Axis: Audit Entities

```xml
<schema id="audit_query_filter">
  <field name="entity_id" type="uuid"/>
  <field name="actor_id" type="uuid"/>
  <field name="event_type" type="string"/>
  <field name="entity_type" type="string"/>
  <field name="start_time" type="timestamp"/>
  <field name="end_time" type="timestamp"/>
  <field name="limit" type="integer"/>
  <field name="offset" type="integer"/>
  <field name="where" type="object"/>
</schema>

<schema id="audit_query_result">
  <field name="events" type="array" required="true"/>
  <field name="total_count" type="integer"/>
  <field name="filtered_count" type="integer"/>
</schema>
```

### X-Axis: Audit Query Operations

```yaml
audit_query: 0x2300                 # Query audit trail
audit_entity_trail: 0x2301          # Get entity history
audit_actor_trail: 0x2302           # Get actor actions
audit_events_by_type: 0x2303        # Get events by type
audit_recent_activity: 0x2304       # Get recent events
audit_summary: 0x2305               # Get summary stats
audit_entity_modified_by: 0x2306    # Check if entity modified
audit_operation_history: 0x2307     # Get operation sequence
```

### Y-Axis: Audit Predicates

```xml
<predicates>
  <predicate id="is_within_time_range">
    <and>
      <gte left="$event.timestamp" right="$filter.start_time"/>
      <lte left="$event.timestamp" right="$filter.end_time"/>
    </and>
  </predicate>
  
  <predicate id="matches_audit_filter">
    <and>
      <if_present field="entity_id">
        <eq left="$event.entity_id" right="$filter.entity_id"/>
      </if_present>
      <if_present field="actor_id">
        <eq left="$event.actor_id" right="$filter.actor_id"/>
      </if_present>
      <if_present field="event_type">
        <eq left="$event.type" right="$filter.event_type"/>
      </if_present>
    </and>
  </predicate>
  
  <predicate id="has_audit_access">
    <or>
      <eq left="$token.sub" right="$filter.actor_id"/>
      <contains left="$token.roles" right="admin"/>
      <contains left="$token.roles" right="auditor"/>
    </or>
  </predicate>
</predicates>
```

### Workflow Example: Entity Audit Trail

```xml
<workflow id="audit_get_entity_trail">
  <entry p="audit" x="entity_trail" node="validate_access"/>
  
  <nodes>
    <node id="validate_access" kind="auth">
      <require predicate="has_audit_access"/>
    </node>
    
    <node id="query_events" kind="external" op="0x2300">
      <filter>
        <field name="entity_id" value="$input.entity_id"/>
        <field name="limit" value="1000"/>
      </filter>
    </node>
    
    <node id="sort_by_timestamp" kind="transform">
      <sort events="$query_events.events" by="timestamp" order="asc"/>
    </node>
    
    <node id="render_timeline" kind="render">
      <template ref="audit_timeline">
        <events value="$sorted_events"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_access" to="query_events"><when><always/></when></edge>
    <edge from="query_events" to="sort_by_timestamp"><when><always/></when></edge>
    <edge from="sort_by_timestamp" to="render_timeline"><when><always/></when></edge>
    <edge from="render_timeline" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 5: INTEGRATION WITH OMAR

### How Agent Core Fits in CRM

```
User Intent
    ↓
AgentService.intentToOperations
    ↓
Business Council Deliberation (0x2010-0x2012)
    ↓
AgentOps.analyzeIntent (0x2200)
    ↓
AgentOps.buildOperation (0x2201)
    ↓
Domain Operation Execution (0x0100-0x1000)
    ↓
EventBus.emit
    ↓
AuditQueryService (0x2300-0x2307)
```

### Agent Builder Flow

```
Markdown Prompt
    ↓
PromptParser (0x2050-0x2052)
    ↓
FlowDefinition
    ↓
KernelSchemaValidator (0x2144)
    ↓
AgentBuilder.createAgent (0x2100)
    ↓
AgentHarness.orchestrate (0x2000)
    ↓
Execution (0x2120)
```

---

## PART 6: OPERATION CODE SUMMARY

| Service | Range | Count | Purpose |
|---------|-------|-------|---------|
| **Agent Builder** | 0x2100-0x2151 | 26 | Agent lifecycle, templates, flows |
| **Agent Service** | 0x2200-0x2203 | 4 | Intent → CRM ops |
| **Agent Ops** | 0x2200-0x2202 | 3 | Intent parsing |
| **Audit Query** | 0x2300-0x2307 | 8 | Audit trail queries |

**Total New Operations**: 41

---

## PART 7: CONFIG-DRIVEN BEHAVIOR

All agent core services use config.json for:

### Agent Builder Config
```json
{
  "defaultAiConfig": {
    "model": "anthropic/claude-3.5-sonnet",
    "maxTokens": 2000,
    "temperature": 0.7
  },
  "confidence": {
    "councilDeliberation": 0.95,
    "nativeTools": 0.9
  },
  "memory": {
    "capacity": 10000,
    "vectorDimensions": 1536
  }
}
```

### Agent Ops Config
```json
{
  "confidence": {
    "explicitIntent": 0.95,
    "implicitIntent": 0.85,
    "linkOperation": 0.9
  }
}
```

### Audit Query Config
```json
{
  "limits": {
    "default": 50,
    "max": 1000
  }
}
```

---

## SUMMARY

Agent Core Services provide:

1. **Agent Builder**: Control plane for agent definitions
   - Create from configs, prompts (markdown/POML), templates
   - Validate against KernelSchema
   - Store/manage agent metadata

2. **Agent Service**: CRM operation facade
   - Convert natural language → CRM operations
   - Use Business Council for NLP
   - Delegate to domain operations

3. **Agent Ops**: Intent → operation mapper
   - Parse intent to operation type
   - Build operation parameters
   - Execute via domain programs

4. **Audit Query**: Event trail queries
   - Query EventBus (no separate audit log)
   - Filter by entity, actor, type, time
   - Generate audit reports

All services are **already PXYZ-native**:
- Use PxyzJson for data
- Emit events for all state changes
- PXYZ coordinates for tracing
- Config-driven behavior

**Integration**: AgentService → AgentOps → Domain Operations → EventBus → AuditQuery
