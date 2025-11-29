# OMAR Prompt Kernel Compiler - Part 2: Workflows

---

## COMPLETE COMPILATION WORKFLOWS

### Workflow 1: Full Kernel Compilation (Main Flow)

**Entry**: `<entry p="kernel_compiler" x="compile" node="start"/>`

```xml
<workflow id="kernel_compile_full">
  <entry p="kernel_compiler" x="compile" node="start"/>
  
  <nodes>
    <!-- Phase 1: Parse & Validate -->
    <node id="start" kind="transform">
      <description>Initialize compilation</description>
      <emit>checkpoint_created</emit>
    </node>
    
    <node id="load_schema" kind="external" op="0x3500">
      <description>Load KernelSchemaV1_4.yaml</description>
      <emit>kernel_schema_loaded</emit>
    </node>
    
    <node id="parse_yaml" kind="external" op="0x3501">
      <description>Parse YAML to structured object</description>
      <emit>kernel_schema_validated</emit>
    </node>
    
    <node id="validate_schema" kind="transform" op="0x3502">
      <description>Validate schema structure</description>
      <schema ref="kernel_schema_validation"/>
      <emit>session_parsed</emit>
    </node>
    
    <node id="validate_session" kind="auth" op="0x3511">
      <require predicate="has_required_metadata_fields"/>
      <require predicate="has_valid_schema_structure"/>
      <emit>session_validated</emit>
    </node>
    
    <node id="enrich_defaults" kind="transform" op="0x3512">
      <description>Add schema defaults to session</description>
      <emit>session_enriched</emit>
    </node>
    
    <!-- Phase 2: Extract Aspects -->
    <node id="extract_metadata" kind="transform" op="0x3520">
      <description>Extract metadata nodes (goal, user, context)</description>
      <emit>metadata_extracted</emit>
    </node>
    
    <node id="check_agent_memory" kind="auth">
      <require predicate="should_extract_agent_memory"/>
    </node>
    
    <node id="extract_agent_memory" kind="external" op="0x3521">
      <description>Extract agent memory chunks via AgentMemoryService</description>
      <emit>agent_memory_extracted</emit>
    </node>
    
    <node id="check_business_memory" kind="auth">
      <require predicate="should_extract_business_memory"/>
    </node>
    
    <node id="extract_business_memory" kind="external" op="0x3522">
      <description>Extract business patterns via BusinessMemoryService</description>
      <emit>business_memory_extracted</emit>
    </node>
    
    <node id="extract_rag_context" kind="external" op="0x3523">
      <description>Extract RAG chunks from retrieval</description>
      <emit>rag_context_extracted</emit>
    </node>
    
    <node id="extract_heuristics" kind="transform" op="0x3524">
      <description>Merge schema + session heuristics</description>
      <emit>heuristics_extracted</emit>
    </node>
    
    <!-- Phase 3: Optimize Context -->
    <node id="check_optimization_needed" kind="auth">
      <require predicate="should_optimize_context"/>
    </node>
    
    <node id="calculate_priorities" kind="transform" op="0x3531">
      <description>Calculate base priorities for all chunks</description>
      <formula>
        base_priority = priorityThresholds[chunk.type]
      </formula>
    </node>
    
    <node id="apply_boosts" kind="transform" op="0x3532">
      <description>Apply recency, business, RAG boosts</description>
      <formula>
        recency_boost = max(0, maxBoost - days_old * decayRate)
        business_boost = min(revenue_boost + tier_boost + deal_boost, 20)
        rag_boost = rag_relevance * 20
        final_priority = base_priority + recency_boost + business_boost + rag_boost
      </formula>
      <emit>boosts_applied</emit>
    </node>
    
    <node id="optimize_context" kind="external" op="0x3530">
      <description>Token budget optimization (knapsack)</description>
      <emit>context_optimized</emit>
    </node>
    
    <!-- Phase 4: Configure Council -->
    <node id="configure_council" kind="transform" op="0x3540">
      <description>Set council weights from schema</description>
    </node>
    
    <node id="check_strategic_context" kind="auth">
      <require predicate="is_strategic_context"/>
    </node>
    
    <node id="boost_operator" kind="transform" op="0x3541">
      <description>Apply strategic context operator boost</description>
      <formula>
        operator_bias = operator_bias + context_based_adjustments.strategic_context_operator_boost
      </formula>
    </node>
    
    <node id="check_tactical_context" kind="auth">
      <require predicate="is_tactical_context"/>
    </node>
    
    <node id="boost_signal" kind="transform" op="0x3541">
      <description>Apply tactical context signal boost</description>
      <formula>
        signal_bias = signal_bias + context_based_adjustments.tactical_context_signal_boost
      </formula>
      <emit>council_biasing_applied</emit>
    </node>
    
    <node id="distribute_context" kind="transform" op="0x3542">
      <description>Distribute context chunks to council members by relevance</description>
      <formula>
        operator_chunks = filter(chunks, chunk.council_member_relevance.operator >= threshold)
        strategist_chunks = filter(chunks, chunk.council_member_relevance.strategist >= threshold)
        signal_chunks = filter(chunks, chunk.council_member_relevance.signal >= threshold)
      </formula>
      <emit>context_distributed</emit>
    </node>
    
    <node id="select_tools" kind="transform" op="0x3543">
      <description>Select tools preferred by council members</description>
      <emit>tools_selected</emit>
    </node>
    
    <!-- Phase 5: Assemble Kernel -->
    <node id="assemble_kernel" kind="transform" op="0x3550">
      <description>Combine all aspects into kernel object</description>
      <emit>kernel_assembled</emit>
    </node>
    
    <node id="build_pxyz" kind="transform" op="0x3551">
      <description>Build kernel PXYZ coordinates</description>
      <formula>
        P = kernel.goal (e.g., "AssistUser")
        X = "DELIBERATE"
        Y = kernel.context_scope (e.g., "Business")
        Z = timestamp
      </formula>
      <emit>kernel_pxyz_built</emit>
    </node>
    
    <node id="generate_graph" kind="transform" op="0x3552">
      <description>Generate executable graph structure</description>
      <emit>kernel_graph_generated</emit>
    </node>
    
    <node id="link_dependencies" kind="transform" op="0x3553">
      <description>Create dependency edges between nodes</description>
      <emit>dependencies_linked</emit>
    </node>
    
    <!-- Phase 6: Validate & Emit -->
    <node id="validate_council_awareness" kind="auth" op="0x3570">
      <require predicate="has_valid_council_deliberation"/>
    </node>
    
    <node id="validate_output_format" kind="auth" op="0x3571">
      <require predicate="has_synthesis"/>
    </node>
    
    <node id="emit_compiled" kind="transform" op="0x3561">
      <description>Emit kernel_compiled event</description>
      <emit>kernel_compiled</emit>
    </node>
    
    <node id="persist_kernel" kind="external" op="0x3562">
      <description>Persist compiled kernel to database</description>
      <emit>kernel_persisted</emit>
    </node>
    
    <node id="complete_checkpoint" kind="transform">
      <description>Mark compilation checkpoint complete</description>
      <emit>checkpoint_completed</emit>
    </node>
    
    <node id="success" kind="terminal"/>
  </nodes>
  
  <edges>
    <!-- Phase 1 -->
    <edge from="start" to="load_schema"><when><always/></when></edge>
    <edge from="load_schema" to="parse_yaml"><when><always/></when></edge>
    <edge from="parse_yaml" to="validate_schema"><when><always/></when></edge>
    <edge from="validate_schema" to="validate_session"><when><always/></when></edge>
    <edge from="validate_session" to="enrich_defaults"><when><always/></when></edge>
    
    <!-- Phase 2 -->
    <edge from="enrich_defaults" to="extract_metadata"><when><always/></when></edge>
    <edge from="extract_metadata" to="check_agent_memory"><when><always/></when></edge>
    <edge from="check_agent_memory" to="extract_agent_memory">
      <when><predicate ref="should_extract_agent_memory"/></when>
    </edge>
    <edge from="check_agent_memory" to="check_business_memory">
      <when><not><predicate ref="should_extract_agent_memory"/></not></when>
    </edge>
    <edge from="extract_agent_memory" to="check_business_memory"><when><always/></when></edge>
    
    <edge from="check_business_memory" to="extract_business_memory">
      <when><predicate ref="should_extract_business_memory"/></when>
    </edge>
    <edge from="check_business_memory" to="extract_rag_context">
      <when><not><predicate ref="should_extract_business_memory"/></not></when>
    </edge>
    <edge from="extract_business_memory" to="extract_rag_context"><when><always/></when></edge>
    
    <edge from="extract_rag_context" to="extract_heuristics"><when><always/></when></edge>
    
    <!-- Phase 3 -->
    <edge from="extract_heuristics" to="check_optimization_needed"><when><always/></when></edge>
    <edge from="check_optimization_needed" to="calculate_priorities">
      <when><predicate ref="should_optimize_context"/></when>
    </edge>
    <edge from="check_optimization_needed" to="configure_council">
      <when><not><predicate ref="should_optimize_context"/></not></when>
    </edge>
    
    <edge from="calculate_priorities" to="apply_boosts"><when><always/></when></edge>
    <edge from="apply_boosts" to="optimize_context"><when><always/></when></edge>
    <edge from="optimize_context" to="configure_council"><when><always/></when></edge>
    
    <!-- Phase 4 -->
    <edge from="configure_council" to="check_strategic_context"><when><always/></when></edge>
    <edge from="check_strategic_context" to="boost_operator">
      <when><predicate ref="is_strategic_context"/></when>
    </edge>
    <edge from="check_strategic_context" to="check_tactical_context">
      <when><not><predicate ref="is_strategic_context"/></not></when>
    </edge>
    <edge from="boost_operator" to="check_tactical_context"><when><always/></when></edge>
    
    <edge from="check_tactical_context" to="boost_signal">
      <when><predicate ref="is_tactical_context"/></when>
    </edge>
    <edge from="check_tactical_context" to="distribute_context">
      <when><not><predicate ref="is_tactical_context"/></not></when>
    </edge>
    <edge from="boost_signal" to="distribute_context"><when><always/></when></edge>
    
    <edge from="distribute_context" to="select_tools"><when><always/></when></edge>
    
    <!-- Phase 5 -->
    <edge from="select_tools" to="assemble_kernel"><when><always/></when></edge>
    <edge from="assemble_kernel" to="build_pxyz"><when><always/></when></edge>
    <edge from="build_pxyz" to="generate_graph"><when><always/></when></edge>
    <edge from="generate_graph" to="link_dependencies"><when><always/></when></edge>
    
    <!-- Phase 6 -->
    <edge from="link_dependencies" to="validate_council_awareness"><when><always/></when></edge>
    <edge from="validate_council_awareness" to="validate_output_format"><when><always/></when></edge>
    <edge from="validate_output_format" to="emit_compiled"><when><always/></when></edge>
    <edge from="emit_compiled" to="persist_kernel"><when><always/></when></edge>
    <edge from="persist_kernel" to="complete_checkpoint"><when><always/></when></edge>
    <edge from="complete_checkpoint" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

**Node Count**: 36 nodes  
**Edge Count**: 38 edges  
**Phases**: 6 (parse, validate, extract, optimize, configure, assemble)

---

### Workflow 2: Schema Validation

**Entry**: `<entry p="kernel_schema" x="validate" node="load"/>`

```xml
<workflow id="kernel_schema_validate">
  <entry p="kernel_schema" x="validate" node="load"/>
  
  <nodes>
    <node id="load" kind="external" op="0x3500">
      <description>Load KernelSchemaV1_4.yaml</description>
    </node>
    
    <node id="parse" kind="external" op="0x3501">
      <description>Parse YAML to object</description>
    </node>
    
    <node id="check_version" kind="auth">
      <require predicate="has_version_field"/>
    </node>
    
    <node id="validate_council" kind="auth">
      <require predicate="has_council_config"/>
      <require predicate="has_all_council_members"/>
    </node>
    
    <node id="validate_services" kind="auth">
      <require predicate="has_supporting_services"/>
    </node>
    
    <node id="validate_heuristics" kind="auth">
      <require predicate="has_heuristics_section"/>
    </node>
    
    <node id="validate_flow" kind="auth" op="0x3502">
      <description>Validate flow structure</description>
      <require predicate="has_valid_schema_structure"/>
    </node>
    
    <node id="check_council_awareness" kind="auth" op="0x3570">
      <description>Check for council-aware steps</description>
      <require predicate="is_council_aware"/>
    </node>
    
    <node id="enrich_flow" kind="transform" op="0x3512">
      <description>Add schema defaults to flow</description>
    </node>
    
    <node id="success" kind="terminal"/>
    <node id="warning" kind="terminal"/> <!-- For missing optional fields -->
  </nodes>
  
  <edges>
    <edge from="load" to="parse"><when><always/></when></edge>
    <edge from="parse" to="check_version"><when><always/></when></edge>
    <edge from="check_version" to="validate_council"><when><always/></when></edge>
    <edge from="validate_council" to="validate_services"><when><always/></when></edge>
    <edge from="validate_services" to="validate_heuristics"><when><always/></when></edge>
    <edge from="validate_heuristics" to="validate_flow"><when><always/></when></edge>
    <edge from="validate_flow" to="check_council_awareness"><when><always/></when></edge>
    
    <edge from="check_council_awareness" to="enrich_flow">
      <when><predicate ref="is_council_aware"/></when>
    </edge>
    <edge from="check_council_awareness" to="warning">
      <when><not><predicate ref="is_council_aware"/></not></when>
    </edge>
    
    <edge from="enrich_flow" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

