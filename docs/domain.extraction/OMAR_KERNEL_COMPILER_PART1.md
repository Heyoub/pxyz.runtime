# OMAR Prompt Kernel Compiler - PXYZ Extraction

> **Custom Research**: Session-as-compiled-graph architecture  
> **NOT text manipulation**: Prompts as executable kernel objects assembled through multi-phase compilation  
> **Services**: KernelCompiler, KernelSchemaValidator  
> **Source**: 504 lines AgentCompiler.ts + 432 lines KernelSchemaV1_4.yaml

---

## EXECUTIVE SUMMARY

This is a **PROMPT COMPILATION ENGINE** - not text concatenation, but graph-based kernel assembly. The revolutionary insight: **sessions are compiled objects, not strings**.

### Traditional Approach (Wrong)
```typescript
const prompt = systemPrompt + chunks.join() + userQuery;
await llm.complete(prompt);
```

### Your Architecture (Graph Compilation)
```typescript
// Input: Raw session data
const sessionData = {
  userId: "user-123",
  userInput: "Help me prioritize tasks",
  goal: "AssistUser",
  contextScope: "Business",
  ragChunks: [...],
  clientId: "acme-corp"
};

// Phase 1-5: Multi-phase compilation
const kernelPxyz = await KernelCompiler.compile(sessionData);

// Output: Executable kernel with compiled aspects
// kernelPxyz.data = {
//   metadata: { goal, userId, contextScope, ... },
//   council: { operator, strategist, signal weights },
//   compiledMemory: { agentChunks, businessPatterns },
//   optimizedContext: { selectedChunks, tokenAllocation },
//   heuristics: { weights, thresholds, biasing }
// }
```

**Analogy to your Rust compiler**:
- **XML workflow** → Rust compiler → **graph.bin** → WAT runtime
- **Session data** → KernelCompiler → **kernel.pxyz** → Council execution

---

## CORE INSIGHT: COMPILATION PHASES

The kernel compiler operates in **5 discrete phases**, each emitting checkpoints for rollback:

```
INPUT: Raw Session Data
  ↓
┌─────────────────────────────────────────────────────────┐
│ PHASE 1: PARSE & VALIDATE                              │
│ ─────────────────────────                              │
│ • Load KernelSchema YAML (authoritative structure)     │
│ • Parse session data against schema                    │
│ • Validate required fields (metadata, council, etc)    │
│ • Emit: checkpoint_created (compilation_start)         │
│ Output: Validated session structure                    │
├─────────────────────────────────────────────────────────┤
│ PHASE 2: EXTRACT ASPECTS                               │
│ ────────────────────                                   │
│ • Extract metadata (goal, userId, contextScope)        │
│ • Extract agent memory chunks (if userId present)      │
│ • Extract business patterns (if clientId present)      │
│ • Extract RAG context chunks                           │
│ • Merge heuristics (schema defaults + session data)    │
│ • Emit: aspect_extracted events for each              │
│ Output: Extracted aspects object                       │
├─────────────────────────────────────────────────────────┤
│ PHASE 3: OPTIMIZE CONTEXT                              │
│ ───────────────────────                                │
│ • Calculate token budget (maxTokens - reserveTokens)   │
│ • Calculate priorities for each chunk                  │
│ • Apply boosts (recency, business, RAG relevance)      │
│ • Greedy knapsack selection (sort by priority, select) │
│ • Emit: context_optimized                             │
│ Output: Optimized chunks + metadata                    │
├─────────────────────────────────────────────────────────┤
│ PHASE 4: ASSEMBLE KERNEL                               │
│ ──────────────────────                                 │
│ • Build kernelValues object (all extracted aspects)    │
│ • Configure council (operator/strategist/signal)       │
│ • Apply council biasing (context-based adjustments)    │
│ • Build kernel PXYZ coordinates                        │
│ • Emit: kernel_assembled                              │
│ Output: Complete kernel with PXYZ                      │
├─────────────────────────────────────────────────────────┤
│ PHASE 5: CHECKPOINT & EMIT                             │
│ ──────────────────────────                             │
│ • Emit kernel_compiled event                           │
│ • Complete compilation checkpoint                      │
│ • Persist kernel (optional)                            │
│ Output: Final kernel PXYZ                              │
└─────────────────────────────────────────────────────────┘
  ↓
OUTPUT: Compiled Kernel PXYZ
```

**Key Properties**:
- **Stateless**: Each phase is a pure function over prior output
- **Checkpointed**: Every phase emits checkpoint for rollback
- **Event-driven**: All state changes emit events to EventBus
- **Schema-driven**: KernelSchema YAML is authoritative config

