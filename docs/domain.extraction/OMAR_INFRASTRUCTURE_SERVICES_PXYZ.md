# OMAR Infrastructure Services - PXYZ Extraction

> **Services**: LibrarianService, RedactionService, TokenManagementService, Rules  
> **Purpose**: Knowledge management, PII protection, cost tracking, business rules

---

## PART 1: LIBRARIAN SERVICE (Knowledge Management)

### Purpose
Advanced knowledge management with compression, RAG retrieval, graph traversal, and temporal memory.

### P-Axis: Librarian Entities

```xml
<schema id="librarian_input">
  <field name="user_id" type="uuid" required="true"/>
  <field name="query" type="string" required="true"/>
  <field name="context" type="object"/>
  <field name="token_budget" type="enum" values="compressed,balanced,full"/>
  <field name="max_tokens" type="number"/>
  <field name="filters" type="object"/>
</schema>

<schema id="librarian_output">
  <field name="retrieved_chunks" type="array" required="true"/>
  <field name="synthesized_response" type="string" required="true"/>
  <field name="confidence" type="number" required="true"/>
  <field name="knowledge_gaps" type="array"/>
  <field name="recommended_actions" type="array"/>
  <field name="processing_metrics" type="object" required="true"/>
  <field name="routing" type="object" required="true"/>
</schema>

<schema id="knowledge_chunk">
  <field name="id" type="uuid" required="true"/>
  <field name="content" type="string" required="true"/>
  <field name="title" type="string"/>
  <field name="category" type="string"/>
  <field name="relevance_score" type="number" required="true"/>
  <field name="compression_stage" type="enum" values="raw,entities,deduplicated,abstraction,temporal"/>
  <field name="tokens" type="number"/>
  <field name="source" type="string"/>
  <field name="created_at" type="timestamp"/>
</schema>

<schema id="compression_stage">
  <field name="name" type="string" required="true"/>
  <field name="target_tokens" type="number" required="true"/>
  <field name="compression_ratio" type="number" required="true"/>
  <field name="input_size" type="number"/>
  <field name="output_size" type="number"/>
</schema>

<schema id="rag_chain_config">
  <field name="cache" type="object"/>
  <field name="vectorization" type="object"/>
  <field name="graph_traversal" type="object"/>
  <field name="inference" type="object"/>
</schema>
```

### X-Axis: Librarian Operations

```yaml
# RAG Retrieval
librarian_rag_retrieve: 0x2700      # Perform RAG retrieval
librarian_knowledge_query: 0x2701   # Query knowledge base
librarian_chunks_retrieve: 0x2702   # Retrieve chunks
librarian_synthesize: 0x2703        # Synthesize response

# Compression (5 stages)
librarian_compress_raw: 0x2710      # Stage 1: Raw (8000 tokens)
librarian_compress_entities: 0x2711 # Stage 2: Entities (2000 tokens)
librarian_compress_dedup: 0x2712    # Stage 3: Dedup (500 tokens)
librarian_compress_abstract: 0x2713 # Stage 4: Abstraction (100 tokens)
librarian_compress_temporal: 0x2714 # Stage 5: Temporal (10 tokens)

# Graph Operations
librarian_graph_traverse: 0x2720    # Graph traversal
librarian_graph_best_first: 0x2721  # Best-first search
librarian_inference_chain: 0x2722   # Multi-step inference

# Context Management
librarian_context_analyze: 0x2730   # Analyze business context
librarian_context_enrich: 0x2731    # Enrich with metadata
librarian_recency_weight: 0x2732    # Apply recency decay

# Caching
librarian_cache_get: 0x2740         # Get from cache
librarian_cache_set: 0x2741         # Set cache entry
librarian_cache_invalidate: 0x2742  # Invalidate cache
```

### Y-Axis: Librarian Predicates

