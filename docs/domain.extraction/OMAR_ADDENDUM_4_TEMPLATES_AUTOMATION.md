# OMAR Addendum 4: Template Systems & Automation Rules

> **Second-Pass Gap Analysis**: Template systems and automation rules found in PRDs but not yet mapped to PXYZ operations.

---

## TEMPLATE ARCHITECTURE PHILOSOPHY

Templates are **reusable patterns stored as data**, not code. They're configuration that gets parameterized and instantiated into actual entities.

### Template Storage Format

```xml
<schema id="template">
  <field name="id" type="uuid" required="true"/>
  <field name="type" type="enum" values="task,email,document,workflow,note"/>
  <field name="name" type="string" required="true"/>
  <field name="description" type="string"/>
  <field name="owner_id" type="uuid"/>
  <field name="is_shared" type="boolean"/>
  <field name="is_system" type="boolean"/>
  <field name="template_data" type="object" required="true"/>
  <field name="variables" type="array"/>
  <field name="version" type="integer"/>
  <field name="created_at" type="timestamp"/>
  <field name="updated_at" type="timestamp"/>
</schema>
```

---

## TASK TEMPLATE SYSTEM

### X-Axis: Task Template Operations

```yaml
task_template_create: 0x0250         # NEW
task_template_read: 0x0251           # NEW
task_template_apply: 0x0252          # NEW
task_template_update: 0x0253         # NEW
task_template_delete: 0x0254         # NEW
task_template_list: 0x0255           # NEW
task_template_share: 0x0256          # NEW
```

### Task Template Structure

```json
{
  "id": "tmpl-task-client-onboarding",
  "type": "task",
  "name": "Client Onboarding Checklist",
  "is_shared": true,
  "variables": [
    {"name": "client_name", "type": "string", "required": true},
    {"name": "project_start_date", "type": "date", "required": true},
    {"name": "account_manager", "type": "user", "required": true}
  ],
  "template_data": {
    "title": "{{client_name}} Onboarding",
    "description": "Complete onboarding for {{client_name}} starting {{project_start_date}}",
    "owner_id": "{{account_manager}}",
    "subtasks": [
      {
        "title": "Send welcome email to {{client_name}}",
        "due_offset_days": 0,
        "owner_id": "{{account_manager}}"
      },
      {
        "title": "Schedule kickoff call",
        "due_offset_days": 2,
        "owner_id": "{{account_manager}}"
      },
      {
        "title": "Create project workspace",
        "due_offset_days": 1,
        "owner_id": "{{account_manager}}"
      },
      {
        "title": "Set up billing",
        "due_offset_days": 3,
        "owner_id": "{{account_manager}}"
      }
    ],
    "tags": ["onboarding", "client"]
  }
}
```

### Workflow Example: Apply Task Template

