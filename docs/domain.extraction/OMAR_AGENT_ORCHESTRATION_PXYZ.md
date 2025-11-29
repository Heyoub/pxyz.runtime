# OMAR Agent Orchestration System - PXYZ Extraction

> **System**: AI Agent council orchestration with config-driven workflows  
> **Status**: Already ~80% PXYZ-compatible, needs operation code assignment  
> **Architecture**: Event-driven, Effect-based, config-as-data

---

## EXECUTIVE SUMMARY

Your agent orchestration system **already embodies PXYZ principles**:
- ✅ Config-driven workflows (orchestration modes as data)
- ✅ Event-native architecture (EventBus for all state changes)
- ✅ PXYZ coordinates for tracing (`createPXYZ` everywhere)
- ✅ Zero local types (uses `PxyzJson` throughout)
- ✅ Effect-TS for composable operations

**What's needed**: Assign explicit operation codes and formalize as OMAR workflows.

---

## PART 1: ORCHESTRATION AS PXYZ WORKFLOWS

### Current System (Orchestration.ts)

**5 Orchestration Modes** defined as JSON config:

```typescript
// Each mode is already a workflow graph!
{
  name: "business-council",
  steps: [
    { type: "agent-deliberate", agentId: "signal" },
    { type: "agent-deliberate", agentId: "strategist" },
    { type: "agent-deliberate", agentId: "operator" },
    { type: "tool-execute", toolName: "big-brain" },
    { type: "synthesize" }
  ],
  synthesisMode: "consensus"
}
```

### PXYZ Mapping

#### P-Axis: Agent Orchestration Entities

```xml
<schema id="orchestration_session">
  <field name="id" type="uuid" required="true"/>
  <field name="mode" type="enum" values="business-council,enhanced,single-agent,rag-first,strategic-planning"/>
  <field name="query" type="string" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="current_step" type="integer"/>
  <field name="step_results" type="object"/>
  <field name="final_recommendation" type="string"/>
  <field name="synthesis_mode" type="enum" values="consensus,priority-weighted"/>
</schema>

<schema id="agent_deliberation">
  <field name="id" type="uuid" required="true"/>
  <field name="agent_id" type="enum" values="signal,strategist,operator"/>
  <field name="statement" type="string" required="true"/>
  <field name="confidence" type="number" min="0" max="1"/>
  <field name="suggested_tool" type="string"/>
  <field name="business_insights" type="object"/>
  <field name="workflow_recommendation" type="object"/>
</schema>

<schema id="council_synthesis">
  <field name="id" type="uuid" required="true"/>
  <field name="mode" type="enum" values="consensus,priority-weighted"/>
  <field name="statements" type="array"/>
  <field name="final_recommendation" type="string"/>
  <field name="confidence" type="number"/>
</schema>
```

#### X-Axis: Orchestration Operations

```yaml
# Orchestration Lifecycle
orchestration_start: 0x2000          # Start orchestration session
orchestration_step_execute: 0x2001  # Execute single step
orchestration_synthesize: 0x2002    # Synthesize results
orchestration_complete: 0x2003      # Mark complete

# Agent Deliberation
agent_deliberate_signal: 0x2010     # Signal agent deliberation
agent_deliberate_strategist: 0x2011 # Strategist deliberation
agent_deliberate_operator: 0x2012   # Operator deliberation
agent_council_vote: 0x2013          # Council voting

# Tool Execution
tool_dispatch: 0x2020               # Dispatch tool
tool_big_brain: 0x2021              # Big brain tool
tool_ucats_analyzer: 0x2022         # UCATS analyzer
tool_note_extractor: 0x2023         # Note extractor
tool_execute_with_rag: 0x2024       # Tool with RAG context

# RAG Operations
rag_retrieve: 0x2030                # Retrieve knowledge chunks
rag_filter_by_member: 0x2031        # Filter chunks for council member
rag_rerank: 0x2032                  # Rerank by relevance

# Memory Operations
memory_retrieve: 0x2040             # Retrieve agent memory
memory_store: 0x2041                # Store interaction
memory_search: 0x2042               # Semantic search

# Prompt Parsing
prompt_parse_markdown: 0x2050       # Parse markdown prompt
prompt_parse_latex: 0x2051          # Parse LaTeX
prompt_parse_poml: 0x2052           # Parse POML (Microsoft)
prompt_compile_flow: 0x2053         # Compile to FlowDefinition

# Kernel Compilation
kernel_compile: 0x2060              # Compile session → kernel
kernel_validate: 0x2061             # Validate kernel schema
```

