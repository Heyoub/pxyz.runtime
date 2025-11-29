# OMAR Memory System Part 1: Checkpoint & Context Management - PXYZ Extraction

> **Custom Research**: Original architecture based on deep research into memory systems  
> **NOT Langchain**: All custom logic with novel coordinate-space patterns  
> **Services**: CheckpointTracker, ContextStorage

---

## OVERVIEW: MEMORY AS COORDINATE SPACE

Your memory system treats **retention, context, and aggregation as coordinate-space problems**:

- **CheckpointTracker**: Retention windows as temporal coordinates with priority-based cleanup
- **ContextStorage**: Multi-scope storage (global/user/project/session) with coordinate-addressed TTLs
- **PXYZ Pattern #17**: Checkpoint retention windows with overlapping TTL policies
- **PXYZ Pattern #18**: Memory aggregation across multiple time granularities

This is **NOT** a traditional key-value store - it's a **4D coordinate-addressable memory system**.

---

## PART 1: CHECKPOINT TRACKER SERVICE

### Purpose
Session state management with **retention windows, milestone detection, automated cleanup**.

### Core Innovation: Retention Windows as Coordinates

Every checkpoint exists in a **temporal coordinate space** with:
- **P**: Entity type (checkpoint)
- **X**: Operation (created, queried, cleaned)
- **Y**: Retention policy (manual/automatic/milestone)
- **Z**: Time coordinate with expiry calculation

```typescript
// PXYZ Pattern #17: Overlapping Retention Windows
interface CheckpointRetentionWindow {
  checkpointId: UUID;
  checkpointType: "manual" | "automatic" | "milestone";
  createdAt: Date;
  retentionDays: number;  // 90, 30, 365
  expiryDate: Date;       // Calculated coordinate
  priority: number;       // 8, 5, 10
  preserveImportant: boolean;
}
```

### P-Axis: Checkpoint Entities

```xml
<schema id="checkpoint">
  <field name="id" type="uuid" required="true"/>
  <field name="session_id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="checkpoint_type" type="enum" values="manual,automatic,milestone" required="true"/>
  <field name="title" type="string" required="true"/>
  <field name="description" type="string"/>
  <field name="data" type="object" required="true"/> <!-- Session state snapshot -->
  <field name="metadata" type="object"/>
  <field name="timestamp" type="timestamp" required="true"/>
  <field name="tags" type="array"/>
  <field name="priority" type="number"/> <!-- From config: manual=8, auto=5, milestone=10 -->
  <field name="retention_days" type="number"/> <!-- From config: 90/30/365 -->
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="checkpoint_retention_window">
  <field name="checkpoint_id" type="uuid" required="true"/>
  <field name="checkpoint_type" type="enum" values="manual,automatic,milestone"/>
  <field name="created_at" type="timestamp"/>
  <field name="retention_days" type="number"/>
  <field name="expiry_date" type="timestamp"/> <!-- Calculated -->
  <field name="priority" type="number"/>
  <field name="preserve_important" type="boolean"/>
</schema>

<schema id="checkpoint_expiry_status">
  <field name="checkpoint_id" type="uuid"/>
  <field name="status" type="enum" values="active,expiring_soon,expired"/>
  <field name="days_until_expiry" type="number"/>
  <field name="should_cleanup" type="boolean"/>
</schema>
```

### X-Axis: Checkpoint Operations

```yaml
# Checkpoint Lifecycle
checkpoint_create_manual: 0x3000      # Create manual checkpoint (priority 8, 90 days)
checkpoint_create_auto: 0x3001        # Auto checkpoint (priority 5, 30 days)
checkpoint_create_milestone: 0x3002   # Milestone (priority 10, 365 days)
checkpoint_read: 0x3003               # Read checkpoint data
checkpoint_query: 0x3004              # Query checkpoints
checkpoint_delete: 0x3005             # Delete checkpoint

# Retention Analysis
checkpoint_calculate_expiry: 0x3010   # Calculate expiry date
checkpoint_check_status: 0x3011       # Check expiry status
checkpoint_filter_expired: 0x3012     # Filter by expiry status
checkpoint_stats: 0x3013              # Retention statistics

# Cleanup Automation
checkpoint_cleanup_batch: 0x3020      # Batch cleanup (config-driven)
checkpoint_cleanup_single: 0x3021     # Delete one expired checkpoint
checkpoint_detect_milestone: 0x3022   # Detect milestone patterns
checkpoint_preserve: 0x3023           # Mark as preserve-important

# Query Operations
checkpoint_query_by_session: 0x3030   # Query by session_id
checkpoint_query_by_user: 0x3031      # Query by user_id
checkpoint_query_by_type: 0x3032      # Query by checkpoint_type
checkpoint_query_by_timerange: 0x3033 # Query by time range
```