```xml
<workflow id="task_template_apply">
  <entry p="task_template" x="apply" node="load_template"/>
  
  <nodes>
    <node id="load_template" kind="external" op="0x0251">
      <load template_id="$input.template_id"/>
    </node>
    
    <node id="render_variable_form" kind="render">
      <template ref="template_variable_form">
        <for_each variable="$template.variables">
          <input 
            name="{{variable.name}}"
            type="{{variable.type}}"
            required="{{variable.required}}"
            label="{{variable.label}}"
          />
        </for_each>
      </template>
    </node>
    
    <node id="await_user_input" kind="signal">
      <await signal="template_variables_submitted"/>
    </node>
    
    <node id="substitute_variables" kind="transform">
      <for_each field="$template.template_data">
        <replace pattern="{{variable}}" with="$user_input[variable]"/>
      </for_each>
    </node>
    
    <node id="create_main_task" kind="external" op="0x0200">
      <data from="$substituted_template_data"/>
      <event>
        <type>task.created</type>
        <data>
          <field name="created_from_template" value="$template.id"/>
        </data>
      </event>
    </node>
    
    <node id="create_subtasks" kind="external" op="0x0212">
      <for_each item="$substituted_template_data.subtasks">
        <create_subtask>
          <parent_id value="$main_task.id"/>
          <title value="$item.title"/>
          <due_date value="$main_task.start_date + $item.due_offset_days"/>
          <owner_id value="$item.owner_id"/>
        </create_subtask>
        <event>
          <type>task.created</type>
          <data>
            <field name="is_subtask" value="true"/>
            <field name="parent_task_id" value="$main_task.id"/>
          </data>
        </event>
      </for_each>
    </node>
    
    <node id="render_created_task" kind="render">
      <template ref="task_detail">
        <task value="$main_task"/>
        <subtasks value="$created_subtasks"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_template" to="render_variable_form"><when><always/></when></edge>
    <edge from="render_variable_form" to="await_user_input"><when><always/></when></edge>
    <edge from="await_user_input" to="substitute_variables"><when><always/></when></edge>
    <edge from="substitute_variables" to="create_main_task"><when><always/></when></edge>
    <edge from="create_main_task" to="create_subtasks">
      <when><gt left="count($substituted_template_data.subtasks)" right="0"/></when>
    </edge>
    <edge from="create_main_task" to="render_created_task">
      <when><eq left="count($substituted_template_data.subtasks)" right="0"/></when>
    </edge>
    <edge from="create_subtasks" to="render_created_task"><when><always/></when></edge>
    <edge from="render_created_task" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## EMAIL TEMPLATE SYSTEM

### X-Axis: Email Template Operations

```yaml
email_template_create: 0x0550        # NEW
email_template_read: 0x0551          # NEW
email_template_apply: 0x0552         # NEW
email_template_update: 0x0553        # NEW
email_template_delete: 0x0554        # NEW
email_template_list: 0x0555          # NEW
email_snippet_insert: 0x0556         # NEW - for keyboard shortcuts
```

### Email Template Structure

```json
{
  "id": "tmpl-email-followup",
  "type": "email",
  "name": "Follow-up After Meeting",
  "variables": [
    {"name": "recipient_name", "type": "string", "required": true},
    {"name": "meeting_date", "type": "date", "required": true},
    {"name": "next_steps", "type": "text", "required": false}
  ],
  "template_data": {
    "subject": "Following up on our {{meeting_date}} conversation",
    "body": "Hi {{recipient_name}},\n\nIt was great talking with you on {{meeting_date}}. I wanted to follow up on the points we discussed:\n\n{{next_steps}}\n\nLet me know if you have any questions or if there's anything else I can help with.\n\nBest,\n{{sender_name}}",
    "tone": "professional_friendly",
    "suggested_send_time": "morning"
  }
}
```

### Email Snippet Structure

```json
{
  "id": "snippet-meeting-times",
  "type": "email_snippet",
  "name": "Meeting Time Options",
  "shortcut": "/meeting",
  "content": "I have availability:\n• Tuesday 2-3pm\n• Wednesday 10-11am\n• Thursday 3-4pm\n\nDo any of these work for you?"
}
```

### Workflow Example: Apply Email Template with AI Enhancement

```xml
<workflow id="email_template_apply_smart">
  <entry p="email_template" x="apply" node="load_template"/>
  
  <nodes>
    <node id="load_template" kind="external" op="0x0551"/>
    
    <node id="detect_context" kind="transform">
      <compute>
        <var name="recipient" value="$current_email_thread.participants[0]"/>
        <var name="last_meeting" value="query_events({type:'meeting', participants:[$recipient.id]}).last()"/>
      </compute>
    </node>
    
    <node id="auto_fill_variables" kind="external" op="0x0801">
      <llm_extract>
        <context>
          Template variables needed: {{template.variables}}
          Available context:
          - Recipient: {{recipient.name}}
          - Last meeting: {{last_meeting.date}} - {{last_meeting.summary}}
          - Recent activity: {{recipient.recent_activity}}
        </context>
        <task>
          Auto-fill template variables where possible. Return JSON:
          {
            "filled_variables": {
              "recipient_name": "string",
              "meeting_date": "date",
              "next_steps": "string (extracted from meeting notes)"
            },
            "confidence": 0-100,
            "needs_user_review": ["variable_names"]
          }
        </task>
      </llm_extract>
    </node>
    
    <node id="substitute_and_enhance" kind="external" op="0x0533">
      <substitute variables="$auto_filled_variables" in="$template.template_data"/>
      <llm_enhance>
        <prompt>
          Personalize this email based on relationship history:
          {{substituted_email}}
          
          Relationship context:
          - Last interaction: {{recipient.last_interaction.summary}}
          - Communication style preference: {{recipient.preferred_style}}
          - Current projects: {{recipient.active_projects}}
        </prompt>
      </llm_enhance>
    </node>
    
    <node id="render_draft" kind="render">
      <template ref="email_compose">
        <pre_fill field="to" value="$recipient.email"/>
        <pre_fill field="subject" value="$enhanced_email.subject"/>
        <pre_fill field="body" value="$enhanced_email.body"/>
        <show_confidence score="$auto_filled_variables.confidence"/>
        <highlight_fields needing_review="$auto_filled_variables.needs_user_review"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_template" to="detect_context"><when><always/></when></edge>
    <edge from="detect_context" to="auto_fill_variables"><when><always/></when></edge>
    <edge from="auto_fill_variables" to="substitute_and_enhance"><when><always/></when></edge>
    <edge from="substitute_and_enhance" to="render_draft"><when><always/></when></edge>
    <edge from="render_draft" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## DOCUMENT TEMPLATE SYSTEM

