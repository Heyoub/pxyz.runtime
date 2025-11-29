# PXYZ Practical Examples & Patterns

This document provides real-world PXYZ workflow examples and common patterns.

---

## Example 1: Email Approval Workflow

**Scenario**: A user requests a document review. An approver receives a notification and must approve or reject.

```xml
<omar>
  <schemas>
    <schema id="ReviewRequest">
      <field name="document_id" type="string" required="true"/>
      <field name="requestor_id" type="string" required="true"/>
      <field name="reason" type="string" required="true"/>
    </schema>
    <schema id="ApprovalResult">
      <field name="status" type="string"/>
      <field name="message" type="string"/>
    </schema>
  </schemas>

  <predicates>
    <!-- Check if user has approval permissions -->
    <predicate id="can_approve">
      <and>
        <contains left="$token.perms" right="approve_documents"/>
        <eq left="$token.tenant" right="$entity.tenant_id"/>
      </and>
    </predicate>
    
    <!-- Check if document is pending -->
    <predicate id="is_pending">
      <eq left="$entity.status" right="pending_review"/>
    </predicate>
  </predicates>

  <workflow>
    <!-- Entry point: When approval request is triggered -->
    <entry p="document" x="review_request" node="validate_request"/>

    <nodes>
      <!-- Validate incoming request -->
      <node id="validate_request" kind="transform" schema="ReviewRequest"/>
      
      <!-- Load the document from database -->
      <node id="fetch_document" kind="external" op="0x0101"/>
      
      <!-- Verify document is pending -->
      <node id="check_status" kind="auth" predicate="is_pending"/>
      
      <!-- Check if user can approve -->
      <node id="check_permission" kind="auth" predicate="can_approve"/>
      
      <!-- Send approval email (irreversible action) -->
      <node id="send_approval" kind="external" op="0x0340" 
            actor="human" confirmation="suggested"/>
      
      <!-- Update document status to approved -->
      <node id="mark_approved" kind="external" op="0x0102"/>
      
      <!-- Return success -->
      <node id="success" kind="terminal" status="200" 
            message="Document approved and notification sent"/>
      
      <!-- Document not pending error -->
      <node id="doc_not_pending" kind="error" status="409" 
            message="Document is not pending review"/>
      
      <!-- Permission denied error -->
      <node id="permission_denied" kind="error" status="403" 
            message="User does not have approval permissions"/>
      
      <!-- Send rejection notification -->
      <node id="send_rejection" kind="external" op="0x0340" 
            actor="human" confirmation="suggested"/>
      
      <!-- Mark as rejected -->
      <node id="mark_rejected" kind="external" op="0x0102"/>
      
      <!-- Return rejection success -->
      <node id="rejection_complete" kind="terminal" status="200" 
            message="Document rejected and requestor notified"/>
    </nodes>

    <edges>
      <!-- Happy path -->
      <edge from="validate_request" to="fetch_document" weight="10"/>
      <edge from="fetch_document" to="check_status" weight="10"/>
      <edge from="check_status" to="check_permission" weight="10"/>
      <edge from="check_permission" to="send_approval" weight="10"/>
      <edge from="send_approval" to="mark_approved" weight="10"/>
      <edge from="mark_approved" to="success" weight="10"/>
      
      <!-- Error paths -->
      <edge from="check_status" to="doc_not_pending" weight="0" fallback="true"/>
      <edge from="check_permission" to="permission_denied" weight="0" fallback="true"/>
      
      <!-- Alternative: Rejection path -->
      <edge from="check_permission" to="send_rejection" weight="5"/>
      <edge from="send_rejection" to="mark_rejected" weight="10"/>
      <edge from="mark_rejected" to="rejection_complete" weight="10"/>
    </edges>
  </workflow>
</omar>
```

**Key Safety Features:**
- ✅ Permission check before approval (PRAG003)
- ✅ Status validation before processing
- ✅ Human confirmation required for email (actor="human")
- ✅ Explicit error paths (PRAG002)
- ✅ No hidden side effects

---

## Example 2: Third-Party API Integration

**Scenario**: Search for items in an external API, validate results, and cache them.