**Node Count**: 11 nodes  
**Validates**: Schema structure, council config, council awareness

---

### Workflow 3: Council Context Distribution

**Entry**: `<entry p="council" x="distribute_context" node="start"/>`

```xml
<workflow id="council_distribute_context">
  <entry p="council" x="distribute_context" node="start"/>
  
  <nodes>
    <node id="start" kind="transform">
      <description>Load context chunks</description>
    </node>
    
    <node id="calculate_operator_relevance" kind="transform">
      <description>Calculate operator relevance scores</description>
      <formula>
        operator_score = 
          strategic_keywords * 0.4 +
          future_oriented_content * 0.3 +
          ecosystem_thinking * 0.3
      </formula>
    </node>
    
    <node id="calculate_strategist_relevance" kind="transform">
      <description>Calculate strategist relevance scores</description>
      <formula>
        strategist_score = 
          structural_content * 0.4 +
          principle_based_content * 0.3 +
          scalability_focus * 0.3
      </formula>
    </node>
    
    <node id="calculate_signal_relevance" kind="transform">
      <description>Calculate signal relevance scores</description>
      <formula>
        signal_score = 
          actionable_content * 0.4 +
          practical_examples * 0.3 +
          implementation_focus * 0.3
      </formula>
    </node>
    
    <node id="apply_threshold" kind="transform">
      <description>Filter chunks by relevance threshold</description>
      <formula>
        threshold = heuristics.thresholds.council_member_relevance_minimum
      </formula>
    </node>
    
    <node id="distribute_by_ratio" kind="transform">
      <description>Distribute chunks according to council_context_distribution</description>
      <formula>
        operator_tokens = total_tokens * context_settings.council_context_distribution.operator_context_ratio
        strategist_tokens = total_tokens * context_settings.council_context_distribution.strategist_context_ratio
        signal_tokens = total_tokens * context_settings.council_context_distribution.signal_context_ratio
      </formula>
    </node>
    
    <node id="select_operator_chunks" kind="transform">
      <description>Select top chunks for operator within token budget</description>
      <formula>
        sort by operator_score desc
        greedy selection while tokens < operator_tokens
      </formula>
    </node>
    
    <node id="select_strategist_chunks" kind="transform">
      <description>Select top chunks for strategist within token budget</description>
      <formula>
        sort by strategist_score desc
        greedy selection while tokens < strategist_tokens
      </formula>
    </node>
    
    <node id="select_signal_chunks" kind="transform">
      <description>Select top chunks for signal within token budget</description>
      <formula>
        sort by signal_score desc
        greedy selection while tokens < signal_tokens
      </formula>
    </node>
    
    <node id="emit_distribution" kind="transform">
      <emit>context_distributed</emit>
    </node>
    
    <node id="success" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="start" to="calculate_operator_relevance"><when><always/></when></edge>
    <edge from="calculate_operator_relevance" to="calculate_strategist_relevance"><when><always/></when></edge>
    <edge from="calculate_strategist_relevance" to="calculate_signal_relevance"><when><always/></when></edge>
    <edge from="calculate_signal_relevance" to="apply_threshold"><when><always/></when></edge>
    <edge from="apply_threshold" to="distribute_by_ratio"><when><always/></when></edge>
    <edge from="distribute_by_ratio" to="select_operator_chunks"><when><always/></when></edge>
    <edge from="select_operator_chunks" to="select_strategist_chunks"><when><always/></when></edge>
    <edge from="select_strategist_chunks" to="select_signal_chunks"><when><always/></when></edge>
    <edge from="select_signal_chunks" to="emit_distribution"><when><always/></when></edge>
    <edge from="emit_distribution" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

**Node Count**: 11 nodes  
**Innovation**: Distributes context chunks to council members based on relevance scores and token ratios

---

### Workflow 4: Tool Selection by Council Preference

**Entry**: `<entry p="council" x="select_tools" node="start"/>`

```xml
<workflow id="council_select_tools">
  <entry p="council" x="select_tools" node="start"/>
  
  <nodes>
    <node id="start" kind="transform">
      <description>Load available tools</description>
    </node>
    
    <node id="filter_operator_tools" kind="transform">
      <description>Filter tools preferred by operator</description>
      <formula>
        filter(tools, tool.preferred_by_council_member IN ['operator', 'any'])
      </formula>
    </node>
    
    <node id="filter_strategist_tools" kind="transform">
      <description>Filter tools preferred by strategist</description>
      <formula>
        filter(tools, tool.preferred_by_council_member IN ['strategist', 'any'])
      </formula>
    </node>
    
    <node id="filter_signal_tools" kind="transform">
      <description>Filter tools preferred by signal</description>
      <formula>
        filter(tools, tool.preferred_by_council_member IN ['signal', 'any'])
      </formula>
    </node>
    
    <node id="check_approval_required" kind="auth">
      <require predicate="tool_requires_approval"/>
    </node>
    
    <node id="mark_approval_needed" kind="transform">
      <description>Mark tools requiring human approval</description>
    </node>
    
    <node id="assemble_toolset" kind="transform">
      <description>Combine all selected tools</description>
    </node>
    
    <node id="emit_selection" kind="transform">
      <emit>tools_selected</emit>
    </node>
    
    <node id="success" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="start" to="filter_operator_tools"><when><always/></when></edge>
    <edge from="filter_operator_tools" to="filter_strategist_tools"><when><always/></when></edge>
    <filter_strategist_tools" to="filter_signal_tools"><when><always/></when></edge>
    <edge from="filter_signal_tools" to="check_approval_required"><when><always/></when></edge>
    
    <edge from="check_approval_required" to="mark_approval_needed">
      <when><predicate ref="tool_requires_approval"/></when>
    </edge>
    <edge from="check_approval_required" to="assemble_toolset">
      <when><not><predicate ref="tool_requires_approval"/></not></when>
    </edge>
    
    <edge from="mark_approval_needed" to="assemble_toolset"><when><always/></when></edge>
    <edge from="assemble_toolset" to="emit_selection"><when><always/></when></edge>
    <edge from="emit_selection" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