### X-Axis: Document Template Operations

```yaml
document_template_create: 0x0650     # NEW
document_template_read: 0x0651       # NEW
document_template_apply: 0x0652      # NEW
document_template_update: 0x0653     # NEW
document_template_delete: 0x0654     # NEW
document_template_list: 0x0655       # NEW
```

### Document Template Structure

```json
{
  "id": "tmpl-doc-proposal",
  "type": "document",
  "name": "Project Proposal",
  "variables": [
    {"name": "client_name", "type": "string", "required": true},
    {"name": "project_scope", "type": "text", "required": true},
    {"name": "budget_range", "type": "string", "required": true},
    {"name": "timeline_weeks", "type": "integer", "required": true}
  ],
  "template_data": {
    "title": "Proposal for {{client_name}}",
    "sections": [
      {
        "heading": "Executive Summary",
        "content": "This proposal outlines our approach to delivering {{project_scope}} for {{client_name}}. We estimate a timeline of {{timeline_weeks}} weeks with a budget in the range of {{budget_range}}."
      },
      {
        "heading": "Scope of Work",
        "content": "{{project_scope}}\n\n[AI will expand this section based on scope keywords]",
        "ai_expand": true
      },
      {
        "heading": "Timeline",
        "content": "[AI will generate Gantt chart based on {{timeline_weeks}} weeks]",
        "ai_generate": "timeline_chart"
      },
      {
        "heading": "Investment",
        "content": "Budget range: {{budget_range}}\n\n[AI will generate line items based on scope]",
        "ai_generate": "budget_breakdown"
      },
      {
        "heading": "Next Steps",
        "content": "Upon approval, we will...\n\n[Standard next steps template]"
      }
    ]
  }
}
```

---

## WORKFLOW TEMPLATE SYSTEM

### X-Axis: Workflow Template Operations

```yaml
workflow_template_create: 0x0460     # NEW
workflow_template_read: 0x0461       # NEW
workflow_template_apply: 0x0462      # NEW
workflow_template_update: 0x0463     # NEW
workflow_template_delete: 0x0464     # NEW
workflow_template_list: 0x0465       # NEW
workflow_template_version: 0x0466    # NEW - version control
```

### Workflow Template Structure

```json
{
  "id": "tmpl-workflow-web-design",
  "type": "workflow",
  "name": "Website Design Project",
  "variables": [
    {"name": "client_name", "type": "string", "required": true},
    {"name": "page_count", "type": "integer", "required": true},
    {"name": "has_ecommerce", "type": "boolean", "required": true}
  ],
  "template_data": {
    "name": "{{client_name}} Website Design",
    "phases": [
      {
        "name": "Discovery",
        "duration_days": 7,
        "steps": [
          {"name": "Kickoff meeting", "duration_days": 1},
          {"name": "Requirements gathering", "duration_days": 3},
          {"name": "Competitor analysis", "duration_days": 3}
        ]
      },
      {
        "name": "Design",
        "duration_days": 14,
        "steps": [
          {"name": "Wireframes", "duration_days": 5},
          {"name": "Visual design", "duration_days": 7},
          {"name": "Design review", "duration_days": 2, "requires_approval": true}
        ]
      },
      {
        "name": "Development",
        "duration_days": 21,
        "conditional": "{{page_count}} > 5",
        "steps": [
          {"name": "Frontend build", "duration_days": 10},
          {"name": "Backend integration", "duration_days": 8},
          {"name": "E-commerce setup", "duration_days": 5, "conditional": "{{has_ecommerce}}"},
          {"name": "Testing", "duration_days": 3}
        ]
      },
      {
        "name": "Launch",
        "duration_days": 3,
        "steps": [
          {"name": "Final review", "duration_days": 1, "requires_approval": true},
          {"name": "Deploy to production", "duration_days": 1},
          {"name": "Post-launch monitoring", "duration_days": 1}
        ]
      }
    ]
  }
}
```