```xml
<omar>
  <predicates>
    <predicate id="has_results">
      <gt left="$state.search_external.result_count" right="0"/>
    </predicate>
    
    <predicate id="valid_results">
      <and>
        <gt left="$state.search_external.result_count" right="0"/>
        <lt left="$state.search_external.result_count" right="1000"/>
      </and>
    </predicate>
  </predicates>

  <workflow>
    <entry p="catalog" x="search" node="validate_query"/>

    <nodes>
      <node id="validate_query" kind="transform" 
            schema="SearchQuery" confirmation="suggested"/>
      
      <!-- Call external search API -->
      <node id="search_external" kind="external" op="0x0400"
            async="true" cacheable="true"/>
      
      <!-- Check if results exist -->
      <node id="check_has_results" kind="auth" predicate="has_results"/>
      
      <!-- Validate result quality -->
      <node id="validate_results" kind="auth" predicate="valid_results"/>
      
      <!-- Transform API response to internal format -->
      <node id="transform_results" kind="transform" 
            schema="SearchResults" selector="$.items"/>
      
      <!-- Cache results for performance -->
      <node id="cache_results" kind="external" op="0x0901"/>
      
      <!-- Return results -->
      <node id="return_results" kind="render" template="SearchResults"/>
      
      <!-- Signal UI to update -->
      <node id="signal_update" kind="signal"/>
      
      <!-- Success terminal -->
      <node id="success" kind="terminal" status="200"/>
      
      <!-- No results found -->
      <node id="no_results" kind="render" template="NoResults"/>
      
      <!-- Error on no results -->
      <node id="error_no_results" kind="error" status="404" 
            message="No results found"/>
      
      <!-- Invalid results -->
      <node id="error_invalid" kind="error" status="422" 
            message="Search results validation failed"/>
    </nodes>

    <edges>
      <edge from="validate_query" to="search_external" weight="10"/>
      <edge from="search_external" to="check_has_results" weight="10"/>
      <edge from="check_has_results" to="validate_results" weight="10"/>
      <edge from="validate_results" to="transform_results" weight="10"/>
      <edge from="transform_results" to="cache_results" weight="10"/>
      <edge from="cache_results" to="return_results" weight="10"/>
      <edge from="return_results" to="signal_update" weight="10"/>
      <edge from="signal_update" to="success" weight="10"/>
      
      <!-- Error paths -->
      <edge from="check_has_results" to="no_results" weight="0" fallback="true"/>
      <edge from="no_results" to="error_no_results" weight="10"/>
      <edge from="validate_results" to="error_invalid" weight="0" fallback="true"/>
    </edges>
  </workflow>
</omar>
```

**Key Patterns:**
- ✅ External API call marked as async and cacheable
- ✅ Results validated at multiple stages
- ✅ Transformation of external data to internal format
- ✅ Caching for performance
- ✅ UI signaling for live updates
- ✅ Comprehensive error handling

---

## Example 3: Multi-Step Data Validation Pipeline

**Scenario**: Validate user input through multiple checks before creating a resource.