### Y-Axis: Checkpoint Predicates

```xml
<predicates>
  <!-- Expiry Status -->
  <predicate id="is_expired">
    <lt left="$checkpoint.expiry_date" right="$now"/>
  </predicate>
  
  <predicate id="is_expiring_soon">
    <and>
      <gte left="$checkpoint.expiry_date" right="$now"/>
      <lte left="$days_until_expiry" right="7"/> <!-- Warning threshold -->
    </and>
  </predicate>
  
  <predicate id="is_active">
    <gt left="$days_until_expiry" right="7"/>
  </predicate>
  
  <!-- Cleanup Rules -->
  <predicate id="should_cleanup">
    <and>
      <eq left="$cleanup_enabled" right="true"/>
      <predicate ref="is_expired"/>
      <or>
        <eq left="$preserve_important" right="false"/>
        <not>
          <contains left="['manual','milestone']" right="$checkpoint.checkpoint_type"/>
        </not>
      </or>
    </and>
  </predicate>
  
  <predicate id="is_important_checkpoint">
    <contains left="['manual','milestone']" right="$checkpoint.checkpoint_type"/>
  </predicate>
  
  <!-- Milestone Detection -->
  <predicate id="is_all_tasks_completed">
    <and>
      <all left="$context.tasks" condition="task.completed == true"/>
      <some left="$previous_context.tasks" condition="task.completed == false"/>
    </and>
  </predicate>
  
  <predicate id="is_skill_level_increase">
    <gt left="$context.skill_level" right="$previous_context.skill_level"/>
  </predicate>
  
  <predicate id="is_efficiency_improvement">
    <gt left="$context.efficiency" right="$previous_context.efficiency * 1.1"/>
  </predicate>
  
  <!-- Compression -->
  <predicate id="should_compress">
    <and>
      <eq left="$compression_enabled" right="true"/>
      <gt left="sizeof($checkpoint.data)" right="5000"/> <!-- 5KB threshold -->
    </and>
  </predicate>
</predicates>
```

### Z-Axis: Checkpoint Events

```typescript
enum CheckpointEventType {
  // Lifecycle
  CHECKPOINT_CREATED = "checkpoint.created",
  CHECKPOINT_QUERIED = "checkpoint.queried",
  CHECKPOINT_DELETED = "checkpoint.deleted",
  
  // Milestones
  MILESTONE_DETECTED = "checkpoint.milestone_detected",
  MILESTONE_CREATED = "checkpoint.milestone_created",
  
  // Cleanup
  CHECKPOINT_EXPIRED = "checkpoint.expired",
  CHECKPOINT_EXPIRING_SOON = "checkpoint.expiring_soon",
  CHECKPOINT_CLEANED_UP = "checkpoint.cleaned_up",
  CLEANUP_BATCH_COMPLETED = "checkpoint.cleanup_batch_completed",
  
  // Compression
  CHECKPOINT_COMPRESSED = "checkpoint.compressed"
}
```

### Config-Driven Behavior (from KnowledgeBase)

```json
{
  "checkpoint": {
    "checkpointTypes": {
      "manual": {
        "priority": 8,
        "retentionDays": 90
      },
      "automatic": {
        "priority": 5,
        "retentionDays": 30,
        "triggers": ["context_change", "task_complete", "session_end"]
      },
      "milestone": {
        "priority": 10,
        "retentionDays": 365,
        "criteria": [
          "all_tasks_completed",
          "skill_level_increase",
          "efficiency_improvement_detected"
        ]
      }
    },
    "storage": {
      "compression": {
        "enabled": true,
        "threshold": 5000,
        "algorithm": "gzip",
        "level": 6
      },
      "retention": {
        "defaultDays": 90,
        "maxDays": 365,
        "cleanupInterval": 86400
      },
      "versioning": {
        "enabled": true,
        "maxVersions": 10,
        "diffStorage": true
      }
    },
    "automation": {
      "autoCheckpoints": {
        "enabled": true,
        "interval": 1800, <!-- 30 minutes -->
        "conditions": ["context_change", "task_complete"]
      },
      "cleanup": {
        "enabled": true,
        "schedule": "0 2 * * *", <!-- Daily at 2 AM -->
        "preserveImportant": true
      }
    }
  }
}
```