```xml
<predicates>
  <!-- Compression Stage Selection -->
  <predicate id="use_compressed_stage">
    <and>
      <eq left="$token_budget" right="compressed"/>
      <lte left="$available_tokens" right="1000"/>
    </and>
  </predicate>
  
  <predicate id="use_balanced_stage">
    <and>
      <eq left="$token_budget" right="balanced"/>
      <lte left="$available_tokens" right="5000"/>
    </and>
  </predicate>
  
  <predicate id="use_full_stage">
    <gte left="$available_tokens" right="5000"/>
  </predicate>
  
  <!-- Relevance Filtering -->
  <predicate id="is_relevant_chunk">
    <gte left="$chunk.relevance_score" right="0.7"/>
  </predicate>
  
  <predicate id="passes_similarity_threshold">
    <gte left="$similarity" right="0.85"/>
  </predicate>
  
  <!-- Cache Hit -->
  <predicate id="has_cache_hit">
    <and>
      <exists left="$cache[$query_hash]"/>
      <gt left="$cache_entry.ttl_remaining" right="0"/>
    </and>
  </predicate>
  
  <!-- Recency -->
  <predicate id="is_recent_knowledge">
    <lte left="$days_since_created" right="7"/>
  </predicate>
</predicates>
```

### Z-Axis: Librarian Events

```typescript
enum LibrarianEventType {
  RAG_RETRIEVAL_STARTED = "librarian.rag.started",
  RAG_RETRIEVAL_SUCCESS = "librarian.rag.success",
  RAG_RETRIEVAL_FAILED = "librarian.rag.failed",
  
  COMPRESSION_EXECUTED = "librarian.compression.executed",
  KNOWLEDGE_CHUNK_RETRIEVED = "librarian.chunk.retrieved",
  CACHE_HIT = "librarian.cache.hit",
  CACHE_MISS = "librarian.cache.miss",
  
  GRAPH_TRAVERSAL_COMPLETED = "librarian.graph.traversal_completed",
  INFERENCE_CHAIN_EXECUTED = "librarian.inference.chain_executed"
}
```

### Compression Stages (from config)

```json
{
  "compressionStages": {
    "stage1": { "name": "raw", "targetTokens": 8000, "compressionRatio": 1 },
    "stage2": { "name": "entities", "targetTokens": 2000, "compressionRatio": 4 },
    "stage3": { "name": "deduplicated", "targetTokens": 500, "compressionRatio": 16 },
    "stage4": { "name": "abstraction", "targetTokens": 100, "compressionRatio": 80 },
    "stage5": { "name": "temporal", "targetTokens": 10, "compressionRatio": 800 }
  },
  "tokenBudgetStrategy": {
    "compressed": { "maxTokens": 1000, "stage": 5 },
    "balanced": { "maxTokens": 5000, "stage": 3 },
    "full": { "minTokens": 5000, "stage": 1 }
  }
}
```

### Workflow Example: RAG Retrieval with Compression

