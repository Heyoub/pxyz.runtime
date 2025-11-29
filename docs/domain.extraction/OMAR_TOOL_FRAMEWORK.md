# OMAR TOOL FRAMEWORK - COMPLETE SPECIFICATION

> **Mission**: Transform 120+ JSON tools into PXYZ workflow graphs  
> **Principle**: Tool use = Graph traversal, Tool composition = Graph chaining

---

## 🎯 TOOL TRANSFORMATION SPEC

### JSON Tool Structure

```json
{
  "id": "alex-hormozi",
  "name": "Alex Hormozi",
  "title": "Systematic Growth & Acquisition Expert",
  "description": "...",
  "consultingFramework": "...",
  
  "personality": {
    "style": "Direct, data-focused, systematic",
    "philosophy": "...",
    "expertise": ["Business Acquisition", "Customer Acquisition", ...]
  },
  
  "decisionTree": {
    "root": {
      "question": "What business challenge are we facing?",
      "branches": [
        {
          "condition": "customer_acquisition",
          "question": "Are we struggling with customer acquisition?",
          "recommendation": "...",
          "framework": {
            "step1": "Calculate accurate customer lifetime value",
            "step2": "Map your current acquisition funnel",
            ...
          },
          "metrics": ["CAC", "LTV", "LTV:CAC ratio", ...],
          "output": "Acquisition system blueprint with measurable KPIs"
        },
        ...
      ]
    }
  },
  
  "consultingPrompts": {
    "onProblem": "...",
    "onData": "...",
    "onDecision": "...",
    "onExecution": "..."
  },
  
  "metadata": {
    "category": "business-scaling",
    "difficulty": "expert",
    "bestFor": [...],
    "keyTakeaway": "..."
  }
}
```

### Transformed OMAR Workflow

