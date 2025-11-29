# OMAR AI Intelligence Services - PXYZ Extraction

> **Services**: AIOperations, CopilotService, AssistantManager  
> **Purpose**: AI-powered intelligence, automation, and assistance

---

## PART 1: AI OPERATIONS SERVICE

### Purpose
Real business AI capabilities - semantic search, summarization, anomaly detection, sentiment analysis, etc.

### P-Axis: AI Operations Entities

```xml
<schema id="ai_operation_request">
  <field name="id" type="uuid" required="true"/>
  <field name="type" type="string" required="true"/>
  <field name="payload" type="object" required="true"/>
  <field name="model_config" type="object"/>
  <field name="created_at" type="timestamp"/>
</schema>

<schema id="ai_operation_result">
  <field name="id" type="uuid" required="true"/>
  <field name="success" type="boolean" required="true"/>
  <field name="operation" type="string" required="true"/>
  <field name="data" type="object" required="true"/>
  <field name="model_used" type="string" required="true"/>
  <field name="processing_time_ms" type="number" required="true"/>
  <field name="tokens_estimated" type="number"/>
  <field name="created_at" type="timestamp"/>
</schema>

<schema id="smart_link_suggestion">
  <field name="source_id" type="uuid"/>
  <field name="target_id" type="uuid"/>
  <field name="link_type" type="string"/>
  <field name="confidence" type="number"/>
  <field name="context" type="string"/>
</schema>

<schema id="anomaly_detection_result">
  <field name="anomalies" type="array" required="true"/>
  <field name="risk_level" type="enum" values="low,medium,high"/>
  <field name="risk_score" type="number"/>
  <field name="statistics" type="object"/>
</schema>

<schema id="sentiment_analysis_result">
  <field name="sentiment_score" type="number" min="-100" max="100"/>
  <field name="sentiment_label" type="enum" values="very_negative,negative,neutral,positive,very_positive"/>
  <field name="confidence" type="number"/>
  <field name="key_phrases" type="array"/>
</schema>
```

### X-Axis: AI Operations

```yaml
# Semantic Search & RAG
ai_semantic_search: 0x2400          # Semantic knowledge search
ai_rag_retrieve: 0x2401             # RAG retrieval
ai_context_analyze: 0x2402          # Context analysis

# Smart Linking
ai_smart_link: 0x2410               # Find entity relationships
ai_mention_extract: 0x2411          # Extract entity mentions
ai_link_suggest: 0x2412             # Suggest new links
ai_link_broken_detect: 0x2413       # Detect broken links

# Predictive Features
ai_predictive_type: 0x2420          # Predictive typing
ai_autocomplete: 0x2421             # Auto-completion
ai_suggestion_generate: 0x2422      # Generate suggestions

# Content Processing
ai_summarize: 0x2430                # Text summarization
ai_summarize_long: 0x2431           # Long document summary
ai_extract_highlights: 0x2432       # Extract key points
ai_extract_entities: 0x2433         # Named entity extraction

# Document Analysis
ai_compare_docs: 0x2440             # Compare documents
ai_doc_similarity: 0x2441           # Similarity scoring
ai_classify_content: 0x2442         # Content classification

# Translation & Language
ai_translate: 0x2450                # Text translation
ai_detect_language: 0x2451          # Language detection

# Sentiment & Emotion
ai_sentiment: 0x2460                # Sentiment analysis
ai_emotion_detect: 0x2461           # Emotion detection
ai_tone_analyze: 0x2462             # Tone analysis

# Anomaly & Quality
ai_anomaly_detect: 0x2470           # Statistical anomaly detection
ai_freshness_check: 0x2471          # Data staleness check
ai_quality_score: 0x2472            # Data quality assessment
```

### Y-Axis: AI Operations Predicates

