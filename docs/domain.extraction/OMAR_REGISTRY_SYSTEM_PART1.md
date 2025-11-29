# OMAR Registry System - PXYZ Extraction

> **Custom Research**: Unified metadata registry pattern for plugins, tools, components, models, and knowledge  
> **NOT package managers**: These are semantic discovery engines with capability-based routing  
> **Services**: PluginService, ToolRegistry, ComponentRegistry, ModelResolver, KnowledgeContext

---

## EXECUTIVE SUMMARY

You have **5 registry systems** that share a common pattern:

```typescript
// Pattern shared by ALL registries
type Registry<T> = {
  // Core structure
  metadata: Manifest<T>;          // What it is
  capabilities: Capability[];     // What it can do
  routing: RoutingRule[];         // How to find it
  
  // Discovery
  search(query: string): T[];     // Semantic search
  findByCapability(cap): T[];     // Capability matching
  routeByDomain(domain): T;       // Domain routing
  
  // Execution
  validate(item: T): boolean;     // Schema validation
  execute(item: T, ctx): Result;  // Sandboxed execution
  
  // Lifecycle
  install/enable/disable/uninstall
};
```

**The Insight**: ALL of these are **coordinate-addressable discovery systems**:
- **Plugins** = P="plugin", X="install/enable/execute", Y={manifest, capabilities, permissions}
- **Tools** = P="tool", X="route/execute", Y={domain, expertise, difficulty}
- **Components** = P="component", X="render/configure", Y={type, category, dataShape}
- **Models** = P="model", X="resolve/fallback", Y={role, operation, capability}
- **Knowledge** = P="knowledge", X="retrieve", Y={query, council, confidence}

---

## CORE ARCHITECTURE INSIGHT

### Traditional Approach (WRONG)
```typescript
// Five separate systems with duplicated logic
class PluginManager { install(), enable(), execute() }
class ToolRegistry { find(), route(), execute() }
class ComponentRegistry { search(), render() }
class ModelResolver { resolve(), fallback() }
class RAGService { retrieve(), search() }
```

### PXYZ Approach (RIGHT)
```
ONE unified registry pattern with 5 specializations:

┌─────────────────────────────────────────────────────┐
│  UNIFIED REGISTRY PATTERN                           │
├─────────────────────────────────────────────────────┤
│  Core Operations:                                   │
│  - register(entity, manifest)                       │
│  - discover(query, capabilities)                    │
│  - validate(entity, schema)                         │
│  - execute(entity, context, sandbox)                │
│  - lifecycle(entity, operation)                     │
├─────────────────────────────────────────────────────┤
│  Specializations:                                   │
│  - PluginRegistry: capabilities + permissions       │
│  - ToolRegistry: expertise + difficulty routing     │
│  - ComponentRegistry: UI primitives + schemas       │
│  - ModelRegistry: role/operation routing            │
│  - KnowledgeRegistry: semantic retrieval + council  │
└─────────────────────────────────────────────────────┘
```

**All registries compile to the same graph structure**:
```xml
<workflow id="registry_discover">
  <entry p="registry" x="discover" node="parse_query"/>
  <nodes>
    <node id="parse_query" kind="transform"/>
    <node id="search_metadata" kind="external" op="0x4000"/>
    <node id="filter_capabilities" kind="auth"/>
    <node id="calculate_relevance" kind="transform"/>
    <node id="route_to_best" kind="transform"/>
    <node id="success" kind="terminal"/>
  </nodes>
</workflow>
```

---

## P-AXIS: REGISTRY ENTITIES

### Plugin Registry Entities