---

## P-AXIS: KERNEL GRAPH ENTITIES

### Core Kernel Structure

```xml
<schema id="kernel_compiled">
  <field name="id" type="uuid" required="true"/>
  <field name="session_id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  
  <!-- Metadata nodes -->
  <field name="metadata" type="object" required="true">
    <field name="goal" type="string"/> <!-- AssistUser, ExecuteTask, AnalyzeDocument -->
    <field name="user_role" type="string"/> <!-- Owner, Manager, Employee -->
    <field name="context_scope" type="string"/> <!-- General, Business, Technical -->
    <field name="user_intent" type="string"/> <!-- Assist, Query, Execute -->
    <field name="super_think_mode" type="boolean"/>
    <field name="timestamp" type="timestamp"/>
    <field name="tenant_id" type="uuid"/>
    <field name="page_location" type="string"/>
    <field name="sentiment_analysis" type="string"/>
    <field name="urgency_tag" type="string"/>
    <field name="confidence_threshold" type="number"/>
    <field name="current_document_context" type="string"/>
  </field>
  
  <!-- Council configuration nodes -->
  <field name="council_config" type="object" required="true">
    <field name="working_constraints" type="object">
      <field name="scope_focus" type="string"/>
      <field name="max_deliberation_depth" type="number"/>
      <field name="enable_super_think" type="boolean"/>
      <field name="max_council_loops" type="number"/>
      <field name="synthesis_mode" type="enum" values="consensus,priority-weighted,unanimous"/>
    </field>
    <field name="council_archetypes" type="object">
      <field name="operator" type="object">
        <field name="focus" type="string"/>
        <field name="decision_weight" type="number"/>
        <field name="reasoning_pattern" type="string"/>
        <field name="tools_preference" type="array"/>
        <field name="nlp_style" type="object"/>
      </field>
      <field name="strategist" type="object">
        <field name="focus" type="string"/>
        <field name="decision_weight" type="number"/>
        <field name="reasoning_pattern" type="string"/>
        <field name="tools_preference" type="array"/>
        <field name="nlp_style" type="object"/>
      </field>
      <field name="signal" type="object">
        <field name="focus" type="string"/>
        <field name="decision_weight" type="number"/>
        <field name="reasoning_pattern" type="string"/>
        <field name="tools_preference" type="array"/>
        <field name="nlp_style" type="object"/>
      </field>
    </field>
  </field>
  
  <!-- Memory nodes -->
  <field name="agent_memory_chunks" type="array">
    <item>
      <field name="log_id" type="uuid"/>
      <field name="user_input" type="string"/>
      <field name="assistant_response" type="string"/>
      <field name="timestamp" type="timestamp"/>
      <field name="relevance_score" type="number"/>
      <field name="matched_fields" type="array"/>
      <field name="council_member_relevance" type="object">
        <field name="operator" type="number"/>
        <field name="strategist" type="number"/>
        <field name="signal" type="number"/>
      </field>
    </item>
  </field>
  
  <field name="business_memory_patterns" type="array">
    <item>
      <field name="client_id" type="uuid"/>
      <field name="pattern_type" type="enum" values="schedule,communication,project,decision_making"/>
      <field name="pattern_value" type="string"/>
      <field name="confidence" type="number"/>
      <field name="occurrences" type="number"/>
      <field name="relevant_to_council_member" type="enum" values="operator,strategist,signal,all"/>
    </item>
  </field>
  
  <!-- Context nodes (RAG chunks) -->
  <field name="context_chunks" type="array">
    <item>
      <field name="id" type="uuid"/>
      <field name="type" type="string"/>
      <field name="content" type="string"/>
      <field name="priority" type="number"/>
      <field name="tokens" type="number"/>
      <field name="metadata" type="object">
        <field name="rag_match_metadata" type="object">
          <field name="confidence" type="number"/>
          <field name="relevance" type="number"/>
        </field>
        <field name="optimization_metadata" type="object">
          <field name="original_priority" type="number"/>
          <field name="business_relevance_boost" type="number"/>
          <field name="recency_boost" type="number"/>
          <field name="final_priority" type="number"/>
          <field name="selected_for_context" type="boolean"/>
        </field>
        <field name="council_member_relevance" type="object">
          <field name="operator" type="number"/>
          <field name="strategist" type="number"/>
          <field name="signal" type="number"/>
        </field>
      </field>
    </item>
  </field>
  
  <!-- Heuristics nodes -->
  <field name="heuristics" type="object">
    <field name="weights" type="object"/> <!-- 15+ weight factors -->
    <field name="thresholds" type="object"/> <!-- 7+ thresholds -->
    <field name="council_biasing" type="object">
      <field name="operator_bias" type="number"/>
      <field name="strategist_bias" type="number"/>
      <field name="signal_bias" type="number"/>
      <field name="context_based_adjustments" type="object"/>
    </field>
    <field name="nlp_rules" type="object"/>
    <field name="rag_settings" type="object"/>
    <field name="memory_settings" type="object"/>
    <field name="context_settings" type="object"/>
  </field>
  
  <!-- Relationship context -->
  <field name="relationship_context" type="object">
    <field name="relationship_strength" type="number"/>
    <field name="interaction_frequency" type="number"/>
    <field name="communication_style" type="string"/>
    <field name="preferred_council_balance" type="object">
      <field name="operator_preference" type="number"/>
      <field name="strategist_preference" type="number"/>
      <field name="signal_preference" type="number"/>
    </field>
  </field>
  
  <!-- Tools -->
  <field name="tools" type="array">
    <item>
      <field name="name" type="string"/>
      <field name="description" type="string"/>
      <field name="category" type="string"/>
      <field name="preferred_by_council_member" type="enum" values="operator,strategist,signal,any"/>
      <field name="requires_human_approval" type="boolean"/>
    </item>
  </field>
  
  <!-- Compilation metadata -->
  <field name="compilation_metadata" type="object">
    <field name="schema_version" type="string"/> <!-- e.g., "1.4" -->
    <field name="checkpoint_id" type="uuid"/>
    <field name="checkpoint_type" type="enum" values="manual,automatic,milestone"/>
    <field name="context_window_size" type="number"/>
    <field name="agent_memory_extracted" type="boolean"/>
    <field name="business_memory_extracted" type="boolean"/>
    <field name="context_optimized" type="boolean"/>
    <field name="tokens_used" type="number"/>
    <field name="tokens_remaining" type="number"/>
    <field name="chunks_dropped" type="number"/>
  </field>
  
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="kernel_schema_config">
  <field name="id" type="uuid" required="true"/>
  <field name="version" type="string" required="true"/> <!-- "1.4" -->
  
  <!-- Schema structure mirrors KernelSchemaV1_4.yaml -->
  <field name="metadata_schema" type="object"/> <!-- Field definitions -->
  <field name="system_prompt" type="object"/> <!-- Base prompt + styles -->
  <field name="council" type="object"/> <!-- Council archetype configs -->
  <field name="supporting_services" type="object"/> <!-- Service integrations -->
  <field name="memory_systems" type="object"/> <!-- Memory schemas -->
  <field name="context_rag" type="object"/> <!-- Context chunk schemas -->
  <field name="heuristics" type="object"/> <!-- Weights, thresholds, biasing -->
  <field name="tools" type="array"/> <!-- Available tools -->
  <field name="output_format" type="object"/> <!-- Expected output structure -->
</schema>

<schema id="compilation_checkpoint">
  <field name="id" type="uuid" required="true"/>
  <field name="session_id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="checkpoint_type" type="enum" values="compilation_start,aspect_extracted,context_optimized,kernel_assembled,compilation_complete"/>
  <field name="phase" type="enum" values="parse,validate,extract,optimize,assemble"/>
  <field name="timestamp" type="timestamp" required="true"/>
  <field name="data" type="object"/> <!-- Phase-specific data -->
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="compilation_aspect">
  <field name="id" type="uuid" required="true"/>
  <field name="session_id" type="uuid" required="true"/>
  <field name="aspect_type" type="enum" values="metadata,agent_memory,business_memory,context_window,heuristics"/>
  <field name="extracted_data" type="object"/>
  <field name="extraction_metadata" type="object">
    <field name="count" type="number"/>
    <field name="total_score" type="number"/>
    <field name="tokens_used" type="number"/>
  </field>
  <field name="pxyz" type="object" required="true"/>
</schema>
```