```xml
<omar>
  <schemas>
    <schema id="UserInput">
      <field name="email" type="string" pattern="^[^@]+@[^@]+\.[^@]+$"/>
      <field name="username" type="string" min_length="3" max_length="32"/>
      <field name="password" type="string" min_length="8"/>
    </schema>
  </schemas>

  <predicates>
    <!-- Email format check -->
    <predicate id="valid_email">
      <matches left="$state.validate_input.email" 
               pattern="^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"/>
    </predicate>
    
    <!-- Username uniqueness -->
    <predicate id="username_available">
      <eq left="$state.check_username.count" right="0"/>
    </predicate>
    
    <!-- Email not already registered -->
    <predicate id="email_available">
      <eq left="$state.check_email.count" right="0"/>
    </predicate>
    
    <!-- Strong password -->
    <predicate id="strong_password">
      <and>
        <gt left="$state.validate_input.password_length" right="8"/>
        <matches left="$state.validate_input.password" pattern="[A-Z]"/>
        <matches left="$state.validate_input.password" pattern="[0-9]"/>
        <matches left="$state.validate_input.password" pattern="[!@#$%^&*]"/>
      </and>
    </predicate>
  </predicates>

  <workflow>
    <entry p="auth" x="register" node="validate_input"/>

    <nodes>
      <!-- Stage 1: Validate input format -->
      <node id="validate_input" kind="transform" 
            schema="UserInput" confirmation="confirmed"/>
      
      <!-- Stage 2: Check email validity -->
      <node id="check_email_format" kind="auth" predicate="valid_email"/>
      
      <!-- Stage 3: Check password strength -->
      <node id="check_password" kind="auth" predicate="strong_password"/>
      
      <!-- Stage 4: Query existing users by username -->
      <node id="check_username" kind="external" op="0x0105"/>
      
      <!-- Verify username is available -->
      <node id="verify_username" kind="auth" predicate="username_available"/>
      
      <!-- Stage 5: Query existing users by email -->
      <node id="check_email" kind="external" op="0x0105"/>
      
      <!-- Verify email is available -->
      <node id="verify_email" kind="auth" predicate="email_available"/>
      
      <!-- Stage 6: Create user (irreversible) -->
      <node id="create_user" kind="external" op="0x0100" 
            actor="human" confirmation="confirmed"/>
      
      <!-- Send welcome email -->
      <node id="send_welcome" kind="external" op="0x0340"/>
      
      <!-- Success -->
      <node id="success" kind="terminal" status="201" 
            message="User created successfully"/>
      
      <!-- Error nodes for each validation stage -->
      <node id="error_email_format" kind="error" status="400" 
            message="Invalid email format"/>
      <node id="error_password" kind="error" status="400" 
            message="Password does not meet strength requirements"/>
      <node id="error_username_taken" kind="error" status="409" 
            message="Username already taken"/>
      <node id="error_email_taken" kind="error" status="409" 
            message="Email already registered"/>
    </nodes>

    <edges>
      <!-- Happy path: all validations pass -->
      <edge from="validate_input" to="check_email_format" weight="10"/>
      <edge from="check_email_format" to="check_password" weight="10"/>
      <edge from="check_password" to="check_username" weight="10"/>
      <edge from="check_username" to="verify_username" weight="10"/>
      <edge from="verify_username" to="check_email" weight="10"/>
      <edge from="check_email" to="verify_email" weight="10"/>
      <edge from="verify_email" to="create_user" weight="10"/>
      <edge from="create_user" to="send_welcome" weight="10"/>
      <edge from="send_welcome" to="success" weight="10"/>
      
      <!-- Error paths: early exit on validation failure -->
      <edge from="check_email_format" to="error_email_format" weight="0" fallback="true"/>
      <edge from="check_password" to="error_password" weight="0" fallback="true"/>
      <edge from="verify_username" to="error_username_taken" weight="0" fallback="true"/>
      <edge from="verify_email" to="error_email_taken" weight="0" fallback="true"/>
    </edges>
  </workflow>
</omar>
```

**Validation Pattern:**
- ✅ Progressive validation (fail fast)
- ✅ Format checks before database queries
- ✅ Uniqueness checks before resource creation
- ✅ Explicit error messages for each failure
- ✅ Human confirmation for irreversible action

---

## Common PXYZ Patterns

### Pattern 1: Gated Irreversible Action

```xml
<workflow>
  <nodes>
    <!-- Risky operation -->
    <node id="delete_data" kind="external" op="0x0103" 
          actor="human" confirmation="confirmed"/>
  </nodes>
  
  <edges>
    <!-- Can only execute if user is admin AND data was confirmed -->
    <edge from="auth_check" to="delete_data">
      <when>
        <and>
          <contains left="$token.perms" right="admin"/>
          <fn name="is_confirmed" arg="$entity"/>
        </and>
      </when>
    </edge>
  </edges>
</workflow>
```

**Enforces**: PRAG003, PRAG004

### Pattern 2: Multiple Parallel Validations

```xml
<workflow>
  <nodes>
    <node id="check_perms" kind="auth" predicate="has_perms"/>
    <node id="check_quota" kind="auth" predicate="within_quota"/>
    <node id="check_rate_limit" kind="auth" predicate="not_rate_limited"/>
    <node id="proceed" kind="external" op="0x0400"/>
  </nodes>
  
  <edges>
    <edge from="check_perms" to="check_quota" weight="10"/>
    <edge from="check_quota" to="check_rate_limit" weight="10"/>
    <edge from="check_rate_limit" to="proceed" weight="10"/>
  </edges>
</workflow>
```

### Pattern 3: LLM with Validation Gate

