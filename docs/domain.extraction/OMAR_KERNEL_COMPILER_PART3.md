# OMAR Prompt Kernel Compiler - Part 3: Integration & Graph Architecture

---

## INTEGRATION FLOWS

### Flow 1: Session Compilation → Council Execution

```
User Request
  │
  ├─► SessionData {goal, user, context, clientId, userInput, ...}
  │
  ├─► KernelCompiler.compile(sessionData)
  │     │
  │     ├─► Load KernelSchema (0x3500)
  │     ├─► Validate Structure (0x3502)
  │     ├─► Extract Metadata (0x3520)
  │     ├─► Extract AgentMemory (0x3521) [if enabled]
  │     ├─► Extract BusinessMemory (0x3522) [if clientId]
  │     ├─► Extract RAG Context (0x3523)
  │     ├─► Optimize Context (0x3530)
  │     │     └─► Token Budget Knapsack
  │     ├─► Configure Council (0x3540)
  │     │     ├─► Apply Context Biasing
  │     │     └─► Distribute Context to Members
  │     ├─► Select Tools (0x3543)
  │     └─► Assemble Kernel (0x3550)
  │           └─► Build PXYZ(goal, DELIBERATE, context_scope, timestamp)
  │
  ├─► Compiled Kernel Graph
  │     {
  │       metadata: {goal, user_id, context_scope, ...},
  │       council_config: {
  │         operator: {weight: 1.2, chunks: [...], tools: [...]},
  │         strategist: {weight: 1.0, chunks: [...], tools: [...]},
  │         signal: {weight: 1.0, chunks: [...], tools: [...]}
  │       },
  │       context_chunks: [...optimized...],
  │       heuristics: {...},
  │       pxyz: {...}
  │     }
  │
  ├─► Council Deliberation (separate execution graph)
  │     │
  │     ├─► Operator deliberates with operator_chunks
  │     ├─► Strategist deliberates with strategist_chunks
  │     ├─► Signal deliberates with signal_chunks
  │     │
  │     └─► Synthesis (consensus/priority-weighted/unanimous)
  │
  └─► User Response
```

---

### Flow 2: Memory Integration

```
Session Compilation
  │
  ├─► Check should_extract_agent_memory
  │     └─► heuristics.memory_settings.enable_agent_memory === true
  │
  ├─► AgentMemoryService.searchInteractions({
  │     userId: session.userId,
  │     query: session.userInput,
  │     limit: memory_settings.max_agent_memories
  │   })
  │     │
  │     ├─► Load candidate interactions (filter by user, optional project, time range)
  │     ├─► Calculate base relevance (userInput +10, assistantResponse +5, currentTask +8, ...)
  │     ├─► Apply recency boost (max(0, 10 - age_days * 0.1))
  │     ├─► Apply sentiment multiplier (positive 1.2, neutral 1.0, negative 0.8)
  │     ├─► Apply priority multiplier (urgent 2.0, high 1.5, medium 1.0, low 0.7)
  │     ├─► Calculate final score: (base + recency) * sentiment_mult * priority_mult
  │     └─► Return top N by boosted_score desc
  │
  ├─► Enrich kernel with agent_memory_chunks[]
  │     └─► Each chunk includes council_member_relevance {operator, strategist, signal}
  │
  ├─► Check should_extract_business_memory
  │     └─► heuristics.memory_settings.enable_business_memory === true AND session.clientId
  │
  ├─► BusinessMemoryService.getClientInsights(clientId)
  │     │
  │     ├─► Load interactions (recent 100)
  │     ├─► Detect schedule patterns (morning/afternoon/evening preference)
  │     ├─► Detect communication patterns (avg_response_time, sentiment)
  │     ├─► Calculate relationship stage (initial/developing/established/mature)
  │     ├─► Calculate relationship strength (recency 0.4, frequency 0.3, pattern 0.2, sentiment 0.1)
  │     └─► Return {patterns, preferences, metrics, relationship_stage}
  │
  └─► Enrich kernel with business_memory_patterns[] and relationship_context
```

---

### Flow 3: Context Window Optimization