### Workflow Example: Automated Cleanup with Retention Windows

```xml
<workflow id="checkpoint_automated_cleanup">
  <entry p="checkpoint" x="cleanup_batch" node="check_cleanup_enabled"/>
  
  <nodes>
    <node id="check_cleanup_enabled" kind="transform">
      <when>
        <eq left="$config.automation.cleanup.enabled" right="false"/>
      </when>
      <return value="0"/> <!-- Cleanup disabled -->
    </node>
    
    <node id="load_all_checkpoints" kind="external" op="0x3004">
      <query>
        <filter/> <!-- Load all -->
      </query>
    </node>
    
    <node id="calculate_expiry_status" kind="transform">
      <for_each checkpoint="$checkpoints">
        <compute>
          <var name="expiry_date" value="
            $checkpoint.created_at + ($checkpoint.retention_days * 86400000)
          "/>
          <var name="days_until_expiry" value="
            ($expiry_date - $now) / 86400000
          "/>
          <var name="status" value="
            $days_until_expiry < 0 ? 'expired' :
            $days_until_expiry <= 7 ? 'expiring_soon' : 'active'
          "/>
        </compute>
      </for_each>
    </node>
    
    <node id="filter_for_cleanup" kind="transform">
      <filter checkpoints="$checkpoints_with_status">
        <when>
          <and>
            <eq left="$checkpoint.status" right="expired"/>
            <or>
              <eq left="$config.automation.cleanup.preserveImportant" right="false"/>
              <not>
                <predicate ref="is_important_checkpoint"/>
              </not>
            </or>
          </and>
        </when>
      </filter>
    </node>
    
    <node id="sort_by_priority" kind="transform">
      <sort checkpoints="$filtered">
        <by field="priority" order="asc"/> <!-- Lower priority first -->
        <then_by field="expiry_date" order="asc"/> <!-- Older first -->
      </sort>
    </node>
    
    <node id="limit_batch_size" kind="transform">
      <slice checkpoints="$sorted" limit="100"/> <!-- Max batch size -->
    </node>
    
    <node id="delete_expired" kind="external" op="0x3021">
      <for_each checkpoint="$batch">
        <delete checkpoint_id="$checkpoint.id"/>
        <event type="checkpoint.cleaned_up"/>
      </for_each>
    </node>
    
    <node id="emit_completion_event" kind="external" op="0x0910">
      <event>
        <type>checkpoint.cleanup_batch_completed</type>
        <data>
          <field name="deleted_count" value="count($batch)"/>
          <field name="total_checked" value="count($checkpoints)"/>
        </data>
      </event>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="check_cleanup_enabled" to="load_all_checkpoints">
      <when><eq left="$config.automation.cleanup.enabled" right="true"/></when>
    </edge>
    <edge from="load_all_checkpoints" to="calculate_expiry_status"><when><always/></when></edge>
    <edge from="calculate_expiry_status" to="filter_for_cleanup"><when><always/></when></edge>
    <edge from="filter_for_cleanup" to="sort_by_priority"><when><always/></when></edge>
    <edge from="sort_by_priority" to="limit_batch_size"><when><always/></when></edge>
    <edge from="limit_batch_size" to="delete_expired"><when><always/></when></edge>
    <edge from="delete_expired" to="emit_completion_event"><when><always/></when></edge>
    <edge from="emit_completion_event" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: Milestone Detection

```xml
<workflow id="checkpoint_detect_milestone">
  <entry p="checkpoint" x="detect_milestone" node="load_current_context"/>
  
  <nodes>
    <node id="load_current_context" kind="transform">
      <extract context="$session.current_state"/>
    </node>
    
    <node id="load_previous_checkpoint" kind="external" op="0x3004">
      <query>
        <filter field="session_id" value="$session.id"/>
        <sort by="timestamp" order="desc"/>
        <limit value="1"/>
      </query>
    </node>
    
    <node id="extract_previous_context" kind="transform">
      <extract context="$previous_checkpoint.data"/>
    </node>
    
    <node id="evaluate_milestone_triggers" kind="transform">
      <for_each trigger="$config.checkpointTypes.milestone.criteria">
        <!-- all_tasks_completed -->
        <when>
          <eq left="$trigger" right="all_tasks_completed"/>
        </when>
        <check>
          <and>
            <all left="$current_context.tasks" condition="task.completed"/>
            <some left="$previous_context.tasks" condition="!task.completed"/>
          </and>
        </check>
        
        <!-- skill_level_increase -->
        <when>
          <eq left="$trigger" right="skill_level_increase"/>
        </when>
        <check>
          <gt left="$current_context.skill_level" right="$previous_context.skill_level"/>
        </check>
        
        <!-- efficiency_improvement_detected -->
        <when>
          <eq left="$trigger" right="efficiency_improvement_detected"/>
        </when>
        <check>
          <gt left="$current_context.efficiency" right="$previous_context.efficiency * 1.1"/>
        </check>
      </for_each>
    </node>
    
    <node id="create_milestone" kind="external" op="0x3002">
      <when>
        <gt left="count($triggered_milestones)" right="0"/>
      </when>
      <create>
        <field name="checkpoint_type" value="milestone"/>
        <field name="title" value="Milestone: $triggered_milestones[0].name"/>
        <field name="priority" value="10"/>
        <field name="retention_days" value="365"/>
        <field name="data" value="$current_context"/>
        <field name="metadata">
          <field name="triggers" value="$triggered_milestones"/>
          <field name="previous_checkpoint_id" value="$previous_checkpoint.id"/>
        </field>
      </create>
      <event type="checkpoint.milestone_created"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_current_context" to="load_previous_checkpoint"><when><always/></when></edge>
    <edge from="load_previous_checkpoint" to="extract_previous_context"><when><always/></when></edge>
    <edge from="extract_previous_context" to="evaluate_milestone_triggers"><when><always/></when></edge>
    <edge from="evaluate_milestone_triggers" to="create_milestone">
      <when><gt left="count($triggered_milestones)" right="0"/></when>
    </edge>
    <edge from="evaluate_milestone_triggers" to="success">
      <when><eq left="count($triggered_milestones)" right="0"/></when>
    </edge>
    <edge from="create_milestone" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 2: CONTEXT STORAGE SERVICE