```xml
<workflow id="librarian_rag_with_compression">
  <entry p="librarian" x="rag_retrieve" node="validate_input"/>
  
  <nodes>
    <node id="validate_input" kind="transform">
      <validate schema="librarian_input"/>
    </node>
    
    <node id="check_cache" kind="external" op="0x2740">
      <compute_hash query="$input.query" context="$input.context"/>
      <lookup cache_key="$query_hash"/>
    </node>
    
    <node id="analyze_context" kind="external" op="0x2730">
      <when>
        <not><predicate ref="has_cache_hit"/></not>
      </when>
      <extract_business_concepts from="$input.query"/>
      <identify_domain category="$business_concepts"/>
      <event type="librarian.context.analyzed"/>
    </node>
    
    <node id="determine_compression_stage" kind="transform">
      <compute>
        <var name="available_tokens" value="$input.max_tokens || 8000"/>
        <var name="stage" value="
          $available_tokens <= 1000 ? 5 :
          $available_tokens <= 5000 ? 3 : 1
        "/>
      </compute>
    </node>
    
    <node id="retrieve_chunks" kind="external" op="0x2702">
      <rag_chain>
        <vectorize query="$input.query" model="text-embedding-3-small"/>
        <search collection="business_knowledge" top_k="20"/>
        <graph_traverse max_depth="3" strategy="best-first"/>
      </rag_chain>
      <event type="librarian.chunk.retrieved"/>
    </node>
    
    <node id="compress_chunks" kind="external" op="0x2714">
      <when>
        <gt left="$compression_stage" right="1"/>
      </when>
      <for_each chunk="$retrieved_chunks">
        <apply_compression stage="$compression_stage">
          <!-- Stage 2: Extract entities -->
          <when><eq left="$stage" right="2"/></when>
          <extract_entities/>
          
          <!-- Stage 3: Deduplicate -->
          <when><eq left="$stage" right="3"/></when>
          <check_similarity threshold="0.85"/>
          <remove_duplicates/>
          
          <!-- Stage 4: Abstract -->
          <when><eq left="$stage" right="4"/></when>
          <create_abstraction/>
          
          <!-- Stage 5: Temporal -->
          <when><eq left="$stage" right="5"/></when>
          <create_temporal_summary/>
        </apply_compression>
      </for_each>
      <event type="librarian.compression.executed"/>
    </node>
    
    <node id="apply_recency_weighting" kind="external" op="0x2732">
      <for_each chunk="$compressed_chunks">
        <compute>
          <var name="days_old" value="days_between($now, $chunk.created_at)"/>
          <var name="decay_factor" value="exp(-$days_old / 7)"/> <!-- Half-life: 7 days -->
          <var name="weighted_score" value="$chunk.relevance_score * $decay_factor"/>
        </compute>
      </for_each>
    </node>
    
    <node id="synthesize_response" kind="external" op="0x2703">
      <llm_synthesize>
        <context chunks="$weighted_chunks"/>
        <query value="$input.query"/>
        <style value="$business_context.communication_style"/>
      </llm_synthesize>
      <event type="librarian.response.synthesized"/>
    </node>
    
    <node id="cache_result" kind="external" op="0x2741">
      <set_cache>
        <key value="$query_hash"/>
        <value>
          <field name="chunks" value="$weighted_chunks"/>
          <field name="response" value="$synthesized"/>
        </value>
        <ttl value="3600"/> <!-- 1 hour -->
      </set_cache>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_input" to="check_cache"><when><always/></when></edge>
    <edge from="check_cache" to="analyze_context">
      <when><not><predicate ref="has_cache_hit"/></not></when>
    </edge>
    <edge from="check_cache" to="success">
      <when><predicate ref="has_cache_hit"/></when>
    </edge>
    <edge from="analyze_context" to="determine_compression_stage"><when><always/></when></edge>
    <edge from="determine_compression_stage" to="retrieve_chunks"><when><always/></when></edge>
    <edge from="retrieve_chunks" to="compress_chunks">
      <when><gt left="$compression_stage" right="1"/></when>
    </edge>
    <edge from="retrieve_chunks" to="apply_recency_weighting">
      <when><eq left="$compression_stage" right="1"/></when>
    </edge>
    <edge from="compress_chunks" to="apply_recency_weighting"><when><always/></when></edge>
    <edge from="apply_recency_weighting" to="synthesize_response"><when><always/></when></edge>
    <edge from="synthesize_response" to="cache_result"><when><always/></when></edge>
    <edge from="cache_result" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 2: REDACTION SERVICE (PII Protection)

### Purpose
Detect and redact PII (Personally Identifiable Information) for compliance.

### P-Axis: Redaction Entities

```xml
<schema id="redaction_request">
  <field name="id" type="uuid" required="true"/>
  <field name="content" type="string" required="true"/>
  <field name="pii_types" type="array"/> <!-- which PII to detect -->
  <field name="redaction_mode" type="enum" values="mask,hash,remove,tokenize"/>
  <field name="preserve_format" type="boolean"/>
  <field name="confidence_threshold" type="number"/>
