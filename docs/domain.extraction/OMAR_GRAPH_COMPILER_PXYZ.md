# OMAR Graph Compiler - PXYZ Fundamental Redesign

> **Revolutionary Insight**: Compilation is NOT imperative code - it's **graph traversal in coordinate space**  
> **Integration**: Compiler USES LibrarianService + ContextWindowManager + Memory services  
> **Output**: graph.bin (not JavaScript objects)

---

## THE CORE INSIGHT: COMPILATION AS COORDINATE SPACE

### Traditional Compiler (Imperative)

```typescript
// Phase 1: Parse
const ast = parse(sourceCode);

// Phase 2: Validate
if (!validate(ast, schema)) throw new Error();

// Phase 3: Enrich
const enriched = enrich(ast, context);

// Phase 4: Optimize
const optimized = optimize(enriched);

// Phase 5: Generate
const code = generate(optimized);
```

### PXYZ Graph Compiler (Declarative)

```xml
<workflow id="kernel_compilation">
  <entry p="source_code" x="compile" node="parse_prompt"/>
  
  <nodes>
    <!-- P-space transitions -->
    <node id="parse_prompt" kind="transform"/>
    <node id="validate_schema" kind="auth"/> <!-- Validation = authorization! -->
    <node id="enrich_context" kind="external" op="librarian_rag_retrieve"/>
    <node id="optimize_context" kind="external" op="context_optimize"/>
    <node id="generate_graph" kind="transform"/>
    <node id="emit_binary" kind="terminal"/>
  </nodes>
  
  <edges>
    <!-- Y-space predicates control flow -->
    <edge from="parse_prompt" to="validate_schema">
      <when><predicate ref="ast_well_formed"/></when>
    </edge>
    <edge from="validate_schema" to="enrich_context">
      <when><predicate ref="passes_kernel_schema"/></when>
    </edge>
    <!-- ... -->
  </edges>
</workflow>
```

**Compilation is just another workflow!** It produces graph.bin as output instead of executing operations.

---

## COMPILATION AS PXYZ COORDINATE SPACE

### P-Axis: Compilation States (Entity Types)

```xml
<schema id="source_code">
  <field name="id" type="uuid"/>
  <field name="format" type="enum" values="markdown,latex,poml,yaml"/> <!-- KernelSchema is YAML! -->
  <field name="content" type="string" required="true"/>
  <field name="metadata" type="object"/>
</schema>

<schema id="abstract_syntax_tree">
  <field name="id" type="uuid"/>
  <field name="source_id" type="uuid"/>
  <field name="format" type="enum" values="markdown,latex,poml,yaml"/>
  <field name="parsed_structure" type="object" required="true"/>
  <field name="parse_errors" type="array"/> <!-- Empty if successful -->
</schema>

<schema id="validated_ast">
  <field name="id" type="uuid"/>
  <field name="ast_id" type="uuid"/>
  <field name="schema_version" type="string"/> <!-- 1.4 from KernelSchema -->
  <field name="validation_result" type="object" required="true">
    <field name="valid" type="boolean"/>
    <field name="errors" type="array"/>
    <field name="warnings" type="array"/>
  </field>
</schema>

<schema id="enriched_ast">
  <field name="id" type="uuid"/>
  <field name="validated_ast_id" type="uuid"/>
  <field name="enrichments" type="object" required="true">
    <field name="agent_memory_chunks" type="array"/> <!-- From LibrarianService -->
    <field name="business_memory_patterns" type="array"/> <!-- From BusinessMemory -->
    <field name="kernel_schema_defaults" type="object"/> <!-- From KernelSchemaV1_4.yaml -->
    <field name="council_context" type="object"/> <!-- operator, strategist, signal -->
  </field>
</schema>

<schema id="optimized_ast">
  <field name="id" type="uuid"/>
  <field name="enriched_ast_id" type="uuid"/>
  <field name="optimizations" type="object" required="true">
    <field name="context_window_optimized" type="boolean"/>
    <field name="tokens_used" type="number"/>
    <field name="tokens_remaining" type="number"/>
    <field name="chunks_dropped" type="number"/>
    <field name="priority_reordering_applied" type="boolean"/>
  </field>
</schema>

<schema id="graph_ir">
  <field name="id" type="uuid"/>
  <field name="optimized_ast_id" type="uuid"/>
  <field name="nodes" type="array" required="true"/> <!-- Graph nodes -->
  <field name="edges" type="array" required="true"/> <!-- Graph edges -->
  <field name="predicates" type="array"/> <!-- Y-constraints -->
  <field name="entry_points" type="array"/> <!-- PXYZ coordinates -->
</schema>

<schema id="compiled_binary">
  <field name="id" type="uuid"/>
  <field name="graph_ir_id" type="uuid"/>
  <field name="binary_format" type="enum" values="graph.bin"/> <!-- Binary graph format -->
  <field name="binary_data" type="bytes" required="true"/>
  <field name="source_hash" type="string"/> <!-- SHA-256 of source -->
  <field name="version" type="string"/> <!-- Compiler version -->
</schema>
```

**Each compilation phase = entity type in P-space!**

---

## X-Axis: Compilation Operations

```yaml
# Parsing Operations
compile_parse_markdown: 0x4000    # Parse markdown → AST
compile_parse_latex: 0x4001       # Parse LaTeX → AST
compile_parse_poml: 0x4002        # Parse POML → AST
compile_parse_yaml: 0x4003        # Parse YAML (KernelSchema) → AST

# Validation Operations
compile_validate_schema: 0x4010   # Validate against KernelSchema
compile_check_council: 0x4011     # Validate council structure
compile_check_services