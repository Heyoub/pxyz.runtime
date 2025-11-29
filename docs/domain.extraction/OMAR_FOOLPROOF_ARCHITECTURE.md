# Omar Foolproof Architecture
## XML → Graph.bin → WAT: The Complete Spec

> "State is a lie. Events are truth. The graph is physics."

---

## The Three Layers

```
┌──────────────────────────────────────────────────────────────────────────┐
│                                                                          │
│  LAYER 1: HUMAN INTERFACE (DSL)                                          │
│  ─────────────────────────────                                           │
│  Format: XML (or TOML/JSON) - Excel-editable                             │
│  Contents: Workflows, Nodes, Edges, Predicates, Schemas                  │
│  Validation: XSD schema, type-checked predicates                         │
│  Purpose: What business users and developers edit                        │
│                                                                          │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  LAYER 2: COMPILED ARTIFACT (graph.bin)                                  │
│  ────────────────────────────────────────                                │
│  Format: Binary with versioned header                                    │
│  Contents: Node table, Edge table, Predicate bytecode, String pool       │
│  Validation: SHA-256 hash, version compatibility check                   │
│  Purpose: What the runtime loads and executes                            │
│                                                                          │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  LAYER 3: EXECUTION ENGINE (pxyz.wat)                                    │
│  ─────────────────────────────────────                                   │
│  Format: WebAssembly Text (~500 lines)                                   │
│  Contents: Graph traversal, Predicate VM, IO dispatch                    │
│  Validation: Formally verifiable, auditable                              │
│  Purpose: THE SYSTEM - deterministic graph execution                     │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## Layer 1: XML DSL (The Human Interface)

### Why XML?

1. **You already have a parser** - leverage existing infrastructure
2. **Excel-friendly** - can round-trip through Excel for power users
3. **XSD validation** - catch errors at author time, not runtime
4. **Strict hierarchy** - perfect for node/edge/predicate structure
5. **Diffable** - version control works great

### The XML Schema

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!--
  OMAR WORKFLOW DSL
  Version: 1.0.0
  
  This schema defines business workflows as directed graphs.
  The compiler validates against this schema before generating graph.bin.
-->
<omar version="1.0.0" xmlns="https://omar.dev/schema/v1">
  
  <!-- ════════════════════════════════════════════════════════════════ -->
  <!-- SCHEMAS: Define the data shapes that flow through the graph      -->
  <!-- ════════════════════════════════════════════════════════════════ -->
  <schemas>
    <schema name="Contact">
      <field name="id" type="uuid" required="true" />
      <field name="name" type="string" required="true" minLength="1" maxLength="200" />
      <field name="email" type="string" required="true" pattern="^[^@]+@[^@]+\.[^@]+$" />
      <field name="phone" type="string" required="false" />
      <field name="company" type="string" required="false" />
      <field name="tags" type="array" items="string" required="false" default="[]" />
      <field name="tenant_id" type="uuid" required="true" />
      <field name="owner_id" type="uuid" required="true" />
    </schema>
    
    <schema name="Deal">
      <field name="id" type="uuid" required="true" />
      <field name="title" type="string" required="true" />
      <field name="amount" type="decimal" required="true" min="0" maxDecimals="2" />
      <field name="status" type="enum" values="open,negotiating,won,lost" default="open" />
      <field name="contact_id" type="uuid" required="true" />
      <field name="tenant_id" type="uuid" required="true" />
    </schema>
    
    <schema name="SearchQuery">
      <field name="query" type="string" required="true" minLength="1" />
      <field name="limit" type="integer" required="false" default="50" min="1" max="1000" />
      <field name="offset" type="integer" required="false" default="0" min="0" />
    </schema>
  </schemas>
  
  <!-- ════════════════════════════════════════════════════════════════ -->
  <!-- PREDICATES: Reusable conditions for edges and auth               -->
  <!-- These compile to bytecode that the WAT predicate VM evaluates    -->
  <!-- ════════════════════════════════════════════════════════════════ -->
  <predicates>
    <!-- Auth predicates -->
    <predicate id="is_admin">
      <contains left="$token.perms" right="admin" />
    </predicate>
    
    <predicate id="is_owner">
      <eq left="$entity.owner_id" right="$token.sub" />
    </predicate>
    
    <predicate id="same_tenant">
      <eq left="$entity.tenant_id" right="$token.tenant" />
    </predicate>
    
    <predicate id="can_read_contact">
      <or>
        <ref predicate="is_admin" />
        <and>
          <ref predicate="same_tenant" />
          <contains left="$token.perms" right="contact:read" />
        </and>
      </or>
    </predicate>
    
    <predicate id="can_write_contact">
      <or>
        <ref predicate="is_admin" />
        <and>
          <ref predicate="same_tenant" />
          <or>
            <ref predicate="is_owner" />
            <contains left="$token.perms" right="contact:write" />
          </or>
        </and>
      </or>
    </predicate>
    
    <!-- Validation predicates -->
    <predicate id="valid_email">
      <matches left="$input.email" pattern="^[^@]+@[^@]+\.[^@]+$" />
    </predicate>
    
    <predicate id="query_not_empty">
      <gt left="$fn.length($input.query)" right="0" />
    </predicate>
    
    <!-- Business predicates -->
    <predicate id="is_high_value_deal">
      <gt left="$entity.amount" right="100000" />
    </predicate>
    
    <predicate id="deal_is_open">
      <eq left="$entity.status" right="open" />
    </predicate>
  </predicates>
  
  <!-- ════════════════════════════════════════════════════════════════ -->
  <!-- WORKFLOWS: The actual business logic as graphs                   -->
  <!-- ════════════════════════════════════════════════════════════════ -->
  <workflow id="contact_search" description="Search contacts by query string">
    
    <!-- Entry point: Maps (P=contact, X=search) to starting node -->
    <entry p="contact" x="search" node="validate" />
    
    <!-- Nodes: The steps in the workflow -->
    <nodes>
      <!-- Transform: Validate input -->
      <node id="validate" kind="transform" op="validate_input">
        <schema ref="SearchQuery" />
        <output name="valid" type="boolean" />
        <output name="error" type="string" optional="true" />
      </node>
      
      <!-- Auth: Check permissions -->
      <node id="auth_check" kind="auth">
        <require predicate="can_read_contact" />
      </node>
      
      <!-- External: Call Google Contacts API -->
      <node id="search_google" kind="external" op="0x0300">
        <input from="$state.validate.query" />
        <output name="results" type="array" />
      </node>
      
      <!-- External: Enrich with Qdrant embeddings -->
      <node id="enrich_qdrant" kind="external" op="0x0700">
        <input from="$state.search_google.results" />
        <output name="enriched" type="array" />
      </node>
      
      <!-- Render: Output HTML fragment -->
      <node id="render_list" kind="render">
        <template ref="contact_list" />
        <selector>#content</selector>
        <data from="$state.enrich_qdrant.enriched" />
      </node>
      
      <!-- Signal: Update UI state -->
      <node id="update_signals" kind="signal">
        <set signal="totalResults" value="$fn.length($state.enrich_qdrant.enriched)" />
        <set signal="loading" value="false" />
      </node>
      
      <!-- Render: Error output -->
      <node id="render_error" kind="render">
        <template ref="error" />
        <selector>#content</selector>
        <data from="$state.validate.error" />
      </node>
      
      <!-- Terminal: Unauthorized -->
      <node id="unauthorized" kind="terminal" status="403">
        <message>You don't have permission to search contacts.</message>
      </node>
    </nodes>
    
    <!-- Edges: Transitions between nodes with predicates -->
    <edges>
      <!-- Validation success → auth check -->
      <edge from="validate" to="auth_check">
        <when><eq left="$state.validate.valid" right="true" /></when>
      </edge>
      
      <!-- Validation failure → error -->
      <edge from="validate" to="render_error">
        <when><eq left="$state.validate.valid" right="false" /></when>
      </edge>
      
      <!-- Auth success → search -->
      <edge from="auth_check" to="search_google">
        <when><always /></when>
      </edge>
      
      <!-- Auth failure → unauthorized (implicit in auth node) -->
      <edge from="auth_check" to="unauthorized">
        <when><fail /></when>
      </edge>
      
      <!-- Search → enrich (always, no predicate) -->
      <edge from="search_google" to="enrich_qdrant" />
      
      <!-- Enrich → parallel render + signal update -->
      <edge from="enrich_qdrant" to="render_list" />
      <edge from="enrich_qdrant" to="update_signals" />
    </edges>
  </workflow>
  
  <!-- ════════════════════════════════════════════════════════════════ -->
  <!-- TEMPLATES: HTML fragments for render nodes                       -->
  <!-- ════════════════════════════════════════════════════════════════ -->
  <templates>
    <template id="contact_list">
      <![CDATA[
      <div class="contact-list">
        {{#each contacts}}
        <div class="contact-card" 
             data-on:click="$selectedContactId = '{{id}}'; @pxyz('contact', 'detail')">
          <h3>{{name}}</h3>
          <p>{{email}}</p>
          {{#if company}}<p class="muted">{{company}}</p>{{/if}}
        </div>
        {{/each}}
      </div>
      ]]>
    </template>
    
    <template id="error">
      <![CDATA[
      <div class="error-message">
        <strong>Error:</strong> {{message}}
      </div>
      ]]>
    </template>
  </templates>
  
</omar>
```