---

## AUTOMATION RULES SYSTEM

### X-Axis: Automation Rule Operations

```yaml
automation_rule_create: 0x1800       # NEW
automation_rule_read: 0x1801         # NEW
automation_rule_update: 0x1802       # NEW
automation_rule_delete: 0x1803       # NEW
automation_rule_list: 0x1804         # NEW
automation_rule_enable: 0x1805       # NEW
automation_rule_disable: 0x1806      # NEW
automation_rule_test: 0x1807         # NEW - dry run
```

### Automation Rule Schema

```xml
<schema id="automation_rule">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="entity_type" type="enum" values="task,email,contact,workflow,document"/>
  <field name="trigger" type="object" required="true"/>
  <field name="conditions" type="array" required="true"/>
  <field name="actions" type="array" required="true"/>
  <field name="is_enabled" type="boolean"/>
  <field name="priority" type="integer"/>
  <field name="created_by" type="uuid"/>
</schema>
```

### Automation Rule Examples

#### Rule 1: Auto-Assign Tasks by Keyword

```json
{
  "id": "rule-auto-assign-design",
  "name": "Auto-assign design tasks to Sarah",
  "entity_type": "task",
  "trigger": {
    "event": "task.created"
  },
  "conditions": [
    {
      "type": "contains",
      "field": "title",
      "value": "design",
      "case_insensitive": true
    },
    {
      "type": "or",
      "conditions": [
        {"field": "title", "contains": "mockup"},
        {"field": "title", "contains": "wireframe"},
        {"field": "tags", "contains": "design"}
      ]
    }
  ],
  "actions": [
    {
      "type": "update_field",
      "field": "owner_id",
      "value": "user-sarah-123"
    },
    {
      "type": "add_tag",
      "value": "auto-assigned"
    },
    {
      "type": "notify",
      "recipient": "user-sarah-123",
      "message": "Auto-assigned design task: {{task.title}}"
    }
  ]
}
```

#### Rule 2: Auto-Escalate Overdue Tasks

```json
{
  "id": "rule-escalate-overdue",
  "name": "Escalate overdue high-priority tasks",
  "entity_type": "task",
  "trigger": {
    "schedule": "daily",
    "time": "09:00"
  },
  "conditions": [
    {
      "type": "lt",
      "field": "due_date",
      "value": "$now"
    },
    {
      "type": "eq",
      "field": "priority",
      "value": "high"
    },
    {
      "type": "ne",
      "field": "state",
      "value": "done"
    }
  ],
  "actions": [
    {
      "type": "update_field",
      "field": "state",
      "value": "blocked"
    },
    {
      "type": "notify",
      "recipient": "$task.owner_id",
      "message": "Task overdue: {{task.title}}"
    },
    {
      "type": "notify",
      "recipient": "$task.owner.manager_id",
      "message": "{{task.owner.name}} has overdue high-priority task: {{task.title}}"
    }
  ]
}
```

#### Rule 3: Auto-Tag Emails by Sender

```json
{
  "id": "rule-tag-client-emails",
  "name": "Tag emails from known clients",
  "entity_type": "email",
  "trigger": {
    "event": "email.received"
  },
  "conditions": [
    {
      "type": "exists",
      "field": "sender.email",
      "in": "contacts.email"
    },
    {
      "type": "eq",
      "field": "contact.type",
      "value": "customer"
    }
  ],
  "actions": [
    {
      "type": "add_tag",
      "value": "client"
    },
    {
      "type": "add_tag",
      "value": "$contact.company.name"
    },
    {
      "type": "move_to_folder",
      "folder": "Clients"
    }
  ]
}
```