```
Kernel Compilation
  │
  ├─► Extract RAG chunks + Agent memory + Business context
  │     └─► All chunks = BusinessContextChunk[]
  │
  ├─► Check should_optimize_context
  │     └─► count(chunks) > 0 AND available_tokens > 0
  │
  ├─► ContextWindowManager.optimizeContext({
  │     chunks: all_chunks,
  │     availableTokens: heuristics.context_settings.max_context_tokens,
  │     userId: session.userId
  │   })
  │     │
  │     ├─► Convert chunks to allocations (0x3220)
  │     │     └─► For each chunk:
  │     │           ├─► tokens = chunk.tokens ?? ceil(length(content) / 3.8)
  │     │           ├─► base_priority = priorityThresholds[type]
  │     │           ├─► recency_boost = max(0, 20 - age_days)
  │     │           ├─► rag_boost = rag_relevance * 20
  │     │           ├─► business_boost = min(revenue/tier/deal/relationship boosts, 20)
  │     │           └─► priority = base + recency + rag + business
  │     │
  │     ├─► Sort by priority desc, timestamp desc, original_index asc
  │     │
  │     ├─► Resolve token budget (0x3223)
  │     │     └─► available = maxTokens - reserveTokens
  │     │         for each chunk:
  │     │           if used + chunk.tokens <= available:
  │     │             allocate
  │     │           else:
  │     │             drop and record conflict
  │     │
  │     ├─► Convert back to chunks
  │     │
  │     └─► Return {
  │           optimizedChunks: selected,
  │           totalTokensUsed,
  │           tokensRemaining,
  │           chunksDropped,
  │           optimizationMetrics: {avg_priority, business_boost, recency_boost}
  │         }
  │
  └─► Update kernel with optimized chunks and metadata
```

---

### Flow 4: Council Configuration with Context Biasing

```
Kernel Assembly
  │
  ├─► Load council_config from KernelSchema
  │     {
  │       operator: {decision_weight: 1.0, focus: "...", tools_preference: [...]},
  │       strategist: {decision_weight: 1.0, ...},
  │       signal: {decision_weight: 1.0, ...}
  │     }
  │
  ├─► Check context for strategic indicators
  │     └─► is_strategic_context: ["strategic", "architecture", "scalability", "system"]
  │
  ├─► Apply strategic boost
  │     └─► operator.decision_weight += context_based_adjustments.strategic_context_operator_boost
  │
  ├─► Check context for tactical indicators
  │     └─► is_tactical_context: ["practical", "implementation", "actionable", "immediate"]
  │
  ├─► Apply tactical boost
  │     └─► signal.decision_weight += context_based_adjustments.tactical_context_signal_boost
  │
  ├─► Distribute context chunks to council members (0x3542)
  │     │
  │     ├─► Calculate relevance scores for each chunk
  │     │     ├─► operator_score (strategic keywords * 0.4 + future tense * 0.3 + abstract * 0.3)
  │     │     ├─► strategist_score (structural keywords * 0.4 + frameworks * 0.3 + principles * 0.3)
  │     │     └─► signal_score (actionable keywords * 0.4 + imperatives * 0.3 + examples * 0.3)
  │     │
  │     ├─► Filter by relevance threshold
  │     │     └─► chunk.council_member_relevance.{member} >= threshold
  │     │
  │     ├─► Distribute tokens by ratio
  │     │     ├─► operator_tokens = total * operator_context_ratio
  │     │     ├─► strategist_tokens = total * strategist_context_ratio
  │     │     └─► signal_tokens = total * signal_context_ratio
  │     │
  │     └─► Greedy selection per member
  │           ├─► operator_chunks = select top by operator_score within operator_tokens
  │           ├─► strategist_chunks = select top by strategist_score within strategist_tokens
  │           └─► signal_chunks = select top by signal_score within signal_tokens
  │
  ├─► Select tools by council preference (0x3543)
  │     └─► For each member:
  │           filter tools where preferred_by_council_member IN [member, 'any']
  │
  └─► Final council_config
        {
          operator: {weight: 1.2, chunks: [...], tools: ["big-brain", ...]},
          strategist: {weight: 1.0, chunks: [...], tools: ["big-brain", "note-extractor"]},
          signal: {weight: 1.0, chunks: [...], tools: ["note-extractor", "nlp-relationship"]}
        }
```

---