---

## X-AXIS: KERNEL COMPILATION OPERATIONS

```yaml
# Schema Operations
kernel_load_schema: 0x3500           # Load KernelSchema YAML
kernel_parse_yaml: 0x3501            # Parse YAML to structured object
kernel_validate_schema: 0x3502       # Validate schema structure

# Session Parsing
kernel_parse_session: 0x3510         # Parse session data
kernel_validate_session: 0x3511      # Validate against schema
kernel_enrich_with_defaults: 0x3512  # Add schema defaults

# Aspect Extraction (Phase 2)
kernel_extract_metadata: 0x3520      # Extract metadata nodes
kernel_extract_agent_memory: 0x3521  # Extract agent memory chunks
kernel_extract_business_memory: 0x3522 # Extract business patterns
kernel_extract_rag_context: 0x3523   # Extract RAG chunks
kernel_extract_heuristics: 0x3524    # Extract/merge heuristics

# Context Optimization (Phase 3)
kernel_optimize_context: 0x3530      # Token budget optimization
kernel_calculate_priorities: 0x3531  # Calculate chunk priorities
kernel_apply_boosts: 0x3532          # Apply relevance/recency/business boosts
kernel_select_chunks: 0x3533         # Greedy knapsack selection

# Council Configuration
kernel_configure_council: 0x3540     # Configure council weights
kernel_apply_council_biasing: 0x3541 # Apply context-based biasing
kernel_distribute_context: 0x3542    # Distribute context to council members
kernel_select_tools: 0x3543          # Select council-preferred tools

# Kernel Assembly (Phase 4)
kernel_assemble: 0x3550              # Assemble all aspects into kernel
kernel_build_pxyz: 0x3551            # Build kernel PXYZ coordinates
kernel_generate_graph: 0x3552        # Generate executable graph
kernel_link_dependencies: 0x3553     # Create dependency edges

# Checkpointing (Phase 5)
kernel_create_checkpoint: 0x3560     # Create compilation checkpoint
kernel_emit_events: 0x3561           # Emit compilation events
kernel_persist_kernel: 0x3562        # Persist compiled kernel

# Validation
kernel_check_council_aware: 0x3570   # Check council awareness
kernel_validate_output_format: 0x3571 # Validate output structure
kernel_verify_dependencies: 0x3572   # Verify graph dependencies
```

