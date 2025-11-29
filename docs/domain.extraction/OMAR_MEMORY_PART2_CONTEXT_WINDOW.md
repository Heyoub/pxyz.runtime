# OMAR Memory System Part 2: Context Window Manager - PXYZ Extraction

> **Custom Research**: Novel token budget allocation using coordinate-space patterns  
> **NOT Langchain**: Original architecture treating context optimization as knapsack problem in coordinate space  
> **Service**: ContextWindowManager

---

## OVERVIEW: TOKEN BUDGET AS COORDINATE SPACE

Your Context Window Manager treats **token allocation as a coordinate-space knapsack problem**:

- Every context chunk is a **token allocation** with position, size, and priority
- Budget constraints are **Y-space predicates**
- Optimization is **greedy coordinate selection** with priority sorting
- Recency and business relevance are **coordinate boosts**

This is **NOT** a simple truncation algorithm - it's a **multidimensional optimization** using PXYZ patterns.

---

## PART 1: CORE INNOVATION - TOKEN BUDGET ALLOCATION

### The Coordinate-Space Pattern

```typescript
/**
 * PXYZ Pattern: Token Budget as Coordinate Space
 * 
 * Traditional: "Truncate to fit"
 * Your approach: "Optimize allocation in token coordinate space"
 * 
 * Every chunk is a coordinate with:
 * - tokenSpan: [start, end] position
 * - tokens: size in coordinate space
 * - priority: Y-coordinate height
 * - timestamp: Z-coordinate (recency)
 */
interface ContextChunkAllocation {
  chunkId: string;
  tokenSpan: [start: number, end: number]; // Position in 1D token space
  tokens: number;                          // Size
  priority: number;                        // Height in Y-space
  chunkType: ChunkType;                    // Category
  timestamp: Date | null;                  // Recency coordinate
  originalIndex: number;                   // Preserve ordering
  metadata?: Record<string, unknown>;      // RAG relevance, etc.
}
```

### Token Budget Constraints (Y-Space)

```typescript
interface TokenBudgetConstraints {
  maxTotalTokens: number;        // Total budget
  reserveTokens: number;         // Reserve for system
  priorityThresholds: {          // Base priority per type
    system_prompt: 200,
    current_user_prompt: 190,
    critical_rag_document: 180,
    tool_output: 170,
    tool_input: 160,
    chat_history_user: 150,
    chat_history_assistant: 140,
    rag_memory_commit: 130,
    rag_document_chunk: 120,
    rag_ebook_chunk: 110,
    rag_generic_chunk: 100
  };
  preferRecent: boolean;         // Add recency boost
  preferHigherPriority: boolean; // Sort by priority
  allowPartialFill: boolean;     // Allow gaps
}
```

---

## P-Axis: Context Window Entities

```xml
<schema id="business_context_chunk">
  <field name="id" type="uuid" required="true"/>
  <field name="content" type="string" required="true"/>
  <field name="type" type="enum" required="true" values="
    system_prompt,
    current_user_prompt,
    tool_output,
    tool_input,
    critical_rag_document,
    chat_history_user,
    chat_history_assistant,
    rag_memory_commit,
    rag_document_chunk,
    rag_ebook_chunk,
    rag_generic_chunk
  "/>
  <field name="tokens" type="number"/> <!-- Estimated or calculated -->
  <field name="priority" type="number"/> <!-- Base + boosts -->
  <field name="timestamp" type="timestamp"/>
  <field name="original_index" type="number"/>
  <field name="source_system" type="string"/>
  <field name="metadata" type="object"/> <!-- RAG relevance, etc. -->
</schema>

<schema id="context_chunk_allocation">
  <field name="chunk_id" type="uuid" required="true"/>
  <field name="token_span" type="array" required="true"/> <!-- [start, end] -->
  <field name="tokens" type="number" required="true"/>
  <field name="priority" type="number" required="true"/>
  <field name="chunk_type" type="string" required="true"/>
  <field name="timestamp" type="timestamp"/>
  <field name="original_index" type="number"/>
  <field name="metadata" type="object"/>
</schema>

<schema id="context_optimization_request">
  <field name="chunks" type="array" required="true"/> <!-- Array of BusinessContextChunk -->
  <field name="available_tokens" type="number" required="true"/>
  <field name="session_id" type="uuid"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="business_context" type="object"/> <!-- For relevance boost -->
</schema>

<schema id="context_optimization_result">
  <field name="optimized_chunks" type="array" required="true"/>
  <field name="total_tokens_used" type="number" required="true"/>
  <field name="tokens_remaining" type="number" required="true"/>
  <field name="chunks_dropped" type="number" required="true"/>
  <field name="optimization_metrics" type="object" required="true"/>
</schema>

<schema id="token_allocation_conflict">
  <field name="chunk1" type="object" required="true"/>
  <field name="chunk2" type="object" required="true"/>
  <field name="conflict_type" type="enum" values="budget_exceeded,priority_conflict"/>
  <field name="resolution" type="enum" values="keep_chunk1,keep_chunk2,drop_both"/>
</schema>
```