#### Y-Axis: Orchestration Predicates

```xml
<predicates>
  <!-- Access Control -->
  <predicate id="can_start_orchestration">
    <and>
      <is_authenticated left="$token"/>
      <has_session left="$session"/>
    </and>
  </predicate>
  
  <!-- Orchestration Flow -->
  <predicate id="has_more_steps">
    <lt left="$current_step" right="count($orchestration_config.steps)"/>
  </predicate>
  
  <predicate id="step_is_parallel">
    <eq left="$step.parallel" right="true"/>
  </predicate>
  
  <predicate id="requires_rag_context">
    <or>
      <eq left="$step.type" right="tool-execute"/>
      <eq left="$step.type" right="agent-deliberate"/>
    </or>
  </predicate>
  
  <!-- Agent Selection -->
  <predicate id="is_valid_agent">
    <contains left="['signal','strategist','operator']" right="$agent_id"/>
  </predicate>
  
  <predicate id="agent_has_deliberated">
    <exists left="$step_results[$agent_id]"/>
  </predicate>
  
  <!-- Synthesis -->
  <predicate id="can_synthesize">
    <and>
      <gte left="count($step_results)" right="1"/>
      <all_complete left="$required_steps"/>
    </and>
  </predicate>
  
  <predicate id="has_high_confidence">
    <gte left="$deliberation.confidence" right="0.7"/>
  </predicate>
  
  <!-- Tool Routing -->
  <predicate id="tool_exists">
    <exists left="$tool_registry[$tool_name]"/>
  </predicate>
  
  <predicate id="tool_needs_rag">
    <contains left="$tool_config.required_context" right="knowledge"/>
  </predicate>
</predicates>
```

#### Z-Axis: Orchestration Events

```typescript
enum OrchestrationEventType {
  // Session Lifecycle
  ORCHESTRATION_STARTED = "orchestration.started",
  ORCHESTRATION_STEP_EXECUTED = "orchestration.step_executed",
  ORCHESTRATION_COMPLETED = "orchestration.completed",
  ORCHESTRATION_FAILED = "orchestration.failed",
  
  // Agent Events
  AGENT_DELIBERATION_STARTED = "agent.deliberation_started",
  AGENT_DELIBERATION_COMPLETED = "agent.deliberation_completed",
  AGENT_CONFIDENCE_LOW = "agent.confidence_low",
  AGENT_SUGGESTED_TOOL = "agent.suggested_tool",
  
  // Council Events
  COUNCIL_CONSENSUS_REACHED = "council.consensus_reached",
  COUNCIL_VOTE_RECORDED = "council.vote_recorded",
  COUNCIL_SYNTHESIS_COMPLETE = "council.synthesis_complete",
  
  // Tool Events
  TOOL_DISPATCHED = "tool.dispatched",
  TOOL_EXECUTED = "tool.executed",
  TOOL_FAILED = "tool.failed",
  
  // RAG Events
  RAG_RETRIEVED = "rag.retrieved",
  RAG_FILTERED = "rag.filtered",
  RAG_CHUNKS_INJECTED = "rag.chunks_injected",
  
  // Memory Events
  MEMORY_RETRIEVED = "memory.retrieved",
  MEMORY_STORED = "memory.stored",
  MEMORY_SEARCH_EXECUTED = "memory.search_executed",
  
  // Kernel Events
  KERNEL_COMPILED = "kernel.compiled",
  KERNEL_VALIDATED = "kernel.validated"
}
```

---

## PART 2: WORKFLOW EXAMPLES IN XML

### Workflow 1: Business Council Mode (Sequential)