</schema>

<schema id="redaction_result">
  <field name="redacted_content" type="string" required="true"/>
  <field name="detections" type="array" required="true"/>
  <field name="compliance_score" type="number"/>
  <field name="risk_assessment" type="object"/>
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="pii_detection">
  <field name="type" type="enum" values="email,phone,ssn,credit_card,ip_address,address,name,dob,passport,driver_license,bank_account"/>
  <field name="value" type="string"/>
  <field name="position" type="object"/> <!-- start, end -->
  <field name="confidence" type="number"/>
  <field name="severity" type="enum" values="low,medium,high,critical"/>
  <field name="redacted_value" type="string"/>
</schema>

<schema id="compliance_assessment">
  <field name="total_pii_found" type="number"/>
  <field name="critical_pii_count" type="number"/>
  <field name="compliance_score" type="number"/>
  <field name="risk_level" type="enum" values="low,medium,high,critical"/>
  <field name="recommendations" type="array"/>
</schema>
```

### X-Axis: Redaction Operations

```yaml
# PII Detection
redaction_detect_pii: 0x2800        # Detect all PII
redaction_detect_email: 0x2801      # Detect emails
redaction_detect_phone: 0x2802      # Detect phone numbers
redaction_detect_ssn: 0x2803        # Detect SSN
redaction_detect_credit_card: 0x2804 # Detect credit cards
redaction_detect_address: 0x2805    # Detect addresses

# Redaction Methods
redaction_mask: 0x2810              # Mask PII (****)
redaction_hash: 0x2811              # Hash PII (SHA-256)
redaction_remove: 0x2812            # Remove PII entirely
redaction_tokenize: 0x2813          # Replace with token

# Compliance
redaction_assess_compliance: 0x2820 # Assess compliance risk
redaction_generate_report: 0x2821   # Generate compliance report
redaction_audit_trail: 0x2822       # Log redactions
```

### Y-Axis: Redaction Predicates

```xml
<predicates>
  <!-- PII Detection -->
  <predicate id="is_valid_email">
    <matches left="$value" regex="\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b"/>
  </predicate>
  
  <predicate id="is_valid_phone">
    <matches left="$value" regex="\\b(\\+?1[-.\\s]?)?\\(?\\d{3}\\)?[-.\\s]?\\d{3}[-.\\s]?\\d{4}\\b"/>
  </predicate>
  
  <predicate id="is_valid_ssn">
    <matches left="$value" regex="\\b\\d{3}-\\d{2}-\\d{4}\\b"/>
  </predicate>
  
  <!-- Severity Assessment -->
  <predicate id="is_critical_pii">
    <contains left="['ssn','credit_card','bank_account','passport']" right="$pii_type"/>
  </predicate>
  
  <predicate id="requires_immediate_redaction">
    <and>
      <predicate ref="is_critical_pii"/>
      <gte left="$confidence" right="0.95"/>
    </and>
  </predicate>
  
  <!-- Compliance -->
  <predicate id="passes_compliance">
    <and>
      <eq left="$critical_pii_count" right="0"/>
      <gte left="$compliance_score" right="80"/>
    </and>
  </predicate>
  
  <predicate id="requires_review">
    <or>
      <gt left="$critical_pii_count" right="0"/>
      <lt left="$compliance_score" right="60"/>
    </or>
  </predicate>
