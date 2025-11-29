# OMAR Infrastructure Services - PXYZ Extraction

> **Custom Research**: Core infrastructure services as graph operations  
> **NOT microservices**: AI operations, prompt parsing, audit trails, token management, redaction as coordinate-addressable operations  
> **Services**: AIOperations, PromptParser, AuditQueryService, TokenManagementService, RedactionService

---

## EXECUTIVE SUMMARY

Your **Infrastructure Services** represent the foundational operations layer that supports the entire OMAR system. Unlike business logic (CRM, workflows, tasks), these services provide:

1. **AIOperations** = LLM invocation with config-driven model routing
2. **PromptParser** = Multi-format prompt compilation (Markdown/LaTeX/POML → FlowDefinition)
3. **AuditQueryService** = EventBus querying as audit trail (no separate logging)
4. **TokenManagementService** = Token tracking, quota enforcement, cost monitoring
5. **RedactionService** = PII detection, compliance validation, reversible masking

**Key Insight**: These are NOT separate microservices. They're **graph-addressable operations** that other workflows call via operation codes.

```typescript
// WRONG (Microservices)
await aiService.semanticSearch(query);
await auditService.queryTrail(filter);
await tokenService.trackUsage(usage);
await redactionService.detectPII(text);

// RIGHT (Your Architecture)
await pxyz('knowledge', 'search', {query}, timestamp);     // op 0x4000
await pxyz('audit', 'query', {filter}, timestamp);         // op 0x4100
await pxyz('tokens', 'track', {usage}, timestamp);         // op 0x4200
await pxyz('redaction', 'detect', {text}, timestamp);      // op 0x4300
```

---

## ARCHITECTURE OVERVIEW

### Five Infrastructure Services

```
┌─────────────────────────────────────────────────────────────┐
│  AI OPERATIONS - LLM Invocation Layer                       │
│  ──────────────────────────────────                         │
│  Semantic search, smart linking, summarization, anomaly     │
│  detection, predictive typing via config-driven models      │
├─────────────────────────────────────────────────────────────┤
│  PROMPT PARSER - Multi-Format Compilation                   │
│  ─────────────────────────────────────                      │
│  Markdown/LaTeX/POML → FlowDefinition → AgentConfig         │
│  Single entry point for all prompt format compilation       │
├─────────────────────────────────────────────────────────────┤
│  AUDIT QUERY SERVICE - EventBus as Audit Trail              │
│  ──────────────────────────────────────────────             │
│  No separate logging - EventBus IS the audit trail          │
│  Query events with filters, pagination, YCtx predicates     │
├─────────────────────────────────────────────────────────────┤
│  TOKEN MANAGEMENT - Usage Tracking & Cost Control           │
│  ──────────────────────────────────────────────────         │
│  Per-user/workspace quotas, multi-provider pricing,         │
│  soft/hard limits, cost monitoring, usage analytics         │
├─────────────────────────────────────────────────────────────┤
│  REDACTION SERVICE - PII Detection & Compliance             │
│  ────────────────────────────────────────────────           │
│  Pattern-based + AI detection, reversible masking,          │
│  GDPR/HIPAA/PCI-DSS validation, coordinate-space PII        │
└─────────────────────────────────────────────────────────────┘
```

---

## P-AXIS: INFRASTRUCTURE ENTITIES

### AI Operations Entities

```xml
<schema id="ai_operation_request">
  <field name="type" type="string" required="true"/> <!-- OperationName -->
  <field name="payload" type="object" required="true"/> <!-- Operation-specific data -->
</schema>

<schema id="ai_operation_result">
  <field name="success" type="boolean" required="true"/>
  <field name="operation" type="string" required="true"/> <!-- Operation name -->
  <field name="data" type="object" required="true"/>
  <field name="model_used" type="string" required="true"/> <!-- EntityName -->
  <field name="processing_time" type="number" required="true"/> <!-- LatencyMs -->
  <field name="tokens_estimated" type="number"/> <!-- Estimated tokens -->
</schema>

<schema id="semantic_search_result">
  <field name="results" type="array"/> <!-- Search results -->
  <field name="total_count" type="number"/>
  <field name="query_time" type="number"/>
  <field name="suggestions" type="array"/>
  <field name="search_strategy" type="enum" values="web_and_knowledge,knowledge_only"/>
  <field name="web_search_enabled" type="boolean"/>
</schema>

<schema id="smart_link_suggestion">
  <field name="source_entity" type="uuid"/>
  <field name="target_entity" type="uuid"/>
  <field name="link_type" type="string"/>
  <field name="confidence" type="number"/>
  <field name="reason" type="string"/>
</schema>

<schema id="anomaly_detection_result">
  <field name="data_points" type="array"/>
  <field name="anomalies" type="array">
    <item>
      <field name="index" type="number"/>
      <field name="value" type="number"/>
      <field name="severity" type="enum" values="low,medium,high,critical"/>
      <field name="zscore" type="number"/>
      <field name="threshold" type="number"/>
    </item>
  </field>
  <field name="statistics" type="object">
    <field name="mean" type="number"/>
    <field name="stddev" type="number"/>
    <field name="min" type="number"/>
    <field name="max" type="number"/>
  </field>
</schema>
```