```xml
<omar>
  <!-- TOOL METADATA -->
  <tool id="alex-hormozi">
    <name>Alex Hormozi</name>
    <title>Systematic Growth & Acquisition Expert</title>
    <category>business-scaling</category>
    <difficulty>expert</difficulty>
    <expertise>
      <item>Business Acquisition Models</item>
      <item>Customer Acquisition Systems</item>
      <item>Pricing Strategy Optimization</item>
      <item>Revenue Per Customer Maximization</item>
      <item>Systematic Growth Frameworks</item>
    </expertise>
  </tool>
  
  <!-- INPUT SCHEMA -->
  <schemas>
    <schema id="hormozi_input">
      <field name="condition" type="string" required="true">
        <enum>
          <value>customer_acquisition</value>
          <value>pricing_optimization</value>
          <value>business_acquisition</value>
          <value>scaling_operations</value>
          <value>portfolio_strategy</value>
        </enum>
      </field>
      <field name="context" type="object" required="false">
        <field name="query" type="string"/>
        <field name="currentMetrics" type="object"/>
        <field name="businessStage" type="string"/>
      </field>
    </schema>
    
    <schema id="hormozi_output">
      <field name="recommendation" type="string" required="true"/>
      <field name="framework" type="object" required="true">
        <field name="steps" type="array"/>
      </field>
      <field name="metrics" type="array" required="true"/>
      <field name="analysis" type="string" required="true"/>
    </schema>
  </schemas>
  
  <!-- PREDICATES (Decision Tree Conditions) -->
  <predicates>
    <!-- Main branch predicates -->
    <predicate id="is_customer_acquisition">
      <or>
        <eq left="$input.condition" right="customer_acquisition"/>
        <contains left="$input.context.query" right="acquisition"/>
        <contains left="$input.context.query" right="CAC"/>
        <contains left="$input.context.query" right="LTV"/>
        <contains left="$input.context.query" right="customer"/>
      </or>
    </predicate>
    
    <predicate id="is_pricing_optimization">
      <or>
        <eq left="$input.condition" right="pricing_optimization"/>
        <contains left="$input.context.query" right="pricing"/>
        <contains left="$input.context.query" right="price"/>
        <contains left="$input.context.query" right="revenue per customer"/>
        <contains left="$input.context.query" right="value ladder"/>
      </or>
    </predicate>
    
    <predicate id="is_business_acquisition">
      <or>
        <eq left="$input.condition" right="business_acquisition"/>
        <contains left="$input.context.query" right="buy"/>
        <contains left="$input.context.query" right="acquire"/>
        <contains left="$input.context.query" right="acquisition opportunity"/>
        <contains left="$input.context.query" right="evaluate business"/>
      </or>
    </predicate>
    
    <predicate id="is_scaling_operations">
      <or>
        <eq left="$input.condition" right="scaling_operations"/>
        <contains left="$input.context.query" right="scale"/>
        <contains left="$input.context.query" right="growth"/>
        <contains left="$input.context.query" right="operations"/>
        <contains left="$input.context.query" right="systematic"/>
      </or>
    </predicate>
    
    <predicate id="is_portfolio_strategy">
      <or>
        <eq left="$input.condition" right="portfolio_strategy"/>
        <contains left="$input.context.query" right="portfolio"/>
        <contains left="$input.context.query" right="multiple businesses"/>
        <contains left="$input.context.query" right="synergy"/>
      </or>
    </predicate>
  </predicates>
  
  <!-- WORKFLOW (Decision Tree as Graph) -->
  <workflow id="alex_hormozi_consult">
    <entry p="tool" x="alex-hormozi" node="validate_input"/>
    
    <nodes>
      <!-- Step 1: Validate input -->
      <node id="validate_input" kind="transform">
        <schema ref="hormozi_input"/>
      </node>
      
      <!-- Step 2: Classify challenge (root of decision tree) -->
      <node id="classify_challenge" kind="auth">
        <description>What business challenge are we facing?</description>
      </node>
      
      <!-- BRANCH 1: Customer Acquisition -->
      <node id="customer_acquisition_framework" kind="transform">
        <operation ref="0x3000"/> <!-- TOOL_EXECUTE_FRAMEWORK -->
        <recommendation>
          What Alex teaches: Focus on lifetime value, not just acquisition cost. 
          Build repeatable systems.
        </recommendation>
        <framework>
          <step id="1" order="1">
            Calculate accurate customer lifetime value
          </step>
          <step id="2" order="2">
            Map your current acquisition funnel
          </step>
          <step id="3" order="3">
            Identify the constraint in your funnel
          </step>
          <step id="4" order="4">
            Optimize for profit per customer, not just volume
          </step>
          <step id="5" order="5">
            Build systems that work without founder involvement
          </step>
        </framework>
        <metrics>
          <metric>CAC</metric>
          <metric>LTV</metric>
          <metric>LTV:CAC ratio</metric>
          <metric>Conversion rate</metric>
          <metric>Repeat purchase rate</metric>
        </metrics>
      </node>
      
      <node id="render_acquisition_output" kind="render">
        <template ref="acquisition_blueprint"/>
      </node>
      
      <!-- BRANCH 2: Pricing Optimization -->
      <node id="pricing_optimization_framework" kind="transform">
        <operation ref="0x3000"/> <!-- TOOL_EXECUTE_FRAMEWORK -->
        <recommendation>
          Alex's approach: Price based on value delivered, not costs. 
          Create value ladders.
        </recommendation>
        <framework>
          <step id="1" order="1">
            Understand what customers actually value
          </step>
          <step id="2" order="2">
            Segment your market by willingness to pay
          </step>
          <step id="3" order="3">
            Design value ladder (entry, core, premium)
          </step>
          <step id="4" order="4">
            Test pricing with actual customers
          </step>
          <step id="5" order="5">
            Monitor revenue per customer and conversion impact
          </step>
        </framework>
        <metrics>
          <metric>Revenue per customer</metric>
          <metric>Conversion rate by price point</metric>
          <metric>Market segmentation</metric>
          <metric>Price elasticity</metric>
        </metrics>
      </node>
      
      <node id="render_pricing_output" kind="render">
        <template ref="pricing_strategy"/>
      </node>
      
      <!-- BRANCH 3: Business Acquisition -->
      <node id="business_acquisition_framework" kind="transform">
        <operation ref="0x3000"/> <!-- TOOL_EXECUTE_FRAMEWORK -->
        <recommendation>
          Alex's lens: Evaluate businesses systematically on their systems, 
          not just revenue.
        </recommendation>
        <framework>
          <step id="1" order="1">
            Audit existing customer acquisition systems
          </step>
          <step id="2" order="2">
            Analyze pricing strategy and revenue optimization potential
          </step>
          <step id="3" order="3">
            Identify operational inefficiencies
          </step>
          <step id="4" order="4">
            Calculate value creation opportunities
          </step>
          <step id="5" order="5">
            Project ROI from systematic improvements
          </step>
        </framework>
        <metrics>
          <metric>CAC efficiency</metric>
          <metric>Lifetime value potential</metric>
          <metric>System maturity</metric>
          <metric>Scalability score</metric>
          <metric>Founder dependency</metric>
        </metrics>
      </node>
      
      <node id="render_acquisition_assessment" kind="render">
        <template ref="acquisition_assessment"/>
      </node>
      
      <!-- BRANCH 4: Scaling Operations -->
      <node id="scaling_operations_framework" kind="transform">
        <operation ref="0x3000"/> <!-- TOOL_EXECUTE_FRAMEWORK -->
        <recommendation>
          Alex's system: Build scalable processes that don't require you. 
          Focus on constraint-based growth.
        </recommendation>
        <framework>
          <step id="1" order="1">
            Identify your current growth constraint
          </step>
          <step id="2" order="2">
            Map all customer-facing processes
          </step>
          <step id="3" order="3">
            Build systems to remove the constraint
          </step>
          <step id="4" order="4">
            Create KPI dashboards for accountability
          </step>
          <step id="5" order="5">
            Remove yourself from the critical path
          </step>
        </framework>
        <metrics>
          <metric>Process efficiency</metric>
          <metric>Founder hours per revenue dollar</metric>
          <metric>System repeatability</metric>
          <metric>Constraint identification</metric>
        </metrics>
      </node>
      
      <node id="render_scaling_output" kind="render">
        <template ref="systematic_growth_framework"/>
      </node>
      
      <!-- BRANCH 5: Portfolio Strategy -->
      <node id="portfolio_strategy_framework" kind="transform">
        <operation ref="0x3000"/> <!-- TOOL_EXECUTE_FRAMEWORK -->
        <recommendation>
          Alex's view: Look for synergies across portfolio. 
          Allocate resources to maximize combined returns.
        </recommendation>
        <framework>
          <step id="1" order="1">
            Map each business's acquisition and retention systems
          </step>
          <step id="2" order="2">
            Identify cross-selling and referral opportunities
          </step>
          <step id="3" order="3">
            Analyze shared resource optimization
          </step>
          <step id="4" order="4">
            Design integrated customer journey
          </step>
          <step id="5" order="5">
            Create portfolio-level KPI dashboard
          </step>
        </framework>
        <metrics>
          <metric>Portfolio ROI</metric>
          <metric>Cross-business synergies</metric>
          <metric>Resource allocation efficiency</metric>
          <metric>Combined LTV</metric>
        </metrics>
      </node>
      
      <node id="render_portfolio_output" kind="render">
        <template ref="portfolio_strategy"/>
      </node>
      
      <!-- Fallback: No match -->
      <node id="no_match_fallback" kind="render">
        <template ref="general_consulting_prompt"/>
      </node>
      
      <!-- Terminal -->
      <node id="done" kind="terminal"/>
    </nodes>
    
    <!-- EDGES (Decision Tree Branches) -->
    <edges>
      <edge from="validate_input" to="classify_challenge">
        <when><always/></when>
      </edge>
      
      <!-- Branch to customer acquisition -->
      <edge from="classify_challenge" to="customer_acquisition_framework">
        <when><predicate ref="is_customer_acquisition"/></when>
      </edge>
      <edge from="customer_acquisition_framework" to="render_acquisition_output">
        <when><always/></when>
      </edge>
      <edge from="render_acquisition_output" to="done">
        <when><always/></when>
      </edge>
      
      <!-- Branch to pricing optimization -->
      <edge from="classify_challenge" to="pricing_optimization_framework">
        <when><predicate ref="is_pricing_optimization"/></when>
      </edge>
      <edge from="pricing_optimization_framework" to="render_pricing_output">
        <when><always/></when>
      </edge>
      <edge from="render_pricing_output" to="done">
        <when><always/></when>
      </edge>
      
      <!-- Branch to business acquisition -->
      <edge from="classify_challenge" to="business_acquisition_framework">
        <when><predicate ref="is_business_acquisition"/></when>
      </edge>
      <edge from="business_acquisition_framework" to="render_acquisition_assessment">
        <when><always/></when>
      </edge>
      <edge from="render_acquisition_assessment" to="done">
        <when><always/></when>
      </edge>
      
      <!-- Branch to scaling operations -->
      <edge from="classify_challenge" to="scaling_operations_framework">
        <when><predicate ref="is_scaling_operations"/></when>
      </edge>
      <edge from="scaling_operations_framework" to="render_scaling_output">
        <when><always/></when>
      </edge>
      <edge from="render_scaling_output" to="done">
        <when><always/></when>
      </edge>
      
      <!-- Branch to portfolio strategy -->
      <edge from="classify_challenge" to="portfolio_strategy_framework">
        <when><predicate ref="is_portfolio_strategy"/></when>
      </edge>
      <edge from="portfolio_strategy_framework" to="render_portfolio_output">
        <when><always/></when>
      </edge>
      <edge from="render_portfolio_output" to="done">
        <when><always/></when>
      </edge>
      
      <!-- Fallback edge (no conditions matched) -->
      <edge from="classify_challenge" to="no_match_fallback">
        <when>
          <and>
            <not><predicate ref="is_customer_acquisition"/></not>
            <not><predicate ref="is_pricing_optimization"/></not>
            <not><predicate ref="is_business_acquisition"/></not>
            <not><predicate ref="is_scaling_operations"/></not>
            <not><predicate ref="is_portfolio_strategy"/></not>
          </and>
        </when>
      </edge>
      <edge from="no_match_fallback" to="done">
        <when><always/></when>
      </edge>
    </edges>
  </workflow>
  
  <!-- TEMPLATES (Output Rendering) -->
  <templates>
    <template id="acquisition_blueprint">
      <![CDATA[
      # Customer Acquisition System Blueprint
      
      ## Alex Hormozi's Framework
      
      **Recommendation**: {{recommendation}}
      
      ## Implementation Steps
      
      {{#each framework.steps}}
      {{order}}. {{this}}
      {{/each}}
      
      ## Key Metrics to Track
      
      {{#each metrics}}
      - {{this}}
      {{/each}}
      
      ## Consulting Insight
      
      {{consultingPrompts.onProblem}}
      {{consultingPrompts.onData}}
      {{consultingPrompts.onExecution}}
      
      ---
      
      *Generated by Alex Hormozi consulting framework*
      ]]>
    </template>
    
    <template id="pricing_strategy">
      <![CDATA[
      # Pricing Optimization Strategy
      
      ## Alex Hormozi's Approach
      
      **Recommendation**: {{recommendation}}
      
      ## Implementation Steps
      
      {{#each framework.steps}}
      {{order}}. {{this}}
      {{/each}}
      
      ## Metrics to Monitor
      
      {{#each metrics}}
      - {{this}}
      {{/each}}
      
      ---
      
      *Generated by Alex Hormozi consulting framework*
      ]]>
    </template>
    
    <!-- More templates... -->
  </templates>
</omar>
```