### XSD Schema (for validation)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           targetNamespace="https://omar.dev/schema/v1"
           xmlns:omar="https://omar.dev/schema/v1">
  
  <!-- Root element -->
  <xs:element name="omar">
    <xs:complexType>
      <xs:sequence>
        <xs:element name="schemas" type="omar:SchemasType" />
        <xs:element name="predicates" type="omar:PredicatesType" />
        <xs:element name="workflow" type="omar:WorkflowType" maxOccurs="unbounded" />
        <xs:element name="templates" type="omar:TemplatesType" />
      </xs:sequence>
      <xs:attribute name="version" type="xs:string" use="required" />
    </xs:complexType>
  </xs:element>
  
  <!-- Field types allowed in schemas -->
  <xs:simpleType name="FieldType">
    <xs:restriction base="xs:string">
      <xs:enumeration value="uuid" />
      <xs:enumeration value="string" />
      <xs:enumeration value="integer" />
      <xs:enumeration value="decimal" />
      <xs:enumeration value="boolean" />
      <xs:enumeration value="date" />
      <xs:enumeration value="datetime" />
      <xs:enumeration value="array" />
      <xs:enumeration value="enum" />
    </xs:restriction>
  </xs:simpleType>
  
  <!-- Node kinds -->
  <xs:simpleType name="NodeKind">
    <xs:restriction base="xs:string">
      <xs:enumeration value="transform" />
      <xs:enumeration value="external" />
      <xs:enumeration value="render" />
      <xs:enumeration value="signal" />
      <xs:enumeration value="auth" />
      <xs:enumeration value="terminal" />
    </xs:restriction>
  </xs:simpleType>
  
  <!-- Predicate operators -->
  <xs:simpleType name="PredicateOp">
    <xs:restriction base="xs:string">
      <xs:enumeration value="eq" />
      <xs:enumeration value="neq" />
      <xs:enumeration value="gt" />
      <xs:enumeration value="gte" />
      <xs:enumeration value="lt" />
      <xs:enumeration value="lte" />
      <xs:enumeration value="contains" />
      <xs:enumeration value="matches" />
      <xs:enumeration value="and" />
      <xs:enumeration value="or" />
      <xs:enumeration value="not" />
      <xs:enumeration value="always" />
      <xs:enumeration value="fail" />
    </xs:restriction>
  </xs:simpleType>
  
  <!-- ... more definitions ... -->