**Node Count**: 9 nodes  
**Innovation**: Tool selection based on council member preferences

---

## FORMULA REFERENCE

### Context Priority Calculation

```javascript
// Base priority from chunk type
base_priority = priorityThresholds[chunk.type];
// system_prompt: 200
// current_user_prompt: 190
// critical_rag_document: 180
// tool_output: 170
// tool_input: 160
// chat_history_user: 150
// chat_history_assistant: 140
// rag_memory_commit: 130
// rag_document_chunk: 120
// rag_ebook_chunk: 110
// rag_generic_chunk: 100

// Recency boost (exponential decay)
age_days = (now - chunk.timestamp) / 86400000;
recency_boost = max(0, maxBoost - age_days * decayRate);
// maxBoost = 15
// decayRate = 0.05 (general), 0.02 (business content)

// Business relevance boost
revenue_boost = min(revenue_impact / 10000, 20);
urgency_boost = {critical: 25, high: 15, medium: 5, low: 0}[urgency];
tier_boost = {enterprise: 12, midMarket: 8, smallBusiness: 4, startup: 2}[tier];
deal_boost = {negotiation: 18, proposal: 12, qualified: 8, prospecting: 4, closed: 1}[deal_status];
relationship_boost = {mature: 15, established: 10, developing: 6, initial: 3}[relationship_stage];
business_boost = min(revenue_boost + urgency_boost + tier_boost + deal_boost + relationship_boost, 20);

// RAG relevance boost
rag_boost = rag_relevance * 20;

// Final priority
final_priority = base_priority + recency_boost + business_boost + rag_boost;
```