```xml
<schema id="plugin_manifest">
  <field name="id" type="string" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="version" type="string" required="true"/>
  <field name="description" type="string" required="true"/>
  
  <!-- Capabilities -->
  <field name="capabilities" type="array" required="true">
    <item>
      <field name="type" type="enum" values="read,write,ui_inject,api_call,webhook"/>
      <field name="resources" type="array"/> <!-- e.g., ["invoices", "contacts"] -->
      <field name="scope" type="enum" values="user,workspace,global"/>
    </item>
  </field>
  
  <!-- Permissions -->
  <field name="permissions" type="object" required="true">
    <field name="required_roles" type="array"/> <!-- ["Admin", "Manager"] -->
    <field name="optional_roles" type="array"/>
    <field name="required_scopes" type="array"/> <!-- ["read:invoices", "write:contacts"] -->
  </field>
  
  <!-- Lifecycle -->
  <field name="lifecycle" type="object">
    <field name="install" type="array"/> <!-- Validation steps -->
    <field name="enable" type="array"/> <!-- Activation hooks -->
    <field name="disable" type="array"/> <!-- Cleanup hooks -->
    <field name="uninstall" type="array"/> <!-- Teardown steps -->
  </field>
  
  <!-- UI Injections -->
  <field name="ui_injections" type="array">
    <item>
      <field name="point" type="enum" values="Dock,Crown,SidePanel,ContextMenu,InvoiceDetailPanel,ContactDetailPanel,WorkflowPanel,DashboardWidget"/>
      <field name="component" type="string"/> <!-- Component name or path -->
      <field name="order" type="number"/>
    </item>
  </field>
  
  <!-- Configuration -->
  <field name="config" type="array">
    <item>
      <field name="key" type="string"/>
      <field name="value" type="unknown"/>
      <field name="type" type="enum" values="string,number,boolean,json"/>
      <field name="required" type="boolean"/>
      <field name="default" type="unknown"/>
    </item>
  </field>
  
  <field name="sandbox" type="boolean" required="true"/> <!-- Always true -->
  <field name="dependencies" type="object"/> <!-- plugin-id -> version -->
  <field name="tags" type="array"/>
</schema>

<schema id="installed_plugin">
  <field name="id" type="uuid" required="true"/>
  <field name="plugin_id" type="string" required="true"/>
  <field name="manifest" type="object" schema="plugin_manifest" required="true"/>
  <field name="status" type="enum" values="installed,enabled,disabled,error,updating" required="true"/>
  
  <field name="metadata" type="object">
    <field name="installed_at" type="timestamp"/>
    <field name="installed_by" type="uuid"/>
    <field name="enabled_at" type="timestamp"/>
    <field name="disabled_at" type="timestamp"/>
    <field name="last_updated" type="timestamp"/>
    <field name="error_message" type="string"/>
    <field name="usage_count" type="number"/>
    <field name="active_users" type="array"/>
  </field>
  
  <field name="config_values" type="object"/>
  <field name="workspace_id" type="string"/>
  <field name="pxyz" type="object" required="true"/>
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
  
  <!-- Discovery metadata -->
  <field name="expertise" type="array" required="true"/> <!-- Skills this tool provides -->
  <field name="domains" type="array" required="true"/> <!-- Business domains it applies to -->
  <field name="difficulty" type="enum" values="beginner,intermediate,advanced,expert" required="true"/>
  
  <!-- Optional attributes -->
  <field name="frameworks" type="array"/> <!-- For tech tools -->
  <field name="consulting" type="boolean"/> <!-- Is this a legend/consultant? -->
  
  <!-- Operations -->
  <field name="operations" type="object">
    <field name="*" type="object"> <!-- Operation name as key -->
      <field name="name" type="string"/>
      <field name="input" type="object"/> <!-- Schema -->
      <field name="route" type="string"/> <!-- service:name:operation -->
      <field name="description" type="string"/>
    </field>
  </field>
</schema>

<schema id="tool_match_result">
  <field name="tool" type="object" schema="unified_tool" required="true"/>
  <field name="confidence" type="number" required="true"/> <!-- 0.0-1.0 -->
  <field name="match_reasons" type="array" required="true"/> <!-- Why this matched -->
  <field name="rank" type="number" required="true"/> <!-- 1-based ranking -->
</schema>

<schema id="business_legend">
  <field name="id" type="string" required="true"/> <!-- e.g., "alex-hormozi" -->
  <field name="name" type="string" required="true"/>
  <field name="expertise" type="array" required="true"/> <!-- ["customer-acquisition", "pricing-optimization"] -->
  <field name="difficulty" type="enum" values="beginner,intermediate,advanced,expert" required="true"/>
  <field name="specialization" type="string" required="true"/> <!-- e.g., "systematic_business_growth" -->
</schema>
```

### Component Registry Entities