```xml
<predicates>
  <!-- Search Validation -->
  <predicate id="is_valid_search_query">
    <and>
      <not_empty left="$query"/>
      <gte left="length($query)" right="3"/>
    </and>
  </predicate>
  
  <predicate id="supports_web_search">
    <contains left="$model_id" right="compound"/>
  </predicate>
  
  <!-- Link Quality -->
  <predicate id="is_high_confidence_link">
    <gte left="$link.confidence" right="0.85"/>
  </predicate>
  
  <predicate id="link_is_broken">
    <or>
      <null left="$link.target"/>
      <eq left="$link.target.deleted" right="true"/>
    </or>
  </predicate>
  
  <!-- Anomaly Detection -->
  <predicate id="has_anomalies">
    <gt left="count($anomalies)" right="0"/>
  </predicate>
  
  <predicate id="is_high_risk">
    <and>
      <gte left="$risk_score" right="70"/>
      <gte left="count($anomalies)" right="3"/>
    </and>
  </predicate>
  
  <!-- Data Quality -->
  <predicate id="is_stale_data">
    <gte left="$days_since_update" right="30"/>
  </predicate>
  
  <predicate id="is_critically_stale">
    <gte left="$days_since_update" right="90"/>
  </predicate>
  
  <!-- Sentiment -->
  <predicate id="is_negative_sentiment">
    <lt left="$sentiment_score" right="-30"/>
  </predicate>
  
  <predicate id="requires_attention">
    <and>
      <lt left="$sentiment_score" right="-50"/>
      <gte left="$confidence" right="0.8"/>
    </and>
  </predicate>
</predicates>
```

### Z-Axis: AI Operations Events

```typescript
enum AIOperationEventType {
  // Search
  AI_SEMANTIC_SEARCH_COMPLETED = "ai.semantic_search_completed",
  AI_RAG_RETRIEVED = "ai.rag_retrieved",
  
  // Linking
  AI_LINKS_SUGGESTED = "ai.links_suggested",
  AI_BROKEN_LINKS_DETECTED = "ai.broken_links_detected",
  
  // Content Processing
  AI_SUMMARIZATION_COMPLETED = "ai.summarization_completed",
  AI_ENTITIES_EXTRACTED = "ai.entities_extracted",
  
  // Analysis
  AI_SENTIMENT_ANALYZED = "ai.sentiment_analyzed",
  AI_ANOMALIES_DETECTED = "ai.anomalies_detected",
  AI_FRESHNESS_CHECKED = "ai.freshness_checked"
}
```

### Workflow Example: Smart Linking with AI