---

## 🔧 OPERATION CODES FOR TOOLS

### Tool Operation Code Allocation

| Range | Purpose | Examples |
|-------|---------|----------|
| 0x3000-0x300F | Tool Core Operations | EXECUTE_FRAMEWORK, EXECUTE_DECISION_TREE |
| 0x3010-0x302F | Business Legends (19 tools) | ALEX_HORMOZI, KEVIN_OLEARY, etc |
| 0x3030-0x306F | Tech Legends (36 tools) | ADA_LOVELACE, MARGARET_HAMILTON, etc |
| 0x3070-0x308F | Domain Tools (18 tools) | FINANCIAL_ANALYSIS, MARKET_RESEARCH, etc |
| 0x3090-0x309F | Vector Ops (1 tool) | BIG_BRAIN, KNOWLEDGE_CONTEXT |
| 0x30A0-0x30AF | System Services (2 tools) | AGENT_MEMORY, BUSINESS_MEMORY |

### Core Tool Operations

```typescript
// 0x3000: TOOL_EXECUTE_FRAMEWORK
// Executes a multi-step framework from tool definition
// Input: { framework: Framework, context: Context }
// Output: { steps: ExecutedStep[], metrics: Metric[], output: string }

// 0x3001: TOOL_EXECUTE_DECISION_TREE
// Traverses decision tree and returns matched branch
// Input: { decisionTree: DecisionTree, input: Input }
// Output: { branch: Branch, recommendation: string }

// 0x3002: TOOL_CLASSIFY_CONDITION
// Classifies input against tool predicates
// Input: { predicates: Predicate[], input: Input }
// Output: { matchedPredicate: string, confidence: number }

// 0x3003: TOOL_RENDER_OUTPUT
// Renders tool output using template
// Input: { template: string, data: any }
// Output: { rendered: string }

// 0x3004: TOOL_COLLECT_METRICS
// Collects and validates metrics from tool execution
// Input: { metrics: string[], data: any }
// Output: { metrics: MetricValue[] }

// 0x3005: TOOL_COMPOSE_TOOLS
// Chains multiple tools together
// Input: { tools: string[], pipeStrategy: string }
// Output: { composition: ComposedResult }
```