### Prompt Parser Entities

```xml
<schema id="flow_definition">
  <field name="id" type="uuid" required="true"/>
  <field name="source_format" type="enum" values="markdown,latex,poml,custom" required="true"/>
  <field name="metadata" type="object">
    <field name="goal" type="string"/>
    <field name="user_id" type="string"/>
    <field name="session_id" type="string"/>
    <field name="context_scope" type="string"/>
  </field>
  <field name="steps" type="array" required="true">
    <item>
      <field name="id" type="string"/>
      <field name="type" type="string"/>
      <field name="config" type="object"/>
    </item>
  </field>
  <field name="edges" type="array"/> <!-- Step dependencies -->
</schema>

<schema id="unified_ast_node">
  <field name="type" type="string" required="true"/>
  <field name="children" type="array"/>
  <field name="value" type="string"/>
  <field name="depth" type="number"/> <!-- For headings -->
  <field name="url" type="string"/> <!-- For links -->
  <field name="lang" type="string"/> <!-- For code blocks -->
  <field name="ordered" type="boolean"/> <!-- For lists -->
</schema>
```

### Audit Query Entities

```xml
<schema id="audit_query_filter">
  <field name="entity_id" type="uuid"/> <!-- Filter by entity -->
  <field name="actor_id" type="uuid"/> <!-- Filter by actor -->
  <field name="event_type" type="string"/> <!-- Filter by operation -->
  <field name="entity_type" type="string"/> <!-- Filter by entity type -->
  <field name="start_time" type="timestamp"/> <!-- Time range start -->
  <field name="end_time" type="timestamp"/> <!-- Time range end -->
  <field name="limit" type="number"/> <!-- Pagination limit -->
  <field name="offset" type="number"/> <!-- Pagination offset -->
  <field name="where" type="object"/> <!-- YCtx predicate spec -->
</schema>

<schema id="audit_query_result">
  <field name="events" type="array" required="true"/> <!-- ComplianceEvent[] -->
  <field name="total_count" type="number" required="true"/>
  <field name="filtered_count" type="number" required="true"/>
</schema>
```

### Token Management Entities

```xml
<schema id="usage_record">
  <field name="id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="workspace_id" type="string"/>
  <field name="operation" type="string" required="true"/>
  <field name="provider" type="enum" values="mistral,openai,anthropic,openrouter,local" required="true"/>
  <field name="model" type="string" required="true"/>
  <field name="tokens_input" type="number" required="true"/>
  <field name="tokens_output" type="number" required="true"/>
  <field name="tokens_total" type="number" required="true"/>
  <field name="cost" type="number" required="true"/> <!-- USD -->
  <field name="timestamp" type="timestamp" required="true"/>
  <field name="metadata" type="object"/>
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="quota_settings">
  <field name="id" type="uuid" required="true"/>
  <field name="user_id" type="uuid"/>
  <field name="workspace_id" type="string"/>
  <field name="daily_token_limit" type="number"/>
  <field name="monthly_token_limit" type="number"/>
  <field name="total_token_limit" type="number"/>
  <field name="daily_cost_limit" type="number"/> <!-- USD -->
  <field name="monthly_cost_limit" type="number"/> <!-- USD -->
  <field name="total_cost_limit" type="number"/> <!-- USD -->
  <field name="alert_threshold" type="number"/> <!-- 0.0-1.0 -->
  <field name="enabled" type="boolean" required="true"/>
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="usage_summary">
  <field name="user_id" type="uuid" required="true"/>
  <field name="workspace_id" type="string"/>
  <field name="period" type="enum" values="daily,weekly,monthly,total" required="true"/>
  <field name="tokens_used" type="number" required="true"/>
  <field name="tokens_remaining" type="number"/>
  <field name="cost_used" type="number" required="true"/> <!-- USD -->
  <field name="cost_remaining" type="number"/> <!-- USD -->
  <field name="quota_percentage" type="number"/> <!-- 0.0-1.0 -->
  <field name="approaching_limit" type="boolean"/>
  <field name="limit_exceeded" type="boolean"/>
</schema>
```

### Redaction Service Entities