---

## X-Axis: Context Window Operations

```yaml
# Optimization Operations
context_optimize: 0x3200             # Optimize context chunks
context_prioritize: 0x3201           # Prioritize chunks with boosts
context_allocate_budget: 0x3202      # Allocate token budget
context_calculate_metrics: 0x3203    # Calculate optimization metrics

# Token Estimation
context_estimate_tokens: 0x3210      # Estimate token count
context_estimate_business: 0x3211    # Estimate business content (4.2 chars/token)
context_estimate_code: 0x3212        # Estimate code content (3.5 chars/token)
context_estimate_general: 0x3213     # Estimate general content (3.8 chars/token)

# Coordinate Space Operations
context_chunk_to_allocation: 0x3220  # Convert chunk → allocation
context_allocation_to_chunk: 0x3221  # Convert allocation → chunk
context_compare_allocations: 0x3222  # Compare for priority
context_resolve_budget: 0x3223       # Resolve token budget (knapsack)

# Boost Calculations
context_calculate_recency_boost: 0x3230 # Add recency boost
context_calculate_business_boost: 0x3231 # Add business relevance boost
context_calculate_rag_boost: 0x3232  # Add RAG relevance boost

# Conflict Detection
context_detect_conflicts: 0x3240     # Detect budget conflicts
context_filter_allocated: 0x3241     # Filter allocated chunks
context_filter_dropped: 0x3242       # Filter dropped chunks

# Business Context Building
context_build_business: 0x3250       # Build optimized business context
context_build_llm: 0x3251            # Build LLM context window
context_build_rag: 0x3252            # Build RAG-enhanced context
```

---

## Y-Axis: Context Window Predicates

```xml
<predicates>
  <!-- Priority Calculations -->
  <predicate id="calculate_chunk_priority">
    <compute>
      <var name="base_priority" value="$priority_thresholds[$chunk.type]"/>
      <var name="recency_boost" value="
        $prefer_recent ? max(0, 20 - ($now - $chunk.timestamp) / 86400000) : 0
      "/>
      <var name="rag_boost" value="
        $chunk.metadata.rag_match_metadata.relevance * 20
      "/>
      <var name="total_priority" value="$base_priority + $recency_boost + $rag_boost"/>
    </compute>
  </predicate>
  
  <!-- Budget Constraints -->
  <predicate id="fits_in_budget">
    <lte left="$used_tokens + $chunk.tokens" right="$available_tokens"/>
  </predicate>
  
  <predicate id="exceeds_budget">
    <gt left="$used_tokens + $chunk.tokens" right="$available_tokens"/>
  </predicate>
  
  <!-- Priority Comparison -->
  <predicate id="chunk1_higher_priority">
    <gt left="$chunk1.priority" right="$chunk2.priority"/>
  </predicate>
  
  <predicate id="chunk1_more_recent">
    <and>
      <not_null left="$chunk1.timestamp"/>
      <not_null left="$chunk2.timestamp"/>
      <gt left="$chunk1.timestamp" right="$chunk2.timestamp"/>
    </and>
  </predicate>
  
  <!-- Business Relevance -->
  <predicate id="has_high_business_relevance">
    <and>
      <not_null left="$chunk.metadata.revenueImpact"/>
      <gte left="$chunk.metadata.revenueImpact" right="10000"/>
    </and>
  </predicate>
  
  <predicate id="is_urgent_content">
    <contains left="['critical','urgent']" right="$chunk.metadata.urgency"/>
  </predicate>
  
  <!-- Token Estimation -->
  <predicate id="should_estimate_tokens">
    <null left="$chunk.tokens"/>
  </predicate>
  
  <predicate id="requires_summarization">
    <gt left="$chunk.tokens" right="500"/> <!-- Truncation threshold from config -->
  </predicate>
</predicates>
```