```xml
<workflow id="business_council_orchestration">
  <entry p="orchestration" x="start" node="load_config"/>
  
  <nodes>
    <!-- Step 1: Load orchestration config -->
    <node id="load_config" kind="transform">
      <load_config mode="business-council"/>
    </node>
    
    <!-- Step 2: Compile kernel (session → executable) -->
    <node id="compile_kernel" kind="external" op="0x2060">
      <compile session="$session" config="default-kernel-config"/>
      <event type="kernel.compiled"/>
    </node>
    
    <!-- Step 3: Retrieve memories -->
    <node id="retrieve_memory" kind="external" op="0x2040">
      <query text="$query"/>
      <event type="memory.retrieved"/>
    </node>
    
    <!-- Step 4: Signal Agent Deliberates -->
    <node id="agent_signal" kind="external" op="0x2010">
      <agent_id value="signal"/>
      <context>
        <query value="$query"/>
        <memories value="$retrieved_memories"/>
        <kernel value="$compiled_kernel"/>
      </context>
      <event type="agent.deliberation_completed"/>
    </node>
    
    <!-- Step 5: Strategist Agent Deliberates -->
    <node id="agent_strategist" kind="external" op="0x2011">
      <agent_id value="strategist"/>
      <context>
        <query value="$query"/>
        <previous_deliberations>
          <signal value="$agent_signal.statement"/>
        </previous_deliberations>
      </context>
      <event type="agent.deliberation_completed"/>
    </node>
    
    <!-- Step 6: Operator Agent Deliberates -->
    <node id="agent_operator" kind="external" op="0x2012">
      <agent_id value="operator"/>
      <context>
        <query value="$query"/>
        <previous_deliberations>
          <signal value="$agent_signal.statement"/>
          <strategist value="$agent_strategist.statement"/>
        </previous_deliberations>
      </context>
      <event type="agent.deliberation_completed"/>
    </node>
    
    <!-- Step 7: Tool Execution (if suggested) -->
    <node id="tool_big_brain" kind="external" op="0x2021">
      <when>
        <or>
          <not_null left="$agent_signal.suggested_tool"/>
          <not_null left="$agent_strategist.suggested_tool"/>
        </or>
      </when>
      <tool_args>
        <query value="$query"/>
        <council_deliberations value="$all_agent_statements"/>
      </tool_args>
      <event type="tool.executed"/>
    </node>
    
    <!-- Step 8: Synthesize Consensus -->
    <node id="synthesize" kind="external" op="0x2002">
      <synthesis_mode value="consensus"/>
      <inputs>
        <signal value="$agent_signal"/>
        <strategist value="$agent_strategist"/>
        <operator value="$agent_operator"/>
        <tool_result value="$tool_big_brain"/>
      </inputs>
      <event type="council.synthesis_complete"/>
    </node>
    
    <!-- Step 9: Emit Completion -->
    <node id="complete" kind="external" op="0x2003">
      <event>
        <type>orchestration.completed</type>
        <data>
          <field name="query" value="$query"/>
          <field name="mode" value="business-council"/>
          <field name="final_recommendation" value="$synthesized.recommendation"/>
          <field name="confidence" value="$synthesized.confidence"/>
        </data>
      </event>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_config" to="compile_kernel"><when><always/></when></edge>
    <edge from="compile_kernel" to="retrieve_memory"><when><always/></when></edge>
    <edge from="retrieve_memory" to="agent_signal"><when><always/></when></edge>
    <edge from="agent_signal" to="agent_strategist"><when><always/></when></edge>
    <edge from="agent_strategist" to="agent_operator"><when><always/></when></edge>
    <edge from="agent_operator" to="tool_big_brain">
      <when>
        <or>
          <not_null left="$agent_signal.suggested_tool"/>
          <not_null left="$agent_strategist.suggested_tool"/>
        </or>
      </when>
    </edge>
    <edge from="agent_operator" to="synthesize">
      <when>
        <and>
          <null left="$agent_signal.suggested_tool"/>
          <null left="$agent_strategist.suggested_tool"/>
        </and>
      </when>
    </edge>
    <edge from="tool_big_brain" to="synthesize"><when><always/></when></edge>
    <edge from="synthesize" to="complete"><when><always/></when></edge>
    <edge from="complete" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow 2: Enhanced Mode (Parallel Execution)

```xml
<workflow id="enhanced_orchestration">
  <entry p="orchestration" x="start" node="load_config"/>
  
  <nodes>
    <node id="load_config" kind="transform">
      <load_config mode="enhanced"/>
    </node>
    
    <node id="compile_kernel" kind="external" op="0x2060"/>
    
    <!-- Parallel execution group -->
    <node id="parallel_group" kind="transform">
      <parallel_tasks>
        <!-- All these execute concurrently -->
        <task ref="agent_operator"/>
        <task ref="agent_strategist"/>
        <task ref="agent_signal"/>
        <task ref="tool_ucats"/>
        <task ref="rag_retrieve"/>
      </parallel_tasks>
    </node>
    
    <!-- Parallel task definitions -->
    <node id="agent_operator" kind="external" op="0x2012"/>
    <node id="agent_strategist" kind="external" op="0x2011"/>
    <node id="agent_signal" kind="external" op="0x2010"/>
    <node id="tool_ucats" kind="external" op="0x2022"/>
    <node id="rag_retrieve" kind="external" op="0x2030"/>
    
    <!-- Wait for all parallel tasks -->
    <node id="await_parallel" kind="transform">
      <await_all tasks="$parallel_group"/>
    </node>
    
    <!-- Priority-weighted synthesis -->
    <node id="synthesize" kind="external" op="0x2002">
      <synthesis_mode value="priority-weighted"/>
      <bias_config>
        <signal value="1.2"/>
        <strategist value="1.5"/>
        <operator value="1.0"/>
      </bias_config>
    </node>
    
    <node id="complete" kind="external" op="0x2003"/>
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_config" to="compile_kernel"><when><always/></when></edge>
    <edge from="compile_kernel" to="parallel_group"><when><always/></when></edge>
    <edge from="parallel_group" to="await_parallel"><when><always/></when></edge>
    <edge from="await_parallel" to="synthesize"><when><always/></when></edge>
    <edge from="synthesize" to="complete"><when><always/></when></edge>
    <edge from="complete" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow 3: RAG-First Mode