### Council Member Relevance

```javascript
// Operator (visionary, strategic)
operator_score = 
  count_keywords(["strategic", "future", "ecosystem", "transformative", "vision"]) * 0.4 +
  future_oriented_tense_ratio * 0.3 +
  abstract_concept_density * 0.3;

// Strategist (architectural, systematic)
strategist_score = 
  count_keywords(["system", "structure", "principle", "scalability", "architecture"]) * 0.4 +
  structural_language_ratio * 0.3 +
  framework_mention_count * 0.3;

// Signal (practical, tactical)
signal_score = 
  count_keywords(["implement", "actionable", "practical", "step", "execute"]) * 0.4 +
  imperative_verb_ratio * 0.3 +
  concrete_example_count * 0.3;
```

### Token Budget Allocation

```javascript
// Available tokens after reserves
available = maxTokens - reserveTokens;

// Greedy knapsack selection (sorted by priority desc)
tokens_used = 0;
selected_chunks = [];

for (chunk of chunks_sorted_by_priority_desc) {
  if (tokens_used + chunk.tokens <= available) {
    selected_chunks.push(chunk);
    tokens_used += chunk.tokens;
  } else {
    dropped_chunks.push(chunk);
  }
}
```

### Council Context Distribution

```javascript
// Get token allocations per council member
operator_tokens = available_tokens * operator_context_ratio;
strategist_tokens = available_tokens * strategist_context_ratio;
signal_tokens = available_tokens * signal_context_ratio;

// Select chunks for each member (greedy by relevance score)
operator_chunks = greedy_select(
  chunks.filter(c => c.council_member_relevance.operator >= threshold),
  operator_tokens,
  sort_by: operator_score_desc
);

strategist_chunks = greedy_select(
  chunks.filter(c => c.council_member_relevance.strategist >= threshold),
  strategist_tokens,
  sort_by: strategist_score_desc
);

signal_chunks = greedy_select(
  chunks.filter(c => c.council_member_relevance.signal >= threshold),
  signal_tokens,
  sort_by: signal_score_desc
);
```

---

**[Continued in next file...]**
