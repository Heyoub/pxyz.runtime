# OMAR Extensibility Systems - PXYZ Extraction

> **Custom Research**: Plugin/Component/Tool architecture as graph-based extensibility  
> **NOT microservices**: Manifest-driven capabilities with coordinate-addressable operations  
> **Services**: PluginService, ComponentRegistry, ToolRegistry, ModelResolver

---

## EXECUTIVE SUMMARY

Your **Extensibility Systems** represent a fundamental reimagining of how plugins, components, and tools work. Instead of traditional plugin architectures (class hierarchies, hooks, events), you're treating **extensibility as graph operations** with:

1. **Plugins** = Manifest-driven capabilities with lifecycle as graph traversal
2. **Components** = Primitive registry with schema-validated compositions
3. **Tools** = Intelligence routing with multi-dimensional confidence scoring
4. **Models** = Config-driven resolution with zero hardcoded dependencies

```typescript
// WRONG (Traditional)
class Plugin extends BasePlugin {
  async onInstall() { /* hooks */ }
  async onEnable() { /* more hooks */ }
  async execute(params) { /* imperative code */ }
}

// RIGHT (Your Architecture)
const pluginGraph = {
  manifest: PluginManifest,          // Declarative capabilities
  lifecycle: {
    install: ["validate", "check_deps", "emit_installed"],
    enable: ["check_access", "load_ui", "emit_enabled"],
    execute: ["check_capability", "sandbox", "emit_executed"]
  },
  pxyz: {
    p: "plugin_installation",
    x: "install",
    y: `plugin_${manifest.id}`,
    z: timestamp
  }
};
```

This enables:
- **Manifest-based security**: Capabilities declared, not coded
- **Lifecycle as graph**: Install/enable/disable as traversable workflows
- **Sandboxed execution**: All operations run in isolated contexts
- **Hot-reload**: Replace graph binaries without downtime
- **Version control**: Rollback = restore previous graph.bin

---

## ARCHITECTURE OVERVIEW

### Four Extensibility Systems

```
┌─────────────────────────────────────────────────────────────┐
│  PLUGIN SYSTEM - Manifest-Driven Extensibility              │
│  ──────────────────────────────────────────                 │
│  Plugins = declarative manifests with capabilities          │
│  Lifecycle = graph traversal (install → enable → execute)   │
│  Security = capability grants, not code inspection          │
├─────────────────────────────────────────────────────────────┤
│  COMPONENT REGISTRY - Primitive-Based UI Generation         │
│  ────────────────────────────────────────────────           │
│  Components = primitives with schema-validated configs      │
│  Codegen = AI-powered artifact generation from intents      │
│  Validation = Effect schemas + PXYZ compliance checks       │
├─────────────────────────────────────────────────────────────┤
│  TOOL REGISTRY - Intelligence Routing                       │
│  ──────────────────────────────                             │
│  Tools = unified interface across domains/legends           │
│  Routing = multi-dimensional confidence scoring             │
│  Discovery = expertise + domain + difficulty matching       │
├─────────────────────────────────────────────────────────────┤
│  MODEL RESOLVER - Config-Driven LLM Routing                 │
│  ───────────────────────────────────────                    │
│  Models = catalog with capabilities, not hardcoded names    │
│  Resolution = role/operation routing with fallbacks         │
│  Zero dependencies = all models defined in config.json      │
└─────────────────────────────────────────────────────────────┘
```

---

## P-AXIS: EXTENSIBILITY ENTITIES

### Plugin System Entities