```xml
<workflow id="rag_first_orchestration">
  <entry p="orchestration" x="start" node="load_config"/>
  
  <nodes>
    <node id="load_config" kind="transform">
      <load_config mode="rag-first"/>
    </node>
    
    <!-- Step 1: RAG Retrieval -->
    <node id="rag_retrieve" kind="external" op="0x2030">
      <query text="$query"/>
      <top_k value="10"/>
      <event type="rag.retrieved"/>
    </node>
    
    <!-- Step 2: Filter chunks for strategist -->
    <node id="rag_filter" kind="external" op="0x2031">
      <council_member value="strategist"/>
      <chunks from="$rag_retrieve.chunks"/>
      <event type="rag.filtered"/>
    </node>
    
    <!-- Step 3: Strategist analyzes with RAG context -->
    <node id="agent_strategist" kind="external" op="0x2011">
      <context>
        <query value="$query"/>
        <rag_chunks value="$rag_filter.filtered_chunks"/>
      </context>
      <event type="agent.deliberation_completed"/>
    </node>
    
    <!-- Step 4: Synthesize (single agent) -->
    <node id="synthesize" kind="external" op="0x2002">
      <synthesis_mode value="consensus"/>
      <inputs>
        <strategist value="$agent_strategist"/>
      </inputs>
    </node>
    
    <node id="complete" kind="external" op="0x2003"/>
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_config" to="rag_retrieve"><when><always/></when></edge>
    <edge from="rag_retrieve" to="rag_filter"><when><always/></when></edge>
    <edge from="rag_filter" to="agent_strategist"><when><always/></when></edge>
    <edge from="agent_strategist" to="synthesize"><when><always/></when></edge>
    <edge from="synthesize" to="complete"><when><always/></when></edge>
    <edge from="complete" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 3: PROMPT PARSING AS PXYZ

### Current System (PromptParser.ts)

Parses multiple formats → FlowDefinition:
- **Markdown**: `# Heading` → agent definition, lists → loops
- **LaTeX**: `$$...$$` → mathematical analysis tool
- **POML**: `<role>`, `<task>` → agent config