</predicates>
```

### Z-Axis: Redaction Events

```typescript
enum RedactionEventType {
  PII_DETECTED = "redaction.pii_detected",
  CONTENT_REDACTED = "redaction.content_redacted",
  COMPLIANCE_ASSESSED = "redaction.compliance_assessed",
  CRITICAL_PII_FOUND = "redaction.critical_pii_found",
  REDACTION_AUDIT = "redaction.audit"
}
```

### PII Patterns (from config)

```json
{
  "piiPatterns": {
    "email": "\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b",
    "phone": "\\b(\\+?1[-.\\s]?)?\\(?\\d{3}\\)?[-.\\s]?\\d{3}[-.\\s]?\\d{4}\\b",
    "ssn": "\\b\\d{3}-\\d{2}-\\d{4}\\b",
    "credit_card": "\\b\\d{4}[\\s-]?\\d{4}[\\s-]?\\d{4}[\\s-]?\\d{4}\\b",
    "ip_address": "\\b(?:\\d{1,3}\\.){3}\\d{1,3}\\b"
  },
  "defaults": {
    "maskChar": "*",
    "preserveFormat": true,
    "confidence": 0.95
  },
  "compliance": {
    "severityWeights": {
      "low": 5,
      "medium": 15,
      "high": 30,
      "critical": 50
    }
  }
}
```

### Workflow Example: Detect and Redact PII

```xml
<workflow id="redaction_detect_and_redact">
  <entry p="redaction" x="detect_pii" node="scan_content"/>
  
  <nodes>
    <node id="scan_content" kind="external" op="0x2800">
      <for_each pattern="$pii_patterns">
        <regex_match content="$input.content" pattern="$pattern.regex"/>
        <when><matches/></when>
        <create_detection>
          <field name="type" value="$pattern.type"/>
          <field name="value" value="$matched_text"/>
          <field name="position" value="$match_position"/>
          <field name="confidence" value="0.95"/>
        </create_detection>
      </for_each>
      <event type="redaction.pii_detected"/>
    </node>
    
    <node id="classify_severity" kind="transform">
      <for_each detection="$detections">
        <compute>
          <var name="severity" value="
            is_critical_pii($detection.type) ? 'critical' :
            contains(['name','email','phone'], $detection.type) ? 'high' :
            contains(['ip_address','address'], $detection.type) ? 'medium' : 'low'
          "/>
        </compute>
      </for_each>
    </node>
    
    <node id="redact_critical_pii" kind="external" op="0x2811">
      <when>
        <gt left="count($critical_detections)" right="0"/>
      </when>
      <for_each detection="$critical_detections">
        <hash value="$detection.value" algorithm="sha256"/>
        <replace_in_content original="$detection.value" replacement="[REDACTED-$hash]"/>
      </for_each>
      <event type="redaction.critical_pii_found"/>
    </node>
    
    <node id="mask_other_pii" kind="external" op="0x2810">
      <for_each detection="$non_critical_detections">
        <mask>
          <when><eq left="$preserve_format" right="true"/></when>
          <preserve_length value="true"/>
          <mask_char value="*"/>
        </mask>
        <replace_in_content/>
      </for_each>
      <event type="redaction.content_redacted"/>
    </node>
    
    <node id="assess_compliance" kind="external" op="0x2820">
      <compute>
        <var name="weighted_score" value="
          sum($detections, d => $severity_weights[d.severity])
        "/>
        <var name="compliance_score" value="max(0, 100 - $weighted_score)"/>
        <var name="risk_level" value="
          $compliance_score >= 80 ? 'low' :
          $compliance_score >= 60 ? 'medium' :
          $compliance_score >= 40 ? 'high' : 'critical'
        "/>
      </compute>
      <event type="redaction.compliance_assessed"/>
    </node>
    
    <node id="generate_audit_log" kind="external" op="0x2822">
      <event>
        <type>redaction.audit</type>
        <data>
          <field name="total_pii_found" value="count($detections)"/>
          <field name="critical_pii_count" value="count($critical_detections)"/>
          <field name="compliance_score" value="$compliance_score"/>
          <field name="risk_level" value="$risk_level"/>
        </data>
      </event>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="scan_content" to="classify_severity"><when><always/></when></edge>
    <edge from="classify_severity" to="redact_critical_pii">
      <when><gt left="count($critical_detections)" right="0"/></when>
    </edge>
    <edge from="classify_severity" to="mask_other_pii">
      <when><eq left="count($critical_detections)" right="0"/></when>
    </edge>
    <edge from="redact_critical_pii" to="mask_other_pii"><when><always/></when></edge>
    <edge from="mask_other_pii" to="assess_compliance"><when><always/></when></edge>
    <edge from="assess_compliance" to="generate_audit_log"><when><always/></when></edge>
    <edge from="generate_audit_log" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 3: TOKEN MANAGEMENT SERVICE