```xml
<schema id="component_primitive">
  <field name="id" type="string" required="true"/> <!-- e.g., "chart-bar" -->
  <field name="type" type="enum" values="chart,table,card,filter,metric" required="true"/>
  <field name="name" type="string" required="true"/>
  <field name="description" type="string" required="true"/>
  <field name="category" type="enum" values="visualization,data,control,summary" required="true"/>
  
  <!-- Configuration schema (Effect Schema object) -->
  <field name="config_schema" type="schema"/> <!-- Cannot be represented in XML - stays in code -->
  <field name="default_config" type="object" required="true"/>
  
  <!-- Data requirements -->
  <field name="required_data_shape" type="string"/> <!-- Expected data structure -->
  <field name="supports_realtime" type="boolean" required="true"/>
  
  <!-- Discovery -->
  <field name="tags" type="array" required="true"/> <!-- ["chart", "visualization", "comparison"] -->
</schema>

<schema id="component_config">
  <!-- Chart Config -->
  <schema id="chart_config">
    <field name="chart_type" type="enum" values="bar,line,pie,area,scatter" required="true"/>
    <field name="x_axis" type="object" required="true">
      <field name="field" type="string"/>
      <field name="label" type="string"/>
    </field>
    <field name="y_axis" type="object" required="true">
      <field name="field" type="string"/>
      <field name="label" type="string"/>
      <field name="aggregation" type="enum" values="sum,avg,count,min,max"/>
    </field>
    <field name="group_by" type="string"/>
    <field name="colors" type="array"/>
    <field name="show_legend" type="boolean"/>
    <field name="show_grid" type="boolean"/>
    <field name="animate" type="boolean"/>
  </schema>
  
  <!-- Table Config -->
  <schema id="table_config">
    <field name="columns" type="array" required="true">
      <item>
        <field name="field" type="string"/>
        <field name="header" type="string"/>
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
  
  <!-- Card Config -->
  <schema id="card_config">
    <field name="title" type="string" required="true"/>
    <field name="subtitle" type="string"/>
    <field name="value_field" type="string" required="true"/>
    <field name="format" type="enum" values="number,currency,percentage"/>
    <field name="trend" type="object">
      <field name="enabled" type="boolean"/>
      <field name="field" type="string"/>
      <field name="period" type="string"/> <!-- "day", "week", "month" -->
    </field>
    <field name="icon" type="string"/>
    <field name="color" type="string"/>
  </schema>
  
  <!-- Filter Config -->
  <schema id="filter_config">
    <field name="filter_type" type="enum" values="select,multiselect,daterange,search,range" required="true"/>
    <field name="field" type="string" required="true"/>
    <field name="label" type="string" required="true"/>
    <field name="options" type="array">
      <item>
        <field name="value" type="string"/>
        <field name="label" type="string"/>
      </item>
    </field>
    <field name="default_value" type="unknown"/>
    <field name="target_components" type="array" required="true"/> <!-- IDs of components this filter affects -->
  </schema>
  
  <!-- Metric Config -->
  <schema id="metric_config">
    <field name="title" type="string" required="true"/>
    <field name="value_field" type="string" required="true"/>
    <field name="aggregation" type="enum" values="sum,avg,count,min,max" required="true"/>
    <field name="format" type="enum" values="number,currency,percentage"/>
    <field name="comparison" type="object">
      <field name="enabled" type="boolean"/>
      <field name="period" type="string"/> <!-- "previous_period", "previous_year" -->
      <field name="show_percentage" type="boolean"/>
    </field>
    <field name="threshold" type="object">
      <field name="value" type="number"/>
      <field name="comparison" type="enum" values="above,below"/>
      <field name="color" type="string"/> <!-- Highlight color when threshold is met -->
    </field>
  </schema>
</schema>
```

### Model Registry Entities

