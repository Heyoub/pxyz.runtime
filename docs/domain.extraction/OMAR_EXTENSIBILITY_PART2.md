# OMAR Extensibility Systems - Part 2: Operations & Workflows

---

## X-AXIS: EXTENSIBILITY OPERATIONS

### Plugin Lifecycle Operations (0x3600-0x3642)

```yaml
# Plugin Installation (0x3600-0x3610)
plugin_install: 0x3600               # Install plugin from manifest
plugin_validate_manifest: 0x3601    # Validate plugin manifest
plugin_check_dependencies: 0x3602   # Check plugin dependencies
plugin_check_existing: 0x3603       # Check if already installed
plugin_create_record: 0x3604        # Create plugin database record
plugin_emit_installed: 0x3605       # Emit plugin.installed event
plugin_run_install_hooks: 0x3606   # Run manifest install hooks

# Plugin Activation (0x3610-0x3620)
plugin_enable: 0x3610                # Enable installed plugin
plugin_check_access: 0x3611          # Check user access permissions
plugin_run_enable_hooks: 0x3612     # Run manifest enable hooks
plugin_load_ui_injections: 0x3613   # Load UI injection points
plugin_emit_enabled: 0x3614          # Emit plugin.enabled event

# Plugin Deactivation (0x3620-0x3630)
plugin_disable: 0x3620               # Disable enabled plugin
plugin_run_disable_hooks: 0x3621    # Run manifest disable hooks
plugin_unload_ui_injections: 0x3622 # Unload UI injection points
plugin_emit_disabled: 0x3623         # Emit plugin.disabled event

# Plugin Removal (0x3630-0x3640)
plugin_uninstall: 0x3630             # Uninstall plugin
plugin_run_uninstall_hooks: 0x3631  # Run manifest uninstall hooks
plugin_delete_record: 0x3632         # Delete plugin database record
plugin_emit_uninstalled: 0x3633      # Emit plugin.uninstalled event

# Plugin Updates (0x3640-0x3650)
plugin_update: 0x3640                # Update plugin to new version
plugin_validate_update: 0x3641      # Validate update compatibility
plugin_rollback: 0x3642              # Rollback to previous version

# Plugin Query (0x3650-0x3660)
plugin_get: 0x3650                   # Get plugin by ID
plugin_list: 0x3651                  # List plugins with filters
plugin_search: 0x3652                # Search plugins by tags

# Plugin Security (0x3660-0x3670)
plugin_check_capability: 0x3660      # Check if plugin has capability
plugin_check_user_access: 0x3661    # Check if user can access plugin
plugin_grant_capability: 0x3662      # Grant capability to plugin
plugin_revoke_capability: 0x3663     # Revoke capability from plugin

# Plugin Execution (0x3670-0x3680)
plugin_execute: 0x3670               # Execute plugin operation in sandbox
plugin_create_sandbox: 0x3671        # Create execution sandbox
plugin_destroy_sandbox: 0x3672       # Destroy execution sandbox
plugin_emit_executed: 0x3673         # Emit plugin.executed event

# UI Injection (0x3680-0x3690)
plugin_get_ui_injections: 0x3680    # Get UI injections for point
plugin_register_injection: 0x3681   # Register UI injection
plugin_unregister_injection: 0x3682 # Unregister UI injection
```

### Component Registry Operations (0x3700-0x3742)

```yaml
# Component Discovery (0x3700-0x3710)
component_get_all_primitives: 0x3700      # Get all component primitives
component_get_by_type: 0x3701             # Get primitives by type
component_get_by_category: 0x3702         # Get primitives by category
component_get_by_id: 0x3703               # Get primitive by ID
component_search_by_tags: 0x3704          # Search primitives by tags

# Component Validation (0x3710-0x3720)
component_validate_config: 0x3710         # Validate component config against schema
component_get_default_config: 0x3711     # Get default config for primitive

# Component Indexing (0x3720-0x3730)
component_index_to_qdrant: 0x3720        # Index component to Qdrant
component_index_all: 0x3721               # Index all components
component_ensure_collection: 0x3722       # Ensure Qdrant collection exists
component_build_search_text: 0x3723      # Build searchable text from primitive

# Artifact Generation (0x3730-0x3742)
artifact_generate_code: 0x3730            # Generate artifact code from intent
artifact_build_prompt: 0x3731             # Build codegen prompt from intent
artifact_extract_code: 0x3732             # Extract code from AI response
artifact_validate_code: 0x3733            # Validate generated code
artifact_validate_imports: 0x3734         # Validate imports in code
artifact_validate_pxyz_usage: 0x3735     # Validate PXYZ usage in code
artifact_calculate_complexity: 0x3736     # Calculate code complexity score
```