### PXYZ Mapping

#### P-Axis: Prompt Entities

```xml
<schema id="prompt_source">
  <field name="id" type="uuid" required="true"/>
  <field name="format" type="enum" values="markdown,latex,poml,mixed"/>
  <field name="content" type="string" required="true"/>
  <field name="parsed_ast" type="object"/>
</schema>

<schema id="flow_definition">
  <field name="id" type="uuid" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="nodes" type="array" required="true"/>
  <field name="connections" type="array"/>
  <field name="triggers" type="array"/>
  <field name="metadata" type="object"/>
</schema>
```

#### Workflow: Parse Markdown Prompt

```xml
<workflow id="prompt_parse_markdown">
  <entry p="prompt" x="parse" node="detect_format"/>
  
  <nodes>
    <node id="detect_format" kind="transform">
      <classify content="$input.content">
        <if contains="<poml>" then="poml"/>
        <if contains="$$" then="latex"/>
        <else value="markdown"/>
      </classify>
    </node>
    
    <node id="parse_markdown" kind="external" op="0x2050">
      <when>
        <eq left="$detected_format" right="markdown"/>
      </when>
      <parse_rules>
        <heading level="1" maps_to="agent_definition" role="main"/>
        <heading level="2" maps_to="agent_definition" role="helper"/>
        <list ordered="true" maps_to="loop_construct"/>
        <list ordered="false" maps_to="condition_check"/>
        <code_block maps_to="tool_execution"/>
        <bold maps_to="type_annotation"/>
        <italic maps_to="return_value"/>
      </parse_rules>
      <event type="prompt.parsed"/>
    </node>
    
    <node id="compile_flow" kind="external" op="0x2053">
      <ast from="$parse_markdown.ast"/>
      <generate_flow>
        <nodes from="$ast.children"/>
        <connections from="$ast.control_flow"/>
      </generate_flow>
      <event type="prompt.flow_compiled"/>
    </node>
    
    <node id="validate_flow" kind="external" op="0x2061">
      <validate schema="flow_definition_schema"/>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="detect_format" to="parse_markdown">
      <when><eq left="$detected_format" right="markdown"/></when>
    </edge>
    <edge from="parse_markdown" to="compile_flow"><when><always/></when></edge>
    <edge from="compile_flow" to="validate_flow"><when><always/></when></edge>
    <edge from="validate_flow" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## PART 4: IO ADAPTER INTEGRATION

### Current Operations → IO Handler Mapping

```typescript
// io-browser.ts additions for agent system