```xml
<schema id="model_config">
  <field name="provider" type="string" required="true"/> <!-- "anthropic", "openai", "google" -->
  <field name="model" type="string" required="true"/> <!-- "claude-sonnet-4-5-20250929" -->
  <field name="temperature" type="number"/>
  <field name="max_tokens" type="number"/>
  <field name="context_window" type="number"/>
  <field name="capabilities" type="array"/> <!-- ["completion", "embedding", "vision"] -->
  <field name="retry_strategy" type="string"/> <!-- "exponential_backoff" -->
  <field name="max_retries" type="number"/>
  <field name="dimension" type="number"/> <!-- For embeddings -->
</schema>

<schema id="role_config">
  <field name="primary_model" type="string" required="true"/> <!-- Model ID in catalog -->
  <field name="temperature" type="number"/>
  <field name="fallbacks" type="array"/> <!-- Fallback model IDs -->
</schema>

<schema id="operation_config">
  <field name="model" type="string" required="true"/> <!-- Model ID in catalog -->
  <field name="temperature" type="number"/>
</schema>

<schema id="model_routing_config">
  <field name="role_routing" type="object" required="true">
    <field name="*" type="object" schema="role_config"/> <!-- Role name as key -->
  </field>
  <field name="operation_routing" type="object" required="true">
    <field name="*" type="object" schema="operation_config"/> <!-- Operation name as key -->
  </field>
  <field name="model_catalog" type="object" required="true">
    <field name="*" type="object" schema="model_config"/> <!-- Model ID as key -->
  </field>
</schema>

<schema id="model_resolution_result">
  <field name="model" type="object" schema="model_config" required="true"/>
  <field name="model_id" type="string" required="true"/>
  <field name="provider" type="string" required="true"/>
  <field name="temperature" type="number" required="true"/>
  <field name="max_tokens" type="number" required="true"/>
  <field name="fallbacks" type="array" required="true"/>
  <field name="retry_strategy" type="string" required="true"/>
  <field name="max_retries" type="number" required="true"/>
</schema>
```

### Knowledge Registry Entities

```xml
<schema id="knowledge_chunk">
  <field name="id" type="string" required="true"/>
  <field name="content" type="string" required="true"/>
  <field name="title" type="string"/>
  <field name="category" type="string" required="true"/>
  <field name="source" type="string" required="true"/>
  
  <!-- Relevance -->
  <field name="relevance_score" type="number" required="true"/> <!-- 0.0-1.0 -->
  <field name="confidence" type="number" required="true"/> <!-- 0.0-1.0 -->
  <field name="business_concepts" type="array"/> <!-- ["strategy", "execution"] -->
  
  <!-- Council relevance -->
  <field name="council_relevance" type="object">
    <field name="operator" type="number"/> <!-- 0.0-1.0 -->
    <field name="strategist" type="number"/>
    <field name="signal" type="number"/>
  </field>
  
  <!-- CoT metadata -->
  <field name="retrieved_at" type="timestamp"/>
  <field name="matched_fields" type="array"/> <!-- Which fields matched the query -->
  <field name="reasoning" type="string"/> <!-- Why this chunk was selected -->
</schema>

<schema id="knowledge_context_request">
  <field name="query" type="string" required="true"/>
  <field name="council_member" type="enum" values="operator,strategist,signal"/>
  
  <!-- Filtering -->
  <field name="min_confidence" type="number"/> <!-- 0.0-1.0, default 0.5 -->
  <field name="categories" type="array"/> <!-- Filter by knowledge category -->
  <field name="max_results" type="number"/> <!-- Default 5 -->
  
  <!-- Output control -->
  <field name="include_metadata" type="boolean"/> <!-- Include relevance/source info -->
  <field name="metadata_only" type="boolean"/> <!-- Only metadata, no content (for CoT) -->
  <field name="compression_level" type="enum" values="full,compressed,summary"/>
  
  <!-- Context hints -->
  <field name="business_context" type="object"/>
  <field name="entity_id" type="string"/>
  <field name="workflow_stage" type="string"/>
</schema>

<schema id="knowledge_context_response">
  <field name="id" type="string" required="true"/>
  <field name="success" type="boolean" required="true"/>
  <field name="query" type="string" required="true"/>
  <field name="chunks" type="array" schema="knowledge_chunk" required="true"/>
  
  <field name="metadata" type="object" required="true">
    <field name="total_retrieved" type="number"/>
    <field name="total_available" type="number"/>
    <field name="avg_confidence" type="number"/>
    <field name="top_confidence" type="number"/>
    <field name="retrieved_at" type="timestamp"/>
    <field name="compression_ratio" type="number"/>
    <field name="council_member" type="string"/>
  </field>
  
  <field name="error" type="string"/>
</schema>
```

---

**[Continued in Part 2...]**