### Tool Registry Operations (0x3800-0x3832)

```yaml
# Tool Discovery (0x3800-0x3810)
tool_find_best: 0x3800                    # Find best tools matching criteria
tool_route_by_domain: 0x3801              # Route to tool by domain
tool_route_by_expertise: 0x3802           # Route by expertise areas
tool_get_by_type: 0x3803                  # Get tools by type
tool_get_by_id: 0x3804                    # Get tool by ID

# Confidence Scoring (0x3810-0x3820)
tool_calculate_domain_confidence: 0x3810   # Calculate domain match confidence
tool_calculate_expertise_confidence: 0x3811 # Calculate expertise match confidence
tool_calculate_difficulty_confidence: 0x3812 # Calculate difficulty match confidence

# Tool Execution (0x3820-0x3830)
tool_execute_intelligent: 0x3820          # Execute with intelligent routing
tool_execute_by_id: 0x3821                # Execute specific tool by ID

# Registry Stats (0x3830-0x3832)
tool_get_registry_stats: 0x3830           # Get registry statistics
tool_get_all_domains: 0x3831              # Get all unique domains
tool_get_all_expertise: 0x3832            # Get all unique expertise areas
```

### Model Resolver Operations (0x3900-0x3920)

```yaml
# Model Resolution (0x3900-0x3910)
model_resolve_by_role: 0x3900             # Resolve model by council role
model_resolve_by_operation: 0x3901       # Resolve model by operation type
model_resolve_embedding: 0x3902           # Resolve embedding model
model_resolve_fallback: 0x3903            # Resolve fallback model

# Model Catalog (0x3910-0x3920)
model_get_all: 0x3910                     # Get all models from catalog
model_get_routing_config: 0x3911         # Get routing configuration
model_validate: 0x3912                    # Validate model exists in catalog
model_has_capability: 0x3913              # Check if model has capability
```

### Knowledge/RAG Operations (0x3950-0x3982)

```yaml
# Knowledge Context (0x3950-0x3960)
knowledge_retrieve: 0x3950                # Retrieve knowledge chunks
knowledge_query_by_council: 0x3951       # Query by council member
knowledge_high_confidence: 0x3952         # High confidence knowledge only
knowledge_chain_of_thought: 0x3953       # Metadata for CoT reasoning
knowledge_by_category: 0x3954             # Filter by knowledge category

# Metadata-Only Queries (0x3960-0x3970)
knowledge_metadata_cot: 0x3960            # Metadata-only CoT query
knowledge_confidence_level: 0x3961        # Assess confidence level
knowledge_concept_mentions: 0x3962        # Verify concept mentions
knowledge_concept_relationships: 0x3963   # Map concept relationships
knowledge_council_perspectives: 0x3964    # Get council perspectives

# RAG Explicit (0x3970-0x3982)
rag_explicit_retrieve: 0x3970             # Explicit RAG retrieval
rag_pipe_to_tool: 0x3971                  # Pipe RAG output to tool
rag_chain_calls: 0x3972                   # Chain multiple RAG calls
rag_compose_reasoning: 0x3973             # Compose reasoning with RAG
rag_build_trace: 0x3974                   # Build reasoning trace
```

---

## COMPLETE WORKFLOWS

### Workflow 1: Plugin Installation

**Entry**: `<entry p="plugin" x="install" node="start"/>`

