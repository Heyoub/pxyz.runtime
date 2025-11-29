# OMAR Addendum 1: Missing Core Operations

> **Gap Analysis Result**: These features were in the PRDs but not mapped to PXYZ operations in the initial extraction.

---

## IMPORT/EXPORT OPERATIONS

### P-Axis: Import/Export Entities

```xml
<schema id="import_job">
  <field name="id" type="uuid" required="true"/>
  <field name="entity_type" type="enum" values="contact,task,workflow,invoice"/>
  <field name="source_format" type="enum" values="csv,json,excel,vcard"/>
  <field name="status" type="enum" values="pending,mapping,validating,importing,complete,failed"/>
  <field name="total_rows" type="integer"/>
  <field name="imported_count" type="integer"/>
  <field name="failed_count" type="integer"/>
  <field name="duplicate_handling" type="enum" values="skip,merge,create"/>
</schema>
```

### X-Axis: Import/Export Operations

```yaml
# Import Operations
import_start: 0x1100
import_map_columns: 0x1101
import_validate: 0x1102
import_preview: 0x1103
import_execute: 0x1104
import_detect_duplicates: 0x1105
import_merge_duplicate: 0x1106

# Export Operations
export_csv: 0x1110
export_json: 0x1111
export_excel: 0x1112
export_vcard: 0x1113
export_filtered: 0x1114
```

### Y-Axis: Import/Export Predicates

```xml
<predicates>
  <predicate id="can_import_contacts">
    <and>
      <contains left="$token.roles" right="admin"/>
      <or>
        <contains left="$token.roles" right="owner"/>
        <contains left="$token.permissions" right="import_data"/>
      </or>
    </and>
  </predicate>
  
  <predicate id="is_valid_csv">
    <and>
      <eq left="$file.extension" right="csv"/>
      <has_headers left="$file.content"/>
      <lt left="$file.size" right="10485760"/> <!-- 10MB -->
    </and>
  </predicate>
  
  <predicate id="has_duplicate_match">
    <or>
      <exists left="$row.email" in="contacts.email"/>
      <similar left="$row.name" to="contacts.name" threshold="0.85"/>
      <and>
        <exists left="$row.phone" in="contacts.phone"/>
        <not_null left="$row.phone"/>
      </and>
    </or>
  </predicate>
  
  <predicate id="can_merge_fields">
    <and>
      <predicate ref="has_duplicate_match"/>
      <eq left="$import.duplicate_handling" right="merge"/>
      <predicate ref="can_edit_contact"/>
    </and>
  </predicate>
</predicates>
```

### Z-Axis: Import/Export Events

```typescript
enum ImportExportEventType {
  IMPORT_STARTED = "import.started",
  IMPORT_VALIDATED = "import.validated",
  IMPORT_COMPLETED = "import.completed",
  IMPORT_FAILED = "import.failed",
  DUPLICATE_DETECTED = "import.duplicate_detected",
  DUPLICATE_MERGED = "import.duplicate_merged",
  DUPLICATE_SKIPPED = "import.duplicate_skipped",
  EXPORT_REQUESTED = "export.requested",
  EXPORT_COMPLETED = "export.completed"
}
```

### Workflow Example: CSV Contact Import

