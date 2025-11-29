# OMAR Addendum 3: AI-Specific Domain Operations

> **Gap Analysis Result**: Domain-specific AI intelligence tools from the stragglers document that need explicit PXYZ operation codes.

---

## AI OPERATION PHILOSOPHY

All AI operations follow this pattern:
1. **Input**: Context from domain entity
2. **LLM Processing**: Claude/local model inference
3. **Output**: Structured data or suggestions
4. **No Auto-Execution**: AI suggests, user confirms (unless explicitly trusted)

---

## CONTACTS DOMAIN AI OPERATIONS

### X-Axis: Contact AI Operations

```yaml
# Relationship Intelligence
contact_health_score: 0x0121         # Already mapped
contact_cooling_detect: 0x0124       # NEW
contact_reengage_suggest: 0x0125    # NEW
contact_ghosting_predict: 0x0126    # NEW

# Stakeholder Intelligence
contact_decision_maker_detect: 0x0130    # NEW
contact_org_chart_infer: 0x0131          # NEW
contact_missing_role_identify: 0x0132   # NEW
contact_champion_detect: 0x0133          # NEW

# Communication Optimization
contact_best_time_predict: 0x0134       # NEW
contact_channel_preference_detect: 0x0135  # NEW
contact_tone_match: 0x0136               # NEW
contact_response_time_estimate: 0x0137   # NEW

# Enrichment
contact_enrich_from_signature: 0x0111    # Already mapped
contact_enrich_firmographic: 0x0138      # NEW
```

### Workflow Example: Cooling Detection with Re-engagement