const agentOperationHandlers = {
  // Orchestration
  0x2000: async (ctx) => {
    // orchestration_start
    const session = await createSession(ctx.input);
    const config = await loadOrchestrationConfig(ctx.input.mode);
    return { session, config };
  },
  
  0x2001: async (ctx) => {
    // orchestration_step_execute
    const step = ctx.input.step;
    const result = await executeStep(step, ctx);
    return { result };
  },
  
  0x2002: async (ctx) => {
    // orchestration_synthesize
    const mode = ctx.input.synthesis_mode;
    const statements = ctx.input.agent_statements;
    const synthesized = await synthesizeResults(statements, mode);
    return { recommendation: synthesized };
  },
  
  // Agent Deliberation
  0x2010: async (ctx) => {
    // agent_deliberate_signal
    const response = await anthropicAPI.messages.create({
      model: "claude-sonnet-4-20250514",
      system: SIGNAL_AGENT_SYSTEM_PROMPT,
      messages: [{ role: "user", content: ctx.input.query }]
    });
    return {
      agent_id: "signal",
      statement: response.content[0].text,
      confidence: 0.85
    };
  },
  
  0x2011: async (ctx) => {
    // agent_deliberate_strategist
    const response = await anthropicAPI.messages.create({
      model: "claude-sonnet-4-20250514",
      system: STRATEGIST_AGENT_SYSTEM_PROMPT,
      messages: buildDeliberationHistory(ctx.input.previous_deliberations)
    });
    return {
      agent_id: "strategist",
      statement: response.content[0].text,
      confidence: 0.90
    };
  },
  
  // RAG Operations
  0x2030: async (ctx) => {
    // rag_retrieve
    const chunks = await qdrant.search({
      collection: "business_knowledge",
      query: ctx.input.query,
      limit: ctx.input.top_k || 10
    });
    return { chunks };
  },
  
  0x2031: async (ctx) => {
    // rag_filter_by_member
    const filtered = ctx.input.chunks.filter(chunk =>
      chunk.councilMemberRelevance[ctx.input.council_member] > 0.5
    );
    return { filtered_chunks: filtered };
  },
  
  // Prompt Parsing
  0x2050: async (ctx) => {
    // prompt_parse_markdown
    const ast = parseMarkdown(ctx.input.content);
    return { ast };
  },
  
  0x2053: async (ctx) => {
    // prompt_compile_flow
    const flow = compileASTToFlow(ctx.input.ast);
    return { flow };
  }
};
```

---

## PART 5: WHAT MAKES IT ALREADY PXYZ-NATIVE

### ✅ Config-Driven Workflows

**Current**:
```typescript
const businessCouncilMode = {
  steps: [
    { type: "agent-deliberate", agentId: "signal" },
    { type: "agent-deliberate", agentId: "strategist" },
    ...
  ]
};
```

**Why it's PXYZ**: Workflow defined as **data**, not code. This IS the XML workflow graph!

### ✅ Event-Native Architecture

**Current**:
```typescript
yield* _(eventBus.emit({
  type: "OrchestrationComplete",
  entityType: "AgentHarness",
  pxyz: resultPxyz,
  data: { ... }
}));
```

**Why it's PXYZ**: Every state change appends event. State = projection over events.

### ✅ PXYZ Coordinates Everywhere

**Current**:
```typescript
const startPxyz = createPXYZ(
  toEntityName("AgentHarness"),
  toOperationName("DELIBERATE"),
  toEntityName("UserQuery"),
  createISODateTime()
);
```

**Why it's PXYZ**: Every operation has (P, X, Y, Z) coordinates for tracing.

### ✅ Zero Local Types

**Current**:
```typescript
// NO custom types - everything uses PxyzJson
const runAgentDeliberation = (
  agentId: string,
  query: EntityName,
  sessionData: PxyzJson,  // <-- Universal type
  previousResults: PxyzJson
): Effect.Effect<PxyzJson, Error, Database> => { ... }
```

**Why it's PXYZ**: Entity<T> universality. No domain-specific types.

### ✅ Effect-TS Composability

**Current**:
```typescript
const orchestrate = (...) => Effect.gen(function* (_) {
  const kernel = yield* _(compileKernel(...));
  const steps = yield* _(executeSteps(...));
  const result = yield* _(synthesize(...));
  return result;
});
```

**Why it's PXYZ**: Operations compose like graph edges. Pure Effect chain.

---

## PART 6: HOW TO MAKE IT 100% PXYZ

### Change 1: Replace TypeScript Config with XML Workflows

**Before** (Orchestration.ts):
```typescript
export const businessCouncilMode: PxyzJson = {
  steps: [
    { type: "agent-deliberate", agentId: "signal" },
    ...
  ]
};
```

**After** (workflow.xml):
```xml
<workflow id="business_council_mode">
  <entry p="orchestration" x="start" node="agent_signal"/>
  <nodes>
    <node id="agent_signal" kind="external" op="0x2010"/>
    <node id="agent_strategist" kind="external" op="0x2011"/>
    ...
  </nodes>
  <edges>...</edges>
