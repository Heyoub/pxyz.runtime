# OMAR/PXYZ Implementation Patterns & Best Practices

> **Companion to**: OMAR_PXYZ_Unified_Reference.md  
> **Focus**: Practical implementation guidance, patterns, and gotchas

---

## Table of Contents

1. [Common Patterns](#common-patterns)
2. [Anti-Patterns to Avoid](#anti-patterns-to-avoid)
3. [Performance Optimization](#performance-optimization)
4. [Error Handling Strategies](#error-handling-strategies)
5. [Testing Strategies](#testing-strategies)
6. [Debugging Techniques](#debugging-techniques)
7. [Security Considerations](#security-considerations)
8. [Production Deployment](#production-deployment)
9. [Monitoring & Observability](#monitoring--observability)
10. [Migration Strategies](#migration-strategies)

---

## Common Patterns

### Pattern 1: Progressive Validation Pipeline

**Use Case**: Multi-step validation where early failures should short-circuit

```xml
<workflow id="user_registration">
  <nodes>
    <!-- Step 1: Basic format validation (fast, no I/O) -->
    <node id="validate_format" kind="transform" schema="UserInput"/>
    
    <!-- Step 2: Business rules (fast, no I/O) -->
    <node id="check_age" kind="auth">
      <predicate>
        <gte left="$input.age" right="18"/>
      </predicate>
    </node>
    
    <!-- Step 3: Uniqueness checks (requires I/O) -->
    <node id="check_email" kind="external" op="0x0105"/>
    <node id="verify_email_unique" kind="auth">
      <predicate>
        <eq left="$state.check_email.count" right="0"/>
      </predicate>
    </node>
    
    <!-- Step 4: External validation (expensive) -->
    <node id="verify_email_deliverable" kind="external" op="0x0400"/>
    
    <!-- Final: Create user -->
    <node id="create_user" kind="external" op="0x0100"/>
    
    <!-- Error nodes -->
    <node id="error_format" kind="error" status="400"/>
    <node id="error_age" kind="error" status="403"/>
    <node id="error_duplicate" kind="error" status="409"/>
    <node id="error_invalid_email" kind="error" status="422"/>
  </nodes>
  
  <edges>
    <!-- Progressive validation chain -->
    <edge from="validate_format" to="check_age" weight="10"/>
    <edge from="check_age" to="check_email" weight="10"/>
    <edge from="check_email" to="verify_email_unique" weight="10"/>
    <edge from="verify_email_unique" to="verify_email_deliverable" weight="10"/>
    <edge from="verify_email_deliverable" to="create_user" weight="10"/>
    
    <!-- Fallback error paths -->
    <edge from="validate_format" to="error_format" fallback="true"/>
    <edge from="check_age" to="error_age" fallback="true"/>
    <edge from="verify_email_unique" to="error_duplicate" fallback="true"/>
    <edge from="verify_email_deliverable" to="error_invalid_email" fallback="true"/>
  </edges>
</workflow>
```

**Why this works:**
- Fast checks first (format, business rules)
- Expensive I/O only if fast checks pass
- Early exit on first failure
- Clear error messages at each stage

---

### Pattern 2: Conditional Branching with Priority

**Use Case**: Different paths based on user role or context

```xml
<workflow id="approval_routing">
  <nodes>
    <node id="load_request" kind="external" op="0x0101"/>
    
    <!-- Check user roles -->
    <node id="check_admin" kind="auth">
      <predicate>
        <contains left="$token.perms" right="admin"/>
      </predicate>
    </node>
    
    <node id="check_manager" kind="auth">
      <predicate>
        <contains left="$token.perms" right="manager"/>
      </predicate>
    </node>
    
    <!-- Different approval paths -->
    <node id="admin_approve" kind="external" op="0x0102"/>
    <node id="manager_request_admin" kind="external" op="0x0340"/>
    <node id="employee_request_manager" kind="external" op="0x0340"/>
  </nodes>
  
  <edges>
    <edge from="load_request" to="check_admin" weight="20"/>
    <edge from="load_request" to="check_manager" weight="10"/>
    <edge from="load_request" to="employee_request_manager" weight="5"/>
    
    <!-- Admins can approve directly -->
    <edge from="check_admin" to="admin_approve" weight="10"/>
    
    <!-- Managers need admin approval -->
    <edge from="check_manager" to="manager_request_admin" weight="10"/>
  </edges>
</workflow>
```

**Key insight**: Higher weight = higher priority. The runtime tries edges in descending weight order.

---

### Pattern 3: LLM with Multi-Stage Validation

**Use Case**: AI-generated content that needs multiple quality checks before use

```xml
<workflow id="ai_email_generation">
  <nodes>
    <!-- Generate email with LLM -->
    <node id="llm_generate" kind="external" op="0x0800"/>
    
    <!-- Validation Stage 1: Structure check -->
    <node id="validate_structure" kind="auth">
      <predicate>
        <and>
          <fn name="is_defined" arg="$state.llm_generate.subject"/>
          <fn name="is_defined" arg="$state.llm_generate.body"/>
          <gt left="$state.llm_generate.body_length" right="50"/>
        </and>
      </predicate>
    </node>
    
    <!-- Validation Stage 2: Content safety -->
    <node id="check_content_safety" kind="external" op="0x0801"/>
    <node id="verify_safe" kind="auth">
      <predicate>
        <eq left="$state.check_content_safety.is_safe" right="true"/>
      </predicate>
    </node>
    
    <!-- Validation Stage 3: Tone check -->
    <node id="check_tone" kind="external" op="0x0801"/>
    <node id="verify_tone" kind="auth">
      <predicate>
        <contains left="$state.check_tone.tone" right="professional"/>
      </predicate>
    </node>
    
    <!-- Human review gate (PRAG001 compliance) -->
    <node id="human_review" kind="external" op="0x0340" 
          actor="human" confirmation="suggested"/>
    
    <!-- If all checks pass, send email -->
    <node id="send_email" kind="external" op="0x0340" 
          actor="human" confirmation="confirmed"/>
    
    <!-- Error paths -->
    <node id="regenerate" kind="external" op="0x0800"/>
    <node id="escalate_to_human" kind="external" op="0x0340" actor="human"/>
  </nodes>
  
  <edges>
    <edge from="llm_generate" to="validate_structure" weight="10"/>
    <edge from="validate_structure" to="check_content_safety" weight="10"/>
    <edge from="check_content_safety" to="verify_safe" weight="10"/>
    <edge from="verify_safe" to="check_tone" weight="10"/>
    <edge from="check_tone" to="verify_tone" weight="10"/>
    <edge from="verify_tone" to="human_review" weight="10"/>
    <edge from="human_review" to="send_email" weight="10"/>
    
    <!-- Regeneration path if any check fails -->
    <edge from="verify_safe" to="regenerate" fallback="true" weight="5"/>
    <edge from="verify_tone" to="regenerate" fallback="true" weight="5"/>
    
    <!-- Human escalation if regeneration fails -->
    <edge from="regenerate" to="escalate_to_human" fallback="true"/>
  </edges>
</workflow>
```

**PRAG001 compliance**: LLM output goes through validation gates before any irreversible action (email send).

---

### Pattern 4: Parallel I/O with Aggregation

**Use Case**: Fetch data from multiple sources simultaneously

```xml
<workflow id="contact_enrichment">
  <nodes>
    <node id="load_contact" kind="external" op="0x0101"/>
    
    <!-- Parallel fetches (all have same weight, can run concurrently) -->
    <node id="fetch_linkedin" kind="external" op="0x0400" 
          async="true" cacheable="true"/>
    <node id="fetch_clearbit" kind="external" op="0x0400" 
          async="true" cacheable="true"/>
    <node id="fetch_company_data" kind="external" op="0x0400" 
          async="true" cacheable="true"/>
    
    <!-- Aggregation node -->
    <node id="merge_data" kind="transform" schema="EnrichedContact"/>
    
    <!-- Update with enriched data -->
    <node id="update_contact" kind="external" op="0x0102"/>
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <!-- Fan-out: one-to-many -->
    <edge from="load_contact" to="fetch_linkedin" weight="10" parallel="true"/>
    <edge from="load_contact" to="fetch_clearbit" weight="10" parallel="true"/>
    <edge from="load_contact" to="fetch_company_data" weight="10" parallel="true"/>
    
    <!-- Fan-in: many-to-one -->
    <edge from="fetch_linkedin" to="merge_data" weight="10"/>
    <edge from="fetch_clearbit" to="merge_data" weight="10"/>
    <edge from="fetch_company_data" to="merge_data" weight="10"/>
    
    <edge from="merge_data" to="update_contact" weight="10"/>
    <edge from="update_contact" to="success" weight="10"/>
  </edges>
</workflow>
```

**Implementation note**: The `parallel="true"` flag hints to the runtime that these can execute concurrently. The runtime waits for all parallel edges before proceeding to `merge_data`.

---

### Pattern 5: Caching with TTL

**Use Case**: Expensive operations that can be cached

```xml
<workflow id="dashboard_data">
  <nodes>
    <!-- Try cache first -->
    <node id="check_cache" kind="external" op="0x0900"/>
    
    <!-- If cache hit, return cached data -->
    <node id="cache_hit" kind="auth">
      <predicate>
        <fn name="is_defined" arg="$state.check_cache.value"/>
      </predicate>
    </node>
    
    <node id="return_cached" kind="render" template="Dashboard"/>
    
    <!-- If cache miss, fetch fresh data -->
    <node id="fetch_analytics" kind="external" op="0x0400" async="true"/>
    <node id="fetch_reports" kind="external" op="0x0400" async="true"/>
    
    <!-- Store in cache -->
    <node id="store_cache" kind="external" op="0x0901"/>
    
    <!-- Return fresh data -->
    <node id="return_fresh" kind="render" template="Dashboard"/>
  </nodes>
  
  <edges>
    <edge from="start" to="check_cache" weight="10"/>
    <edge from="check_cache" to="cache_hit" weight="10"/>
    
    <!-- Cache hit path -->
    <edge from="cache_hit" to="return_cached" weight="10"/>
    
    <!-- Cache miss path -->
    <edge from="cache_hit" to="fetch_analytics" fallback="true" parallel="true"/>
    <edge from="cache_hit" to="fetch_reports" fallback="true" parallel="true"/>
    <edge from="fetch_analytics" to="store_cache" weight="10"/>
    <edge from="fetch_reports" to="store_cache" weight="10"/>
    <edge from="store_cache" to="return_fresh" weight="10"/>
  </edges>
</workflow>
```

---

### Pattern 6: Retry with Exponential Backoff

**Use Case**: Handling transient failures in external APIs

```xml
<workflow id="resilient_api_call">
  <nodes>
    <!-- Try primary API -->
    <node id="call_api_attempt1" kind="external" op="0x0400" async="true"/>
    
    <!-- Check if successful -->
    <node id="check_success_1" kind="auth">
      <predicate>
        <eq left="$state.call_api_attempt1.status" right="200"/>
      </predicate>
    </node>
    
    <!-- Wait 1 second -->
    <node id="wait_1s" kind="transform"/>
    
    <!-- Retry -->
    <node id="call_api_attempt2" kind="external" op="0x0400" async="true"/>
    <node id="check_success_2" kind="auth">
      <predicate>
        <eq left="$state.call_api_attempt2.status" right="200"/>
      </predicate>
    </node>
    
    <!-- Wait 2 seconds -->
    <node id="wait_2s" kind="transform"/>
    
    <!-- Final attempt -->
    <node id="call_api_attempt3" kind="external" op="0x0400" async="true"/>
    
    <!-- Success path -->
    <node id="process_response" kind="transform"/>
    
    <!-- Failure path -->
    <node id="log_failure" kind="external" op="0x0910"/>
    <node id="error" kind="error" status="503"/>
  </nodes>
  
  <edges>
    <!-- Attempt 1 -->
    <edge from="start" to="call_api_attempt1" weight="10"/>
    <edge from="call_api_attempt1" to="check_success_1" weight="10"/>
    <edge from="check_success_1" to="process_response" weight="10"/>
    
    <!-- Retry after 1s if failed -->
    <edge from="check_success_1" to="wait_1s" fallback="true"/>
    <edge from="wait_1s" to="call_api_attempt2" weight="10"/>
    <edge from="call_api_attempt2" to="check_success_2" weight="10"/>
    <edge from="check_success_2" to="process_response" weight="10"/>
    
    <!-- Retry after 2s if failed -->
    <edge from="check_success_2" to="wait_2s" fallback="true"/>
    <edge from="wait_2s" to="call_api_attempt3" weight="10"/>
    <edge from="call_api_attempt3" to="process_response" weight="10"/>
    
    <!-- Final failure -->
    <edge from="call_api_attempt3" to="log_failure" fallback="true"/>
    <edge from="log_failure" to="error" weight="10"/>
  </edges>
</workflow>
```

**Note**: For actual delays, the `transform` nodes would need to be implemented in the IO adapter to introduce timing.

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: God Nodes

**❌ BAD**: Single node that does everything
```xml
<node id="do_everything" kind="external" op="0x0100"/>
```

**✅ GOOD**: Decompose into single-responsibility nodes
```xml
<node id="validate_input" kind="transform"/>
<node id="check_permissions" kind="auth"/>
<node id="create_entity" kind="external" op="0x0100"/>
<node id="send_notification" kind="external" op="0x0340"/>
<node id="log_event" kind="external" op="0x0910"/>
```

**Why**: Fine-grained nodes = better observability, caching, error handling

---

### Anti-Pattern 2: Deep Predicate Nesting

**❌ BAD**: Complex nested predicates
```xml
<predicate id="complex_check">
  <and>
    <or>
      <and>
        <eq left="$a" right="1"/>
        <or>
          <gt left="$b" right="5"/>
          <lt left="$c" right="3"/>
        </or>
      </and>
      <not>
        <contains left="$d" right="x"/>
      </not>
    </or>
    <eq left="$e" right="true"/>
  </and>
</predicate>
```

**✅ GOOD**: Decompose into named sub-predicates
```xml
<predicate id="condition_A">
  <and>
    <eq left="$a" right="1"/>
    <or>
      <gt left="$b" right="5"/>
      <lt left="$c" right="3"/>
    </or>
  </and>
</predicate>

<predicate id="condition_B">
  <not>
    <contains left="$d" right="x"/>
  </not>
</predicate>

<predicate id="final_check">
  <and>
    <or>
      <ref predicate="condition_A"/>
      <ref predicate="condition_B"/>
    </or>
    <eq left="$e" right="true"/>
  </and>
</predicate>
```

**Why**: Readable, testable, reusable, doesn't hit 256-byte limit

---

### Anti-Pattern 3: Implicit State Dependencies

**❌ BAD**: Node depends on implicit execution order
```xml
<!-- Assumes fetch_user ran before check_status -->
<node id="check_status" kind="auth">
  <predicate>
    <eq left="$state.fetch_user.status" right="active"/>
  </predicate>
</node>
```

**✅ GOOD**: Explicit edge dependency
```xml
<edge from="fetch_user" to="check_status" weight="10"/>
```

**Why**: Graph explicitly shows data flow, no hidden dependencies

---

### Anti-Pattern 4: Ignoring Error Paths

**❌ BAD**: No fallback edges
```xml
<edge from="risky_operation" to="next_step" weight="10"/>
<!-- What if risky_operation fails? -->
```

**✅ GOOD**: Always have error handling
```xml
<edge from="risky_operation" to="next_step" weight="10"/>
<edge from="risky_operation" to="error_handler" fallback="true"/>
```

**Why**: PRAG002 compliance, graceful degradation

---

### Anti-Pattern 5: Putting Business Logic in IO Adapter

**❌ BAD**: IO adapter contains business rules
```javascript
// io-browser.ts
async function createContact(ctx) {
  // ❌ BAD: Business logic in IO layer
  if (ctx.data.industry === 'healthcare') {
    ctx.data.owner = 'sarah';
  }
  
  return await db.contacts.create(ctx.data);
}
```

**✅ GOOD**: Business logic in workflow graph
```xml
<workflow>
  <nodes>
    <node id="check_industry" kind="auth">
      <predicate>
        <eq left="$input.industry" right="healthcare"/>
      </predicate>
    </node>
    <node id="assign_to_sarah" kind="external" op="0x0102"/>
    <node id="create_contact" kind="external" op="0x0100"/>
  </nodes>
</workflow>
```

```javascript
// io-browser.ts
async function createContact(ctx) {
  // ✅ GOOD: Pure I/O, no business logic
  return await db.contacts.create(ctx.data);
}
```

**Why**: Business logic belongs in auditable graph, not hidden in code

---

## Performance Optimization

### 1. Mark Cacheable Operations

```xml
<node id="fetch_user_profile" kind="external" op="0x0101" 
      cacheable="true"/>
```

IO adapter can implement caching:
```javascript
const cache = new Map();

async function io_call(op_code, data_ptr) {
  const node = current_node;
  const data = memory.read(data_ptr);
  
  if (node.cacheable) {
    const cache_key = hash(op_code, data);
    if (cache.has(cache_key)) {
      return cache.get(cache_key);
    }
    
    const result = await execute_operation(op_code, data);
    cache.set(cache_key, result);
    return result;
  }
  
  return await execute_operation(op_code, data);
}
```

### 2. Use Parallel Edges

```xml
<!-- These can run concurrently -->
<edge from="start" to="fetch_a" weight="10" parallel="true"/>
<edge from="start" to="fetch_b" weight="10" parallel="true"/>
<edge from="start" to="fetch_c" weight="10" parallel="true"/>
```

### 3. Early Exit on Validation

Order nodes from cheapest to most expensive:

```xml
<!-- Cheap validation first -->
<edge from="start" to="validate_format" weight="10"/>

<!-- Medium cost -->
<edge from="validate_format" to="check_duplicates" weight="10"/>

<!-- Expensive external call only if cheap checks pass -->
<edge from="check_duplicates" to="verify_email" weight="10"/>
```

### 4. Optimize Predicate Bytecode

Avoid redundant operations:

**❌ BAD**:
```xml
<predicate id="inefficient">
  <and>
    <eq left="$token.sub" right="$entity.owner"/>
    <eq left="$token.sub" right="$entity.owner"/>  <!-- Duplicate! -->
  </and>
</predicate>
```

**✅ GOOD**:
```xml
<predicate id="efficient">
  <eq left="$token.sub" right="$entity.owner"/>
</predicate>
```

The compiler should deduplicate, but manual optimization helps.

### 5. Batch Operations

Instead of:
```xml
<!-- ❌ N separate calls -->
<node id="fetch_1" kind="external" op="0x0101"/>
<node id="fetch_2" kind="external" op="0x0101"/>
<node id="fetch_3" kind="external" op="0x0101"/>
```

Use:
```xml
<!-- ✅ One batched call -->
<node id="fetch_all" kind="external" op="0x0104"/>
```

---

## Error Handling Strategies

### Strategy 1: Graceful Degradation

```xml
<workflow id="dashboard">
  <nodes>
    <!-- Try to fetch live data -->
    <node id="fetch_live" kind="external" op="0x0400"/>
    
    <!-- If fails, use cached data -->
    <node id="check_cache" kind="external" op="0x0900"/>
    
    <!-- If no cache, use default data -->
    <node id="use_defaults" kind="transform"/>
    
    <node id="render" kind="render" template="Dashboard"/>
  </nodes>
  
  <edges>
    <edge from="start" to="fetch_live" weight="10"/>
    <edge from="fetch_live" to="render" weight="10"/>
    <edge from="fetch_live" to="check_cache" fallback="true"/>
    <edge from="check_cache" to="render" weight="10"/>
    <edge from="check_cache" to="use_defaults" fallback="true"/>
    <edge from="use_defaults" to="render" weight="10"/>
  </edges>
</workflow>
```

### Strategy 2: Error Categorization

```xml
<nodes>
  <node id="risky_op" kind="external" op="0x0400"/>
  
  <!-- Different errors go to different handlers -->
  <node id="check_status" kind="auth">
    <predicate>
      <eq left="$state.risky_op.status" right="200"/>
    </predicate>
  </node>
  
  <node id="error_400" kind="error" status="400" message="Bad Request"/>
  <node id="error_401" kind="error" status="401" message="Unauthorized"/>
  <node id="error_500" kind="error" status="500" message="Server Error"/>
</nodes>

<edges>
  <edge from="risky_op" to="check_status" weight="10"/>
  <edge from="check_status" to="success" weight="10"/>
  
  <!-- Categorize errors by status code -->
  <edge from="check_status" to="error_400" fallback="true">
    <when>
      <eq left="$state.risky_op.status" right="400"/>
    </when>
  </edge>
  <edge from="check_status" to="error_401" fallback="true">
    <when>
      <eq left="$state.risky_op.status" right="401"/>
    </when>
  </edge>
  <edge from="check_status" to="error_500" fallback="true"/>
</edges>
```

### Strategy 3: Circuit Breaker

```xml
<workflow id="api_with_circuit_breaker">
  <nodes>
    <!-- Check if circuit is open -->
    <node id="check_circuit" kind="external" op="0x0900"/>
    
    <node id="circuit_closed" kind="auth">
      <predicate>
        <eq left="$state.check_circuit.is_open" right="false"/>
      </predicate>
    </node>
    
    <!-- Circuit closed, try API -->
    <node id="call_api" kind="external" op="0x0400"/>
    
    <!-- Circuit open, return cached/fallback -->
    <node id="use_fallback" kind="transform"/>
    
    <!-- Update circuit state -->
    <node id="record_success" kind="external" op="0x0901"/>
    <node id="record_failure" kind="external" op="0x0901"/>
  </nodes>
  
  <edges>
    <edge from="start" to="check_circuit" weight="10"/>
    <edge from="check_circuit" to="circuit_closed" weight="10"/>
    
    <!-- Circuit closed path -->
    <edge from="circuit_closed" to="call_api" weight="10"/>
    <edge from="call_api" to="record_success" weight="10"/>
    <edge from="call_api" to="record_failure" fallback="true"/>
    
    <!-- Circuit open path -->
    <edge from="circuit_closed" to="use_fallback" fallback="true"/>
  </edges>
</workflow>
```

---

## Testing Strategies

### Unit Testing Predicates

```javascript
// predicate-tests.js
import { evalPredicate } from 'omar-runtime';

describe('is_admin predicate', () => {
  it('returns true when user has admin permission', () => {
    const context = {
      token: { perms: ['read', 'write', 'admin'] }
    };
    
    const result = evalPredicate(PREDICATE_IS_ADMIN, context);
    expect(result).toBe(true);
  });
  
  it('returns false when user lacks admin permission', () => {
    const context = {
      token: { perms: ['read', 'write'] }
    };
    
    const result = evalPredicate(PREDICATE_IS_ADMIN, context);
    expect(result).toBe(false);
  });
});
```

### Integration Testing Workflows

```javascript
// workflow-tests.js
import { Runtime } from 'omar-runtime';
import { readFileSync } from 'fs';

describe('Contact Creation Workflow', () => {
  let runtime;
  
  beforeAll(async () => {
    const graph = readFileSync('dist/graph.bin');
    runtime = new Runtime();
    await runtime.loadGraph(graph);
  });
  
  it('creates contact successfully with valid input', async () => {
    const result = await runtime.execute('contact', 'create', {
      token: { sub: 'user_123', perms: ['write'], tenant: 'T1' },
      input: {
        name: 'John Doe',
        email: 'john@example.com'
      }
    });
    
    expect(result.status).toBe(201);
    expect(result.data.id).toBeDefined();
  });
  
  it('rejects duplicate email', async () => {
    // First creation succeeds
    await runtime.execute('contact', 'create', { /* ... */ });
    
    // Second creation with same email should fail
    const result = await runtime.execute('contact', 'create', {
      token: { sub: 'user_123', perms: ['write'], tenant: 'T1' },
      input: {
        name: 'Jane Doe',
        email: 'john@example.com'  // Duplicate
      }
    });
    
    expect(result.status).toBe(409);
    expect(result.message).toContain('duplicate');
  });
});
```

### Property-Based Testing

```javascript
// property-tests.js
import fc from 'fast-check';

describe('Graph invariants', () => {
  it('all entry points resolve to valid nodes', () => {
    fc.assert(
      fc.property(
        fc.string(), // P
        fc.string(), // X
        (p, x) => {
          const entryNode = graph.findEntry(p, x);
          if (entryNode) {
            return graph.nodes.has(entryNode);
          }
          return true; // No entry is okay
        }
      )
    );
  });
  
  it('no cycles exist in graph', () => {
    const visited = new Set();
    const recStack = new Set();
    
    function hasCycle(nodeId) {
      visited.add(nodeId);
      recStack.add(nodeId);
      
      const edges = graph.getEdges(nodeId);
      for (const edge of edges) {
        if (!visited.has(edge.target)) {
          if (hasCycle(edge.target)) return true;
        } else if (recStack.has(edge.target)) {
          return true; // Cycle detected
        }
      }
      
      recStack.delete(nodeId);
      return false;
    }
    
    for (const nodeId of graph.nodes.keys()) {
      expect(hasCycle(nodeId)).toBe(false);
    }
  });
});
```

---

## Debugging Techniques

### 1. Enable Trace Mode

```javascript
// Enable tracing
runtime.setTraceMode(1);

// Execute workflow
await runtime.execute('contact', 'search', context);

// Read trace
const trace = runtime.getTrace();
console.log(trace);
/*
Step 0: Enter node 'validate_query' (transform)
Step 1: Exit node 'validate_query' → success
Step 2: Enter node 'execute_search' (external:0x0105)
Step 3: IO call: ENTITY_SEARCH with data {...}
Step 4: IO return: { count: 5, results: [...] }
Step 5: Exit node 'execute_search' → success
Step 6: Enter node 'render_results' (render:ContactList)
Step 7: Exit node 'render_results' → success
*/
```

### 2. Inspect Binary

```bash
# View as Mermaid flowchart
pxyz inspect graph.bin --format mermaid > workflow.mmd

# Open in Mermaid Live Editor
# https://mermaid.live/
```

### 3. Log Predicate Evaluations

```javascript
// In IO adapter
function io_resolve_var(path_ptr) {
  const path = memory.readString(path_ptr);
  const value = resolvePath(context, path);
  
  console.log(`[PREDICATE] Resolved ${path} → ${JSON.stringify(value)}`);
  
  return value;
}
```

### 4. Dry-Run Mode

```javascript
// Mock all IO operations
runtime.setDryRun(true);

// Execute workflow
const result = await runtime.execute('contact', 'create', context);

// Check which operations would have been called
console.log(result.io_calls);
/*
[
  { op: 0x0105, node: 'check_duplicates', data: {...} },
  { op: 0x0100, node: 'create_contact', data: {...} },
  { op: 0x0340, node: 'send_welcome', data: {...} }
]
*/
```

### 5. Audit Log Analysis

```javascript
// Enable audit logging
const auditLog = [];

function io_call(op_code, data_ptr) {
  const entry = {
    timestamp: Date.now(),
    node_id: current_node.id,
    op_code: op_code,
    op_name: OP_CODES[op_code].name,
    data: memory.read(data_ptr),
    actor: current_actor
  };
  
  auditLog.push(entry);
  
  // ... execute operation
}

// After execution, analyze log
console.table(auditLog);
```

---

## Security Considerations

### 1. Input Validation

**Always validate at workflow entry:**
```xml
<entry p="contact" x="create" node="validate_input"/>

<node id="validate_input" kind="transform" schema="ContactInput"/>
```

**Never trust client input:**
```xml
<!-- ❌ BAD: Direct use of input -->
<node id="create" kind="external" op="0x0100"/>

<!-- ✅ GOOD: Validate first -->
<node id="validate" kind="transform" schema="ContactInput"/>
<edge from="validate" to="create" weight="10"/>
```

### 2. Authorization Checks

**Check permissions before every sensitive operation:**
```xml
<node id="check_auth" kind="auth" predicate="can_delete"/>
<node id="delete" kind="external" op="0x0103" actor="human"/>
```

**Don't rely on client-side checks:**
```xml
<!-- Server-side authorization required -->
<predicate id="can_delete">
  <and>
    <contains left="$token.perms" right="delete"/>
    <eq left="$entity.owner_id" right="$token.sub"/>
  </and>
</predicate>
```

### 3. Data Quarantine

**Use Y-constraints to mark untrusted data:**
```typescript
// In IO adapter
function handleUserInput(input) {
  return {
    data: input,
    yCtx: {
      quarantine: true,
      source: 'user_input',
      sanitized: false
    }
  };
}
```

**PRAG005: Prevent quarantined data from escaping:**
```xml
<node id="sanitize_input" kind="transform" schema="SanitizedInput"/>

<!-- Only non-quarantined data can go to external APIs -->
<node id="send_to_api" kind="external" op="0x0400">
  <require>
    <not>
      <eq left="$entity.yCtx.quarantine" right="true"/>
    </not>
  </require>
</node>
```

### 4. Rate Limiting

**Enforce rate limits in IO adapter:**
```javascript
const rateLimiter = new Map();

function io_call(op_code, data_ptr) {
  const userId = context.token.sub;
  const key = `${userId}:${op_code}`;
  
  const now = Date.now();
  const window = rateLimiter.get(key) || { count: 0, reset: now + 60000 };
  
  if (now > window.reset) {
    window.count = 0;
    window.reset = now + 60000;
  }
  
  if (window.count >= 100) { // 100 requests per minute
    throw new Error('RATE_LIMIT_EXCEEDED');
  }
  
  window.count++;
  rateLimiter.set(key, window);
  
  // ... execute operation
}
```

### 5. Audit Logging

**Log all sensitive operations:**
```javascript
function io_call(op_code, data_ptr) {
  const operation = OP_CODES[op_code];
  
  if (operation.irreversible || operation.sensitive) {
    auditLog.append({
      timestamp: Date.now(),
      user: context.token.sub,
      tenant: context.token.tenant,
      operation: operation.name,
      op_code: op_code,
      node: current_node.id,
      data: sanitize(memory.read(data_ptr)),
      actor: current_actor,
      ip_address: context.ip
    });
  }
  
  // ... execute operation
}
```

---

## Production Deployment

### 1. Build Process

```bash
#!/bin/bash
# build-production.sh

set -e

echo "Building OMAR for production..."

# Validate workflows
echo "Validating workflows..."
pxyz check workflow.xml --strict

# Compile to binary
echo "Compiling workflow..."
pxyz compile \
  --input workflow.xml \
  --output dist/graph.bin \
  --audit dist/audit.json \
  --optimize

# Compile WASM runtime
echo "Compiling WASM runtime..."
wat2wasm pxyz.wat -o dist/pxyz.wasm

# Minify IO adapter
echo "Minifying IO adapter..."
terser io-browser.ts --compress --mangle -o dist/io.min.js

# Generate checksums
echo "Generating checksums..."
sha256sum dist/graph.bin > dist/checksums.txt
sha256sum dist/pxyz.wasm >> dist/checksums.txt

# Create deployment package
echo "Creating deployment package..."
tar -czf omar-deploy-$(date +%Y%m%d-%H%M%S).tar.gz dist/

echo "✓ Build complete"
```

### 2. Deployment Checklist

- [ ] All workflows validated with `--strict`
- [ ] Audit log reviewed
- [ ] No PRAG violations
- [ ] All tests passing
- [ ] Performance benchmarks acceptable
- [ ] Security review completed
- [ ] Rollback plan prepared
- [ ] Monitoring configured
- [ ] Rate limits configured
- [ ] Error handling tested

### 3. Blue-Green Deployment

```javascript
// deployment-manager.js
class DeploymentManager {
  constructor() {
    this.blue = null;  // Current production
    this.green = null; // Staging/new version
  }
  
  async deployNew(graphPath) {
    const graph = await readFile(graphPath);
    const runtime = new Runtime();
    await runtime.loadGraph(graph);
    
    // Deploy to green slot
    this.green = runtime;
    
    // Health check
    await this.healthCheck(this.green);
  }
  
  async healthCheck(runtime) {
    // Run smoke tests
    const result = await runtime.execute('health', 'check', {});
    if (result.status !== 200) {
      throw new Error('Health check failed');
    }
  }
  
  async cutover() {
    if (!this.green) throw new Error('No green deployment');
    
    // Swap blue and green
    const old = this.blue;
    this.blue = this.green;
    this.green = old;
    
    console.log('✓ Cutover complete, new version live');
  }
  
  async rollback() {
    if (!this.green) throw new Error('No rollback available');
    
    // Swap back
    [this.blue, this.green] = [this.green, this.blue];
    
    console.log('✓ Rolled back to previous version');
  }
  
  getRuntime() {
    return this.blue;
  }
}

// Usage
const deployer = new DeploymentManager();
await deployer.deployNew('dist/graph-v2.bin');
await deployer.cutover();

// If issues arise
await deployer.rollback();
```

### 4. Hot Reload

```javascript
// hot-reload.js
import { watch } from 'fs';

const runtime = new Runtime();
await runtime.loadGraph(await readFile('dist/graph.bin'));

// Watch for changes
watch('dist/graph.bin', async (event) => {
  if (event === 'change') {
    console.log('Detected graph.bin change, reloading...');
    
    const newGraph = await readFile('dist/graph.bin');
    await runtime.loadGraph(newGraph);
    
    console.log('✓ Hot reload complete');
  }
});
```

---

## Monitoring & Observability

### 1. Metrics to Track

```javascript
const metrics = {
  // Execution metrics
  workflow_executions_total: 0,
  workflow_execution_duration_ms: [],
  workflow_errors_total: 0,
  
  // Node metrics
  node_executions: {}, // { node_id: count }
  node_errors: {},     // { node_id: count }
  
  // Predicate metrics
  predicate_evaluations: {}, // { predicate_id: count }
  predicate_true_rate: {},   // { predicate_id: percentage }
  
  // IO metrics
  io_operations: {},      // { op_code: count }
  io_duration_ms: {},     // { op_code: [durations] }
  io_errors: {},          // { op_code: count }
  
  // Cache metrics
  cache_hits: 0,
  cache_misses: 0,
  cache_hit_rate: 0
};

function recordExecution(p, x, duration_ms, result) {
  metrics.workflow_executions_total++;
  metrics.workflow_execution_duration_ms.push(duration_ms);
  
  if (result.status >= 400) {
    metrics.workflow_errors_total++;
  }
}
```

### 2. Structured Logging

```javascript
const logger = {
  debug: (msg, data) => log('DEBUG', msg, data),
  info: (msg, data) => log('INFO', msg, data),
  warn: (msg, data) => log('WARN', msg, data),
  error: (msg, data) => log('ERROR', msg, data)
};

function log(level, msg, data) {
  const entry = {
    timestamp: new Date().toISOString(),
    level: level,
    message: msg,
    tenant: context.token?.tenant,
    user: context.token?.sub,
    request_id: context.request_id,
    ...data
  };
  
  console.log(JSON.stringify(entry));
}

// Usage
logger.info('Workflow execution started', {
  p: 'contact',
  x: 'create',
  input_size: JSON.stringify(context.input).length
});
```

### 3. Distributed Tracing

```javascript
import { trace, context } from '@opentelemetry/api';

async function executeWorkflow(p, x, ctx) {
  const tracer = trace.getTracer('omar-runtime');
  
  return tracer.startActiveSpan(`workflow.${p}.${x}`, async (span) => {
    try {
      span.setAttribute('workflow.p', p);
      span.setAttribute('workflow.x', x);
      span.setAttribute('user.id', ctx.token.sub);
      
      const result = await runtime.execute(p, x, ctx);
      
      span.setAttribute('workflow.status', result.status);
      span.setStatus({ code: result.status >= 400 ? 2 : 0 });
      
      return result;
    } catch (err) {
      span.recordException(err);
      span.setStatus({ code: 2, message: err.message });
      throw err;
    } finally {
      span.end();
    }
  });
}
```

### 4. Alerting Rules

```yaml
# alerting.yml
groups:
  - name: omar_alerts
    rules:
      - alert: HighErrorRate
        expr: rate(workflow_errors_total[5m]) > 0.1
        annotations:
          summary: "High workflow error rate"
          description: "Error rate is {{ $value }} per second"
      
      - alert: SlowWorkflowExecution
        expr: histogram_quantile(0.95, workflow_execution_duration_ms) > 1000
        annotations:
          summary: "Slow workflow execution"
          description: "P95 latency is {{ $value }}ms"
      
      - alert: UnauthorizedAccessAttempts
        expr: rate(workflow_errors{status="403"}[5m]) > 5
        annotations:
          summary: "High rate of 403 errors"
          description: "{{ $value }} unauthorized attempts per second"
```

---

## Migration Strategies

### Strategy 1: Incremental Migration

**Step 1**: Start with read-only operations
```xml
<!-- Migrate search/list operations first (safe, no writes) -->
<workflow id="contact_search">
  <entry p="contact" x="search" node="validate"/>
  <!-- ... -->
</workflow>
```

**Step 2**: Add create operations (with validation)
```xml
<workflow id="contact_create">
  <entry p="contact" x="create" node="validate"/>
  <!-- ... -->
</workflow>
```

**Step 3**: Migrate update operations
```xml
<workflow id="contact_update">
  <entry p="contact" x="update" node="load"/>
  <!-- ... -->
</workflow>
```

**Step 4**: Finally, migrate delete (most risky)
```xml
<workflow id="contact_delete">
  <entry p="contact" x="delete" node="auth_check"/>
  <node id="delete" kind="external" op="0x0103" 
        actor="human" confirmation="confirmed"/>
</workflow>
```

### Strategy 2: Parallel Run

Run both old and new systems in parallel:

```javascript
async function createContact(input) {
  // Run both systems
  const [oldResult, newResult] = await Promise.allSettled([
    oldSystem.createContact(input),
    omarRuntime.execute('contact', 'create', { input })
  ]);
  
  // Compare results
  if (!deepEqual(oldResult.value, newResult.value)) {
    logger.warn('Result mismatch', {
      old: oldResult.value,
      new: newResult.value
    });
  }
  
  // Use old system result (for now)
  return oldResult.value;
}
```

### Strategy 3: Shadow Mode

New system observes but doesn't affect production:

```javascript
async function createContact(input) {
  // Production write
  const result = await oldSystem.createContact(input);
  
  // Shadow execution (async, non-blocking)
  omarRuntime.execute('contact', 'create', { input })
    .then(shadowResult => {
      metrics.record('shadow.execution', {
        matches: deepEqual(result, shadowResult),
        old: result,
        new: shadowResult
      });
    })
    .catch(err => {
      logger.error('Shadow execution failed', { error: err });
    });
  
  return result;
}
```

Once shadow execution has high confidence, cutover to new system.

---

## Conclusion

These patterns and practices will help you:

1. **Write better workflows** - Clear, maintainable, correct
2. **Avoid common pitfalls** - Learn from others' mistakes
3. **Optimize performance** - Cache, parallelize, early-exit
4. **Handle errors gracefully** - Fallbacks, retries, circuits
5. **Test thoroughly** - Unit, integration, property-based
6. **Debug effectively** - Traces, logs, dry-runs
7. **Deploy safely** - Blue-green, hot-reload, rollback
8. **Monitor production** - Metrics, logs, traces, alerts
9. **Migrate incrementally** - Parallel run, shadow mode, cutover

Remember: **Business logic is data, not code**. Keep workflows simple, compose through edges, and let the graph speak for itself.

---

*End of Document*