```xml
<workflow id="ai_smart_linking">
  <entry p="ai" x="smart_link" node="extract_mentions"/>
  
  <nodes>
    <node id="extract_mentions" kind="external" op="0x2411">
      <analyze content="$input.content"/>
      <patterns>
        <pattern type="person" regex="@[A-Za-z]+"/>
        <pattern type="company" regex="[A-Z][a-z]+ (Inc|Corp|LLC)"/>
        <pattern type="task" regex="#[0-9]+"/>
      </patterns>
      <event type="ai.mentions_extracted"/>
    </node>
    
    <node id="find_existing_links" kind="external" op="0x0911">
      <query>
        <filter field="source_id" value="$input.source_id"/>
        <filter field="type" value="link"/>
      </query>
    </node>
    
    <node id="suggest_new_links" kind="external" op="0x2412">
      <for_each mention="$extracted_mentions">
        <when>
          <not>
            <exists left="$existing_links[$mention.id]"/>
          </not>
        </when>
        <calculate_confidence>
          <context_similarity value="0.4"/>
          <name_match value="0.6"/>
        </calculate_confidence>
      </for_each>
      <event type="ai.links_suggested"/>
    </node>
    
    <node id="detect_broken_links" kind="external" op="0x2413">
      <for_each link="$existing_links">
        <check_target exists="$link.target_id"/>
        <when>
          <predicate ref="link_is_broken"/>
        </when>
        <flag_broken link="$link"/>
      </for_each>
      <event type="ai.broken_links_detected"/>
    </node>
    
    <node id="render_suggestions" kind="render">
      <template ref="link_suggestions">
        <new_links value="$suggested_links"/>
        <broken_links value="$broken_links"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="extract_mentions" to="find_existing_links"><when><always/></when></edge>
    <edge from="find_existing_links" to="suggest_new_links"><when><always/></when></edge>
    <edge from="suggest_new_links" to="detect_broken_links"><when><always/></when></edge>
    <edge from="detect_broken_links" to="render_suggestions"><when><always/></when></edge>
    <edge from="render_suggestions" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: Anomaly Detection

```xml
<workflow id="ai_anomaly_detection">
  <entry p="ai" x="anomaly_detect" node="load_dataset"/>
  
  <nodes>
    <node id="load_dataset" kind="external" op="0x0911">
      <query>
        <filter field="entity_type" value="$input.entity_type"/>
        <filter field="metric_name" value="$input.metric_name"/>
        <limit value="1000"/>
      </query>
    </node>
    
    <node id="calculate_statistics" kind="transform">
      <compute>
        <var name="mean" value="avg($dataset.values)"/>
        <var name="std_dev" value="stddev($dataset.values)"/>
        <var name="z_threshold" value="2"/>
      </compute>
    </node>
    
    <node id="detect_anomalies" kind="external" op="0x2470">
      <for_each value="$dataset.values">
        <calculate_z_score>
          <formula value="($value - $mean) / $std_dev"/>
        </calculate_z_score>
        <when>
          <gt left="abs($z_score)" right="$z_threshold"/>
        </when>
        <flag_anomaly>
          <value value="$value"/>
          <z_score value="$z_score"/>
          <severity value="calculate_severity($z_score)"/>
        </flag_anomaly>
      </for_each>
      <event type="ai.anomalies_detected"/>
    </node>
    
    <node id="assess_risk" kind="transform">
      <compute>
        <var name="risk_score" value="
          count($anomalies) * 10 + 
          avg($anomalies.severity) * 20
        "/>
        <var name="risk_level" value="
          $risk_score > 70 ? 'high' :
          $risk_score > 40 ? 'medium' : 'low'
        "/>
      </compute>
    </node>
    
    <node id="notify_if_high_risk" kind="external" op="0x0300">
      <when>
        <predicate ref="is_high_risk"/>
      </when>
      <notification>
        <recipient value="$input.owner_id"/>
        <template ref="anomaly_alert"/>
        <data>
          <field name="anomaly_count" value="count($anomalies)"/>
          <field name="risk_level" value="$risk_level"/>
        </data>
      </notification>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_dataset" to="calculate_statistics"><when><always/></when></edge>
    <edge from="calculate_statistics" to="detect_anomalies"><when><always/></when></edge>
    <edge from="detect_anomalies" to="assess_risk"><when><always/></when></edge>
    <edge from="assess_risk" to="notify_if_high_risk">
      <when><predicate ref="is_high_risk"/></when>
    </edge>
    <edge from="assess_risk" to="success">
      <when><not><predicate ref="is_high_risk"/></not></when>
    </edge>
    <edge from="notify_if_high_risk" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 2: COPILOT SERVICE

### Purpose
AI-powered user assistance: suggestions, dashboard generation, action queuing, contextual help.

### P-Axis: Copilot Entities

```xml
<schema id="inline_suggestion">
  <field name="suggestion" type="string" required="true"/>
  <field name="confidence" type="number" required="true"/>
  <field name="context" type="string"/>
  <field name="alternatives" type="array"/>
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="dashboard_spec">
  <field name="success" type="boolean" required="true"/>
  <field name="dashboard_spec" type="object"/>
  <field name="processing_time_ms" type="number"/>
  <field name="error" type="string"/>
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="queued_action">
  <field name="id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="action_type" type="enum" values="email,task,workflow,reminder,custom"/>
  <field name="action_data" type="object" required="true"/>
  <field name="status" type="enum" values="pending,in_progress,completed,failed,cancelled"/>
  <field name="priority" type="enum" values="low,medium,high,urgent"/>
  <field name="scheduled_for" type="timestamp"/>
  <field name="executed_at" type="timestamp"/>
  <field name="error" type="string"/>
  <field name="created_at" type="timestamp"/>
</schema>

<schema id="contextual_help">
  <field name="suggestions" type="array" required="true"/>
  <field name="related_topics" type="array"/>
  <field name="confidence" type="number"/>
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="workflow_recommendation">
  <field name="workflow_name" type="string" required="true"/>
  <field name="workflow_category" type="string"/>
  <field name="confidence" type="number"/>
  <field name="rationale" type="string"/>
  <field name="estimated_time" type="string"/>
</schema>
```

### X-Axis: Copilot Operations