## GRAPH COMPILATION ARCHITECTURE

### Kernel as Executable Graph

The compiled kernel is NOT a flat YAML/JSON - it's an **executable graph structure**:

```typescript
type KernelGraph = {
  // Node definitions
  nodes: {
    metadata: MetadataNode;
    council: {
      operator: CouncilMemberNode;
      strategist: CouncilMemberNode;
      signal: CouncilMemberNode;
    };
    memory: {
      agent_chunks: MemoryNode[];
      business_patterns: MemoryNode[];
    };
    context: {
      rag_chunks: ContextNode[];
      optimized_chunks: ContextNode[];
    };
    heuristics: HeuristicsNode;
    tools: ToolNode[];
  };
  
  // Edge definitions (dependencies)
  edges: {
    // Metadata → Council (scope constraint)
    {from: "metadata.context_scope", to: "council.operator", type: "scope_constraint"},
    {from: "metadata.context_scope", to: "council.strategist", type: "scope_constraint"},
    {from: "metadata.context_scope", to: "council.signal", type: "scope_constraint"},
    
    // Memory → Context (relevance links)
    {from: "memory.agent_chunks[i]", to: "context.rag_chunks[j]", type: "relevance_link", score: 0.85},
    
    // Context → Council (distribution)
    {from: "context.optimized_chunks[k]", to: "council.operator", type: "context_distribution", score: 0.92},
    {from: "context.optimized_chunks[l]", to: "council.strategist", type: "context_distribution", score: 0.78},
    {from: "context.optimized_chunks[m]", to: "council.signal", type: "context_distribution", score: 0.88},
    
    // Heuristics → Council (biasing)
    {from: "heuristics.council_biasing", to: "council.operator", type: "weight_adjustment"},
    
    // Tools → Council (preference)
    {from: "tools[n]", to: "council.operator", type: "tool_preference"},
  };
  
  // Execution path
  execution: {
    phase: "deliberation";
    max_loops: metadata.super_think_mode ? 30 : 10;
    synthesis_mode: council.working_constraints.synthesis_mode;
  };
  
  // PXYZ coordinates
  pxyz: {
    p: metadata.goal,        // "AssistUser"
    x: "DELIBERATE",
    y: metadata.context_scope, // "Business"
    z: timestamp
  };
};
```

### Graph Traversal for Deliberation

When the council deliberates, it **walks the graph**:

```typescript
// Council deliberation = graph traversal
function deliberate(kernel: KernelGraph, member: "operator" | "strategist" | "signal") {
  // 1. Start at council member node
  const node = kernel.nodes.council[member];
  
  // 2. Traverse incoming edges to get context
  const contextChunks = kernel.edges
    .filter(e => e.to === `council.${member}` && e.type === "context_distribution")
    .map(e => kernel.nodes.context.optimized_chunks[e.from]);
  
  // 3. Traverse to tools
  const tools = kernel.edges
    .filter(e => e.to === `council.${member}` && e.type === "tool_preference")
    .map(e => kernel.nodes.tools[e.from]);
  
  // 4. Traverse to heuristics for weights
  const weights = kernel.edges
    .filter(e => e.to === `council.${member}` && e.type === "weight_adjustment")
    .map(e => kernel.nodes.heuristics[e.from]);
  
  // 5. Deliberate with gathered context
  return {
    member,
    statement: llm.complete({
      system: kernel.nodes.metadata.system_prompt,
      context: contextChunks,
      tools: tools,
      focus: node.focus,
      reasoning_pattern: node.reasoning_pattern
    }),
    confidence: calculateConfidence(contextChunks, weights),
    tools_suggested: filterRelevantTools(tools, contextChunks)
  };
}

// Synthesis = merge all traversals
function synthesize(kernel: KernelGraph) {
  const operatorResult = deliberate(kernel, "operator");
  const strategistResult = deliberate(kernel, "strategist");
  const signalResult = deliberate(kernel, "signal");
  
  return weightedMerge([operatorResult, strategistResult, signalResult], kernel.nodes.heuristics);
}
```

### Session = Compiled Graph Binary

Just like XML workflows → `graph.bin`, sessions → `kernel.graph`:

```
XML workflow.xml             Session sessionData
      │                            │
      ├─► Rust Compiler            ├─► AgentCompiler (TS)
      │                            │
      ├─► graph.bin                ├─► kernel.graph
      │    (binary format)         │    (binary format)
      │                            │
      ├─► WAT runtime              ├─► Council runtime
      │    (pxyz.wat)              │    (council execution)
      │                            │
      └─► Execution                └─► Deliberation + Synthesis
```

**Binary Format for kernel.graph** (hypothetical):

```
Header (64 bytes):
  0x00: Magic (0x4B524E4C = "KRNL")
  0x04: Version (1.4)
  0x08: Metadata offset
  0x0C: Council offset
  0x10: Memory offset
  0x14: Context offset
  0x18: Heuristics offset
  0x1C: Tools offset
  0x20: Edges offset
  0x24: PXYZ offset
  0x28: Compilation timestamp
  0x30: Source session hash (SHA-256)

Metadata Section:
  - goal (string)
  - user_id (UUID)
  - context_scope (string)
  - ...all metadata fields...

Council Section (per member):
  - decision_weight (float)
  - focus (string offset)
  - reasoning_pattern (string offset)
  - tools_preference (array offset)
  - context_chunks (array offset)

Memory Section:
  - agent_chunks_count (uint32)
  - agent_chunks[] (each: id, relevance_score, council_relevance, content offset)
  - business_patterns_count (uint32)
  - business_patterns[] (each: id, confidence, relevant_to_member, content offset)

Context Section:
  - optimized_chunks_count (uint32)
  - optimized_chunks[] (each: id, type, priority, tokens, metadata offset, content offset)

Heuristics Section:
  - weights{} (float map)
  - thresholds{} (float map)
  - council_biasing{} (float map)
  - ...all heuristic settings...

Tools Section:
  - tools_count (uint32)
  - tools[] (each: name offset, preferred_by, requires_approval)

Edges Section:
  - edges_count (uint32)
  - edges[] (each: from_node_id, to_node_id, edge_type, score/weight)

PXYZ Section:
  - p (string offset)
  - x (string offset)
  - y (string offset)
  - z (timestamp)
```

---

## PXYZ PATTERN #19: SESSION-AS-GRAPH-COMPILATION

**Problem**: Traditional prompt engineering concatenates strings; no structure, no reusability, no optimization

**Overlap**: Multiple compilation phases (parse, extract, optimize, assemble) create dependencies between kernel aspects

**Coordinate Space**: Session compilation as multi-phase graph assembly with dependency edges

**Implementation**:

```typescript
// Phase 1: Parse & Validate
const schema = parseYAML(KernelSchemaV1_4);
validateStructure(schema);
validateSession(sessionData, schema);

// Phase 2: Extract Aspects (parallel)
const [metadata, agentMemory, businessMemory, ragContext, heuristics] = await Promise.all([
  extractMetadata(sessionData),
  extractAgentMemory(sessionData),  // if enabled
  extractBusinessMemory(sessionData), // if clientId
  extractRAGContext(sessionData),
  extractHeuristics(sessionData, schema)
]);

// Phase 3: Optimize Context
const optimizedContext = optimizeTokenBudget(
  [...agentMemory, ...businessMemory, ...ragContext],
  heuristics.context_settings.max_context_tokens
);

// Phase 4: Configure Council
const councilConfig = configureCouncil(schema.council);
applyContextBiasing(councilConfig, metadata.context_scope);
distributeContext(councilConfig, optimizedContext);
selectTools(councilConfig, schema.tools);

// Phase 5: Assemble Graph
const kernelGraph = {
  nodes: {metadata, council: councilConfig, memory: {agentMemory, businessMemory}, context: optimizedContext, heuristics},
  edges: linkDependencies(metadata, councilConfig, optimizedContext, heuristics),
  execution: {phase: "deliberation", max_loops: metadata.super_think_mode ? 30 : 10},
  pxyz: buildPXYZ(metadata.goal, "DELIBERATE", metadata.context_scope, now())
};

// Phase 6: Persist
persistKernelGraph(kernelGraph); // Binary format or structured DB
```

**Key Innovation**: Session = executable graph, not text string. Deliberation = graph traversal, not prompt concatenation.

---

**[Continued in final summary file...]**