```xml
<workflow>
  <nodes>
    <!-- LLM generates response -->
    <node id="llm_complete" kind="external" op="0x0800"/>
    
    <!-- Validate output quality -->
    <node id="validate_output" kind="auth" predicate="output_quality_ok"/>
    
    <!-- If invalid, escalate to human -->
    <node id="escalate_to_human" kind="external" op="0x0340" 
          actor="human"/>
    
    <!-- Proceed with valid output -->
    <node id="send_response" kind="render" template="Response"/>
  </nodes>
  
  <edges>
    <edge from="llm_complete" to="validate_output" weight="10"/>
    <edge from="validate_output" to="send_response" weight="10"/>
    <edge from="validate_output" to="escalate_to_human" weight="0" fallback="true"/>
  </edges>
</workflow>
```

**Enforces**: PRAG001 - LLM output validated before any irreversible action

### Pattern 4: Error Recovery with Fallback

```xml
<workflow>
  <nodes>
    <node id="call_primary_api" kind="external" op="0x0400" async="true"/>
    <node id="parse_primary" kind="transform"/>
    <node id="call_backup_api" kind="external" op="0x0400" async="true"/>
    <node id="parse_backup" kind="transform"/>
    <node id="return_result" kind="render" template="Result"/>
  </nodes>
  
  <edges>
    <!-- Try primary API first -->
    <edge from="start" to="call_primary_api" weight="10"/>
    <edge from="call_primary_api" to="parse_primary" weight="10"/>
    <edge from="parse_primary" to="return_result" weight="10"/>
    
    <!-- Fallback to backup if primary fails -->
    <edge from="parse_primary" to="call_backup_api" weight="0" fallback="true"/>
    <edge from="call_backup_api" to="parse_backup" weight="10"/>
    <edge from="parse_backup" to="return_result" weight="10"/>
  </edges>
</workflow>
```

---

## Best Practices

### 1. Always Validate External Input
```xml
<!-- Good: Validate before using -->
<edge from="receive_input" to="validate_schema" weight="10"/>
<edge from="validate_schema" to="process" weight="10"/>

<!-- Bad: Using input directly -->
<edge from="receive_input" to="process" weight="10"/>
```

### 2. Gate Irreversible Actions
```xml
<!-- Good: Multiple gates -->
<node id="send_email" kind="external" op="0x0340" 
      actor="human" confirmation="confirmed"/>

<!-- Bad: No gates -->
<node id="send_email" kind="external" op="0x0340"/>
```

### 3. Provide Error Paths
```xml
<!-- Good: Every risk has a fallback -->
<edge from="risky_op" to="success" weight="10"/>
<edge from="risky_op" to="error_handler" weight="0" fallback="true"/>

<!-- Bad: No fallback -->
<edge from="risky_op" to="success" weight="10"/>
```

### 4. Use Meaningful Entry Points
```xml
<!-- Good: Clear P and X coordinates -->
<entry p="billing" x="create_invoice" node="validate_invoice"/>
<entry p="billing" x="send_reminder" node="load_unpaid"/>

<!-- Bad: Vague coordinates -->
<entry p="data" x="process" node="start"/>
```

### 5. Mark Async/Cacheable Operations
```xml
<!-- Good: Hints for optimization -->
<node id="fetch_user" kind="external" op="0x0101" 
      async="true" cacheable="true"/>

<!-- Bad: Could block or duplicate work -->
<node id="fetch_user" kind="external" op="0x0101"/>
```

---

## Testing Your Workflows

### 1. Validate Syntax
```bash
pxyz check workflow.xml
```

### 2. Inspect Binary
```bash
pxyz inspect graph.bin --format mermaid
```

### 3. Test Entry Points
Load graph.bin with different (P, X) coordinates:
```javascript
const result = runtime.execute(graph, "billing", "create_invoice", context);
```

### 4. Verify Predicates
Test predicate evaluation independently:
```javascript
const predicateResult = runtime.evalPredicate(predicateId, context);
```

---

## Constraint Checklist

Before deploying a workflow:

- [ ] **SYN001**: All edges target existing nodes
- [ ] **SYN005**: At least one entry point defined
- [ ] **SEM001**: All auth nodes have predicates
- [ ] **SEM002**: All external nodes have op codes
- [ ] **SEM004**: No cycles in graph
- [ ] **PRAG001**: LLM → Irreversible paths have validation
- [ ] **PRAG002**: Write operations have error branches
- [ ] **PRAG003**: Irreversible actions have human gates
- [ ] **PRAG004**: Irreversible actions require confirmed inputs
- [ ] **PRAG005**: Quarantined data doesn't escape to I/O

---

This document should be used alongside the PXYZ Quick Start Guide and the full System Reference Manual.