### Purpose
Track AI token usage, estimate costs, enforce quotas.

### P-Axis: Token Management Entities

```xml
<schema id="token_usage">
  <field name="id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="provider" type="enum" values="anthropic,openai,mistral,openrouter,local"/>
  <field name="model" type="string" required="true"/>
  <field name="operation" type="string" required="true"/>
  <field name="input_tokens" type="number" required="true"/>
  <field name="output_tokens" type="number" required="true"/>
  <field name="estimated_cost_usd" type="number"/>
  <field name="timestamp" type="timestamp" required="true"/>
</schema>

<schema id="quota_constraints">
  <field name="user_id" type="uuid" required="true"/>
  <field name="daily_token_limit" type="number"/>
  <field name="monthly_token_limit" type="number"/>
  <field name="daily_cost_limit_usd" type="number"/>
  <field name="monthly_cost_limit_usd" type="number"/>
  <field name="alert_threshold" type="number"/> <!-- 0.0-1.0 -->
  <field name="enabled" type="boolean"/>
</schema>

<schema id="usage_summary">
  <field name="total_tokens" type="number"/>
  <field name="total_cost_usd" type="number"/>
  <field name="by_provider" type="object"/>
  <field name="by_operation" type="object"/>
  <field name="time_period" type="object"/>
</schema>
```

### X-Axis: Token Management Operations

```yaml
# Tracking
token_record_usage: 0x2900          # Record token usage
token_estimate: 0x2901              # Estimate token count
token_calculate_cost: 0x2902        # Calculate cost

# Quotas
token_check_quota: 0x2910           # Check quota limits
token_get_remaining: 0x2911         # Get remaining quota
token_set_quota: 0x2912             # Set quota limits
token_alert_threshold: 0x2913       # Alert on threshold

# Reporting
token_usage_summary: 0x2920         # Get usage summary
token_usage_by_period: 0x2921       # Usage by time period
token_usage_by_provider: 0x2922     # Usage by AI provider
token_usage_by_operation: 0x2923    # Usage by operation
```

### Y-Axis: Token Management Predicates

```xml
<predicates>
  <!-- Quota Enforcement -->
  <predicate id="exceeds_daily_quota">
    <and>
      <eq left="$quota.enabled" right="true"/>
      <gt left="$usage.daily_tokens" right="$quota.daily_token_limit"/>
    </and>
  </predicate>
  
  <predicate id="exceeds_monthly_quota">
    <and>
      <eq left="$quota.enabled" right="true"/>
      <gt left="$usage.monthly_tokens" right="$quota.monthly_token_limit"/>
    </and>
  </predicate>
  
  <predicate id="approaching_quota_limit">
    <gte left="$usage_ratio" right="$quota.alert_threshold"/>
  </predicate>
  
  <!-- Cost Limits -->
  <predicate id="exceeds_cost_limit">
    <gt left="$usage.total_cost" right="$quota.monthly_cost_limit"/>
  </predicate>
</predicates>
```

### Provider Pricing (from config)

```json
{
  "providerPricing": {
    "mistral": {
      "input": 0.25,
      "output": 0.75,
      "unit": "per_1m_tokens",
      "currency": "USD",
      "model": "mistral-small"
    },
    "openai": {
      "input": 0.6,
      "output": 1.2,
      "unit": "per_1m_tokens",
      "currency": "USD",
      "model": "gpt-4o-mini"
    },
    "anthropic": {
      "input": 3.0,
      "output": 15.0,
      "unit": "per_1m_tokens",
      "currency": "USD",
      "model": "claude-3.5-sonnet"
    }
  },
  "defaults": {
    "alertThreshold": 0.8,
    "enabled": true
  }
}
```