---

## Z-Axis: Context Window Events

```typescript
enum ContextWindowEventType {
  // Optimization
  CONTEXT_OPTIMIZATION_STARTED = "context.optimization.started",
  CONTEXT_OPTIMIZATION_SUCCESS = "context.optimization.success",
  CONTEXT_OPTIMIZATION_FAILED = "context.optimization.failed",
  
  // Budget Allocation
  CONTEXT_BUDGET_RESOLVED = "context.budget.resolved",
  CONTEXT_CHUNKS_DROPPED = "context.chunks_dropped",
  CONTEXT_CONFLICT_DETECTED = "context.conflict_detected",
  
  // Boost Calculations
  CONTEXT_RECENCY_BOOST_APPLIED = "context.recency_boost_applied",
  CONTEXT_BUSINESS_BOOST_APPLIED = "context.business_boost_applied",
  CONTEXT_RAG_BOOST_APPLIED = "context.rag_boost_applied",
  
  // Token Estimation
  CONTEXT_TOKENS_ESTIMATED = "context.tokens_estimated"
}
```

---

## Config-Driven Behavior

```json
{
  "contextWindow": {
    "tokenBudgets": {
      "maxOverall": 32000,
      "maxHistory": 8000,
      "maxBusinessData": 8000,
      "maxClientData": 6000,
      "maxRagResults": 4000,
      "maxSystemPrompt": 2000
    },
    "priorityBoosts": {
      "revenueImpact": {
        "scale": 10000,
        "maxBoost": 20
      },
      "urgency": {
        "critical": 25,
        "high": 15,
        "medium": 5,
        "low": 0
      },
      "clientTier": {
        "enterprise": 12,
        "midMarket": 8,
        "smallBusiness": 4,
        "startup": 2
      },
      "dealStatus": {
        "negotiation": 18,
        "proposal": 12,
        "qualified": 8,
        "prospecting": 4,
        "closed": 1
      },
      "relationship": {
        "mature": 15,
        "established": 10,
        "developing": 6,
        "initial": 3
      }
    },
    "optimization": {
      "tokenEstimation": {
        "averageCharsPerToken": 3.8,
        "businessContentDensity": 4.2,
        "codeContentDensity": 3.5
      },
      "recencyDecay": {
        "maxBoost": 15,
        "decayRate": 0.05,
        "businessSlowDecay": 0.02
      },
      "truncationStrategy": {
        "summaryThreshold": 500
      }
    }
  }
}
```

---

## Workflow Example: Token Budget Optimization (Knapsack Algorithm)