</xs:schema>
```

---

## Layer 2: Binary Format (graph.bin)

### Header Structure

```
┌────────────────────────────────────────────────────────────────────┐
│ Offset │ Size │ Field           │ Description                      │
├────────┼──────┼─────────────────┼──────────────────────────────────┤
│ 0x0000 │ 4    │ magic           │ 0x504E5958 ("PXYZ")              │
│ 0x0004 │ 2    │ version_major   │ Binary format version (major)    │
│ 0x0006 │ 2    │ version_minor   │ Binary format version (minor)    │
│ 0x0008 │ 4    │ node_count      │ Number of nodes                  │
│ 0x000C │ 4    │ edge_count      │ Number of edges                  │
│ 0x0010 │ 4    │ predicate_count │ Number of predicates             │
│ 0x0014 │ 4    │ string_pool_size│ Size of string pool in bytes     │
│ 0x0018 │ 4    │ entry_count     │ Number of entry points           │
│ 0x001C │ 4    │ schema_count    │ Number of schemas                │
│ 0x0020 │ 32   │ source_hash     │ SHA-256 of source XML            │
│ 0x0040 │ 4    │ nodes_offset    │ Offset to node table             │
│ 0x0044 │ 4    │ edges_offset    │ Offset to edge table             │
│ 0x0048 │ 4    │ predicates_off  │ Offset to predicate table        │
│ 0x004C │ 4    │ strings_offset  │ Offset to string pool            │
│ 0x0050 │ 4    │ entries_offset  │ Offset to entry point table      │
│ 0x0054 │ 4    │ schemas_offset  │ Offset to schema table           │
│ 0x0058 │ 8    │ reserved        │ Reserved for future use          │
└────────────────────────────────────────────────────────────────────┘
Total header size: 0x0060 (96 bytes)
```

### Node Table Entry (16 bytes per node)

```
┌────────────────────────────────────────────────────────────────────┐
│ Offset │ Size │ Field           │ Description                      │
├────────┼──────┼─────────────────┼──────────────────────────────────┤
│ 0x00   │ 4    │ id              │ Node ID (unique within workflow) │
│ 0x04   │ 1    │ kind            │ Node kind (transform=0, ext=1..) │
│ 0x05   │ 1    │ flags           │ Node flags (see below)           │
│ 0x06   │ 2    │ op_code         │ Operation code (for external)    │
│ 0x08   │ 4    │ data_offset     │ Offset into string pool          │
│ 0x0C   │ 2    │ edge_start      │ First outgoing edge index        │
│ 0x0E   │ 2    │ edge_count      │ Number of outgoing edges         │
└────────────────────────────────────────────────────────────────────┘

Node kinds:
  0 = transform   (validate, transform data)
  1 = external    (call host IO)
  2 = render      (produce HTML fragment)
  3 = signal      (emit Datastar signals)
  4 = auth        (check permissions)
  5 = terminal    (end traversal)

Node flags:
  bit 0 = async            (may block)
  bit 1 = requires_auth    (must pass auth predicate first)
  bit 2 = emits_event      (produces audit event)
  bit 3 = cacheable        (result can be memoized)
```

### Edge Table Entry (12 bytes per edge)

```
┌────────────────────────────────────────────────────────────────────┐
│ Offset │ Size │ Field           │ Description                      │
├────────┼──────┼─────────────────┼──────────────────────────────────┤
│ 0x00   │ 4    │ target_node     │ Target node ID                   │
│ 0x04   │ 4    │ predicate_id    │ Predicate ID (0 = always true)   │
│ 0x08   │ 2    │ weight          │ Edge weight (for optimization)   │
│ 0x0A   │ 2    │ flags           │ Edge flags                       │
└────────────────────────────────────────────────────────────────────┘