```xml
<workflow id="contact_cooling_detection">
  <entry p="contact" x="cooling_detect" node="load_interaction_history"/>
  
  <nodes>
    <node id="load_interaction_history" kind="external" op="0x0120">
      <query>
        <filter field="contact_id" value="$input.contact_id"/>
        <filter field="timestamp" gte="$now-180days"/>
      </query>
    </node>
    
    <node id="calculate_engagement_baseline" kind="transform">
      <compute>
        <var name="avg_response_time" value="avg($interactions.response_time)"/>
        <var name="typical_frequency" value="avg($interactions.days_between)"/>
        <var name="typical_sentiment" value="avg($interactions.sentiment)"/>
      </compute>
    </node>
    
    <node id="detect_cooling_pattern" kind="external" op="0x0124">
      <llm_classify>
        <context>
          Historical baseline:
          - Typical response time: {{avg_response_time}} hours
          - Usual contact frequency: every {{typical_frequency}} days
          - Normal sentiment: {{typical_sentiment}}
          
          Recent pattern (last 60 days):
          - Last contact: {{days_since_last_contact}} days ago
          - Recent response time: {{recent_response_time}} hours
          - Recent sentiment: {{recent_sentiment}}
          - Engagement frequency: {{recent_frequency}} days
        </context>
        <question>
          Is this relationship cooling? Return JSON:
          {
            "is_cooling": boolean,
            "confidence": 0-100,
            "severity": "mild|moderate|severe",
            "indicators": [string],
            "expected_vs_actual": {
              "frequency": {"expected": N, "actual": N},
              "response_time": {"expected": N, "actual": N},
              "sentiment": {"expected": "positive|neutral", "actual": "..."}
            }
          }
        </question>
      </llm_classify>
    </node>
    
    <node id="generate_reengagement_suggestion" kind="external" op="0x0125">
      <when>
        <and>
          <eq left="$cooling_detection.is_cooling" right="true"/>
          <gte left="$cooling_detection.confidence" right="70"/>
        </and>
      </when>
      <llm_prompt>
        Relationship with {{contact.name}} is cooling. Generate personalized re-engagement strategy.
        
        Context:
        - Last interaction: {{last_interaction.date}} - {{last_interaction.summary}}
        - Cooling indicators: {{cooling_detection.indicators}}
        - Relationship history: {{relationship_summary}}
        - Past successful engagement patterns: {{successful_patterns}}
        
        Return JSON:
        {
          "strategy": "string",
          "suggested_actions": [
            {
              "action": "email|call|meeting|gift",
              "timing": "immediate|this_week|next_week",
              "subject": "string",
              "talking_points": [string],
              "rationale": "string"
            }
          ],
          "draft_message": "string (if action=email)"
        }
      </llm_prompt>
    </node>
    
    <node id="create_followup_task" kind="external" op="0x0200">
      <when>
        <eq left="$user_action" right="accept_suggestion"/>
      </when>
      <event>
        <type>task.created</type>
        <data>
          <field name="title" value="Re-engage with {{contact.name}}"/>
          <field name="description" value="$reengagement.strategy"/>
          <field name="contact_id" value="$contact.id"/>
          <field name="due_date" value="$reengagement.suggested_actions[0].timing"/>
        </data>
      </event>
    </node>
    
    <node id="render_suggestion" kind="render">
      <template ref="cooling_reengagement_suggestion"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_interaction_history" to="calculate_engagement_baseline"><when><always/></when></edge>
    <edge from="calculate_engagement_baseline" to="detect_cooling_pattern"><when><always/></when></edge>
    <edge from="detect_cooling_pattern" to="generate_reengagement_suggestion">
      <when>
        <and>
          <eq left="$cooling_detection.is_cooling" right="true"/>
          <gte left="$cooling_detection.confidence" right="70"/>
        </and>
      </when>
    </edge>
    <edge from="detect_cooling_pattern" to="success">
      <when>
        <or>
          <eq left="$cooling_detection.is_cooling" right="false"/>
          <lt left="$cooling_detection.confidence" right="70"/>
        </or>
      </when>
    </edge>
    <edge from="generate_reengagement_suggestion" to="render_suggestion"><when><always/></when></edge>
    <edge from="render_suggestion" to="create_followup_task">
      <when><eq left="$user_action" right="accept_suggestion"/></when>
    </edge>
    <edge from="render_suggestion" to="success">
      <when><ne left="$user_action" right="accept_suggestion"/></when>
    </edge>
    <edge from="create_followup_task" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: Stakeholder Org Chart Inference

```xml
<workflow id="contact_org_chart_infer">
  <entry p="contact" x="org_chart_infer" node="load_org_contacts"/>
  
  <nodes>
    <node id="load_org_contacts" kind="external" op="0x0104">
      <query>
        <filter field="company_id" value="$input.company_id"/>
      </query>
    </node>
    
    <node id="load_email_patterns" kind="external" op="0x0911">
      <query>
        <type value="email.sent,email.received"/>
        <filter field="participants" contains_any="$org_contacts.emails"/>
      </query>
    </node>
    
    <node id="infer_hierarchy" kind="external" op="0x0131">
      <llm_analyze>
        <context>
          Organization: {{company.name}}
          Contacts: {{#each org_contacts}}
            - {{name}} ({{title}})
          {{/each}}
          
          Email patterns:
          {{#each email_patterns}}
            - CC patterns: {{cc_analysis}}
            - "Reports to" mentions: {{reports_to_mentions}}
            - Meeting attendees: {{meeting_patterns}}
          {{/each}}
        </context>
        <task>
          Infer organizational hierarchy. Return JSON:
          {
            "org_chart": [
              {
                "contact_id": "uuid",
                "name": "string",
                "title": "string",
                "level": 1-5,
                "reports_to": "uuid or null",
                "confidence": 0-100,
                "evidence": [string]
              }
            ],
            "identified_roles": {
              "ceo": "uuid",
              "cfo": "uuid",
              "decision_makers": ["uuid"],
              "influencers": ["uuid"]
            }
          }
        </task>
      </llm_analyze>
    </node>
    
    <node id="detect_missing_roles" kind="external" op="0x0132">
      <llm_analyze>
        <context>
          Current contacts: {{org_chart.identified_roles}}
          Industry: {{company.industry}}
          Company size: {{company.size}}
        </context>
        <task>
          Identify missing key roles we should connect with. Return JSON:
          {
            "missing_roles": [
              {
                "role": "string (e.g., CFO, CTO)",
                "importance": "critical|high|medium",
                "rationale": "string",
                "suggested_action": "string"
              }
            ]
          }
        </task>
      </llm_analyze>
    </node>
    
    <node id="update_contact_metadata" kind="external" op="0x0102">
      <for_each item="$org_chart">
        <update contact_id="$item.contact_id">
          <field name="org_level" value="$item.level"/>
          <field name="reports_to_contact_id" value="$item.reports_to"/>
          <field name="is_decision_maker" value="contains($identified_roles.decision_makers, $item.contact_id)"/>
        </update>
      </for_each>
    </node>
    
    <node id="render_org_chart" kind="render">
      <template ref="org_chart_visualization"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_org_contacts" to="load_email_patterns"><when><always/></when></edge>
    <edge from="load_email_patterns" to="infer_hierarchy"><when><always/></when></edge>
    <edge from="infer_hierarchy" to="detect_missing_roles"><when><always/></when></edge>
    <edge from="detect_missing_roles" to="update_contact_metadata"><when><always/></when></edge>
    <edge from="update_contact_metadata" to="render_org_chart"><when><always/></when></edge>
    <edge from="render_org_chart" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## EMAIL DOMAIN AI OPERATIONS

### X-Axis: Email AI Operations

```yaml
# Already mapped:
email_summarize: 0x0530
email_draft_reply: 0x0533

# NEW operations:
email_commitment_extract: 0x0531      # Already mapped in part
email_sentiment_analyze: 0x0532       # Already mapped in part
email_prioritize: 0x0540              # NEW
email_vip_detect: 0x0541              # NEW
email_followup_detect: 0x0542         # NEW
email_quick_reply_generate: 0x0543    # NEW
email_tone_adjust: 0x0544             # NEW
email_brevity_optimize: 0x0545        # NEW
```

### Workflow Example: Smart Email Prioritization

```xml
<workflow id="email_smart_prioritization">
  <entry p="email" x="prioritize" node="load_inbox"/>
  
  <nodes>
    <node id="load_inbox" kind="external" op="0x0505">
      <query>
        <filter field="account_id" value="$input.account_id"/>
        <filter field="status" value="unread"/>
        <limit value="100"/>
      </query>
    </node>
    
    <node id="classify_each_email" kind="external" op="0x0540">
      <for_each item="$inbox_threads">
        <llm_classify>
          <context>
            From: {{thread.from}}
            Subject: {{thread.subject}}
            Preview: {{thread.preview}}
            Thread length: {{thread.message_count}}
            Participants: {{thread.participants}}
          </context>
          <classify>
            urgency: critical|high|medium|low
            importance: high|medium|low
            category: decision|action|info|marketing
            requires_response: boolean
            deadline_mentioned: boolean|iso8601
            vip_sender: boolean
          </classify>
        </llm_classify>
      </for_each>
    </node>
    
    <node id="calculate_priority_score" kind="transform">
      <for_each item="$classified_emails">
        <compute>
          <var name="score" value="
            ($urgency == 'critical' ? 100 : 0) +
            ($urgency == 'high' ? 75 : 0) +
            ($importance == 'high' ? 50 : 0) +
            ($vip_sender ? 25 : 0) +
            ($requires_response ? 20 : 0) +
            ($deadline_mentioned ? 30 : 0) +
            ($category == 'decision' ? 40 : 0)
          "/>
        </compute>
      </for_each>
    </node>
    
    <node id="sort_by_priority" kind="transform">
      <sort by="$priority_score" direction="desc"/>
    </node>
    
    <node id="group_by_category" kind="transform">
      <group_by field="category"/>
      <within_groups>
        <sort by="priority_score" direction="desc"/>
      </within_groups>
    </node>
    
    <node id="render_prioritized_inbox" kind="render">
      <template ref="email_inbox_prioritized">
        <section name="Needs Immediate Action" filter="urgency:critical OR deadline_mentioned"/>
        <section name="VIP Senders" filter="vip_sender:true"/>
        <section name="Requires Response" filter="requires_response:true"/>
        <section name="Informational" filter="category:info"/>
        <section name="Marketing/Newsletters" filter="category:marketing"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_inbox" to="classify_each_email"><when><always/></when></edge>
    <edge from="classify_each_email" to="calculate_priority_score"><when><always/></when></edge>
    <edge from="calculate_priority_score" to="sort_by_priority"><when><always/></when></edge>
    <edge from="sort_by_priority" to="group_by_category"><when><always/></when></edge>
    <edge from="group_by_category" to="render_prioritized_inbox"><when><always/></when></edge>
    <edge from="render_prioritized_inbox" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: Quick Reply Generation

```xml
<workflow id="email_quick_reply_generate">
  <entry p="email" x="quick_reply_generate" node="analyze_email"/>
  
  <nodes>
    <node id="analyze_email" kind="external" op="0x0801">
      <llm_classify>
        <context>
          Email body: {{email.body}}
        </context>
        <classify>
          intent: question|request|update|scheduling|thanks
          complexity: simple|moderate|complex
          requires_detailed_response: boolean
        </classify>
      </llm_classify>
    </node>
    
    <node id="generate_quick_options" kind="external" op="0x0543">
      <when>
        <and>
          <eq left="$classification.complexity" right="simple"/>
          <eq left="$classification.requires_detailed_response" right="false"/>
        </and>
      </when>
      <llm_generate>
        <context>
          Email intent: {{classification.intent}}
          Email preview: {{email.preview}}
        </context>
        <task>
          Generate 3 quick reply options (each under 50 words). Return JSON:
          {
            "options": [
              {
                "tone": "positive|neutral|formal",
                "text": "string",
                "label": "string (e.g., 'Confirm', 'Decline politely', 'Request more info')"
              }
            ]
          }
        </task>
      </llm_generate>
    </node>
    
    <node id="render_quick_reply_buttons" kind="render">
      <template ref="email_quick_reply_buttons">
        <button action="send_reply" data="$options[0]">{{options[0].label}}</button>
        <button action="send_reply" data="$options[1]">{{options[1].label}}</button>
        <button action="send_reply" data="$options[2]">{{options[2].label}}</button>
        <button action="draft_custom">Write custom reply</button>
      </template>
    </node>
    
    <node id="send_quick_reply" kind="external" op="0x0503">
      <when>
        <eq left="$user_action" right="send_reply"/>
      </when>
      <email>
        <to value="$email.from"/>
        <subject value="Re: {{email.subject}}"/>
        <body value="$selected_option.text"/>
      </email>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="analyze_email" to="generate_quick_options">
      <when>
        <and>
          <eq left="$classification.complexity" right="simple"/>
          <eq left="$classification.requires_detailed_response" right="false"/>
        </and>
      </when>
    </edge>
    <edge from="generate_quick_options" to="render_quick_reply_buttons"><when><always/></when></edge>
    <edge from="render_quick_reply_buttons" to="send_quick_reply">
      <when><eq left="$user_action" right="send_reply"/></when>
    </edge>
    <edge from="send_quick_reply" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## TASKS DOMAIN AI OPERATIONS

### X-Axis: Task AI Operations

```yaml
# Already mapped:
task_add_subtask: 0x0212            # AI can suggest subtasks

# NEW operations:
task_blocker_detect: 0x0230         # NEW
task_duration_estimate: 0x0231      # NEW
task_owner_suggest: 0x0232          # NEW
task_completion_predict: 0x0233     # NEW
task_overload_detect: 0x0234        # NEW
task_delegation_suggest: 0x0235     # NEW
task_batch_suggest: 0x0236          # NEW
```

### Workflow Example: Workload Overload Detection

```xml
<workflow id="task_overload_detection">
  <entry p="task" x="overload_detect" node="load_user_tasks"/>
  
  <nodes>
    <node id="load_user_tasks" kind="external" op="0x0204">
      <query>
        <filter field="owner_id" value="$input.user_id"/>
        <filter field="state" in="planned,in_progress"/>
      </query>
    </node>
    
    <node id="calculate_workload" kind="external" op="0x0234">
      <llm_analyze>
        <context>
          User: {{user.name}}
          Total tasks: {{count(tasks)}}
          
          Task breakdown:
          {{#each tasks}}
          - {{title}} ({{priority}}, due: {{due_date}}, est: {{estimated_hours}}h)
          {{/each}}
          
          Upcoming deadlines:
          - Next 7 days: {{tasks_due_this_week}} tasks
          - Next 30 days: {{tasks_due_this_month}} tasks
          
          Historical velocity:
          - Avg tasks completed per week: {{historical_velocity.weekly}}
          - Avg hours worked per week: {{historical_velocity.hours}}
        </context>
        <task>
          Analyze workload and detect overload. Return JSON:
          {
            "is_overloaded": boolean,
            "severity": "mild|moderate|severe",
            "confidence": 0-100,
            "analysis": {
              "total_estimated_hours": number,
              "hours_available": number,
              "overload_by_hours": number,
              "tasks_at_risk": ["uuid"],
              "bottleneck_period": "this_week|next_week|etc"
            },
            "recommendations": [
              {
                "type": "delegate|reschedule|deprioritize|break_down",
                "task_id": "uuid",
                "rationale": "string",
                "suggested_action": "string"
              }
            ]
          }
        </task>
      </llm_analyze>
    </node>
    
    <node id="generate_delegation_suggestions" kind="external" op="0x0235">
      <when>
        <and>
          <eq left="$overload.is_overloaded" right="true"/>
          <contains left="$overload.recommendations.type" right="delegate"/>
        </and>
      </when>
      <for_each item="$overload.recommendations" filter="type:delegate">
        <llm_analyze>
          <context>
            Task: {{task.title}}
            Skills required: {{task.tags}}
            Current owner: {{user.name}}
            
            Available team members:
            {{#each team}}
            - {{name}} (skills: {{skills}}, workload: {{current_task_count}} tasks)
            {{/each}}
          </context>
          <task>
            Suggest best team member to delegate this task to. Return JSON:
            {
              "suggested_owner_id": "uuid",
              "confidence": 0-100,
              "rationale": "string",
              "draft_message": "string (delegation request message)"
            }
          </task>
        </llm_analyze>
      </for_each>
    </node>
    
    <node id="render_overload_alert" kind="render">
      <template ref="task_overload_alert">
        <severity value="$overload.severity"/>
        <analysis value="$overload.analysis"/>
        <recommendations value="$overload.recommendations"/>
        <delegation_suggestions value="$delegation_suggestions"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_user_tasks" to="calculate_workload"><when><always/></when></edge>
    <edge from="calculate_workload" to="generate_delegation_suggestions">
      <when>
        <and>
          <eq left="$overload.is_overloaded" right="true"/>
          <contains left="$overload.recommendations.type" right="delegate"/>
        </and>
      </when>
    </edge>
    <edge from="calculate_workload" to="render_overload_alert">
      <when>
        <not>
          <contains left="$overload.recommendations.type" right="delegate"/>
        </not>
      </when>
    </edge>
    <edge from="generate_delegation_suggestions" to="render_overload_alert"><when><always/></when></edge>
    <edge from="render_overload_alert" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## WORKFLOWS DOMAIN AI OPERATIONS

### X-Axis: Workflow AI Operations

```yaml
# Already mapped:
workflow_health_check: 0x0440

# NEW operations:
workflow_bottleneck_detect: 0x0450     # NEW
workflow_risk_predict: 0x0451          # NEW
workflow_optimize_suggest: 0x0452      # NEW
workflow_delay_cause_analyze: 0x0453   # NEW
```

### Workflow Example: Bottleneck Detection

```xml
<workflow id="workflow_bottleneck_detect">
  <entry p="workflow" x="bottleneck_detect" node="load_workflow"/>
  
  <nodes>
    <node id="load_workflow" kind="external" op="0x0401"/>
    
    <node id="load_all_steps_with_history" kind="external" op="0x0911">
      <query>
        <filter field="workflow_id" value="$input.workflow_id"/>
        <include field="state_history"/>
        <include field="time_in_state"/>
      </query>
    </node>
    
    <node id="analyze_bottlenecks" kind="external" op="0x0450">
      <llm_analyze>
        <context>
          Workflow: {{workflow.name}}
          Total duration so far: {{workflow.days_running}} days
          
          Phase analysis:
          {{#each phases}}
          - {{name}}: {{duration_days}} days (planned: {{planned_duration}} days)
            Status: {{status}}
            Steps: {{step_count}} ({{completed_count}} done, {{blocked_count}} blocked)
          {{/each}}
          
          Step timing analysis:
          {{#each steps}}
          - {{name}}: {{time_in_current_state}} days in {{status}}
            Owner: {{owner.name}}
            Dependencies: {{dependency_count}}
          {{/each}}
        </context>
        <task>
          Identify bottlenecks slowing this workflow. Return JSON:
          {
            "bottlenecks": [
              {
                "type": "step|phase|dependency|owner",
                "location": "step_id or phase_id",
                "severity": "minor|moderate|critical",
                "impact_days": number,
                "root_cause": "string",
                "affected_downstream": ["step_ids"],
                "recommendations": [string]
              }
            ],
            "critical_path": ["step_ids in order"],
            "predicted_delay_days": number
          }
        </task>
      </llm_analyze>
    </node>
    
    <node id="generate_optimization_suggestions" kind="external" op="0x0452">
      <llm_analyze>
        <context>
          Bottlenecks identified: {{bottlenecks}}
          Current workflow structure: {{workflow.phases}}
        </context>
        <task>
          Suggest workflow optimizations to remove bottlenecks. Return JSON:
          {
            "suggestions": [
              {
                "type": "parallelize|skip|reassign|add_resources|simplify",
                "target": "step_id or phase_id",
                "description": "string",
                "estimated_time_savings_days": number,
                "implementation_effort": "low|medium|high",
                "risks": [string]
              }
            ]
          }
        </task>
      </llm_analyze>
    </node>
    
    <node id="notify_workflow_owner" kind="external" op="0x0300">
      <notification>
        <recipient value="$workflow.owner_id"/>
        <template ref="workflow_bottleneck_alert"/>
        <data>
          <field name="bottlenecks" value="$bottlenecks"/>
          <field name="optimization_suggestions" value="$suggestions"/>
        </data>
      </notification>
    </node>
    
    <node id="render_analysis" kind="render">
      <template ref="workflow_bottleneck_analysis"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_workflow" to="load_all_steps_with_history"><when><always/></when></edge>
    <edge from="load_all_steps_with_history" to="analyze_bottlenecks"><when><always/></when></edge>
    <edge from="analyze_bottlenecks" to="generate_optimization_suggestions"><when><always/></when></edge>
    <edge from="generate_optimization_suggestions" to="notify_workflow_owner"><when><always/></when></edge>
    <edge from="notify_workflow_owner" to="render_analysis"><when><always/></when></edge>
    <edge from="render_analysis" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PORTAL DOMAIN AI OPERATIONS

### X-Axis: Portal AI Operations

```yaml
portal_personalize: 0x0960           # NEW
portal_nudge_generate: 0x0961        # NEW
portal_activity_predict: 0x0962      # NEW
portal_confusion_detect: 0x0963      # NEW
portal_message_optimize: 0x0964      # NEW
```

### Workflow Example: Client Confusion Detection

```xml
<workflow id="portal_confusion_detection">
  <entry p="portal" x="confusion_detect" node="load_client_activity"/>
  
  <nodes>
    <node id="load_client_activity" kind="external" op="0x0950">
      <query>
        <filter field="client_id" value="$input.client_id"/>
        <filter field="timestamp" gte="$now-7days"/>
        <include field="page_views"/>
        <include field="help_accesses"/>
        <include field="support_tickets"/>
      </query>
    </node>
    
    <node id="detect_confusion_signals" kind="external" op="0x0963">
      <llm_analyze>
        <context>
          Client: {{client.name}}
          Recent activity (last 7 days):
          
          Page views:
          {{#each page_views}}
          - {{page}}: {{view_count}} views, avg duration {{avg_duration}}s
          {{/each}}
          
          Help section access:
          - Total help views: {{help_access_count}}
          - Topics viewed: {{help_topics}}
          
          Unusual patterns:
          - Pages with >5 views: {{repeated_pages}}
          - Pages with <10s duration: {{quick_exits}}
          - Support tickets: {{support_ticket_count}}
        </context>
        <task>
          Detect client confusion or difficulty. Return JSON:
          {
            "is_confused": boolean,
            "confidence": 0-100,
            "confusion_areas": [
              {
                "feature": "string",
                "indicators": [string],
                "severity": "mild|moderate|severe"
              }
            ],
            "recommended_actions": [
              {
                "action": "send_tutorial|schedule_call|simplify_ui|add_tooltip",
                "target_feature": "string",
                "rationale": "string",
                "draft_message": "string (if action=send_tutorial)"
              }
            ]
          }
        </task>
      </llm_analyze>
    </node>
    
    <node id="generate_helping_nudge" kind="external" op="0x0961">
      <when>
        <and>
          <eq left="$confusion.is_confused" right="true"/>
          <gte left="$confusion.confidence" right="70"/>
        </and>
      </when>
      <llm_generate>
        <context>
          Client is confused about: {{confusion.confusion_areas}}
          Recommended action: {{confusion.recommended_actions[0]}}
        </context>
        <task>
          Generate helpful, non-patronizing message to guide client. Return JSON:
          {
            "subject": "string",
            "message": "string (warm, helpful tone)",
            "offer_type": "tutorial|call|faq_link",
            "cta": "string"
          }
        </task>
      </llm_generate>
    </node>
    
    <node id="send_nudge_notification" kind="external" op="0x0300">
      <notification>
        <recipient value="$client.user_id"/>
        <channel value="portal"/>
        <template ref="portal_helping_nudge"/>
        <data>
          <field name="message" value="$nudge.message"/>
          <field name="offer_type" value="$nudge.offer_type"/>
        </data>
      </notification>
    </node>
    
    <node id="notify_internal_team" kind="external" op="0x0300">
      <notification>
        <recipient value="$client.account_manager_id"/>
        <template ref="client_confusion_alert"/>
        <data>
          <field name="client" value="$client"/>
          <field name="confusion_areas" value="$confusion.confusion_areas"/>
        </data>
      </notification>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_client_activity" to="detect_confusion_signals"><when><always/></when></edge>
    <edge from="detect_confusion_signals" to="generate_helping_nudge">
      <when>
        <and>
          <eq left="$confusion.is_confused" right="true"/>
          <gte left="$confusion.confidence" right="70"/>
        </and>
      </when>
    </edge>
    <edge from="generate_helping_nudge" to="send_nudge_notification"><when><always/></when></edge>
    <edge from="send_nudge_notification" to="notify_internal_team"><when><always/></when></edge>
    <edge from="notify_internal_team" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## UNIVERSAL AI OPERATIONS

These work across all domains:

```yaml
# Cross-Domain AI
ai_semantic_search: 0x1700           # Already via Qdrant (0x0700)
ai_entity_extract: 0x1701            # NEW
ai_summarize_any: 0x1702             # NEW
ai_sentiment_analyze: 0x1703         # NEW
ai_anomaly_detect: 0x1704            # NEW
ai_link_suggest: 0x1705              # NEW
```

---

## SUMMARY

This addendum adds **40+ AI-specific operations**:

### By Domain:
- **Contacts**: 13 AI operations (cooling detection, org chart inference, communication optimization)
- **Email**: 8 AI operations (prioritization, quick replies, tone adjustment)
- **Tasks**: 7 AI operations (overload detection, delegation suggestions, duration estimation)
- **Workflows**: 4 AI operations (bottleneck detection, optimization suggestions)
- **Portal**: 5 AI operations (confusion detection, personalization, nudging)
- **Universal**: 6 cross-domain AI operations

### Pattern:
All AI operations follow:
1. **Context gathering** (load relevant data)
2. **LLM analysis** (classify, predict, suggest)
3. **User confirmation** (render suggestions, await approval)
4. **Optional execution** (if user accepts)

### Integration:
- All operations use op codes in 0x0124-0x0964 range
- All append events when state changes
- All respect user preferences for AI automation level
- All provide confidence scores and explainability

**Key Principle**: AI suggests, humans decide. No autonomous execution unless explicitly trusted by user.