```xml
<workflow id="plugin_install_lifecycle">
  <entry p="plugin" x="install" node="start"/>
  
  <nodes>
    <node id="start" kind="transform">
      <description>Initialize plugin installation</description>
      <emit>plugin.install.started</emit>
    </node>
    
    <node id="validate_manifest" kind="transform" op="0x3601">
      <description>Validate plugin manifest against schema</description>
      <schema ref="plugin_manifest"/>
      <emit>plugin.manifest.validated</emit>
    </node>
    
    <node id="check_dependencies" kind="transform" op="0x3602">
      <description>Check plugin dependencies</description>
      <formula>
        for each dependency in manifest.dependencies:
          check if dependency plugin exists and version compatible
      </formula>
    </node>
    
    <node id="check_existing" kind="auth" op="0x3603">
      <description>Check if plugin already installed</description>
      <require predicate="not_already_installed"/>
    </node>
    
    <node id="run_install_hooks" kind="external" op="0x3606">
      <description>Run manifest install hooks</description>
      <formula>
        for each hook in manifest.lifecycle.install:
          execute validation step
      </formula>
    </node>
    
    <node id="create_record" kind="external" op="0x3604">
      <description>Create plugin database record</description>
      <io_call>
        {
          "operation": "0x0100",
          "entity": "plugin",
          "data": {
            "id": "uuid",
            "plugin_id": "manifest.id",
            "manifest": "full_manifest",
            "status": "installed",
            "metadata": {
              "installed_at": "now",
              "installed_by": "actor_id"
            },
            "config_values": "input.config_values",
            "workspace_id": "input.workspace_id"
          }
        }
      </io_call>
    </node>
    
    <node id="emit_installed" kind="transform" op="0x3605">
      <description>Emit plugin.installed event</description>
      <emit>plugin.installed</emit>
    </node>
    
    <node id="success" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="start" to="validate_manifest"><when><always/></when></edge>
    <edge from="validate_manifest" to="check_dependencies"><when><always/></when></edge>
    <edge from="check_dependencies" to="check_existing"><when><always/></when></edge>
    <edge from="check_existing" to="run_install_hooks"><when><always/></when></edge>
    <edge from="run_install_hooks" to="create_record"><when><always/></when></edge>
    <edge from="create_record" to="emit_installed"><when><always/></when></edge>
    <edge from="emit_installed" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

**Node Count**: 8 nodes  
**Phases**: Validate → Check → Install → Emit

---

### Workflow 2: Plugin Execution in Sandbox

**Entry**: `<entry p="plugin" x="execute" node="start"/>`

```xml
<workflow id="plugin_execute_sandbox">
  <entry p="plugin" x="execute" node="start"/>
  
  <nodes>
    <node id="start" kind="transform">
      <description>Initialize plugin execution</description>
    </node>
    
    <node id="get_plugin" kind="external" op="0x3650">
      <description>Get plugin record</description>
    </node>
    
    <node id="check_enabled" kind="auth">
      <require predicate="plugin_is_enabled"/>
    </node>
    
    <node id="check_capability" kind="auth" op="0x3660">
      <description>Check if plugin has required capability</description>
      <require predicate="has_required_capability"/>
    </node>
    
    <node id="check_user_access" kind="auth" op="0x3661">
      <description>Check if user can access plugin</description>
      <require predicate="user_has_required_roles"/>
    </node>
    
    <node id="create_sandbox" kind="transform" op="0x3671">
      <description>Create isolated execution sandbox</description>
      <formula>
        sandbox = {
          plugin_id: plugin.id,
          user_id: user.id,
          capabilities: plugin.manifest.capabilities,
          config_values: plugin.config_values,
          isolation: "vm" // or "worker"
        }
      </formula>
    </node>
    
    <node id="execute_operation" kind="external" op="0x3670">
      <description>Execute plugin operation in sandbox</description>
      <io_call>
        {
          "sandbox_id": "sandbox.id",
          "operation": "input.operation",
          "parameters": "input.parameters",
          "timeout": 30000
        }
      </io_call>
    </node>
    
    <node id="destroy_sandbox" kind="transform" op="0x3672">
      <description>Destroy execution sandbox</description>
    </node>
    
    <node id="emit_executed" kind="transform" op="0x3673">
      <description>Emit plugin.executed event</description>
      <emit>plugin.executed</emit>
    </node>
    
    <node id="success" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="start" to="get_plugin"><when><always/></when></edge>
    <edge from="get_plugin" to="check_enabled"><when><always/></when></edge>
    <edge from="check_enabled" to="check_capability"><when><always/></when></edge>
    <edge from="check_capability" to="check_user_access"><when><always/></when></edge>
    <edge from="check_user_access" to="create_sandbox"><when><always/></when></edge>
    <edge from="create_sandbox" to="execute_operation"><when><always/></when></edge>
    <edge from="execute_operation" to="destroy_sandbox"><when><always/></when></edge>
    <edge from="destroy_sandbox" to="emit_executed"><when><always/></when></edge>
    <edge from="emit_executed" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

**Node Count**: 10 nodes  
**Security Checks**: 3 (enabled, capability, user_access)  
**Sandboxing**: Isolated VM/worker execution