Edge flags:
  bit 0 = parallel         (can execute with siblings)
  bit 1 = fallback         (only if others fail)
  bit 2 = conditional_only (don't execute node, just check)
```

### Predicate Bytecode

Predicates compile to a simple stack-based bytecode:

```
┌────────────────────────────────────────────────────────────────────┐
│ Opcode │ Size │ Name      │ Stack Effect          │ Description    │
├────────┼──────┼───────────┼───────────────────────┼────────────────┤
│ 0x00   │ 1    │ NOOP      │ -                     │ No operation   │
│ 0x01   │ 5    │ PUSH_INT  │ - → value             │ Push i32       │
│ 0x02   │ 5    │ PUSH_STR  │ - → str_offset        │ Push string ref│
│ 0x03   │ 2    │ LOAD_VAR  │ - → value             │ Load from ctx  │
│ 0x04   │ 5    │ LOAD_FIELD│ obj → value           │ Get field      │
│ 0x10   │ 1    │ EQ        │ a, b → bool           │ a == b         │
│ 0x11   │ 1    │ NEQ       │ a, b → bool           │ a != b         │
│ 0x12   │ 1    │ GT        │ a, b → bool           │ a > b          │
│ 0x13   │ 1    │ GTE       │ a, b → bool           │ a >= b         │
│ 0x14   │ 1    │ LT        │ a, b → bool           │ a < b          │
│ 0x15   │ 1    │ LTE       │ a, b → bool           │ a <= b         │
│ 0x20   │ 1    │ AND       │ a, b → bool           │ a && b         │
│ 0x21   │ 1    │ OR        │ a, b → bool           │ a || b         │
│ 0x22   │ 1    │ NOT       │ a → bool              │ !a             │
│ 0x30   │ 1    │ CONTAINS  │ arr, val → bool       │ arr has val    │
│ 0x31   │ 5    │ MATCHES   │ str → bool            │ regex match    │
│ 0x40   │ 1    │ LEN       │ arr → int             │ array length   │
│ 0x41   │ 1    │ GET       │ arr, idx → val        │ array access   │
│ 0xF0   │ 3    │ CALL_PRED │ - → bool              │ call predicate │
│ 0xFF   │ 1    │ RET       │ bool → -              │ return result  │
└────────────────────────────────────────────────────────────────────┘

Maximum predicate bytecode size: 256 bytes
Maximum stack depth: 16
Maximum CALL_PRED depth: 4 (prevents infinite recursion)
```

### Entry Point Table (8 bytes per entry)

```
┌────────────────────────────────────────────────────────────────────┐
│ Offset │ Size │ Field           │ Description                      │
├────────┼──────┼─────────────────┼──────────────────────────────────┤
│ 0x00   │ 4    │ px_hash         │ Hash of (P, X) coordinate        │
│ 0x04   │ 4    │ node_id         │ Starting node ID                 │
└────────────────────────────────────────────────────────────────────┘
```

---

## Layer 3: WAT Runtime (pxyz.wat)

### Core API

```wat
(module
  ;; ══════════════════════════════════════════════════════════════════
  ;; IMPORTS: Host provides all IO
  ;; ══════════════════════════════════════════════════════════════════
  
  ;; Generic external call
  (import "io" "call"
    (func $io_call
      (param $op i32)           ;; Operation code
      (param $input_ptr i32)    ;; Input data pointer
      (param $input_len i32)    ;; Input length
      (param $output_ptr i32)   ;; Where to write result
      (result i32)))            ;; Result length or -error
  
  ;; Logging
  (import "io" "log"
    (func $io_log
      (param $level i32)        ;; 0=trace, 1=debug, 2=info, 3=warn, 4=error
      (param $msg_ptr i32)
      (param $msg_len i32)))
  
  ;; Audit event emission
  (import "io" "audit"
    (func $io_audit
      (param $event_ptr i32)
      (param $event_len i32)))
  
  ;; ══════════════════════════════════════════════════════════════════
  ;; MEMORY LAYOUT
  ;; ══════════════════════════════════════════════════════════════════
  
  (memory (export "memory") 64)  ;; 4MB initial
  
  ;; Memory regions:
  ;; 0x000000 - 0x00FFFF: Graph data (loaded from graph.bin)
  ;; 0x010000 - 0x01FFFF: Execution state (stack, visited nodes)
  ;; 0x020000 - 0x02FFFF: Input/output buffers
  ;; 0x030000 - 0x03FFFF: Predicate evaluation stack
  ;; 0x040000+:           Heap (dynamic allocation)
  
  (global $graph_loaded (mut i32) (i32.const 0))
  (global $heap_ptr (mut i32) (i32.const 0x040000))
  
  ;; ══════════════════════════════════════════════════════════════════
  ;; EXPORTS: The complete API
  ;; ══════════════════════════════════════════════════════════════════
  
  ;; Load graph binary (call once at startup)
  (func (export "load_graph")
    (param $data_ptr i32)
    (param $data_len i32)
    (result i32)                ;; 0 = success, <0 = error
    
    ;; 1. Validate magic number
    ;; 2. Check version compatibility
    ;; 3. Copy to graph region
    ;; 4. Set $graph_loaded = 1
    ;; ... implementation ...
    (i32.const 0)
  )
  
  ;; Main entry point: Execute PXYZ operation
  (func (export "pxyz")
    (param $p i32)              ;; P coordinate (entity hash)
    (param $x i32)              ;; X coordinate (operation hash)
    (param $y_ptr i32)          ;; Y context (JSON in buffer)
    (param $y_len i32)
    (param $z i64)              ;; Z timestamp
    (param $input_ptr i32)      ;; Input data pointer
    (param $input_len i32)
    (param $output_ptr i32)     ;; Output buffer pointer
    (result i32)                ;; Output length or -error
    
    (local $entry_node i32)
    (local $traversal_result i32)
    
    ;; 1. Check graph is loaded
    (if (i32.eqz (global.get $graph_loaded))
      (then (return (i32.const -1))))  ;; Error: graph not loaded
    
    ;; 2. Find entry node for (P, X)
    (local.set $entry_node
      (call $find_entry
        (local.get $p)
        (local.get $x)))
    
    (if (i32.lt_s (local.get $entry_node) (i32.const 0))
      (then (return (i32.const -2))))  ;; Error: no entry point
    
    ;; 3. Initialize traversal state
    (call $init_traversal_state)
    
    ;; 4. Traverse graph
    (local.set $traversal_result
      (call $traverse
        (local.get $entry_node)
        (local.get $y_ptr)
        (local.get $y_len)
        (local.get $input_ptr)
        (local.get $input_len)
        (local.get $output_ptr)))
    
    ;; 5. Return result
    (local.get $traversal_result)
  )
  
  ;; ══════════════════════════════════════════════════════════════════
  ;; TRAVERSAL ENGINE
  ;; ══════════════════════════════════════════════════════════════════
  
  (func $traverse
    (param $node_id i32)
    (param $y_ptr i32)
    (param $y_len i32)
    (param $input_ptr i32)
    (param $input_len i32)
    (param $output_ptr i32)
    (result i32)
    
    (local $node_ptr i32)
    (local $node_kind i32)
    (local $visited_count i32)
    (local $edge_start i32)
    (local $edge_count i32)
    (local $i i32)
    (local $edge_ptr i32)
    (local $target_node i32)
    (local $predicate_id i32)
    
    ;; SAFETY: Check visited count to prevent infinite loops
    (local.set $visited_count (call $get_visited_count))
    (if (i32.gt_u (local.get $visited_count) (i32.const 1000))
      (then (return (i32.const -3))))  ;; Error: traversal depth exceeded
    
    ;; SAFETY: Check if already visited (cycle detection)
    (if (call $is_visited (local.get $node_id))
      (then (return (i32.const 0))))   ;; Already visited, skip
    
    ;; Mark as visited
    (call $mark_visited (local.get $node_id))
    
    ;; Get node pointer and kind
    (local.set $node_ptr (call $get_node_ptr (local.get $node_id)))
    (local.set $node_kind (i32.load8_u offset=4 (local.get $node_ptr)))
    
    ;; Execute node based on kind
    (block $done
      (block $terminal
      (block $auth
      (block $signal
      (block $render
      (block $external
      (block $transform
        (br_table $transform $external $render $signal $auth $terminal $done
          (local.get $node_kind))
      ) ;; transform
        (call $exec_transform
          (local.get $node_ptr)
          (local.get $input_ptr)
          (local.get $input_len)
          (local.get $output_ptr))
        (br $done)
      ) ;; external
        (call $exec_external
          (local.get $node_ptr)
          (local.get $input_ptr)
          (local.get $input_len)
          (local.get $output_ptr))
        (br $done)
      ) ;; render
        (call $exec_render
          (local.get $node_ptr)
          (local.get $output_ptr))
        (br $done)
      ) ;; signal
        (call $exec_signal
          (local.get $node_ptr)
          (local.get $output_ptr))
        (br $done)
      ) ;; auth
        (if (i32.eqz (call $exec_auth (local.get $node_ptr) (local.get $y_ptr) (local.get $y_len)))
          (then (return (i32.const -403))))  ;; Auth failed
        (br $done)
      ) ;; terminal
        (return (call $exec_terminal (local.get $node_ptr) (local.get $output_ptr)))
    )
    
    ;; Traverse outgoing edges
    (local.set $edge_start (i32.load16_u offset=12 (local.get $node_ptr)))
    (local.set $edge_count (i32.load16_u offset=14 (local.get $node_ptr)))
    
    (local.set $i (i32.const 0))
    (block $edge_done
      (loop $edge_loop
        (br_if $edge_done (i32.ge_u (local.get $i) (local.get $edge_count)))
        
        ;; Get edge
        (local.set $edge_ptr
          (call $get_edge_ptr
            (i32.add (local.get $edge_start) (local.get $i))))
        
        (local.set $target_node (i32.load offset=0 (local.get $edge_ptr)))
        (local.set $predicate_id (i32.load offset=4 (local.get $edge_ptr)))
        
        ;; Evaluate predicate
        (if (call $eval_predicate
              (local.get $predicate_id)
              (local.get $y_ptr)
              (local.get $y_len))
          (then
            ;; Recurse into target node
            (drop (call $traverse
              (local.get $target_node)
              (local.get $y_ptr)
              (local.get $y_len)
              (local.get $input_ptr)
              (local.get $input_len)
              (local.get $output_ptr)))))
        
        ;; Next edge
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $edge_loop)
      )
    )
    
    (i32.const 0)  ;; Success
  )
  
  ;; ══════════════════════════════════════════════════════════════════
  ;; PREDICATE VM
  ;; ══════════════════════════════════════════════════════════════════
  
  (func $eval_predicate
    (param $pred_id i32)
    (param $y_ptr i32)
    (param $y_len i32)
    (result i32)  ;; 0 = false, 1 = true
    
    (local $bytecode_ptr i32)
    (local $bytecode_len i32)
    (local $stack_ptr i32)
    (local $pc i32)
    (local $op i32)
    (local $call_depth i32)
    (local $steps i32)
    
    ;; Predicate 0 = always true
    (if (i32.eqz (local.get $pred_id))
      (then (return (i32.const 1))))
    
    ;; Get bytecode location
    (local.set $bytecode_ptr (call $get_predicate_ptr (local.get $pred_id)))
    (local.set $bytecode_len (i32.load16_u (local.get $bytecode_ptr)))
    (local.set $bytecode_ptr (i32.add (local.get $bytecode_ptr) (i32.const 2)))
    
    ;; Initialize predicate stack
    (local.set $stack_ptr (i32.const 0x030000))
    (local.set $pc (i32.const 0))
    (local.set $call_depth (i32.const 0))
    (local.set $steps (i32.const 0))
    
    ;; SAFETY: Maximum 256 steps per predicate
    (block $vm_done (result i32)
      (loop $vm_loop
        ;; Step limit
        (local.set $steps (i32.add (local.get $steps) (i32.const 1)))
        (br_if $vm_done (i32.gt_u (local.get $steps) (i32.const 256))
          (i32.const 0))  ;; Return false on timeout
        
        ;; Bounds check
        (br_if $vm_done (i32.ge_u (local.get $pc) (local.get $bytecode_len))
          (i32.const 0))
        
        ;; Fetch opcode
        (local.set $op
          (i32.load8_u
            (i32.add (local.get $bytecode_ptr) (local.get $pc))))
        (local.set $pc (i32.add (local.get $pc) (i32.const 1)))
        
        ;; Execute opcode
        (block $op_ret
        (block $op_call_pred
        (block $op_not
        (block $op_or
        (block $op_and
        (block $op_eq
        (block $op_push_int
        (block $op_noop
          (br_table
            $op_noop      ;; 0x00
            $op_push_int  ;; 0x01
            ;; ... more opcodes ...
            $op_eq        ;; 0x10
            ;; ...
            $op_and       ;; 0x20
            $op_or        ;; 0x21
            $op_not       ;; 0x22
            ;; ...
            $op_call_pred ;; 0xF0
            $op_ret       ;; 0xFF
            (local.get $op))
        ) ;; op_noop
          (br $vm_loop)
        ) ;; op_push_int
          ;; Push 4-byte integer from bytecode
          ;; ... implementation ...
          (br $vm_loop)
        ) ;; op_eq
          ;; Pop two values, push (a == b)
          ;; ... implementation ...
          (br $vm_loop)
        ) ;; op_and
          ;; Pop two bools, push (a && b)
          ;; ... implementation ...
          (br $vm_loop)
        ) ;; op_or
          ;; Pop two bools, push (a || b)
          ;; ... implementation ...
          (br $vm_loop)
        ) ;; op_not
          ;; Pop bool, push !a
          ;; ... implementation ...
          (br $vm_loop)
        ) ;; op_call_pred
          ;; SAFETY: Max call depth
          (br_if $vm_done (i32.ge_u (local.get $call_depth) (i32.const 4))
            (i32.const 0))
          ;; ... recursive predicate call ...
          (br $vm_loop)
        ) ;; op_ret
          ;; Return top of stack
          (br $vm_done (i32.load (local.get $stack_ptr)))
        )
        
        ;; Unreachable - unknown opcode
        (br $vm_done (i32.const 0))
      )
    )
  )
  
  ;; ... helper functions ...
)
```

---

## Safety Guarantees

### 1. Predicate VM is NOT Turing-Complete

```
┌────────────────────────────────────────────────────────────────────┐
│ WHAT PREDICATES CAN DO:                                            │
│ ✓ Compare values (eq, lt, gt, etc.)                                │
│ ✓ Boolean operations (and, or, not)                                │
│ ✓ Read from Y-context (token, entity, input)                       │
│ ✓ Call other predicates (max depth 4)                              │
│ ✓ Simple functions (length, contains, matches)                     │
│                                                                    │
│ WHAT PREDICATES CANNOT DO:                                         │
│ ✗ Unbounded loops                                                  │
│ ✗ Arbitrary memory access                                          │
│ ✗ Side effects                                                     │
│ ✗ Call host IO                                                     │
│ ✗ Create new data                                                  │
│                                                                    │
│ LIMITS:                                                            │
│ • Max 256 bytecode bytes per predicate                             │
│ • Max 256 VM steps per evaluation                                  │
│ • Max 16 stack depth                                               │
│ • Max 4 nested predicate calls                                     │
└────────────────────────────────────────────────────────────────────┘
```

### 2. Traversal is Bounded

```
┌────────────────────────────────────────────────────────────────────┐
│ TRAVERSAL SAFETY:                                                  │
│                                                                    │
│ • Max 1000 nodes visited per traversal                             │
│ • Cycle detection via visited set                                  │
│ • No recursive workflows (validated at compile time)               │
│ • Each node executes exactly once per traversal                    │
│                                                                    │
│ COMPILE-TIME VALIDATION:                                           │
│ • Compiler detects cycles in non-loop workflows                    │
│ • All predicate references must exist                              │
│ • All node references must exist                                   │
│ • Entry points must have valid targets                             │
└────────────────────────────────────────────────────────────────────┘
```

### 3. Memory Isolation

```
┌────────────────────────────────────────────────────────────────────┐
│ WASM SANDBOX:                                                      │
│                                                                    │
│ • WAT cannot access host memory directly                           │
│ • All IO goes through explicit imports                             │
│ • Graph data is immutable after load                               │
│ • Execution state is reset per call                                │
│                                                                    │
│ MEMORY REGIONS:                                                    │
│ • 0x000000-0x00FFFF: Graph (read-only after load)                  │
│ • 0x010000-0x01FFFF: Traversal state (reset per call)              │
│ • 0x020000-0x02FFFF: I/O buffers (caller-controlled)               │
│ • 0x030000-0x03FFFF: Predicate stack (reset per eval)              │
└────────────────────────────────────────────────────────────────────┘
```

---

## Compilation Pipeline

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│   1. PARSE                                                              │
│      workflow.xml → parse → AST                                         │
│                                                                         │
│   2. VALIDATE                                                           │
│      AST → XSD validation                                               │
│      AST → type-check predicates against schemas                        │
│      AST → detect cycles                                                │
│      AST → verify all refs exist                                        │
│                                                                         │
│   3. OPTIMIZE                                                           │
│      AST → deduplicate predicates                                       │
│      AST → inline trivial predicates (always, fail)                     │
│      AST → compute optimal node ordering                                │
│                                                                         │
│   4. COMPILE                                                            │
│      AST → node table                                                   │
│      AST → edge table                                                   │
│      AST → predicate bytecode                                           │
│      AST → string pool                                                  │
│      AST → entry point table                                            │
│                                                                         │
│   5. LINK                                                               │
│      All tables → graph.bin                                             │
│      Hash source XML → embed in header                                  │
│                                                                         │
│   6. VERIFY                                                             │
│      Decompile graph.bin → verify matches AST                           │
│      Run property tests on graph structure                              │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Compiler Command

```bash
# Compile XML to binary
omar compile workflow.xml -o graph.bin