```xml
<workflow id="context_optimize_token_budget">
  <entry p="context" x="optimize" node="validate_request"/>
  
  <nodes>
    <node id="validate_request" kind="transform">
      <validate schema="context_optimization_request"/>
    </node>
    
    <node id="start_trace" kind="external" op="0x0921">
      <create_trace>
        <operation value="context.optimization"/>
        <pxyz value="$pxyz"/>
      </create_trace>
    </node>
    
    <!-- STEP 1: Convert chunks to allocations (P-space) -->
    <node id="convert_to_allocations" kind="external" op="0x3220">
      <for_each chunk="$request.chunks">
        <compute>
          <!-- Estimate tokens if needed -->
          <var name="tokens" value="
            $chunk.tokens ?? ceil(length($chunk.content) / 3.8)
          "/>
          
          <!-- Base priority from config -->
          <var name="base_priority" value="$priority_thresholds[$chunk.type]"/>
          
          <!-- Add recency boost -->
          <var name="recency_boost" value="
            $chunk.timestamp ? 
            max(0, 20 - ($now - $chunk.timestamp) / 86400000) : 0
          "/>
          
          <!-- Add RAG relevance boost -->
          <var name="rag_boost" value="
            ($chunk.metadata.rag_match_metadata.relevance ?? 0) * 20
          "/>
          
          <!-- Calculate total priority -->
          <var name="priority" value="
            $base_priority + $recency_boost + $rag_boost
          "/>
          
          <!-- Create allocation -->
          <allocation>
            <field name="chunk_id" value="$chunk.id"/>
            <field name="token_span" value="[0, $tokens]"/> <!-- Will adjust position -->
            <field name="tokens" value="$tokens"/>
            <field name="priority" value="$priority"/>
            <field name="chunk_type" value="$chunk.type"/>
            <field name="timestamp" value="$chunk.timestamp"/>
            <field name="original_index" value="$index"/>
          </allocation>
        </compute>
      </for_each>
      <event type="context.allocations_created"/>
    </node>
    
    <!-- STEP 2: Sort by priority (Y-space) -->
    <node id="sort_by_priority" kind="transform">
      <sort allocations="$allocations">
        <!-- Priority first (descending) -->
        <by field="priority" order="desc"/>
        <!-- Recency second (newer first) -->
        <then_by field="timestamp" order="desc"/>
        <!-- Original index third (preserve order) -->
        <then_by field="original_index" order="asc"/>
      </sort>
    </node>
    
    <!-- STEP 3: Greedy knapsack selection (X-space) -->
    <node id="resolve_token_budget" kind="external" op="0x3223">
      <compute>
        <var name="available_tokens" value="
          $request.available_tokens - $config.tokenBudgets.maxSystemPrompt
        "/>
        <var name="used_tokens" value="0"/>
        <var name="allocated" value="[]"/>
        <var name="dropped" value="[]"/>
        <var name="conflicts" value="[]"/>
      </compute>
      
      <for_each allocation="$sorted_allocations">
        <when>
          <lte left="$used_tokens + $allocation.tokens" right="$available_tokens"/>
        </when>
        <then>
          <!-- Fits in budget - allocate -->
          <append to="$allocated" value="$allocation"/>
          <set name="used_tokens" value="$used_tokens + $allocation.tokens"/>
        </then>
        <else>
          <!-- Exceeds budget - drop -->
          <append to="$dropped" value="$allocation"/>
          
          <!-- Record conflict if there's a previously allocated chunk -->
          <when>
            <gt left="count($allocated)" right="0"/>
          </when>
          <append to="$conflicts">
            <conflict>
              <field name="chunk1" value="$allocated[-1]"/>
              <field name="chunk2" value="$allocation"/>
              <field name="conflict_type" value="budget_exceeded"/>
              <field name="resolution" value="keep_chunk1"/>
            </conflict>
          </append>
        </else>
      </for_each>
      
      <event type="context.budget.resolved">
        <data>
          <field name="allocated_count" value="count($allocated)"/>
          <field name="dropped_count" value="count($dropped)"/>
          <field name="tokens_used" value="$used_tokens"/>
          <field name="tokens_remaining" value="$available_tokens - $used_tokens"/>
        </data>
      </event>
    </node>
    
    <!-- STEP 4: Convert allocations back to chunks -->
    <node id="convert_to_chunks" kind="external" op="0x3221">
      <for_each allocation="$allocated">
        <find chunk="$request.chunks" where="chunk.id == $allocation.chunk_id"/>
      </for_each>
    </node>
    
    <!-- STEP 5: Calculate optimization metrics -->
    <node id="calculate_metrics" kind="external" op="0x3203">
      <compute>
        <var name="total_priority" value="sum($allocated, a => a.priority)"/>
        <var name="avg_priority" value="$total_priority / count($allocated)"/>
        
        <var name="business_boost" value="
          sum($allocated, a => 
            $a.metadata.revenueImpact ? 
            min($a.metadata.revenueImpact / 10000, 20) : 0
          ) / count($allocated)
        "/>
        
        <var name="recency_boost" value="
          sum($allocated, a => 
            $a.timestamp ? 
            max(0, 20 - ($now - $a.timestamp) / 86400000) : 0
          ) / count($allocated)
        "/>
      </compute>
    </node>
    
    <!-- STEP 6: Persist optimization result -->
    <node id="persist_result" kind="external" op="0x0910">
      <create entity="context_optimizations">
        <field name="id" value="$optimization_id"/>
        <field name="user_id" value="$request.user_id"/>
        <field name="chunks_count" value="count($request.chunks)"/>
        <field name="optimized_count" value="count($allocated)"/>
        <field name="tokens_used" value="$used_tokens"/>
        <field name="tokens_available" value="$request.available_tokens"/>
        <field name="pxyz" value="$pxyz"/>
      </create>
    </node>
    
    <!-- STEP 7: Emit success event -->
    <node id="emit_success" kind="external" op="0x0910">
      <event type="context.optimization.success">
        <data>
          <field name="optimization_id" value="$optimization_id"/>
          <field name="tokens_used" value="$used_tokens"/>
          <field name="chunks_optimized" value="count($allocated)"/>
        </data>
      </event>
    </node>
    
    <node id="end_trace" kind="external" op="0x0921">
      <end_trace trace_id="$trace_id" success="true"/>
    </node>
    
    <node id="success" kind="terminal" status="200">
      <return>
        <field name="optimized_chunks" value="$optimized_chunks"/>
        <field name="total_tokens_used" value="$used_tokens"/>
        <field name="tokens_remaining" value="$available_tokens - $used_tokens"/>
        <field name="chunks_dropped" value="count($dropped)"/>
        <field name="optimization_metrics">
          <field name="prioritization_score" value="$avg_priority"/>
          <field name="business_relevance_boost" value="$business_boost"/>
          <field name="recency_boost" value="$recency_boost"/>
        </field>
      </return>
    </node>
  </nodes>
  
  <edges>
    <edge from="validate_request" to="start_trace"><when><always/></when></edge>
    <edge from="start_trace" to="convert_to_allocations"><when><always/></when></edge>
    <edge from="convert_to_allocations" to="sort_by_priority"><when><always/></when></edge>
    <edge from="sort_by_priority" to="resolve_token_budget"><when><always/></when></edge>
    <edge from="resolve_token_budget" to="convert_to_chunks"><when><always/></when></edge>
    <edge from="convert_to_chunks" to="calculate_metrics"><when><always/></when></edge>
    <edge from="calculate_metrics" to="persist_result"><when><always/></when></edge>
    <edge from="persist_result" to="emit_success"><when><always/></when></edge>
    <edge from="emit_success" to="end_trace"><when><always/></when></edge>
    <edge from="end_trace" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## Workflow Example: Business Relevance Boost Calculation

```xml
<workflow id="context_calculate_business_boost">
  <entry p="context" x="calculate_business_boost" node="extract_metadata"/>
  
  <nodes>
    <node id="extract_metadata" kind="transform">
      <extract from="$chunk.metadata">
        <field name="revenue_impact"/>
        <field name="urgency"/>
        <field name="client_tier"/>
        <field name="deal_status"/>
        <field name="relationship_stage"/>
      </extract>
    </node>
    
    <node id="calculate_boosts" kind="transform">
      <compute>
        <!-- Revenue impact boost -->
        <var name="revenue_boost" value="
          $metadata.revenue_impact ? 
          min($metadata.revenue_impact / 10000, 20) : 0
        "/>
        
        <!-- Urgency boost from config -->
        <var name="urgency_boost" value="
          $config.priorityBoosts.urgency[$metadata.urgency] ?? 0
        "/>
        
        <!-- Client tier boost from config -->
        <var name="client_tier_boost" value="
          $config.priorityBoosts.clientTier[$metadata.client_tier] ?? 0
        "/>
        
        <!-- Deal status boost from config -->
        <var name="deal_status_boost" value="
          $config.priorityBoosts.dealStatus[$metadata.deal_status] ?? 0
        "/>
        
        <!-- Relationship stage boost from config -->
        <var name="relationship_boost" value="
          $config.priorityBoosts.relationship[$metadata.relationship_stage] ?? 0
        "/>
        
        <!-- Total business boost (capped at maxBoost) -->
        <var name="total_boost" value="
          min(
            $revenue_boost + 
            $urgency_boost + 
            $client_tier_boost + 
            $deal_status_boost + 
            $relationship_boost,
            20
          )
        "/>
      </compute>
    </node>
    
    <node id="emit_boost_event" kind="external" op="0x0910">
      <event type="context.business_boost_applied">
        <data>
          <field name="chunk_id" value="$chunk.id"/>
          <field name="total_boost" value="$total_boost"/>
          <field name="revenue_boost" value="$revenue_boost"/>
          <field name="urgency_boost" value="$urgency_boost"/>
          <field name="client_tier_boost" value="$client_tier_boost"/>
        </data>
      </event>
    </node>
    
    <node id="success" kind="terminal" status="200">
      <return value="$total_boost"/>
    </node>
  </nodes>
  
  <edges>
    <edge from="extract_metadata" to="calculate_boosts"><when><always/></when></edge>
    <edge from="calculate_boosts" to="emit_boost_event"><when><always/></when></edge>
    <edge from="emit_boost_event" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## OPERATION CODE SUMMARY