### Workflow Example: Execute Automation Rule

```xml
<workflow id="automation_rule_execute">
  <!-- Triggered by event matching rule trigger -->
  <entry p="automation_rule" x="execute" node="load_rule"/>
  
  <nodes>
    <node id="load_rule" kind="external" op="0x1801">
      <load rule_id="$input.rule_id"/>
    </node>
    
    <node id="check_enabled" kind="auth">
      <require>
        <eq left="$rule.is_enabled" right="true"/>
      </require>
    </node>
    
    <node id="evaluate_conditions" kind="transform">
      <for_each condition="$rule.conditions">
        <evaluate condition="$condition" entity="$input.entity"/>
      </for_each>
      <compute>
        <var name="all_conditions_met" value="all($evaluated_conditions)"/>
      </compute>
    </node>
    
    <node id="check_conditions_met" kind="auth">
      <require>
        <eq left="$all_conditions_met" right="true"/>
      </require>
    </node>
    
    <node id="execute_actions" kind="external" op="0x1800">
      <for_each action="$rule.actions">
        <when>
          <eq left="$action.type" right="update_field"/>
        </when>
        <update entity_id="$input.entity.id">
          <field name="$action.field" value="$action.value"/>
        </update>
        <event>
          <type>automation.action_executed</type>
          <data>
            <field name="rule_id" value="$rule.id"/>
            <field name="action_type" value="$action.type"/>
            <field name="entity_id" value="$input.entity.id"/>
          </data>
        </event>
      </for_each>
      
      <for_each action="$rule.actions">
        <when>
          <eq left="$action.type" right="add_tag"/>
        </when>
        <add_tag entity_id="$input.entity.id" tag="$action.value"/>
      </for_each>
      
      <for_each action="$rule.actions">
        <when>
          <eq left="$action.type" right="notify"/>
        </when>
        <external op="0x0300">
          <notification>
            <recipient value="$action.recipient"/>
            <message value="substitute($action.message, $input.entity)"/>
          </notification>
        </external>
      </for_each>
    </node>
    
    <node id="log_execution" kind="external" op="0x0910">
      <event>
        <type>automation.rule_executed</type>
        <data>
          <field name="rule_id" value="$rule.id"/>
          <field name="entity_id" value="$input.entity.id"/>
          <field name="actions_count" value="count($rule.actions)"/>
          <field name="timestamp" value="$now"/>
        </data>
      </event>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_rule" to="check_enabled"><when><always/></when></edge>
    <edge from="check_enabled" to="evaluate_conditions"><when><always/></when></edge>
    <edge from="evaluate_conditions" to="check_conditions_met"><when><always/></when></edge>
    <edge from="check_conditions_met" to="execute_actions"><when><always/></when></edge>
    <edge from="execute_actions" to="log_execution"><when><always/></when></edge>
    <edge from="log_execution" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## SUMMARY

This addendum adds **5 major template/automation categories**:

### Template Systems (25 operations):
1. **Task Templates** (7 ops): Save with subtasks, apply with variables, share with team
2. **Email Templates** (7 ops): Subject/body templates, snippets with shortcuts, AI enhancement
3. **Document Templates** (6 ops): Section-based templates, AI content generation, variables
4. **Workflow Templates** (7 ops): Phase/step patterns, conditional logic, versioning

### Automation Rules (8 operations):
- Create, read, update, delete, list, enable/disable, test
- Trigger types: event-based, schedule-based, condition-based
- Action types: update_field, add_tag, notify, move, create_related

### Key Patterns:
- All templates stored as JSON with variable placeholders
- Variables type-checked and required/optional
- AI can auto-fill variables from context
- Templates versioned and team-shareable
- Automation rules evaluated via predicate VM
- All actions append events to truth log

**Total New Operations**: 40
- Task templates: 7
- Email templates: 7
- Document templates: 6
- Workflow templates: 7
- Note templates: (implied, similar to tasks)
- Automation rules: 8
- Template sharing: (integrated into each type)