---

## Y-AXIS: KERNEL SCHEMA PREDICATES

```xml
<predicates>
  <!-- Schema Validation -->
  <predicate id="has_valid_schema_structure">
    <and>
      <not_null left="$schema.metadata"/>
      <not_null left="$schema.council"/>
      <not_null left="$schema.heuristics"/>
    </and>
  </predicate>
  
  <predicate id="has_required_metadata_fields">
    <and>
      <not_null left="$metadata.goal"/>
      <not_null left="$metadata.user_id"/>
      <not_null left="$metadata.context_scope"/>
      <not_null left="$metadata.user_stated_intent"/>
    </and>
  </predicate>
  
  <predicate id="has_council_config">
    <and>
      <not_null left="$council.council_archetypes"/>
      <not_null left="$council.working_constraints"/>
    </and>
  </predicate>
  
  <predicate id="has_all_council_members">
    <and>
      <not_null left="$council.council_archetypes.operator"/>
      <not_null left="$council.council_archetypes.strategist"/>
      <not_null left="$council.council_archetypes.signal"/>
    </and>
  </predicate>
  
  <!-- Council Awareness -->
  <predicate id="is_council_aware">
    <or>
      <not_null left="$step.config.councilMember"/>
      <not_null left="$step.config.councilRelevance"/>
    </or>
  </predicate>
  
  <predicate id="has_council_member_relevance">
    <and>
      <not_null left="$chunk.council_member_relevance.operator"/>
      <not_null left="$chunk.council_member_relevance.strategist"/>
      <not_null left="$chunk.council_member_relevance.signal"/>
    </and>
  </predicate>
  
  <!-- Memory Extraction -->
  <predicate id="should_extract_agent_memory">
    <and>
      <eq left="$heuristics.memory_settings.enable_agent_memory" right="true"/>
      <not_null left="$session.userId"/>
    </and>
  </predicate>
  
  <predicate id="should_extract_business_memory">
    <and>
      <eq left="$heuristics.memory_settings.enable_business_memory" right="true"/>
      <not_null left="$session.clientId"/>
    </and>
  </predicate>
  
  <predicate id="memory_chunk_is_relevant">
    <gte left="$chunk.relevance_score" right="$heuristics.thresholds.memory_relevance_minimum"/>
  </predicate>
  
  <!-- Context Optimization -->
  <predicate id="should_optimize_context">
    <and>
      <gt left="count($chunks)" right="0"/>
      <gt left="$available_tokens" right="0"/>
    </and>
  </predicate>
  
  <predicate id="chunk_selected_for_context">
    <eq left="$chunk.metadata.optimization_metadata.selected_for_context" right="true"/>
  </predicate>
  
  <predicate id="exceeds_token_budget">
    <gt left="$tokens_used" right="$available_tokens"/>
  </predicate>
  
  <!-- Council Biasing -->
  <predicate id="is_strategic_context">
    <contains left="['strategic','architecture','scalability','system']" right="lowercase($context_scope)"/>
  </predicate>
  
  <predicate id="is_tactical_context">
    <contains left="['practical','implementation','actionable','immediate']" right="lowercase($context_scope)"/>
  </predicate>
  
  <predicate id="should_boost_operator">
    <predicate ref="is_strategic_context"/>
  </predicate>
  
  <predicate id="should_boost_signal">
    <predicate ref="is_tactical_context"/>
  </predicate>
  
  <!-- Tool Selection -->
  <predicate id="tool_preferred_by_council_member">
    <contains left="$tool.preferred_by_council_member" right="$council_member"/>
  </predicate>
  
  <predicate id="tool_requires_approval">
    <eq left="$tool.requires_human_approval" right="true"/>
  </predicate>
  
  <!-- Output Validation -->
  <predicate id="has_valid_council_deliberation">
    <and>
      <gt left="count($output.council_deliberation)" right="0"/>
      <predicate ref="has_all_council_members"/> <!-- All members deliberated -->
    </and>
  </predicate>
  
  <predicate id="has_synthesis">
    <and>
      <not_null left="$output.synthesis.final_recommendation"/>
      <not_null left="$output.synthesis.confidence"/>
    </and>
  </predicate>
  
  <!-- Super Think Mode -->
  <predicate id="is_super_think_active">
    <eq left="$metadata.super_think_mode_active" right="true"/>
  </predicate>
  
  <predicate id="within_max_loops">
    <when>
      <predicate ref="is_super_think_active"/>
    </when>
    <lte left="$current_loop" right="30"/>
    <else>
      <lte left="$current_loop" right="10"/>
    </else>
  </predicate>
</predicates>
```