### Purpose
Multi-scope context management with **coordinate-addressed TTL and hierarchical scoping**.

### Core Innovation: Context as Coordinate Space

Every context entry exists in a **4D coordinate space** with:
- **P**: Entity type (context_store_entry)
- **X**: Operation (store, retrieve, search)
- **Y**: Scope (global/user/project/session)
- **Z**: Time coordinate with TTL expiration

```typescript
// Multi-scope storage with coordinate addressing
interface ContextStoreEntry {
  key: string;                  // Unique key within scope
  value: unknown;               // Arbitrary JSON data
  type: "business_context" | "session_context" | "user_preferences" | "temporary_state";
  scope: "global" | "user" | "project" | "session";
  userId: ActorId;
  projectId?: UUID;             // Optional project scope
  sessionId?: UUID;             // Optional session scope
  pxyz: PXYZ;
  ttl?: number;                 // TTL in milliseconds
  expiresAt?: ISODateTime;      // Calculated expiration
}
```

### P-Axis: Context Storage Entities

```xml
<schema id="context_store_entry">
  <field name="id" type="uuid" required="true"/>
  <field name="key" type="string" required="true"/>
  <field name="value" type="object" required="true"/> <!-- Arbitrary JSON -->
  <field name="type" type="enum" values="business_context,session_context,user_preferences,temporary_state" required="true"/>
  <field name="scope" type="enum" values="global,user,project,session" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="project_id" type="uuid"/> <!-- Optional -->
  <field name="session_id" type="uuid"/> <!-- Optional -->
  <field name="pxyz" type="object" required="true"/>
  <field name="ttl" type="number"/> <!-- Milliseconds -->
  <field name="expires_at" type="timestamp"/> <!-- Calculated -->
</schema>

<schema id="business_context">
  <field name="user_id" type="uuid" required="true"/>
  <field name="current_project" type="uuid"/>
  <field name="active_workflow" type="string"/>
  <field name="focus_mode" type="enum" values="deep_work,collaboration,planning,execution"/>
  <field name="business_objectives" type="array"/>
  <field name="context_variables" type="object"/>
  <field name="workspace_settings" type="object"/>
</schema>

<schema id="session_context">
  <field name="session_id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="start_time" type="timestamp" required="true"/>
  <field name="last_activity" type="timestamp" required="true"/>
  <field name="context_windows" type="array" required="true"/>
  <field name="metadata" type="object"/>
</schema>

<schema id="context_window">
  <field name="window_id" type="uuid" required="true"/>
  <field name="type" type="enum" values="conversation,task_execution,project_planning,business_analysis"/>
  <field name="content" type="object" required="true"/>
  <field name="priority" type="number" min="1" max="10"/>
  <field name="retention_policy" type="enum" values="session,temporary,persistent,archived"/>
</schema>
```