```xml
<workflow id="contact_csv_import">
  <entry p="import" x="start" node="validate_file"/>
  
  <nodes>
    <!-- Step 1: Validate file -->
    <node id="validate_file" kind="transform">
      <validate>
        <field name="file" type="file" required="true"/>
        <constraint predicate="is_valid_csv"/>
      </validate>
    </node>
    
    <!-- Step 2: Parse CSV -->
    <node id="parse_csv" kind="external" op="0x1100">
      <parse format="csv" encoding="utf-8"/>
    </node>
    
    <!-- Step 3: Column Mapping UI -->
    <node id="map_columns" kind="render">
      <template ref="import_column_mapper">
        <available_fields from="contact_schema"/>
        <detected_headers from="$parsed_csv.headers"/>
      </template>
    </node>
    
    <!-- Step 4: Validate Mapped Data -->
    <node id="validate_data" kind="external" op="0x1102">
      <for_each item="$parsed_csv.rows">
        <validate against="contact_schema"/>
        <collect_errors/>
      </for_each>
    </node>
    
    <!-- Step 5: Detect Duplicates -->
    <node id="detect_duplicates" kind="external" op="0x1105">
      <for_each item="$validated_rows">
        <check predicate="has_duplicate_match"/>
      </for_each>
    </node>
    
    <!-- Step 6: Preview with Duplicates -->
    <node id="preview" kind="render">
      <template ref="import_preview">
        <show field="total_rows"/>
        <show field="valid_rows"/>
        <show field="invalid_rows"/>
        <show field="duplicate_rows"/>
        <action label="Merge Duplicates" when="duplicates_exist"/>
        <action label="Skip Duplicates" when="duplicates_exist"/>
        <action label="Create All" always="true"/>
      </template>
    </node>
    
    <!-- Step 7: Execute Import -->
    <node id="execute_import" kind="external" op="0x1104">
      <for_each item="$approved_rows">
        <when>
          <and>
            <not><predicate ref="has_duplicate_match"/></not>
          </and>
        </when>
        <external op="0x0100"> <!-- contact_create -->
          <event type="contact.created"/>
        </external>
      </for_each>
    </node>
    
    <!-- Step 8: Merge Duplicates -->
    <node id="merge_duplicates" kind="external" op="0x1106">
      <for_each item="$duplicate_rows">
        <when>
          <and>
            <predicate ref="has_duplicate_match"/>
            <eq left="$user_action" right="merge"/>
          </and>
        </when>
        <load_existing contact_id="$match.id"/>
        <merge_fields strategy="$merge_strategy"/>
        <external op="0x0102"> <!-- contact_update -->
          <event type="contact.updated"/>
          <event type="import.duplicate_merged"/>
        </external>
      </for_each>
    </node>
    
    <!-- Step 9: Log Import Completion -->
    <node id="log_completion" kind="external" op="0x0910">
      <event>
        <type>import.completed</type>
        <data>
          <field name="entity_type" value="contact"/>
          <field name="total_rows" value="$parsed_csv.row_count"/>
          <field name="imported" value="$imported_count"/>
          <field name="merged" value="$merged_count"/>
          <field name="skipped" value="$skipped_count"/>
          <field name="failed" value="$failed_count"/>
        </data>
      </event>
    </node>
    
    <!-- Step 10: Render Summary -->
    <node id="render_summary" kind="render">
      <template ref="import_summary"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_file" to="parse_csv"><when><always/></when></edge>
    <edge from="parse_csv" to="map_columns"><when><always/></when></edge>
    <edge from="map_columns" to="validate_data"><when><always/></when></edge>
    <edge from="validate_data" to="detect_duplicates"><when><always/></when></edge>
    <edge from="detect_duplicates" to="preview"><when><always/></when></edge>
    <edge from="preview" to="execute_import"><when><always/></when></edge>
    <edge from="execute_import" to="merge_duplicates">
      <when><gt left="$duplicate_count" right="0"/></when>
    </edge>
    <edge from="execute_import" to="log_completion">
      <when><eq left="$duplicate_count" right="0"/></when>
    </edge>
    <edge from="merge_duplicates" to="log_completion"><when><always/></when></edge>
    <edge from="log_completion" to="render_summary"><when><always/></when></edge>
    <edge from="render_summary" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## CALENDAR INTEGRATION OPERATIONS

### P-Axis: Calendar Entities

```xml
<schema id="calendar_event">
  <field name="id" type="uuid" required="true"/>
  <field name="title" type="string" required="true"/>
  <field name="start_time" type="timestamp" required="true"/>
  <field name="end_time" type="timestamp" required="true"/>
  <field name="attendees" type="array"/>
  <field name="location" type="string"/>
  <field name="description" type="string"/>
  <field name="calendar_provider" type="enum" values="google,outlook,apple"/>
  <field name="provider_event_id" type="string"/>
  <field name="linked_task_id" type="uuid"/>
  <field name="linked_contact_ids" type="array"/>