# Options:
#   --validate    Run full validation suite
#   --optimize    Enable optimizations
#   --debug       Include debug info (source maps)
#   --hash        Print SHA-256 of output
#   --decompile   Also output readable dump
#   --mermaid     Generate Mermaid diagram

# Example with all options:
omar compile \
  --validate \
  --optimize \
  --debug \
  --mermaid workflow.mmd \
  workflow.xml \
  -o graph.bin

# Output:
# ✓ Parsed workflow.xml (23 nodes, 31 edges, 12 predicates)
# ✓ Validated against schema
# ✓ Type-checked predicates
# ✓ No cycles detected
# ✓ Optimized (deduped 3 predicates, inlined 2)
# ✓ Compiled to graph.bin (4,728 bytes)
# ✓ SHA-256: a7f3e2b1...
# ✓ Generated workflow.mmd
```

---

## Debugging Story

### 1. Trace Mode

```javascript
// In host IO adapter (io/browser.js)

const traceBuffer = [];

const debugIO = {
  ...browserIO,
  
  // Override to capture traces
  call: async (op, input) => {
    const start = performance.now();
    const result = await browserIO.call(op, input);
    const duration = performance.now() - start;
    
    traceBuffer.push({
      op,
      input: JSON.stringify(input).slice(0, 100),
      duration,
      success: result.success
    });
    
    return result;
  }
};