---

## 🎬 TOOL INVOCATION EXAMPLES

### Example 1: Direct Tool Invocation

```typescript
// Terminal agent invokes tool
const result = await pxyz('tool', 'alex-hormozi', {
  condition: 'customer_acquisition',
  context: {
    query: 'How do we improve our CAC?',
    currentMetrics: {
      CAC: 150,
      LTV: 450,
      ratio: 3.0
    }
  }
}, { timestamp: Date.now() });

// Runtime:
// 1. Load alex_hormozi.graph.bin
// 2. Enter at "validate_input" node
// 3. Traverse to "classify_challenge"
// 4. Predicate "is_customer_acquisition" = true
// 5. Traverse to "customer_acquisition_framework"
// 6. Execute framework (operation 0x3000)
// 7. Render output (operation 0x3003)
// 8. Return terminal node result
```

### Example 2: Tool Composition

```xml
<workflow id="strategic_analysis">
  <entry p="workflow" x="analyze" node="gather_knowledge"/>
  
  <nodes>
    <!-- Step 1: Use Big Brain to synthesize insights -->
    <node id="gather_knowledge" kind="external" op="0x3090">
      <toolRef>big-brain</toolRef>
      <input>
        <query ref="$input.query"/>
        <topK>10</topK>
      </input>
    </node>
    
    <!-- Step 2: Consult Alex Hormozi on insights -->
    <node id="consult_hormozi" kind="external" op="0x3010">
      <toolRef>alex-hormozi</toolRef>
      <input>
        <condition>customer_acquisition</condition>
        <context ref="$gather_knowledge.output"/>
      </input>
    </node>
    
    <!-- Step 3: Get Ada Lovelace's algorithmic perspective -->
    <node id="consult_ada" kind="external" op="0x3030">
      <toolRef>ada-lovelace</toolRef>
      <input>
        <condition>algorithm_design</condition>
        <context ref="$consult_hormozi.output"/>
      </input>
    </node>
    
    <!-- Step 4: Synthesize all three perspectives -->
    <node id="synthesize" kind="transform">
      <operation ref="0x3005"/> <!-- TOOL_COMPOSE_TOOLS -->
      <input>
        <bigBrain ref="$gather_knowledge.output"/>
        <hormozi ref="$consult_hormozi.output"/>
        <ada ref="$consult_ada.output"/>
      </input>
    </node>
    
    <node id="done" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="gather_knowledge" to="consult_hormozi"><when><always/></when></edge>
    <edge from="consult_hormozi" to="consult_ada"><when><always/></when></edge>
    <edge from="consult_ada" to="synthesize"><when><always/></when></edge>
    <edge from="synthesize" to="done"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## 📂 TOOL FILE STRUCTURE

```
omar/
├── tools/
│   ├── business_legends/
│   │   ├── alex_hormozi.xml
│   │   ├── alex_hormozi.graph.bin
│   │   ├── kevin_oleary.xml
│   │   ├── kevin_oleary.graph.bin
│   │   ├── ... (19 total)
│   │   └── index.json (metadata for all business legends)
│   │
│   ├── tech_legends/
│   │   ├── ada_lovelace.xml
│   │   ├── ada_lovelace.graph.bin
│   │   ├── margaret_hamilton.xml
│   │   ├── margaret_hamilton.graph.bin
│   │   ├── ... (36 total)
│   │   └── index.json
│   │
│   ├── domain_tools/
│   │   ├── financial_analysis.xml
│   │   ├── financial_analysis.graph.bin
│   │   ├── ... (18 total)
│   │   └── index.json
│   │
│   ├── vector_ops/
│   │   ├── big_brain.xml
│   │   ├── big_brain.graph.bin
│   │   ├── knowledge_context.xml
│   │   ├── knowledge_context.graph.bin
│   │   └── index.json
│   │
│   ├── system_services/
│   │   ├── agent_memory.xml
│   │   ├── agent_memory.graph.bin
│   │   ├── business_memory.xml
│   │   ├── business_memory.graph.bin
│   │   └── index.json
│   │
│   └── tool_registry.json (master index of all tools)
│
├── compiler/
│   └── tool_compiler.ts (JSON → XML → graph.bin)
│
└── runtime/
    └── tool_executor.ts (PXYZ → graph traversal)