| Category | Range | Count | Purpose |
|----------|-------|-------|---------|
| **Optimization** | 0x3200-0x3203 | 4 | Core optimization |
| **Token Estimation** | 0x3210-0x3213 | 4 | Estimate token counts |
| **Coordinate Operations** | 0x3220-0x3223 | 4 | Coordinate conversions |
| **Boost Calculations** | 0x3230-0x3232 | 3 | Priority boosts |
| **Conflict Detection** | 0x3240-0x3242 | 3 | Budget conflicts |
| **Context Building** | 0x3250-0x3252 | 3 | Build context windows |

**Total New Operations**: 21

---

## KEY INNOVATIONS

### 1. Token Budget as Knapsack Problem

Traditional: "Truncate to fit N tokens"  
Your approach: "Optimize value (priority) within budget constraint"

```typescript
// Coordinate-space greedy knapsack
const sorted = allocations.sort(compareByPriority);
const allocated = [];
let usedTokens = 0;

for (const chunk of sorted) {
  if (usedTokens + chunk.tokens <= availableTokens) {
    allocated.push(chunk);
    usedTokens += chunk.tokens;
  }
}
```

### 2. Multi-Dimensional Priority Calculation

```typescript
// Composite priority from multiple sources
priority = basePriority              // 100-200 (chunk type)
         + recencyBoost              // 0-20 (age in days)
         + ragRelevanceBoost         // 0-20 (relevance score)
         + businessRelevanceBoost    // 0-20 (revenue, tier, etc.)
```