// Expose trace buffer for devtools
window.__omar_trace = () => traceBuffer;
```

### 2. Explain Endpoint

```javascript
// Given PXYZ coordinates, show the execution path

window.explain = async (p, x, y = {}) => {
  const instance = window.__omar_instance;
  
  // Enable trace mode
  instance.exports.set_trace_mode(1);
  
  // Execute
  const result = await window.pxyz(p, x, y);
  
  // Read trace
  const tracePtr = instance.exports.get_trace_ptr();
  const traceLen = instance.exports.get_trace_len();
  const trace = new Uint32Array(
    instance.exports.memory.buffer,
    tracePtr,
    traceLen / 4
  );
  
  // Format as readable path
  const path = [];
  for (let i = 0; i < trace.length; i += 3) {
    path.push({
      node: trace[i],
      edge: trace[i + 1],
      predicate_result: trace[i + 2] === 1
    });
  }
  
  // Disable trace mode
  instance.exports.set_trace_mode(0);
  
  return {
    result,
    path,
    mermaid: pathToMermaid(path)
  };
};
```

### 3. Decompiler Output

```bash
# Decompile graph.bin to readable format
omar decompile graph.bin

# Output:
# 
# OMAR GRAPH v1.0
# Source hash: a7f3e2b1...
# Compiled: 2025-01-20T15:30:00Z
#
# ENTRY POINTS:
#   (contact, search) → node 0 (validate)
#   (contact, detail) → node 10 (load_contact)
#   (deal, transition) → node 20 (auth_check)
#
# NODE TABLE:
#   [0] validate       transform  edges: [1, 2]
#   [1] auth_check     auth       edges: [3, 4]
#   [2] render_error   render     edges: []
#   [3] search_google  external   edges: [5]
#   ...
#
# EDGE TABLE:
#   [0] 0 → 1  when: predicate 1 (valid == true)
#   [1] 0 → 2  when: predicate 2 (valid == false)
#   ...
#
# PREDICATES:
#   [1] valid_true:
#       LOAD_VAR state.validate.valid
#       PUSH_INT 1
#       EQ
#       RET
#   [2] valid_false:
#       LOAD_VAR state.validate.valid
#       PUSH_INT 0
#       EQ
#       RET
#   ...
```

---

## Excel Integration

### Round-Trip Safety

```javascript
// compiler/excel.js