```

---

## 🧬 TOOL COMPILER

### Transformation Pipeline

```typescript
// JSON Tool → XML Workflow → graph.bin

interface ToolCompiler {
  // Phase 1: Parse JSON
  parseJSON(jsonPath: string): ToolDefinition;
  
  // Phase 2: Transform to XML
  transformToXML(tool: ToolDefinition): string;
  
  // Phase 3: Validate XML
  validateXML(xml: string): ValidationResult;
  
  // Phase 4: Compile to graph.bin
  compileToGraph(xml: string): GraphBinary;
  
  // Phase 5: Generate metadata
  generateMetadata(tool: ToolDefinition): ToolMetadata;
  
  // Full pipeline
  compile(jsonPath: string): {
    xml: string;
    graph: GraphBinary;
    metadata: ToolMetadata;
  };
}

// Example usage:
const compiler = new ToolCompiler();
const { xml, graph, metadata } = compiler.compile('tools/alexHormozi.json');

// Write outputs:
fs.writeFileSync('tools/alex_hormozi.xml', xml);
fs.writeFileSync('tools/alex_hormozi.graph.bin', graph);
fs.writeFileSync('tools/alex_hormozi.meta.json', JSON.stringify(metadata));
```

### Transformation Rules

```typescript
// Decision Tree → Graph Nodes + Edges