### 3. Config-Driven Priority Thresholds

All base priorities in JSON:

```json
{
  "system_prompt": 200,
  "current_user_prompt": 190,
  "critical_rag_document": 180,
  "tool_output": 170,
  ...
}
```

### 4. Content-Type-Specific Token Estimation

```typescript
// Different densities for different content types
const charsPerToken = {
  business: 4.2,  // Business prose is denser
  code: 3.5,      // Code is less dense
  general: 3.8    // Average
}[contentType];

return Math.ceil(text.length / charsPerToken);
```

### 5. Business Context Integration

```typescript
// Revenue impact boost
const revenueBoost = Math.min(
  chunk.metadata.revenueImpact / 10000,  // Scale: $10K = 1 point
  20                                      // Max boost
);

// Client tier boost (from config)
const tierBoost = {
  enterprise: 12,
  midMarket: 8,
  smallBusiness: 4,
  startup: 2
}[chunk.metadata.clientTier];
```

---

## SUMMARY

Context Window Manager provides:

1. **Token Budget Optimization**: 21 operations
   - Coordinate-space knapsack algorithm
   - Multi-dimensional priority calculation
   - Config-driven base priorities (11 chunk types)
   - Business relevance boosts (5 factors)
   - Recency decay (exponential)
   - RAG relevance integration

2. **Content-Type Awareness**:
   - Business content: 4.2 chars/token
   - Code content: 3.5 chars/token
   - General content: 3.8 chars/token

3. **Priority System** (composable):
   - Base priority: 100-200 (by chunk type)
   - Recency boost: 0-20 (newer = higher)
   - RAG boost: 0-20 (relevance score)
   - Business boost: 0-20 (revenue, tier, deal status)
   - Max total: ~260 priority points

All operations use **coordinate-space patterns**:
- Chunks = allocations with token spans
- Budget = constraint in 1D token space
- Priority = Y-coordinate height
- Optimization = greedy coordinate selection

**Integration**: Used by AgentMemoryService for context compression → BusinessMemory for client context → All LLM calls