```yaml
# Inline Assistance
copilot_suggest_inline: 0x2500      # Inline suggestions
copilot_autocomplete: 0x2501        # Auto-completion
copilot_context_help: 0x2502        # Contextual help

# Dashboard Generation
copilot_dashboard_generate: 0x2510  # Generate dashboard
copilot_dashboard_update: 0x2511    # Update dashboard
copilot_artifact_create: 0x2512     # Create artifact

# Action Queue
copilot_action_queue: 0x2520        # Queue action
copilot_action_execute: 0x2521      # Execute queued action
copilot_action_cancel: 0x2522       # Cancel action
copilot_action_reschedule: 0x2523   # Reschedule action
copilot_action_list: 0x2524         # List actions

# Workflow Suggestions
copilot_workflow_suggest: 0x2530    # Suggest workflows
copilot_workflow_recommend: 0x2531  # Recommend based on context
copilot_workflow_analyze: 0x2532    # Analyze current workflow

# Smart Assistance
copilot_next_action_predict: 0x2540 # Predict next action
copilot_pattern_detect: 0x2541      # Detect user patterns
copilot_automation_suggest: 0x2542  # Suggest automation
```

### Y-Axis: Copilot Predicates

```xml
<predicates>
  <!-- Suggestion Quality -->
  <predicate id="is_high_confidence_suggestion">
    <gte left="$suggestion.confidence" right="75"/>
  </predicate>
  
  <predicate id="has_alternatives">
    <gt left="count($suggestion.alternatives)" right="0"/>
  </predicate>
  
  <!-- Action Queue -->
  <predicate id="can_queue_action">
    <and>
      <lt left="count($user.queued_actions)" right="50"/>
      <is_valid_action_type left="$action.type"/>
    </and>
  </predicate>
  
  <predicate id="is_ready_to_execute">
    <and>
      <eq left="$action.status" right="pending"/>
      <lte left="$action.scheduled_for" right="$now"/>
    </and>
  </predicate>
  
  <!-- Priority -->
  <predicate id="is_urgent_action">
    <eq left="$action.priority" right="urgent"/>
  </predicate>
  
  <!-- Workflow Detection -->
  <predicate id="matches_workflow_category">
    <contains left="$context.keywords" right="$category.keywords"/>
  </predicate>
</predicates>
```

### Z-Axis: Copilot Events

```typescript
enum CopilotEventType {
  // Suggestions
  SUGGESTION_GENERATED = "copilot.suggestion_generated",
  SUGGESTION_ACCEPTED = "copilot.suggestion_accepted",
  SUGGESTION_REJECTED = "copilot.suggestion_rejected",
  
  // Dashboard
  DASHBOARD_GENERATED = "copilot.dashboard_generated",
  ARTIFACT_CREATED = "copilot.artifact_created",
  
  // Actions
  ACTION_QUEUED = "copilot.action_queued",
  ACTION_EXECUTED = "copilot.action_executed",
  ACTION_FAILED = "copilot.action_failed",
  ACTION_CANCELLED = "copilot.action_cancelled",
  
  // Workflows
  WORKFLOW_SUGGESTED = "copilot.workflow_suggested",
  AUTOMATION_SUGGESTED = "copilot.automation_suggested"
}
```

### Workflow Example: Queue and Execute Action