decisionTree.branches.forEach(branch => {
  // Create framework node
  const node = {
    id: `${branch.condition}_framework`,
    kind: 'transform',
    operation: '0x3000', // TOOL_EXECUTE_FRAMEWORK
    framework: transformFramework(branch.framework),
    metrics: branch.metrics,
    recommendation: branch.recommendation
  };
  
  // Create render node
  const renderNode = {
    id: `render_${branch.condition}_output`,
    kind: 'render',
    template: branch.output
  };
  
  // Create edges
  const edges = [
    {
      from: 'classify_challenge',
      to: node.id,
      predicate: `is_${branch.condition}`
    },
    {
      from: node.id,
      to: renderNode.id,
      always: true
    },
    {
      from: renderNode.id,
      to: 'done',
      always: true
    }
  ];
});
```

---

## 🎯 TERMINAL AGENT INTERFACE

### CLI Tool Invocation

```bash
# Direct invocation
$ omar tool alex-hormozi --condition customer_acquisition \
    --query "How do we improve CAC?" \
    --metrics '{"CAC": 150, "LTV": 450}'

# Output:
# Customer Acquisition System Blueprint
# 
# Alex Hormozi's Framework
# 
# Recommendation: Focus on lifetime value, not just acquisition cost...
# 
# Implementation Steps:
# 1. Calculate accurate customer lifetime value
# 2. Map your current acquisition funnel
# ...

# Tool composition
$ omar compose \
    big-brain --query "market dynamics" \
    | alex-hormozi --condition customer_acquisition \
    | ada-lovelace --condition algorithm_design