</schema>
```

### X-Axis: Calendar Operations

```yaml
calendar_sync: 0x1200
calendar_create_event: 0x1201
calendar_update_event: 0x1202
calendar_delete_event: 0x1203
calendar_list_events: 0x1204
calendar_send_invite: 0x1210
calendar_track_rsvp: 0x1211
calendar_propose_time: 0x1212
calendar_find_availability: 0x1213
calendar_time_block_task: 0x1220
calendar_generate_meeting_note: 0x1221
```

### Y-Axis: Calendar Predicates

```xml
<predicates>
  <predicate id="has_calendar_permission">
    <or>
      <contains left="$token.scopes" right="calendar.read"/>
      <contains left="$token.scopes" right="calendar.write"/>
    </or>
  </predicate>
  
  <predicate id="is_time_available">
    <and>
      <no_conflicts left="$proposed_time" in="$user.calendar"/>
      <within_working_hours left="$proposed_time" user="$user"/>
    </and>
  </predicate>
  
  <predicate id="can_send_invite">
    <and>
      <predicate ref="has_calendar_permission"/>
      <not_in_past left="$event.start_time"/>
      <all_valid left="$event.attendees" type="email"/>
    </and>
  </predicate>
</predicates>
```

### Workflow Example: Propose Meeting Time in Email

```xml
<workflow id="email_propose_meeting">
  <entry p="email" x="propose_meeting" node="parse_request"/>
  
  <nodes>
    <node id="parse_request" kind="external" op="0x0800">
      <llm_prompt>
        Extract meeting details from email:
        {{email.body}}
        
        Return JSON:
        {
          "duration_minutes": number,
          "attendees": ["email"],
          "preferred_dates": ["YYYY-MM-DD"],
          "time_preferences": "morning|afternoon|evening"
        }
      </llm_prompt>
    </node>
    
    <node id="find_availability" kind="external" op="0x1213">
      <query_calendar>
        <for_each attendee="$parsed.attendees">
          <load_calendar/>
          <find_free_slots duration="$parsed.duration_minutes"/>
        </for_each>
      </query_calendar>
    </node>
    
    <node id="rank_times" kind="transform">
      <compute>
        <var name="ranked_slots" value="rank_by_convenience($free_slots, $parsed.time_preferences)"/>
        <var name="top_3" value="take($ranked_slots, 3)"/>
      </compute>
    </node>
    
    <node id="draft_proposal" kind="external" op="0x0800">
      <llm_prompt>
        Draft email proposing these meeting times:
        {{#each top_3}}
        - {{datetime}} ({{duration}} minutes)
        {{/each}}
        
        Keep tone professional but friendly.
      </llm_prompt>
    </node>
    
    <node id="render_draft" kind="render">
      <template ref="email_compose">
        <pre_fill field="body" value="$draft"/>
        <action label="Send Proposal"/>
        <action label="Edit First"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="parse_request" to="find_availability"><when><always/></when></edge>
    <edge from="find_availability" to="rank_times"><when><always/></when></edge>
    <edge from="rank_times" to="draft_proposal"><when><always/></when></edge>
    <edge from="draft_proposal" to="render_draft"><when><always/></when></edge>
    <edge from="render_draft" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## SMART LISTS OPERATIONS

### P-Axis: Smart List Entities

```xml
<schema id="smart_list">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="entity_type" type="enum" values="contact,task,workflow,email,document"/>
  <field name="criteria" type="object" required="true"/>
  <field name="owner_id" type="uuid"/>
  <field name="shared_with" type="array"/>
  <field name="notification_enabled" type="boolean"/>
  <field name="notification_frequency" type="enum" values="realtime,daily,weekly"/>
  <field name="last_evaluated" type="timestamp"/>
  <field name="member_count" type="computed"/>
</schema>
```

### X-Axis: Smart List Operations

```yaml
smart_list_create: 0x1300
smart_list_read: 0x1301
smart_list_update: 0x1302
smart_list_delete: 0x1303
smart_list_evaluate: 0x1310
smart_list_subscribe: 0x1311
smart_list_get_members: 0x1312
smart_list_detect_changes: 0x1313
```

### Y-Axis: Smart List Predicates

```xml
<predicates>
  <predicate id="matches_smart_list_criteria">
    <!-- Dynamically evaluate stored criteria against entity -->
    <eval_dynamic criteria="$smart_list.criteria" entity="$entity"/>
  </predicate>
  
  <predicate id="should_notify_list_change">
    <and>
      <eq left="$smart_list.notification_enabled" right="true"/>
      <or>
        <gt left="$new_member_count" right="$old_member_count"/>
        <lt left="$new_member_count" right="$old_member_count"/>
      </or>
    </and>
  </predicate>
  
  <predicate id="can_share_smart_list">
    <and>
      <predicate ref="can_view_all_members"/>
      <or>
        <eq left="$token.sub" right="$smart_list.owner_id"/>
        <contains left="$token.roles" right="admin"/>
      </or>
    </and>
  </predicate>
</predicates>
```

### Workflow Example: Create and Subscribe to Smart List

```xml
<workflow id="smart_list_create_subscribe">
  <entry p="smart_list" x="create" node="validate_criteria"/>
  
  <nodes>
    <node id="validate_criteria" kind="transform">
      <validate>
        <field name="name" type="string" required="true"/>
        <field name="entity_type" type="enum" required="true"/>
        <field name="criteria" type="object" required="true"/>
      </validate>
    </node>
    
    <node id="test_criteria" kind="external" op="0x1310">
      <!-- Evaluate criteria against sample entities to verify it works -->
      <sample_evaluate entity_type="$input.entity_type" criteria="$input.criteria"/>
    </node>
    
    <node id="create_list" kind="external" op="0x0910">
      <event>
        <type>smart_list.created</type>
        <data>
          <field name="id" value="$uuid()"/>
          <field name="name" value="$input.name"/>
          <field name="entity_type" value="$input.entity_type"/>
          <field name="criteria" value="$input.criteria"/>
          <field name="owner_id" value="$token.sub"/>
        </data>
      </event>
    </node>
    
    <node id="evaluate_initial" kind="external" op="0x1310">
      <query>
        <type value="$smart_list.entity_type"/>
        <where criteria="$smart_list.criteria"/>
      </query>
    </node>
    
    <node id="subscribe_if_requested" kind="external" op="0x1311">
      <when>
        <eq left="$input.subscribe" right="true"/>
      </when>
      <event>
        <type>smart_list.subscribed</type>
        <data>
          <field name="list_id" value="$smart_list.id"/>
          <field name="user_id" value="$token.sub"/>
          <field name="frequency" value="$input.notification_frequency"/>
        </data>
      </event>
    </node>
    
    <node id="render_list" kind="render">
      <template ref="smart_list_view">
        <show field="name"/>
        <show field="member_count"/>
        <show field="members" limit="20"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_criteria" to="test_criteria"><when><always/></when></edge>
    <edge from="test_criteria" to="create_list"><when><always/></when></edge>
    <edge from="create_list" to="evaluate_initial"><when><always/></when></edge>
    <edge from="evaluate_initial" to="subscribe_if_requested">
      <when><eq left="$input.subscribe" right="true"/></when>
    </edge>
    <edge from="evaluate_initial" to="render_list">
      <when><ne left="$input.subscribe" right="true"/></when>
    </edge>
    <edge from="subscribe_if_requested" to="render_list"><when><always/></when></edge>
    <edge from="render_list" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: Smart List Change Detection

```xml
<workflow id="smart_list_detect_changes">
  <!-- This runs as a background job -->
  <entry p="smart_list" x="detect_changes" node="load_all_lists"/>
  
  <nodes>
    <node id="load_all_lists" kind="external" op="0x0911">
      <query>
        <type value="smart_list"/>
        <filter field="notification_enabled" value="true"/>
      </query>
    </node>
    
    <node id="evaluate_each_list" kind="external" op="0x1310">
      <for_each item="$smart_lists">
        <query>
          <type value="$item.entity_type"/>
          <where criteria="$item.criteria"/>
        </query>
        <compare>
          <old value="$item.last_member_ids"/>
          <new value="$current_member_ids"/>
        </compare>
      </for_each>
    </node>
    
    <node id="detect_new_members" kind="transform">
      <for_each item="$evaluated_lists">
        <compute>
          <var name="added" value="$new_member_ids - $old_member_ids"/>
          <var name="removed" value="$old_member_ids - $new_member_ids"/>
        </compute>
      </for_each>
    </node>
    
    <node id="notify_subscribers" kind="external" op="0x0300">
      <for_each item="$lists_with_changes">
        <when>
          <predicate ref="should_notify_list_change"/>
        </when>
        <notification>
          <recipients from="$item.subscribers"/>
          <template ref="smart_list_changed"/>
          <data>
            <field name="list_name" value="$item.name"/>
            <field name="added_count" value="count($item.added)"/>
            <field name="removed_count" value="count($item.removed)"/>
          </data>
        </notification>
      </for_each>
    </node>
    
    <node id="update_last_evaluated" kind="external" op="0x1302">
      <for_each item="$evaluated_lists">
        <update field="last_evaluated" value="$now"/>
        <update field="last_member_ids" value="$current_member_ids"/>
      </for_each>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_all_lists" to="evaluate_each_list"><when><always/></when></edge>
    <edge from="evaluate_each_list" to="detect_new_members"><when><always/></when></edge>
    <edge from="detect_new_members" to="notify_subscribers">
      <when><gt left="count($lists_with_changes)" right="0"/></when>
    </edge>
    <edge from="detect_new_members" to="update_last_evaluated">
      <when><eq left="count($lists_with_changes)" right="0"/></when>
    </edge>
    <edge from="notify_subscribers" to="update_last_evaluated"><when><always/></when></edge>
    <edge from="update_last_evaluated" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## RECURRING TASKS OPERATIONS

### P-Axis: Recurring Task Configuration

```xml
<schema id="recurring_task">
  <field name="id" type="uuid" required="true"/>
  <field name="template_task_id" type="uuid" required="true"/>
  <field name="recurrence_pattern" type="enum" values="daily,weekly,monthly,custom"/>
  <field name="interval" type="integer"/> <!-- every N days/weeks/months -->
  <field name="days_of_week" type="array"/> <!-- for weekly -->
  <field name="day_of_month" type="integer"/> <!-- for monthly -->
  <field name="end_condition" type="enum" values="never,after_count,on_date"/>
  <field name="end_after_count" type="integer"/>
  <field name="end_on_date" type="timestamp"/>
  <field name="next_occurrence" type="timestamp"/>
  <field name="is_active" type="boolean"/>
</schema>
```

### X-Axis: Recurring Task Operations

```yaml
recurring_task_create: 0x1400
recurring_task_update: 0x1401
recurring_task_pause: 0x1402
recurring_task_resume: 0x1403
recurring_task_delete: 0x1404
recurring_task_generate_instance: 0x1410
recurring_task_calculate_next: 0x1411
```

### Workflow Example: Generate Recurring Task Instances

```xml
<workflow id="recurring_task_generate">
  <!-- Background job runs every hour -->
  <entry p="recurring_task" x="generate_instance" node="load_due_recurrences"/>
  
  <nodes>
    <node id="load_due_recurrences" kind="external" op="0x0911">
      <query>
        <type value="recurring_task"/>
        <filter field="is_active" value="true"/>
        <filter field="next_occurrence" lte="$now"/>
      </query>
    </node>
    
    <node id="generate_tasks" kind="external" op="0x1410">
      <for_each item="$due_recurrences">
        <!-- Load template task -->
        <load_template task_id="$item.template_task_id"/>
        
        <!-- Create new task instance -->
        <external op="0x0200"> <!-- task_create -->
          <data>
            <field name="title" value="$template.title"/>
            <field name="description" value="$template.description"/>
            <field name="owner_id" value="$template.owner_id"/>
            <field name="due_date" value="$item.next_occurrence"/>
            <field name="recurring_task_id" value="$item.id"/>
          </data>
          <event type="task.created"/>
          <event type="recurring_task.instance_generated"/>
        </external>
      </for_each>
    </node>
    
    <node id="calculate_next_occurrence" kind="external" op="0x1411">
      <for_each item="$generated_recurrences">
        <compute>
          <var name="next" value="calculate_next_occurrence($item)"/>
        </compute>
        <update field="next_occurrence" value="$next"/>
      </for_each>
    </node>
    
    <node id="check_end_condition" kind="transform">
      <for_each item="$updated_recurrences">
        <when>
          <or>
            <and>
              <eq left="$item.end_condition" right="on_date"/>
              <gte left="$item.next_occurrence" right="$item.end_on_date"/>
            </and>
            <and>
              <eq left="$item.end_condition" right="after_count"/>
              <gte left="$item.generated_count" right="$item.end_after_count"/>
            </and>
          </or>
        </when>
        <update field="is_active" value="false"/>
        <event type="recurring_task.completed"/>
      </for_each>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_due_recurrences" to="generate_tasks"><when><always/></when></edge>
    <edge from="generate_tasks" to="calculate_next_occurrence"><when><always/></when></edge>
    <edge from="calculate_next_occurrence" to="check_end_condition"><when><always/></when></edge>
    <edge from="check_end_condition" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## INTEGRATION WITH EXISTING OPERATIONS

All of these new operations integrate with the existing PXYZ coordinate system:

### Import/Export Integration
- Contacts can be imported: `P=import, X=start, Y={entity_type: contact, file}`
- Tasks exported to CSV: `P=export, X=csv, Y={entity_type: task, filter}`

### Calendar Integration
- Tasks time-block on calendar: `P=calendar, X=time_block_task, Y={task_id}`
- Meetings generate notes: `P=calendar, X=generate_meeting_note, Y={event_id}`

### Smart Lists Integration
- Contact smart lists: `P=smart_list, X=create, Y={entity_type: contact, criteria}`
- Task smart lists: `P=smart_list, X=create, Y={entity_type: task, criteria}`

### Recurring Tasks Integration
- Weekly standup: `P=recurring_task, X=create, Y={pattern: weekly, days: [monday]}`
- Monthly review: `P=recurring_task, X=create, Y={pattern: monthly, day: 1}`

---

## IO Adapter Extensions

Add these handlers to `io-browser.ts`:

```typescript
// Import/Export handlers
0x1100: async (ctx) => {
  // import_start - parse uploaded file
  const file = ctx.input.file;
  const parsed = await parseCSV(file);
  return { headers: parsed.headers, rows: parsed.rows };
},

0x1105: async (ctx) => {
  // import_detect_duplicates
  const rows = ctx.input.rows;
  const duplicates = await findDuplicates(rows);
  return { duplicates };
},

// Calendar handlers
0x1200: async (ctx) => {
  // calendar_sync - sync with Google Calendar
  const events = await googleCalendar.list();
  return { events };
},

0x1213: async (ctx) => {
  // calendar_find_availability
  const slots = await findFreeSlots(ctx.input);
  return { available_slots: slots };
},

// Smart List handlers
0x1310: async (ctx) => {
  // smart_list_evaluate
  const members = await evaluateCriteria(ctx.input.criteria);
  return { members };
},

// Recurring Task handlers
0x1410: async (ctx) => {
  // recurring_task_generate_instance
  const task = await createTaskFromTemplate(ctx.input);
  return { task_id: task.id };
}
```

---

## Summary

This addendum adds **4 major operation categories**:

1. **Import/Export** (11 operations): CSV import with duplicate detection, merge strategies, validation
2. **Calendar Integration** (11 operations): Event management, time proposals, availability checking
3. **Smart Lists** (8 operations): Dynamic list creation, evaluation, subscription, change detection
4. **Recurring Tasks** (7 operations): Pattern-based task generation, scheduling, end conditions

All operations follow the PXYZ coordinate system:
- **P**: Import, Export, Calendar, Smart List, Recurring Task entities
- **X**: Operation codes in 0x1100-0x1400 range
- **Y**: Predicates for validation and access control
- **Z**: Events for all state changes

Total new operations: **37**