```xml
<schema id="plugin_manifest">
  <field name="id" type="string" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="version" type="string" required="true"/> <!-- Semver -->
  <field name="description" type="string" required="true"/>
  <field name="author" type="string"/>
  <field name="homepage" type="string"/>
  
  <!-- Capabilities (what plugin can do) -->
  <field name="capabilities" type="array" required="true">
    <item>
      <field name="type" type="enum" values="read,write,ui_inject,api_call,webhook"/>
      <field name="resources" type="array"/> <!-- e.g., ["invoices", "contacts"] -->
      <field name="scope" type="enum" values="user,workspace,global"/>
    </item>
  </field>
  
  <!-- Permissions (who can use it) -->
  <field name="permissions" type="object" required="true">
    <field name="required_roles" type="array"/> <!-- e.g., ["Admin", "Manager"] -->
    <field name="optional_roles" type="array"/>
    <field name="required_scopes" type="array"/> <!-- e.g., ["read:invoices"] -->
  </field>
  
  <!-- Lifecycle hooks -->
  <field name="lifecycle" type="object">
    <field name="install" type="array"/> <!-- Validation steps -->
    <field name="enable" type="array"/> <!-- Activation hooks -->
    <field name="disable" type="array"/> <!-- Cleanup hooks -->
    <field name="uninstall" type="array"/> <!-- Teardown steps -->
  </field>
  
  <!-- UI injection points -->
  <field name="ui_injections" type="array">
    <item>
      <field name="point" type="enum" values="Dock,Crown,SidePanel,ContextMenu,InvoiceDetailPanel,ContactDetailPanel,WorkflowPanel,DashboardWidget"/>
      <field name="component" type="string"/> <!-- Component name or path -->
      <field name="order" type="number"/> <!-- Injection order -->
    </item>
  </field>
  
  <!-- Plugin configuration -->
  <field name="config" type="array">
    <item>
      <field name="key" type="string"/>
      <field name="value" type="unknown"/>
      <field name="type" type="enum" values="string,number,boolean,json"/>
      <field name="required" type="boolean"/>
      <field name="default" type="unknown"/>
    </item>
  </field>
  
  <!-- Security -->
  <field name="sandbox" type="boolean" required="true"/> <!-- Always true -->
  
  <!-- Dependencies -->
  <field name="dependencies" type="object"/> <!-- plugin-id -> version -->
  
  <!-- Metadata -->
  <field name="tags" type="array"/>
  <field name="changelog" type="string"/>
  <field name="license" type="string"/>
</schema>

<schema id="installed_plugin">
  <field name="id" type="uuid" required="true"/>
  <field name="plugin_id" type="string" required="true"/> <!-- From manifest -->
  <field name="manifest" type="object" required="true"/> <!-- Full PluginManifest -->
  <field name="status" type="enum" values="installed,enabled,disabled,error,updating" required="true"/>
  <field name="metadata" type="object" required="true">
    <field name="installed_at" type="timestamp"/>
    <field name="installed_by" type="uuid"/> <!-- ActorId -->
    <field name="enabled_at" type="timestamp"/>
    <field name="disabled_at" type="timestamp"/>
    <field name="last_updated" type="timestamp"/>
    <field name="error_message" type="string"/>
    <field name="usage_count" type="number"/>
    <field name="active_users" type="array"/>
  </field>
  <field name="config_values" type="object"/> <!-- User-provided config -->
  <field name="workspace_id" type="string"/> <!-- Multi-tenancy -->
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="plugin_execution_context">
  <field name="plugin_id" type="string" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="capabilities" type="array"/> <!-- Granted capabilities -->
  <field name="config_values" type="object"/> <!-- Runtime config -->
  <field name="injection_points" type="array"/> <!-- UI injection points -->
</schema>

<schema id="plugin_execution_result">
  <field name="success" type="boolean" required="true"/>
  <field name="output" type="unknown"/>
  <field name="error" type="string"/>
  <field name="execution_time" type="number"/> <!-- Milliseconds -->
  <field name="pxyz" type="object" required="true"/>
</schema>
```

### Component Registry Entities

```xml
<schema id="component_primitive">
  <field name="id" type="uuid" required="true"/> <!-- EntityName -->
  <field name="type" type="enum" values="chart,table,card,filter,metric" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="description" type="string" required="true"/>
  <field name="category" type="enum" values="visualization,data,control,summary" required="true"/>
  
  <!-- Schema validation (Effect Schema) -->
  <field name="config_schema" type="schema" required="true"/> <!-- Unique to components -->
  <field name="default_config" type="object" required="true"/>
  
  <!-- Data requirements -->
  <field name="required_data_shape" type="uuid"/> <!-- EntityName -->
  <field name="supports_realtime" type="boolean" required="true"/>
  
  <!-- Discovery -->
  <field name="tags" type="array" required="true"/>
</schema>

<schema id="chart_config">
  <field name="chart_type" type="enum" values="bar,line,pie,area,scatter" required="true"/>
  <field name="x_axis" type="object" required="true">
    <field name="field" type="string" required="true"/>
    <field name="label" type="string"/>
  </field>
  <field name="y_axis" type="object" required="true">
    <field name="field" type="string" required="true"/>
    <field name="label" type="string"/>
    <field name="aggregation" type="enum" values="sum,avg,count,min,max"/>
  </field>
  <field name="group_by" type="string"/>
  <field name="colors" type="array"/>
  <field name="show_legend" type="boolean"/>
  <field name="show_grid" type="boolean"/>
  <field name="animate" type="boolean"/>
</schema>

<schema id="table_config">
  <field name="columns" type="array" required="true">
    <item>
      <field name="field" type="string" required="true"/>
      <field name="header" type="string" required="true"/>
      <field name="sortable" type="boolean"/>
      <field name="filterable" type="boolean"/>
      <field name="width" type="number"/>
      <field name="format" type="enum" values="text,number,currency,date,percentage"/>
    </item>
  </field>
  <field name="page_size" type="number"/>
  <field name="sort_by" type="object">
    <field name="field" type="string"/>
    <field name="direction" type="enum" values="asc,desc"/>
  </field>
  <field name="selectable" type="boolean"/>
  <field name="exportable" type="boolean"/>
</schema>

<schema id="artifact_intent">
  <field name="description" type="string" required="true"/>
  <field name="category" type="string" required="true"/>
  <field name="data_needs" type="array" required="true"/>
  <field name="requires_custom_calculations" type="boolean"/>
  <field name="requires_complex_workflow" type="boolean"/>
  <field name="requires_data_transformation" type="boolean"/>
  <field name="primary_metric" type="string"/>
  <field name="primary_dimension" type="string"/>
</schema>
```