# Interactive mode
$ omar tool --interactive
> Which tool? alex-hormozi
> Condition? customer_acquisition
> Query? How to improve CAC?
> [Executing framework...]
> [Here's your blueprint...]

# List available tools
$ omar tools list --category business-legends
alex-hormozi (expert) - Systematic Growth & Acquisition
kevin-oleary (expert) - ROI-Focused Financial Discipline
...

# Search tools
$ omar tools search "customer acquisition"
Found 3 matching tools:
1. alex-hormozi (confidence: 0.95)
2. gary-vaynerchuk (confidence: 0.82)
3. david-ogilvy (confidence: 0.75)
```

---

## 🔮 ADVANCED PATTERNS

### Pattern 1: Multi-Tool Deliberation

```xml
<workflow id="council_with_tools">
  <entry p="council" x="deliberate_with_tools" node="operator"/>
  
  <nodes>
    <!-- Operator consults Alex Hormozi -->
    <node id="operator" kind="external" op="0x2001">
      <memberKey>operator</memberKey>
      <tools>
        <toolRef>alex-hormozi</toolRef>
        <toolRef>margaret-hamilton</toolRef>
      </tools>
    </node>
    
    <!-- Strategist consults Jim Keller -->
    <node id="strategist" kind="external" op="0x2001">
      <memberKey>strategist</memberKey>
      <tools>
        <toolRef>jim-keller</toolRef>
        <toolRef>jim-collins</toolRef>
      </tools>
    </node>
    
    <!-- Signal consults Gary Vee -->
    <node id="signal" kind="external" op="0x2001">
      <memberKey>signal</memberKey>
      <tools>
        <toolRef>gary-vaynerchuk</toolRef>
        <toolRef>scott-galloway</toolRef>
      </tools>
    </node>
    
    <node id="synthesize" kind="external" op="0x2002"/>
    <node id="done" kind="terminal"/>
  </nodes>
</workflow>
```

### Pattern 2: Iterative Tool Refinement

```xml
<workflow id="iterative_analysis">
  <nodes>
    <!-- Loop: Get insights, consult tool, refine, repeat -->
    <node id="get_insights" kind="external" op="0x3090">
      <toolRef>big-brain</toolRef>
    </node>
    
    <node id="consult_expert" kind="external" op="0x3010">
      <toolRef ref="$selected_tool"/>
      <input ref="$insights"/>
    </node>
    
    <node id="check_confidence" kind="auth">
      <predicate ref="confidence_sufficient"/>
    </node>
    
    <!-- If confidence low, loop back with refined query -->
    <node id="refine_query" kind="transform">
      <input ref="$expert_output"/>
    </node>
  </nodes>
  
  <edges>
    <edge from="get_insights" to="consult_expert"><when><always/></when></edge>
    <edge from="consult_expert" to="check_confidence"><when><always/></when></edge>
    
    <!-- High confidence: done -->
    <edge from="check_confidence" to="done">
      <when><predicate ref="confidence_sufficient"/></when>
    </edge>
    
    <!-- Low confidence: refine and loop -->
    <edge from="check_confidence" to="refine_query">
      <when><not><predicate ref="confidence_sufficient"/></not></when>
    </edge>
    <edge from="refine_query" to="get_insights"><when><always/></when></edge>
  </edges>
</workflow>
```

### Pattern 3: Tool Recommendation Engine

```xml
<workflow id="recommend_tool">
  <entry p="tool" x="recommend" node="analyze_query"/>
  
  <nodes>
    <!-- Analyze query to determine best tool -->
    <node id="analyze_query" kind="transform">
      <operation ref="0x3002"/> <!-- TOOL_CLASSIFY_CONDITION -->
    </node>
    
    <!-- Route to best matching tool -->
    <node id="route_to_tool" kind="external">
      <toolRef ref="$recommended_tool"/>
      <input ref="$input"/>
    </node>
    
    <node id="done" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="analyze_query" to="route_to_tool"><when><always/></when></edge>
    <edge from="route_to_tool" to="done"><when><always/></when></edge>
  </edges>
  
  <predicates>
    <!-- Predicates for different domains -->
    <predicate id="needs_business_expertise">
      <or>
        <contains left="$input.query" right="revenue"/>
        <contains left="$input.query" right="customer"/>
        <contains left="$input.query" right="growth"/>
      </or>
    </predicate>
    
    <predicate id="needs_tech_expertise">
      <or>
        <contains left="$input.query" right="algorithm"/>
        <contains left="$input.query" right="architecture"/>
        <contains left="$input.query" right="code"/>
      </or>
    </predicate>
  </predicates>
</workflow>
```

---

## 📊 TOOL METRICS & OBSERVABILITY

### Event Emission

Every tool execution emits PXYZ events:

```typescript
// Tool invocation start
{
  P: "tool",
  X: "alex-hormozi",
  Y: {
    condition: "customer_acquisition",
    context: { query: "..." }
  },
  Z: "2025-01-20T10:30:00Z"
}

// Tool framework execution
{
  P: "tool_framework",
  X: "execute",
  Y: {
    toolId: "alex-hormozi",
    branch: "customer_acquisition",
    step: 1,
    stepDescription: "Calculate accurate customer lifetime value"
  },
  Z: "2025-01-20T10:30:05Z"
}

// Tool completion
{
  P: "tool",
  X: "completed",
  Y: {
    toolId: "alex-hormozi",
    branch: "customer_acquisition",
    metricsCollected: ["CAC", "LTV", "LTV:CAC ratio"],
    outputGenerated: true
  },
  Z: "2025-01-20T10:30:15Z"
}
```

### Tool Analytics

```sql
-- Most used tools
SELECT tool_id, COUNT(*) as invocations
FROM tool_events
WHERE event_type = 'tool.invoked'
GROUP BY tool_id
ORDER BY invocations DESC;

-- Tool effectiveness (by confidence)
SELECT tool_id, AVG(confidence) as avg_confidence
FROM tool_events
WHERE event_type = 'tool.completed'
GROUP BY tool_id
ORDER BY avg_confidence DESC;

-- Tool composition patterns
SELECT tool_sequence, COUNT(*) as occurrences
FROM (
  SELECT STRING_AGG(tool_id, ' -> ' ORDER BY timestamp) as tool_sequence
  FROM tool_events
  WHERE event_type = 'tool.invoked'
  GROUP BY session_id
) AS sequences
GROUP BY tool_sequence
ORDER BY occurrences DESC;
```

---

## 🎓 KEY INSIGHTS

### 1. Tools Are Just Workflows

Every tool is a workflow graph. The only difference is the entry point coordinate.

```
Workflow:  P="workflow", X="process_order", Y={...}
Tool:      P="tool", X="alex-hormozi", Y={...}
           ↑ Same execution, different P-coordinate
```

### 2. Decision Trees = Predicated Graph Traversal

```
Traditional Decision Tree:
  if (condition === 'A') branch_A();
  else if (condition === 'B') branch_B();

OMAR Decision Tree:
  <edge from="classify" to="branch_A">
    <when><predicate ref="is_A"/></when>
  </edge>
  <edge from="classify" to="branch_B">
    <when><predicate ref="is_B"/></when>
  </edge>
```

### 3. Tool Composition = Graph References

```
Traditional: const result = await tool2(await tool1(input));
OMAR:        <node ref="tool1"/> → <node ref="tool2"/>
```

### 4. Consulting Frameworks = Multi-Step Nodes

```
Framework:
  step1: Do X
  step2: Do Y
  step3: Do Z

OMAR:
  <framework>
    <step order="1">Do X</step>
    <step order="2">Do Y</step>
    <step order="3">Do Z</step>
  </framework>
```

---

## ✅ NEXT STEPS

### Phase 1: Tool Compiler
- [ ] Build JSON → XML transformer
- [ ] Implement decision tree → graph converter
- [ ] Generate predicate definitions from conditions
- [ ] Create template system for outputs

### Phase 2: Tool Registry
- [ ] Compile all 120+ tools to graph.bin
- [ ] Generate tool_registry.json index
- [ ] Create searchable metadata
- [ ] Build CLI tool browser

### Phase 3: Tool Executor
- [ ] Implement PXYZ → tool routing
- [ ] Load and execute tool graphs
- [ ] Handle tool composition
- [ ] Event emission and logging

### Phase 4: Terminal Interface
- [ ] Build CLI tool invocation
- [ ] Implement interactive mode
- [ ] Create tool search/discovery
- [ ] Add tool composition commands

---

**Status**: ✅ **FRAMEWORK COMPLETE - READY FOR IMPLEMENTATION**

This framework transforms your 120+ JSON tools into pure OMAR workflows with:
- ✅ Tool use = Graph traversal
- ✅ Tool composition = Graph chaining
- ✅ Tool discovery = Graph index
- ✅ Terminal interface = PXYZ operations

**The tools are the graph. The graph is physics.**