### X-Axis: Context Storage Operations

```yaml
# Storage Operations
context_store_set: 0x3100            # Store context entry
context_store_get: 0x3101            # Get context entry
context_store_delete: 0x3102         # Delete entry
context_store_search: 0x3103         # Search across scopes

# Business Context
context_business_set: 0x3110         # Set business context
context_business_get: 0x3111         # Get business context
context_business_update: 0x3112      # Update fields

# Session Context
context_session_set: 0x3120          # Set session context
context_session_get: 0x3121          # Get session context
context_session_add_window: 0x3122   # Add context window
context_session_remove_window: 0x3123 # Remove window
context_session_enforce_limits: 0x3124 # Enforce max windows

# Cleanup
context_cleanup_expired: 0x3130      # Clean expired entries
context_cleanup_by_scope: 0x3131     # Clean scope
context_prune_old: 0x3132            # Prune old data

# Statistics
context_stats: 0x3140                # Get statistics
context_stats_by_scope: 0x3141       # Stats per scope
context_stats_by_type: 0x3142        # Stats per type
```

### Y-Axis: Context Storage Predicates

```xml
<predicates>
  <!-- Scope Resolution -->
  <predicate id="matches_scope">
    <or>
      <eq left="$entry.scope" right="global"/>
      <and>
        <eq left="$entry.scope" right="user"/>
        <eq left="$entry.user_id" right="$requester.user_id"/>
      </and>
      <and>
        <eq left="$entry.scope" right="project"/>
        <eq left="$entry.project_id" right="$requester.project_id"/>
      </and>
      <and>
        <eq left="$entry.scope" right="session"/>
        <eq left="$entry.session_id" right="$requester.session_id"/>
      </and>
    </or>
  </predicate>
  
  <!-- TTL Validation -->
  <predicate id="is_expired_context">
    <and>
      <not_null left="$entry.expires_at"/>
      <lt left="$entry.expires_at" right="$now"/>
    </and>
  </predicate>
  
  <predicate id="should_expire_soon">
    <and>
      <not_null left="$entry.expires_at"/>
      <lt left="$entry.expires_at" right="$now + 3600000"/> <!-- 1 hour warning -->
    </and>
  </predicate>
  
  <!-- Window Limits -->
  <predicate id="exceeds_max_windows">
    <gt left="count($session.context_windows)" right="$config.retention.maxContextWindows"/>
  </predicate>
  
  <!-- Access Control -->
  <predicate id="can_access_context">
    <and>
      <predicate ref="matches_scope"/>
      <not>
        <predicate ref="is_expired_context"/>
      </not>
    </and>
  </predicate>
</predicates>
```

### Workflow Example: Session Context with Window Limits