### Tool Registry Entities

```xml
<schema id="unified_tool">
  <field name="id" type="string" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="type" type="enum" values="domain-tool,business-legend,tech-legend,system-service" required="true"/>
  <field name="category" type="string" required="true"/>
  <field name="description" type="string" required="true"/>
  <field name="expertise" type="array" required="true"/> <!-- Expertise areas -->
  <field name="domains" type="array" required="true"/> <!-- Domain coverage -->
  <field name="difficulty" type="enum" values="beginner,intermediate,advanced,expert" required="true"/>
  <field name="frameworks" type="array"/> <!-- Tech frameworks -->
  <field name="consulting" type="boolean"/> <!-- Is this a consulting legend? -->
  <field name="operations" type="object"> <!-- Available operations -->
    <field name="[operation_name]" type="object">
      <field name="name" type="string"/>
      <field name="input" type="object"/>
      <field name="route" type="string"/>
      <field name="description" type="string"/>
    </field>
  </field>
</schema>

<schema id="tool_match_result">
  <field name="tool" type="object" required="true"/> <!-- UnifiedTool -->
  <field name="confidence" type="number" required="true"/> <!-- 0.0-1.0 -->
  <field name="match_reasons" type="array" required="true"/> <!-- Why this tool matches -->
  <field name="rank" type="number" required="true"/> <!-- 1-N ranking -->
</schema>

<schema id="business_legend">
  <field name="id" type="string" required="true"/> <!-- e.g., "alex-hormozi" -->
  <field name="name" type="string" required="true"/> <!-- e.g., "Alex Hormozi" -->
  <field name="expertise" type="array" required="true"/> <!-- e.g., ["customer-acquisition", "pricing-optimization"] -->
  <field name="difficulty" type="enum" values="beginner,intermediate,advanced,expert" required="true"/>
  <field name="specialization" type="string" required="true"/> <!-- Core methodology -->
</schema>
```

### Model Resolver Entities

```xml
<schema id="model_config">
  <field name="provider" type="string" required="true"/> <!-- "anthropic", "openai", "local" -->
  <field name="model" type="string" required="true"/> <!-- Model name -->
  <field name="temperature" type="number"/> <!-- Default temperature -->
  <field name="max_tokens" type="number"/> <!-- Max output tokens -->
  <field name="context_window" type="number"/> <!-- Context window size -->
  <field name="capabilities" type="array"/> <!-- ["chat", "embedding", "vision"] -->
  <field name="retry_strategy" type="string"/> <!-- "exponential_backoff" -->
  <field name="max_retries" type="number"/> <!-- Retry count -->
  <field name="dimension" type="number"/> <!-- For embeddings -->
</schema>

<schema id="role_config">
  <field name="primary_model" type="string" required="true"/> <!-- Model catalog ID -->
  <field name="temperature" type="number"/> <!-- Override temperature -->
  <field name="fallbacks" type="array"/> <!-- Fallback model IDs -->
</schema>

<schema id="operation_config">
  <field name="model" type="string" required="true"/> <!-- Model catalog ID -->
  <field name="temperature" type="number"/> <!-- Override temperature -->
</schema>

<schema id="model_resolution_result">
  <field name="model" type="object" required="true"/> <!-- ModelConfig -->
  <field name="model_id" type="string" required="true"/> <!-- Catalog ID -->
  <field name="provider" type="string" required="true"/>
  <field name="temperature" type="number" required="true"/>
  <field name="max_tokens" type="number" required="true"/>
  <field name="fallbacks" type="array" required="true"/> <!-- Fallback model IDs -->
  <field name="retry_strategy" type="string" required="true"/>
  <field name="max_retries" type="number" required="true"/>
</schema>
```

---

**[Continued in Part 2...]**