```xml
<workflow id="copilot_queue_execute_action">
  <entry p="copilot" x="action_queue" node="validate_action"/>
  
  <nodes>
    <node id="validate_action" kind="auth">
      <require predicate="can_queue_action"/>
    </node>
    
    <node id="assign_priority" kind="transform">
      <compute>
        <var name="priority" value="
          contains($action_data.keywords, 'urgent') ? 'urgent' :
          contains($action_data.keywords, 'high') ? 'high' :
          'medium'
        "/>
      </compute>
    </node>
    
    <node id="create_queued_action" kind="external" op="0x0910">
      <event>
        <type>copilot.action_queued</type>
        <data>
          <field name="id" value="$uuid()"/>
          <field name="user_id" value="$token.sub"/>
          <field name="action_type" value="$input.action_type"/>
          <field name="action_data" value="$input.action_data"/>
          <field name="status" value="pending"/>
          <field name="priority" value="$assigned_priority"/>
          <field name="scheduled_for" value="$input.scheduled_for || $now"/>
        </data>
      </event>
    </node>
    
    <node id="wait_for_execution_time" kind="transform">
      <when>
        <gt left="$action.scheduled_for" right="$now"/>
      </when>
      <delay_until time="$action.scheduled_for"/>
    </node>
    
    <node id="execute_action" kind="external" op="0x2521">
      <when>
        <predicate ref="is_ready_to_execute"/>
      </when>
      <dispatch>
        <when>
          <eq left="$action.action_type" right="email"/>
        </when>
        <operation op="0x0502"/> <!-- email_send -->
      </dispatch>
      <dispatch>
        <when>
          <eq left="$action.action_type" right="task"/>
        </when>
        <operation op="0x0200"/> <!-- task_create -->
      </dispatch>
      <event type="copilot.action_executed"/>
    </node>
    
    <node id="update_status" kind="external" op="0x0910">
      <event>
        <type>copilot.action_executed</type>
        <data>
          <field name="action_id" value="$action.id"/>
          <field name="status" value="completed"/>
          <field name="executed_at" value="$now"/>
        </data>
      </event>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_action" to="assign_priority"><when><always/></when></edge>
    <edge from="assign_priority" to="create_queued_action"><when><always/></when></edge>
    <edge from="create_queued_action" to="wait_for_execution_time">
      <when><gt left="$action.scheduled_for" right="$now"/></when>
    </edge>
    <edge from="create_queued_action" to="execute_action">
      <when><lte left="$action.scheduled_for" right="$now"/></when>
    </edge>
    <edge from="wait_for_execution_time" to="execute_action"><when><always/></when></edge>
    <edge from="execute_action" to="update_status"><when><always/></when></edge>
    <edge from="update_status" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 3: ASSISTANT MANAGER

### Purpose
Manages AI assistant contexts, token budgets, conversation threading.

### P-Axis: Assistant Entities

```xml
<schema id="assistant_context">
  <field name="id" type="uuid" required="true"/>
  <field name="conversation_id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="context_tokens" type="number"/>
  <field name="max_context_tokens" type="number"/>
  <field name="max_tokens" type="number"/>
  <field name="temperature" type="number"/>
  <field name="model" type="string"/>
  <field name="system_prompt" type="string"/>
  <field name="messages" type="array"/>
  <field name="created_at" type="timestamp"/>
  <field name="updated_at" type="timestamp"/>
</schema>

<schema id="conversation_thread">
  <field name="id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="messages" type="array" required="true"/>
  <field name="total_tokens" type="number"/>
  <field name="created_at" type="timestamp"/>
  <field name="updated_at" type="timestamp"/>
</schema>
```

### X-Axis: Assistant Manager Operations

```yaml
assistant_context_create: 0x2600   # Create assistant context
assistant_context_update: 0x2601   # Update context
assistant_context_get: 0x2602      # Get context
assistant_message_add: 0x2610      # Add message to thread
assistant_complete: 0x2611          # Get completion
assistant_optimize_context: 0x2620 # Optimize token usage
assistant_compress_history: 0x2621  # Compress conversation
```

### Config-Driven Behavior

```json
{
  "assistantManager": {
    "defaults": {
      "maxContextTokens": 8000,
      "maxTokens": 2000,
      "temperature": 0.7,
      "complexity": "standard",
      "conversationTokensBuffer": 2000,
      "optimizationLevel": 3
    }
  }
}
```

---

## PART 4: OPERATION CODE SUMMARY

| Service | Range | Count | Purpose |
|---------|-------|-------|---------|
| **AI Operations** | 0x2400-0x2472 | 25 | AI intelligence (search, NLP, anomaly) |
| **Copilot** | 0x2500-0x2542 | 19 | User assistance & automation |
| **Assistant Manager** | 0x2600-0x2621 | 8 | Context & conversation management |

**Total New Operations**: 52

---

## SUMMARY

AI Intelligence Services provide:

1. **AI Operations**: 25 capabilities
   - Semantic search with RAG
   - Smart linking and mention extraction
   - Predictive typing
   - Summarization and entity extraction
   - Document comparison
   - Translation
   - Sentiment analysis
   - Anomaly detection
   - Data freshness checking

2. **Copilot Service**: 19 capabilities
   - Inline suggestions
   - Dashboard generation
   - Action queue with scheduling
   - Contextual help
   - Workflow recommendations
   - Pattern detection
   - Automation suggestions

3. **Assistant Manager**: 8 capabilities
   - Conversation context management
   - Token budget optimization
   - Message threading
   - Context compression

All services are **config-driven** with model resolution, temperature settings, and confidence thresholds from config.json.

**Integration**: AI Operations power Copilot suggestions → AssistantManager manages context → All operations emit events.