import * as XLSX from 'xlsx';

/**
 * Export workflow XML to Excel (for power users)
 */
export function toExcel(xmlPath, excelPath) {
  const xml = parseXML(fs.readFileSync(xmlPath));
  const workbook = XLSX.utils.book_new();
  
  // Sheet 1: Nodes
  const nodesData = xml.workflow.nodes.map(node => ({
    id: node.id,
    kind: node.kind,
    op: node.op || '',
    template: node.template || '',
    description: node.description || ''
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(nodesData),
    'Nodes'
  );
  
  // Sheet 2: Edges
  const edgesData = xml.workflow.edges.map(edge => ({
    from: edge.from,
    to: edge.to,
    predicate: edge.predicate || 'always',
    weight: edge.weight || 0
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(edgesData),
    'Edges'
  );
  
  // Sheet 3: Predicates
  // ...
  
  XLSX.writeFile(workbook, excelPath);
}

/**
 * Import Excel back to XML
 * 
 * SAFETY: Validates that round-trip produces identical XML
 */
export function fromExcel(excelPath, xmlPath) {
  const workbook = XLSX.readFile(excelPath);
  
  // Reconstruct XML from sheets
  const nodes = XLSX.utils.sheet_to_json(workbook.Sheets['Nodes']);
  const edges = XLSX.utils.sheet_to_json(workbook.Sheets['Edges']);
  // ...
  
  // Build XML
  const xml = buildXML({ nodes, edges, ... });
  
  // SAFETY: Validate against schema
  validateXSD(xml);
  
  fs.writeFileSync(xmlPath, xml);
}

/**
 * Test round-trip integrity
 */
export function verifyRoundTrip(xmlPath) {
  const original = fs.readFileSync(xmlPath, 'utf-8');
  
  // XML → Excel → XML
  toExcel(xmlPath, '/tmp/test.xlsx');
  fromExcel('/tmp/test.xlsx', '/tmp/test.xml');
  
  const reconstructed = fs.readFileSync('/tmp/test.xml', 'utf-8');
  
  // Compare normalized forms
  const originalNorm = normalizeXML(original);
  const reconstructedNorm = normalizeXML(reconstructed);
  
  if (originalNorm !== reconstructedNorm) {
    throw new Error('Round-trip integrity failure!');
  }
  
  console.log('✓ Round-trip verified');
}
```

### Excel Gotcha Protection

```javascript
// Protect against Excel's "helpful" formatting

const EXCEL_SAFE_CONFIG = {
  // Force text mode for these columns
  textColumns: ['id', 'from', 'to', 'predicate', 'pattern'],
  
  // Validate on import
  validators: {
    id: (val) => /^[a-z_][a-z0-9_]*$/.test(val),
    from: (val) => /^[a-z_][a-z0-9_]*$/.test(val),
    kind: (val) => ['transform', 'external', 'render', 'signal', 'auth', 'terminal'].includes(val)
  },
  
  // Excel mangles these - detect and reject
  dangerPatterns: [
    /^\d{1,2}\/\d{1,2}\/\d{2,4}$/,  // Dates (1/2/25)
    /^[\d.]+E[+-]\d+$/,              // Scientific notation
    /^0\d+$/,                         // Leading zeros stripped
  ]
};

function validateExcelImport(cell, column) {
  // Check for Excel mangling
  for (const pattern of EXCEL_SAFE_CONFIG.dangerPatterns) {
    if (pattern.test(String(cell))) {
      throw new Error(
        `Excel mangled value in column "${column}": "${cell}"\n` +
        `Tip: Format the column as Text before entering data.`
      );
    }
  }
  
  // Run column-specific validator
  const validator = EXCEL_SAFE_CONFIG.validators[column];
  if (validator && !validator(cell)) {
    throw new Error(
      `Invalid value in column "${column}": "${cell}"`
    );
  }
}
```

---

## Hot Reload

```javascript
// io/browser.js

let currentGraph = null;
let currentVersion = 0;

/**
 * Load or hot-reload graph
 */
export async function loadGraph(graphBin) {
  const instance = window.__omar_instance;
  const view = new DataView(graphBin);
  
  // Check magic
  const magic = view.getUint32(0, true);
  if (magic !== 0x504E5958) {
    throw new Error('Invalid graph: bad magic number');
  }
  
  // Check version
  const major = view.getUint16(4, true);
  const minor = view.getUint16(6, true);
  const version = major * 1000 + minor;
  
  // Version compatibility check
  const runtimeVersion = instance.exports.get_runtime_version();
  if (major > (runtimeVersion / 1000) >> 0) {
    throw new Error(
      `Graph version ${major}.${minor} requires newer runtime. ` +
      `Current runtime: ${(runtimeVersion / 1000) >> 0}.${runtimeVersion % 1000}`
    );
  }
  
  // Load into WASM memory
  const graphPtr = 0x000000;  // Graph region
  const bytes = new Uint8Array(graphBin);
  new Uint8Array(instance.exports.memory.buffer).set(bytes, graphPtr);
  
  // Initialize graph
  const result = instance.exports.load_graph(graphPtr, bytes.length);
  if (result < 0) {
    throw new Error(`Failed to load graph: error ${result}`);
  }
  
  // Update state
  currentGraph = graphBin;
  currentVersion = version;
  
  console.log(`✓ Loaded graph v${major}.${minor}`);
  return { version, nodeCount: view.getUint32(8, true) };
}

/**
 * Hot reload with zero downtime
 */
export async function hotReload(newGraphBin) {
  // Keep reference to old graph
  const oldGraph = currentGraph;
  const oldVersion = currentVersion;
  
  try {
    // Load new graph
    const { version } = await loadGraph(newGraphBin);
    
    console.log(`✓ Hot reloaded: v${oldVersion} → v${version}`);
    
    // In-flight requests complete on old graph
    // New requests use new graph
    // (This is automatic because WASM is single-threaded)
    
  } catch (error) {
    // Rollback on failure
    if (oldGraph) {
      await loadGraph(oldGraph);
    }
    throw error;
  }
}
```

---

## Testing Strategy

### 1. Property Tests on Compiler

```typescript
// tests/compiler.test.ts

import { fc } from '@fast-check/vitest';
import { compile, decompile } from '../compiler';

describe('Compiler', () => {
  it('round-trips any valid workflow', () => {
    fc.assert(
      fc.property(arbitraryWorkflow(), (workflow) => {
        const xml = workflowToXML(workflow);
        const bin = compile(xml);
        const decompiled = decompile(bin);
        
        // Structural equality (ignore whitespace/formatting)
        expect(normalizeWorkflow(decompiled))
          .toEqual(normalizeWorkflow(workflow));
      })
    );
  });
  
  it('detects all cycles', () => {
    fc.assert(
      fc.property(arbitraryWorkflowWithCycle(), (workflow) => {
        const xml = workflowToXML(workflow);
        expect(() => compile(xml)).toThrow(/cycle detected/i);
      })
    );
  });
  
  it('produces deterministic output', () => {
    fc.assert(
      fc.property(arbitraryWorkflow(), (workflow) => {
        const xml = workflowToXML(workflow);
        const bin1 = compile(xml);
        const bin2 = compile(xml);
        
        // Byte-for-byte identical
        expect(bin1).toEqual(bin2);
      })
    );
  });
});
```

### 2. Property Tests on Predicate VM

```typescript
// tests/predicate-vm.test.ts

describe('Predicate VM', () => {
  it('always terminates', () => {
    fc.assert(
      fc.property(arbitraryPredicate(), arbitraryContext(), (pred, ctx) => {
        const bytecode = compilePredicate(pred);
        
        // Must complete within step limit
        const { result, steps } = evalPredicate(bytecode, ctx);
        expect(steps).toBeLessThanOrEqual(256);
        expect(typeof result).toBe('boolean');
      })
    );
  });
  
  it('matches reference implementation', () => {
    fc.assert(
      fc.property(arbitraryPredicate(), arbitraryContext(), (pred, ctx) => {
        const bytecode = compilePredicate(pred);
        
        // Compare WAT VM result with JS reference
        const wasmResult = wasmEvalPredicate(bytecode, ctx);
        const refResult = jsEvalPredicate(pred, ctx);
        
        expect(wasmResult).toBe(refResult);
      })
    );
  });
});
```

### 3. Integration Tests

```typescript
// tests/integration.test.ts

describe('Full Pipeline', () => {
  it('contact search workflow works end-to-end', async () => {
    // Load real workflow
    const xml = await fs.readFile('workflows/contact_search.xml');
    const bin = compile(xml);
    
    // Load into WASM
    const instance = await loadWasm();
    await loadGraph(instance, bin);
    
    // Execute
    const result = await instance.exports.pxyz(
      hash('contact'),
      hash('search'),
      encodeContext({ token: mockToken, input: { query: 'John' } }),
      encodeInput({ query: 'John' })
    );
    
    // Verify
    const output = decodeOutput(result);
    expect(output.html).toContain('contact-list');
    expect(output.signals.loading).toBe(false);
  });
});
```

---

## Summary: The Foolproof Stack

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│  1. XML DSL                                                             │
│     └─ XSD validated                                                    │
│     └─ Type-checked predicates                                          │
│     └─ Excel round-trip safe                                            │
│     └─ Version controlled                                               │
│                                                                         │
│  2. Compiler                                                            │
│     └─ Cycle detection                                                  │
│     └─ Dead code elimination                                            │
│     └─ Predicate deduplication                                          │
│     └─ Deterministic output (SHA-256 verifiable)                        │
│     └─ Property tested                                                  │
│                                                                         │
│  3. graph.bin                                                           │
│     └─ Versioned header                                                 │
│     └─ Source hash embedded                                             │
│     └─ Decompilable                                                     │
│     └─ Hot-reloadable                                                   │
│                                                                         │
│  4. pxyz.wat                                                            │
│     └─ ~500 lines (auditable)                                           │
│     └─ No unbounded loops                                               │
│     └─ Memory isolated                                                  │
│     └─ Deterministic execution                                          │
│     └─ Trace mode for debugging                                         │
│                                                                         │
│  5. io.js                                                               │
│     └─ ~200 lines per platform                                          │
│     └─ All side effects here                                            │
│     └─ Mockable for testing                                             │
│                                                                         │
│  ATTACK SURFACE: ~700 lines total (WAT + io.js)                         │
│  DEPENDENCIES: Zero                                                     │
│  BUILD PIPELINE: xml2bin + wat2wasm                                     │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

*The graph is physics. The predicates are laws. The runtime is the universe.*