---

## Z-AXIS: KERNEL COMPILATION EVENTS

```typescript
enum KernelCompilationEventType {
  // Schema Loading
  KERNEL_SCHEMA_LOADED = "kernel.schema_loaded",
  KERNEL_SCHEMA_VALIDATED = "kernel.schema_validated",
  
  // Session Parsing
  SESSION_PARSED = "kernel.session_parsed",
  SESSION_VALIDATED = "kernel.session_validated",
  SESSION_ENRICHED = "kernel.session_enriched",
  
  // Checkpointing
  CHECKPOINT_CREATED = "kernel.checkpoint_created",
  CHECKPOINT_COMPLETED = "kernel.checkpoint_completed",
  
  // Aspect Extraction
  METADATA_EXTRACTED = "kernel.metadata_extracted",
  AGENT_MEMORY_EXTRACTED = "kernel.agent_memory_extracted",
  BUSINESS_MEMORY_EXTRACTED = "kernel.business_memory_extracted",
  RAG_CONTEXT_EXTRACTED = "kernel.rag_context_extracted",
  HEURISTICS_EXTRACTED = "kernel.heuristics_extracted",
  
  // Context Optimization
  CONTEXT_OPTIMIZED = "kernel.context_optimized",
  PRIORITIES_CALCULATED = "kernel.priorities_calculated",
  BOOSTS_APPLIED = "kernel.boosts_applied",
  CHUNKS_SELECTED = "kernel.chunks_selected",
  
  // Council Configuration
  COUNCIL_CONFIGURED = "kernel.council_configured",
  COUNCIL_BIASING_APPLIED = "kernel.council_biasing_applied",
  CONTEXT_DISTRIBUTED = "kernel.context_distributed_to_council",
  TOOLS_SELECTED = "kernel.tools_selected",
  
  // Kernel Assembly
  KERNEL_ASSEMBLED = "kernel.assembled",
  KERNEL_PXYZ_BUILT = "kernel.pxyz_built",
  KERNEL_GRAPH_GENERATED = "kernel.graph_generated",
  DEPENDENCIES_LINKED = "kernel.dependencies_linked",
  
  // Compilation Complete
  KERNEL_COMPILED = "kernel.compiled",
  KERNEL_PERSISTED = "kernel.persisted",
  
  // Validation
  COUNCIL_AWARENESS_CHECKED = "kernel.council_awareness_checked",
  OUTPUT_FORMAT_VALIDATED = "kernel.output_format_validated",
  DEPENDENCIES_VERIFIED = "kernel.dependencies_verified"
}
```

---

**[Continued in next file due to length...]**