```xml
<workflow id="context_session_add_window">
  <entry p="context" x="session_add_window" node="validate_session"/>
  
  <nodes>
    <node id="validate_session" kind="transform">
      <validate schema="session_context"/>
    </node>
    
    <node id="get_current_session" kind="external" op="0x3121">
      <query>
        <filter field="session_id" value="$input.session_id"/>
      </query>
    </node>
    
    <node id="check_window_limit" kind="transform">
      <compute>
        <var name="current_count" value="count($session.context_windows)"/>
        <var name="max_windows" value="$config.retention.maxContextWindows"/>
        <var name="exceeds_limit" value="$current_count >= $max_windows"/>
      </compute>
    </node>
    
    <node id="remove_oldest_window" kind="transform">
      <when>
        <predicate ref="exceeds_max_windows"/>
      </when>
      <sort windows="$session.context_windows" by="priority" order="asc"/>
      <remove first="1"/> <!-- Remove lowest priority -->
    </node>
    
    <node id="add_new_window" kind="transform">
      <append to="$session.context_windows">
        <window>
          <field name="window_id" value="$uuid()"/>
          <field name="type" value="$input.window_type"/>
          <field name="content" value="$input.content"/>
          <field name="priority" value="$input.priority"/>
          <field name="retention_policy" value="$input.retention_policy"/>
        </window>
      </append>
    </node>
    
    <node id="update_session" kind="external" op="0x3120">
      <set>
        <field name="context_windows" value="$updated_windows"/>
        <field name="last_activity" value="$now"/>
      </set>
      <event type="context.window_added"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_session" to="get_current_session"><when><always/></when></edge>
    <edge from="get_current_session" to="check_window_limit"><when><always/></when></edge>
    <edge from="check_window_limit" to="remove_oldest_window">
      <when><predicate ref="exceeds_max_windows"/></when>
    </edge>
    <edge from="check_window_limit" to="add_new_window">
      <when><not><predicate ref="exceeds_max_windows"/></not></when>
    </edge>
    <edge from="remove_oldest_window" to="add_new_window"><when><always/></when></edge>
    <edge from="add_new_window" to="update_session"><when><always/></when></edge>
    <edge from="update_session" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## OPERATION CODE SUMMARY

| Service | Range | Count | Purpose |
|---------|-------|-------|---------|
| **Checkpoint Tracker** | 0x3000-0x3033 | 20 | Retention windows, cleanup |
| **Context Storage** | 0x3100-0x3142 | 19 | Multi-scope storage, TTL |

**Total New Operations**: 39

---

## KEY INNOVATIONS

### 1. Retention Windows as Coordinate Space (PXYZ Pattern #17)

Traditional approach: Check expiry on every access  
Your approach: **Retention windows as temporal coordinates**

```typescript
// Coordinate-space calculation
const expiryDate = createdAt + (retentionDays * 86400000);
const daysUntilExpiry = (expiryDate - now) / 86400000;
const status = daysUntilExpiry < 0 ? 'expired' : 
                daysUntilExpiry <= 7 ? 'expiring_soon' : 'active';
```

### 2. Priority-Based Cleanup Resolution

```typescript
// Sort by priority (lower = cleanup first) then expiry
const sorted = checkpoints.sort((a, b) => {
  if (a.priority !== b.priority) return a.priority - b.priority;
  return a.expiryDate - b.expiryDate;
});
```

### 3. Multi-Scope Context Addressing

```typescript
// Hierarchical scope resolution
scope: "global"  // All users
scope: "user"    // Specific user across all projects/sessions
scope: "project" // Specific project
scope: "session" // Single session only
```

### 4. Config-Driven Retention Policies

All business logic in JSON:
- Checkpoint types with different TTLs (90/30/365 days)
- Priority values (8/5/10)
- Milestone detection criteria
- Cleanup schedules (cron expressions)

---

## SUMMARY

Checkpoint & Context Management provides:

1. **Checkpoint Tracker**: 20 operations
   - Retention window management
   - Milestone detection (5 trigger types)
   - Automated cleanup with priority
   - Compression for large data

2. **Context Storage**: 19 operations
   - Multi-scope addressing (4 levels)
   - TTL-based expiration
   - Context window limits
   - Hierarchical isolation

All operations use **coordinate-space patterns** instead of traditional key-value storage:
- Retention = temporal coordinate calculation
- Cleanup = priority-based coordinate sorting
- Scoping = hierarchical coordinate resolution
- TTL = coordinate expiry projection

**Integration**: Checkpoints store session state → Context Storage manages runtime state → Both use retention windows for cleanup