</workflow>
```

Compile to `graph.bin` → Load in WASM runtime.

### Change 2: Replace executeStep Dispatch with Graph Traversal

**Before** (AgentHarness.ts):
```typescript
const stepHandlers: Record<string, () => Effect> = {
  "agent-deliberate": () => runAgentDeliberation(...),
  "tool-execute": () => dispatchTool(...),
  ...
};
```

**After** (WASM runtime):
```wat
(func $execute_node (param $node_id i32) (result i32)
  ;; Load node from graph
  (local.get $node_id)
  (call $get_node)
  
  ;; Dispatch by node kind
  (i32.load8_u offset=4) ;; node.kind
  (call $dispatch_external_op) ;; Uses op code
)
```

Node kind + op code → IO adapter dispatch.

### Change 3: Move Agent Prompts to Templates

**Before** (Hardcoded in BusinessCouncil.ts):
```typescript
const SIGNAL_PROMPT = `You are Signal, the visionary...`;
```

**After** (workflow.xml):
```xml
<templates>
  <template id="signal_agent_prompt">
    <![CDATA[
    You are Signal, the visionary member of the Business Council.
    
    Context:
    {{context.query}}
    
    Previous deliberations:
    {{#each context.previous_deliberations}}
    - {{member}}: {{statement}}
    {{/each}}
    
    Provide your visionary perspective.
    ]]>
  </template>
</templates>
```

### Change 4: Formalize Synthesis as Predicate Evaluation

**Before** (AgentHarness.ts):
```typescript
const synthesisStrategies = {
  "consensus": () => {
    const statements = agents.map(a => a.statement);
    return statements.join(" → ");
  }
};
```

**After** (Predicates + Transform):
```xml
<node id="synthesize_consensus" kind="transform">
  <compute>
    <var name="all_statements" value="collect($step_results, 'statement')"/>
    <var name="avg_confidence" value="avg($step_results, 'confidence')"/>
    <var name="consensus" value="join($all_statements, ' → ')"/>
  </compute>
  <when>
    <gte left="$avg_confidence" right="0.7"/>
  </when>
</node>
```

---

## PART 7: COMPLETE OPERATION REGISTRY

### Agent Orchestration (0x2000-0x206F)

| Operation | Code | Description |
|-----------|------|-------------|
| orchestration_start | 0x2000 | Initialize orchestration session |
| orchestration_step_execute | 0x2001 | Execute workflow step |
| orchestration_synthesize | 0x2002 | Synthesize agent results |
| orchestration_complete | 0x2003 | Mark orchestration complete |
| agent_deliberate_signal | 0x2010 | Signal agent deliberation |
| agent_deliberate_strategist | 0x2011 | Strategist deliberation |
| agent_deliberate_operator | 0x2012 | Operator deliberation |
| agent_council_vote | 0x2013 | Council voting |
| tool_dispatch | 0x2020 | Generic tool dispatch |
| tool_big_brain | 0x2021 | Big brain analysis |
| tool_ucats_analyzer | 0x2022 | UCATS analysis |
| tool_note_extractor | 0x2023 | Extract notes |
| tool_execute_with_rag | 0x2024 | Tool with RAG context |
| rag_retrieve | 0x2030 | Retrieve knowledge |
| rag_filter_by_member | 0x2031 | Filter for council member |
| rag_rerank | 0x2032 | Rerank by relevance |
| memory_retrieve | 0x2040 | Retrieve memories |
| memory_store | 0x2041 | Store interaction |
| memory_search | 0x2042 | Semantic search |
| prompt_parse_markdown | 0x2050 | Parse markdown |
| prompt_parse_latex | 0x2051 | Parse LaTeX |
| prompt_parse_poml | 0x2052 | Parse POML |
| prompt_compile_flow | 0x2053 | Compile to flow |
| kernel_compile | 0x2060 | Compile kernel |
| kernel_validate | 0x2061 | Validate schema |

**Total**: 26 operations

---

## PART 8: INTEGRATION WITH EXISTING OMAR

### How Agent System Fits in CRM

Agents enhance every PXYZ operation:

```xml
<!-- Contact health scoring enhanced by strategist -->
<workflow id="contact_health_score_enhanced">
  <entry p="contact" x="health_score" node="load_contact"/>
  
  <nodes>
    <node id="load_contact" kind="external" op="0x0101"/>
    
    <!-- Traditional calculation -->
    <node id="calculate_baseline" kind="transform">
      <compute health_score="..."/>
    </node>
    
    <!-- AI enhancement via orchestration -->
    <node id="agent_analyze" kind="external" op="0x2011">
      <agent_id value="strategist"/>
      <context>
        <contact value="$contact"/>
        <baseline_score value="$calculate_baseline.health_score"/>
        <interaction_history value="$contact.timeline"/>
      </context>
    </node>
    
    <node id="synthesize_score" kind="transform">
      <compute>
        <var name="final_score" value="
          ($baseline_score * 0.6) + 
          ($agent_analyze.confidence * $agent_analyze.suggested_score * 0.4)
        "/>
      </compute>
    </node>
    
    <node id="success" kind="terminal"/>
  </nodes>
</workflow>
```

### Agent-Enhanced Operations Across Domains

**Contacts**: Cooling detection, re-engagement suggestions  
**Tasks**: Workload analysis, delegation recommendations  
**Workflows**: Bottleneck detection, optimization  
**Email**: Sentiment analysis, draft generation  
**Documents**: Summarization, extraction  

All use orchestration operation codes (0x2000-0x206F).

---

## PART 9: ARCHITECTURAL BENEFITS

### Before: 11 Orchestrator Files (~3,000 lines)

```
AgentOrchestrator.ts
BusinessOrchestrator.ts
BusinessPersonality.ts
CommunicationStyleManager.ts
EnhancedOrchestrator.ts
ExecutorRouter.ts
Orchestrator.ts
BusinessCouncilOrchestrator.ts
BusinessKernelCompiler.ts
KernelInputBuilder.ts
Compiler.ts
```

### After: 1 AgentHarness + XML Configs (~700 lines)

```
AgentHarness.ts (200 lines)
orchestration-modes.xml (5 workflows)
agent-prompts.xml (templates)
io-adapter.ts (26 handlers)
```

**Reduction**: ~75% less code  
**Auditability**: All logic in XML  
**Flexibility**: Change orchestration without touching code

---

## PART 10: NEXT STEPS

### Immediate Actions

1. **Assign Operation Codes**
   - ✅ Defined 0x2000-0x206F range
   - Add to complete operation registry
   
2. **Convert TypeScript Configs → XML Workflows**
   - `businessCouncilMode` → `business-council.xml`
   - `enhancedMode` → `enhanced-mode.xml`
   - etc.

3. **Implement IO Adapter Handlers**
   - Add 26 agent operation handlers
   - Integrate with Anthropic API
   - Connect to RAG system

4. **Formalize Agent Prompts as Templates**
   - Extract from BusinessCouncil.ts
   - Move to XML `<templates>`
   - Support Handlebars interpolation

5. **Test Orchestration Graph Execution**
   - Compile XML → graph.bin
   - Execute via WASM runtime
   - Verify event log

### Future Enhancements

- **Multi-model support**: Route to GPT-4, Gemini, local models
- **Agent marketplace**: Shareable agent configs
- **A/B testing**: Compare orchestration modes
- **Observability**: Real-time orchestration visualization

---

## SUMMARY

Your agent orchestration system is **already ~80% PXYZ-compliant**:

✅ **Config-driven**: Orchestration modes as data  
✅ **Event-native**: EventBus for all state changes  
✅ **PXYZ coordinates**: Tracing everywhere  
✅ **Zero local types**: PxyzJson universality  
✅ **Effect composition**: Pure functional operations  

**What remains**:
- Assign operation codes (0x2000-0x206F) ✅ Done
- Convert TS configs → XML workflows
- Implement IO adapter handlers
- Move agent prompts to templates

**Result**: 700-line auditable orchestration system replacing 3,000 lines of imperative code.

The agents are just **external operations** in the graph. The council is a **workflow**. Synthesis is a **predicate-driven transform**. It's all PXYZ.