---

## PART 4: BUSINESS RULES (Config-Driven Logic)

### Purpose
Centralized business rules extracted from code into configuration.

### Rules Categories (from rules.ts)

#### 1. AI Operation Patterns
```json
{
  "extractMentions": {
    "person": "@[A-Za-z]+",
    "company": "[A-Z][a-z]+ (Inc|Corp|LLC)",
    "task": "#[0-9]+"
  },
  "detectAnomalies": {
    "zScoreThreshold": 2,
    "minDataPoints": 10
  }
}
```

#### 2. Extraction Patterns
```json
{
  "email": "\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b",
  "url": "https?://[^\\s]+",
  "date": "\\d{1,2}/\\d{1,2}/\\d{4}"
}
```

#### 3. Priority Detection
```json
{
  "urgent": ["urgent", "asap", "critical", "emergency"],
  "high": ["important", "high priority", "soon"],
  "medium": ["normal", "medium"],
  "low": ["low priority", "whenever"]
}
```

#### 4. Status Transitions
```json
{
  "contact": {
    "lead": ["prospect", "qualified"],
    "prospect": ["client", "lost"],
    "client": ["inactive"],
    "inactive": ["lead"]
  },
  "task": {
    "idea": ["planned"],
    "planned": ["in_progress", "blocked", "cancelled"],
    "in_progress": ["delivered", "blocked"],
    "blocked": ["in_progress", "cancelled"],
    "delivered": ["follow_up", "done"],
    "follow_up": ["done"],
    "done": [],
    "cancelled": []
  }
}
```

### Rules as Predicates

All business rules become predicates in workflows:

```xml
<predicates>
  <!-- From rules.ts: detectPriority -->
  <predicate id="is_urgent_keyword">
    <contains left="['urgent','asap','critical','emergency']" right="$text"/>
  </predicate>
  
  <!-- From rules.ts: extractStatusTransition -->
  <predicate id="can_transition_task">
    <contains left="$valid_transitions[$current_status]" right="$new_status"/>
  </predicate>
  
  <!-- From rules.ts: checkStaleness -->
  <predicate id="is_stale">
    <gte left="$days_since_update" right="30"/>
  </predicate>
</predicates>
```

---

## PART 5: OPERATION CODE SUMMARY

| Service | Range | Count | Purpose |
|---------|-------|-------|---------|
| **Librarian** | 0x2700-0x2742 | 19 | Knowledge management, RAG, compression |
| **Redaction** | 0x2800-0x2822 | 17 | PII detection & redaction |
| **Token Management** | 0x2900-0x2923 | 14 | Cost tracking, quotas |

**Total New Operations**: 50

---

## SUMMARY

Infrastructure Services provide:

1. **Librarian Service**: 19 capabilities
   - 5-stage compression (raw → temporal)
   - RAG retrieval with caching
   - Graph traversal & inference
   - Recency weighting
   - Context analysis

2. **Redaction Service**: 17 capabilities
   - 11 PII types detected
   - 4 redaction methods (mask, hash, remove, tokenize)
   - Compliance assessment
   - Severity classification
   - Audit trail

3. **Token Management**: 14 capabilities
   - Usage tracking (5 providers)
   - Cost estimation
   - Quota enforcement (daily/monthly)
   - Alert thresholds
   - Usage reporting

4. **Business Rules**: Config-driven
   - All logic extracted to JSON
   - Patterns, priorities, transitions
   - Becomes predicates in workflows

All services use **config.json** for behavior - zero hardcoded business logic.

**Integration**: Librarian powers search → Redaction protects output → TokenManagement tracks costs → Rules validate state changes.