---

### Workflow 3: Component Artifact Generation

**Entry**: `<entry p="artifact" x="generate" node="start"/>`

```xml
<workflow id="artifact_generate_from_intent">
  <entry p="artifact" x="generate" node="start"/>
  
  <nodes>
    <node id="start" kind="transform">
      <description>Initialize artifact generation</description>
    </node>
    
    <node id="build_prompt" kind="transform" op="0x3731">
      <description>Build codegen prompt from intent</description>
      <formula>
        prompt = `
        Create a React component for: ${intent.description}
        Category: ${intent.category}
        Data Needs: ${intent.dataNeeds.join(', ')}
        
        Requirements:
        - ${intent.requiresCustomCalculations ? 'Include custom calculation logic' : 'Use simple data display'}
        - ${intent.requiresComplexWorkflow ? 'Include workflow integration' : 'Keep workflow simple'}
        - ${intent.requiresDataTransformation ? 'Include data transformation' : 'Use data as-is'}
        `
      </formula>
    </node>
    
    <node id="generate_code" kind="external" op="0x3730">
      <description>Generate artifact code via AI</description>
      <io_call>
        {
          "operation": "0x0800",
          "model": "claude-3.5-sonnet",
          "messages": [
            { "role": "system", "content": "CODEGEN_SYSTEM_PROMPT" },
            { "role": "user", "content": "prompt" }
          ],
          "max_tokens": 4000,
          "temperature": 0.3
        }
      </io_call>
    </node>
    
    <node id="extract_code" kind="transform" op="0x3732">
      <description>Extract code from AI response</description>
      <formula>
        codeBlockRegex = /```(?:typescript|tsx|ts)?\n([\s\S]*?)```/
        match = response.match(codeBlockRegex)
        code = match ? match[1].trim() : response.trim()
      </formula>
    </node>
    
    <node id="validate_imports" kind="transform" op="0x3734">
      <description>Validate imports are from allowed sources</description>
      <formula>
        allowedSources = [
          "react",
          "@server/core/types/P",
          "@server/core/types/X",
          "@server/core/infra/GraphHarness",
          "@/shell/forms/",
          "@/shell/feedback/"
        ]
        
        for each import in code:
          if import not from allowedSources:
            error "Import from '${import}' is not allowed"
      </formula>
    </node>
    
    <node id="validate_pxyz" kind="transform" op="0x3735">
      <description>Validate PXYZ usage</description>
      <formula>
        if code has operations (onClick, onChange, useEffect):
          if code includes createPXYZ:
            check createPXYZ has 4 parameters (P, X, Y, Z)
        
        check for state mutations (push, pop, shift, unshift, splice, sort, reverse)
      </formula>
    </node>
    
    <node id="calculate_complexity" kind="transform" op="0x3736">
      <description>Calculate code complexity score</description>
      <formula>
        score = 0
        score += (if_count * 2) + (for_count * 3) + (while_count * 3) + (switch_count * 2)
        score += function_count + arrow_count
        score += nesting_depth
        score += (lines_of_code / 10)
        score += (import_count * 2)
        
        level = score < 20 ? "low" : score < 50 ? "medium" : score < 100 ? "high" : "extreme"
      </formula>
    </node>
    
    <node id="check_complexity" kind="auth">
      <require predicate="complexity_not_extreme"/>
    </node>
    
    <node id="validate_export" kind="auth">
      <require predicate="has_component_export"/>
    </node>
    
    <node id="success" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="start" to="build_prompt"><when><always/></when></edge>
    <edge from="build_prompt" to="generate_code"><when><always/></when></edge>
    <edge from="generate_code" to="extract_code"><when><always/></when></edge>
    <edge from="extract_code" to="validate_imports"><when><always/></when></edge>
    <edge from="validate_imports" to="validate_pxyz"><when><always/></when></edge>
    <edge from="validate_pxyz" to="calculate_complexity"><when><always/></when></edge>
    <edge from="calculate_complexity" to="check_complexity"><when><always/></when></edge>
    <edge from="check_complexity" to="validate_export"><when><always/></when></edge>
    <edge from="validate_export" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

**Node Count**: 10 nodes  
**Validation Steps**: 4 (imports, PXYZ, complexity, export)  
**AI Integration**: Uses operation 0x0800 (LLM_COMPLETE)

---

**[Continued in Part 3...]**