```xml
<schema id="pii_region">
  <field name="type" type="enum" values="email,phone,ssn,credit_card,ip_address,address,name,dob,passport,driver_license,bank_account,custom" required="true"/>
  <field name="span" type="array" required="true"> <!-- [start, end] coordinates -->
    <item type="number"/>
  </field>
  <field name="confidence" type="number" required="true"/> <!-- 0.0-1.0 -->
  <field name="value" type="string" required="true"/> <!-- Original text -->
  <field name="detected_by" type="string"/> <!-- Detector name -->
</schema>

<schema id="overlap_conflict">
  <field name="region1" type="object" required="true"/> <!-- PIIRegion -->
  <field name="region2" type="object" required="true"/> <!-- PIIRegion -->
  <field name="overlap_span" type="array" required="true"/> <!-- [start, end] -->
</schema>

<schema id="redaction_result">
  <field name="redacted_text" type="string" required="true"/>
  <field name="regions_detected" type="array" required="true"/> <!-- PIIRegion[] -->
  <field name="conflicts_resolved" type="array"/> <!-- OverlapConflict[] -->
  <field name="redaction_map" type="object" required="true"/> <!-- For reversal -->
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="compliance_validation_result">
  <field name="compliant" type="boolean" required="true"/>
  <field name="framework" type="enum" values="GDPR,HIPAA,PCI-DSS,CCPA,SOC2" required="true"/>
  <field name="violations" type="array"/> <!-- Detected violations -->
  <field name="warnings" type="array"/> <!-- Potential issues -->
  <field name="compliance_score" type="number"/> <!-- 0.0-1.0 -->
  <field name="pxyz" type="object" required="true"/>
</schema>
```

---

## X-AXIS: INFRASTRUCTURE OPERATIONS

```yaml
# AI Operations (0x4000-0x4050)
ai_semantic_search: 0x4000              # Semantic search with RAG
ai_smart_link: 0x4001                   # Suggest entity relationships
ai_summarize: 0x4002                    # Summarize content
ai_detect_anomalies: 0x4003             # Detect statistical anomalies
ai_predictive_type: 0x4004              # Predictive typing suggestions
ai_data_freshness: 0x4005               # Assess data staleness
ai_risk_assessment: 0x4006              # Calculate risk scores
ai_invoke_model: 0x4007                 # Direct model invocation

# Prompt Parser (0x4100-0x4130)
prompt_parse_markdown: 0x4100           # Parse Markdown to AST
prompt_parse_latex: 0x4101              # Parse LaTeX to AST
prompt_parse_poml: 0x4102               # Parse POML to AST
prompt_compile_to_flow: 0x4110          # Compile AST → FlowDefinition
prompt_validate_flow: 0x4111            # Validate flow against schema
prompt_enrich_flow: 0x4112              # Enrich flow with defaults
prompt_extract_metadata: 0x4120         # Extract flow metadata
prompt_extract_steps: 0x4121            # Extract flow steps
prompt_extract_edges: 0x4122            # Extract flow edges

# Audit Query (0x4200-0x4220)
audit_query_trail: 0x4200               # Query audit trail
audit_get_entity_trail: 0x4201          # Get entity audit trail
audit_get_actor_trail: 0x4202           # Get actor audit trail
audit_get_compliance_events: 0x4203    # Get compliance events
audit_filter_by_time: 0x4210            # Filter events by time range
audit_filter_by_type: 0x4211            # Filter by event type
audit_paginate: 0x4212                  # Paginate results

# Token Management (0x4300-0x4350)
token_track_usage: 0x4300               # Track token usage
token_check_quota: 0x4301               # Check quota availability
token_get_summary: 0x4302               # Get usage summary
token_set_quota: 0x4310                 # Set quota limits
token_get_quota: 0x4311                 # Get quota settings
token_update_quota: 0x4312              # Update quota settings
token_calculate_cost: 0x4320            # Calculate operation cost
token_estimate_tokens: 0x4321           # Estimate tokens for text
token_get_usage_history: 0x4330         # Get usage history
token_get_cost_breakdown: 0x4331        # Get cost by provider/model
token_alert_threshold: 0x4340           # Check alert thresholds
token_enforce_quota: 0x4341             # Enforce quota limits

# Redaction Service (0x4400-0x4450)
redaction_detect_pii: 0x4400            # Detect PII in text
redaction_redact_text: 0x4401           # Redact PII from text
redaction_restore_text: 0x4402          # Restore redacted text
redaction_validate_compliance: 0x4410   # Validate compliance framework
redaction_resolve_overlaps: 0x4411      # Resolve PII region overlaps
redaction_mask: 0x4420                  # Apply masking strategy
redaction_hash: 0x4421                  # Apply hashing strategy
redaction_encrypt: 0x4422               # Apply encryption strategy
redaction_tokenize: 0x4423              # Apply tokenization strategy
redaction_partial: 0x4424               # Apply partial redaction
```

**Total Infrastructure Operations**: 53 codes (0x4000-0x4450)

---

**[Continued in Part 2 with complete workflows...]**
