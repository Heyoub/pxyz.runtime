# OMAR Addendum 5: Bulk Operations & Advanced Actions

> **Second-Pass Gap Analysis**: Bulk operations, email snooze/scheduling, file versioning, document approval chains found but not yet mapped.

---

## BULK OPERATIONS PHILOSOPHY

Bulk operations are **graph traversals over multiple entities**, applying the same operation to each while maintaining transaction integrity.

### Bulk Operation Pattern

```
1. Select entities (via UI or filter)
2. Validate permission for ALL entities
3. Preview changes (dry-run)
4. User confirms
5. Execute in transaction
6. Append single bulk event OR one event per entity
7. Render summary with success/failure counts
```

---

## BULK TASK OPERATIONS

### X-Axis: Bulk Task Operations

```yaml
task_bulk_update: 0x0260             # NEW
task_bulk_delete: 0x0261             # NEW
task_bulk_archive: 0x0262            # NEW
task_bulk_assign: 0x0263             # NEW
task_bulk_tag: 0x0264                # NEW
task_bulk_change_state: 0x0265       # NEW
task_bulk_export: 0x0266             # NEW
task_bulk_create_workflow: 0x0267    # NEW - convert tasks to workflow
```

### Workflow Example: Bulk Update Tasks

```xml
<workflow id="task_bulk_update">
  <entry p="task" x="bulk_update" node="validate_selection"/>
  
  <nodes>
    <node id="validate_selection" kind="transform">
      <validate>
        <field name="task_ids" type="array" required="true"/>
        <field name="updates" type="object" required="true"/>
        <constraint>
          <gte left="count($input.task_ids)" right="1"/>
          <lte left="count($input.task_ids)" right="100"/> <!-- Max 100 at once -->
        </constraint>
      </validate>
    </node>
    
    <node id="load_tasks" kind="external" op="0x0204">
      <query>
        <filter field="id" in="$input.task_ids"/>
      </query>
    </node>
    
    <node id="check_permissions" kind="auth">
      <for_each task="$loaded_tasks">
        <require>
          <predicate ref="can_edit_task">
            <task value="$task"/>
          </predicate>
        </require>
      </for_each>
    </node>
    
    <node id="detect_conflicts" kind="transform">
      <for_each task="$loaded_tasks">
        <for_each update_field="$input.updates">
          <check>
            <if field_exists="$task[$update_field.key]"/>
            <then>
              <var name="conflict" value="{
                task_id: $task.id,
                field: $update_field.key,
                current_value: $task[$update_field.key],
                new_value: $update_field.value
              }"/>
            </then>
          </check>
        </for_each>
      </for_each>
    </node>
    
    <node id="preview_changes" kind="render">
      <template ref="bulk_update_preview">
        <summary>
          <field name="total_tasks" value="count($loaded_tasks)"/>
          <field name="fields_changing" value="keys($input.updates)"/>
          <field name="conflicts" value="$detected_conflicts"/>
        </summary>
        <table>
          <for_each task="$loaded_tasks">
            <row>
              <cell>{{task.title}}</cell>
              <for_each update="$input.updates">
                <cell class="changing">
                  <old>{{task[update.key]}}</old>
                  <new>{{update.value}}</new>
                </cell>
              </for_each>
            </row>
          </for_each>
        </table>
        <actions>
          <button action="confirm_bulk_update">Apply to {{count}} tasks</button>
          <button action="cancel">Cancel</button>
        </actions>
      </template>
    </node>
    
    <node id="await_confirmation" kind="signal">
      <await signal="bulk_update_confirmed"/>
    </node>
    
    <node id="execute_updates" kind="external" op="0x0260">
      <transaction>
        <for_each task="$loaded_tasks">
          <update task_id="$task.id">
            <for_each update="$input.updates">
              <field name="$update.key" value="$update.value"/>
            </for_each>
          </update>
          <event>
            <type>task.updated</type>
            <data>
              <field name="task_id" value="$task.id"/>
              <field name="updated_fields" value="keys($input.updates)"/>
              <field name="bulk_operation_id" value="$operation_id"/>
            </data>
          </event>
        </for_each>
      </transaction>
    </node>
    
    <node id="render_summary" kind="render">
      <template ref="bulk_operation_summary">
        <success_count value="count($updated_tasks)"/>
        <failure_count value="count($failed_tasks)"/>
        <failures value="$failed_tasks"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_selection" to="load_tasks"><when><always/></when></edge>
    <edge from="load_tasks" to="check_permissions"><when><always/></when></edge>
    <edge from="check_permissions" to="detect_conflicts"><when><always/></when></edge>
    <edge from="detect_conflicts" to="preview_changes"><when><always/></when></edge>
    <edge from="preview_changes" to="await_confirmation"><when><always/></when></edge>
    <edge from="await_confirmation" to="execute_updates"><when><always/></when></edge>
    <edge from="execute_updates" to="render_summary"><when><always/></when></edge>
    <edge from="render_summary" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## EMAIL SNOOZE & SCHEDULING

### X-Axis: Email Snooze/Schedule Operations

```yaml
email_snooze: 0x0511                 # Already mentioned, now fully specified
email_unsnooze: 0x0560               # NEW
email_schedule_send: 0x0561          # NEW
email_cancel_scheduled: 0x0562       # NEW
email_snooze_list: 0x0563            # NEW - list all snoozed
email_scheduled_list: 0x0564         # NEW - list all scheduled
```

### Email Snooze Schema

```xml
<schema id="email_snooze">
  <field name="id" type="uuid" required="true"/>
  <field name="thread_id" type="uuid" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="snooze_until" type="timestamp" required="true"/>
  <field name="reason" type="enum" values="later_today,tomorrow,next_week,custom"/>
  <field name="created_at" type="timestamp"/>
</schema>
```

### Workflow Example: Snooze Email Thread

```xml
<workflow id="email_snooze_thread">
  <entry p="email" x="snooze" node="validate_snooze"/>
  
  <nodes>
    <node id="validate_snooze" kind="transform">
      <validate>
        <field name="thread_id" type="uuid" required="true"/>
        <field name="snooze_option" type="enum" values="later_today,tomorrow,next_week,custom"/>
        <field name="custom_datetime" type="timestamp"/>
      </validate>
    </node>
    
    <node id="calculate_snooze_time" kind="transform">
      <compute>
        <var name="snooze_until" value="
          $input.snooze_option == 'later_today' ? $now + 4.hours :
          $input.snooze_option == 'tomorrow' ? $tomorrow.09:00 :
          $input.snooze_option == 'next_week' ? $next_monday.09:00 :
          $input.custom_datetime
        "/>
      </compute>
    </node>
    
    <node id="create_snooze_record" kind="external" op="0x0910">
      <event>
        <type>email.snoozed</type>
        <data>
          <field name="thread_id" value="$input.thread_id"/>
          <field name="user_id" value="$token.sub"/>
          <field name="snooze_until" value="$snooze_until"/>
          <field name="reason" value="$input.snooze_option"/>
        </data>
      </event>
    </node>
    
    <node id="hide_from_inbox" kind="signal">
      <signal>inbox_update</signal>
      <data>
        <field name="action" value="hide"/>
        <field name="thread_id" value="$input.thread_id"/>
      </data>
    </node>
    
    <node id="schedule_unsnooze" kind="external" op="0x1400">
      <!-- Schedule a background job to unsnooze -->
      <schedule>
        <time value="$snooze_until"/>
        <action value="email_unsnooze"/>
        <params>
          <field name="thread_id" value="$input.thread_id"/>
          <field name="user_id" value="$token.sub"/>
        </params>
      </schedule>
    </node>
    
    <node id="render_confirmation" kind="render">
      <template ref="email_snooze_toast">
        <message>Snoozed until {{snooze_until | format_friendly}}</message>
        <action label="Undo" op="email_unsnooze"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_snooze" to="calculate_snooze_time"><when><always/></when></edge>
    <edge from="calculate_snooze_time" to="create_snooze_record"><when><always/></when></edge>
    <edge from="create_snooze_record" to="hide_from_inbox"><when><always/></when></edge>
    <edge from="hide_from_inbox" to="schedule_unsnooze"><when><always/></when></edge>
    <edge from="schedule_unsnooze" to="render_confirmation"><when><always/></when></edge>
    <edge from="render_confirmation" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Email Scheduled Send Schema

```xml
<schema id="email_scheduled_send">
  <field name="id" type="uuid" required="true"/>
  <field name="from_account_id" type="uuid" required="true"/>
  <field name="to" type="array" required="true"/>
  <field name="subject" type="string" required="true"/>
  <field name="body" type="string" required="true"/>
  <field name="scheduled_for" type="timestamp" required="true"/>
  <field name="status" type="enum" values="pending,sent,cancelled"/>
  <field name="created_at" type="timestamp"/>
</schema>
```

---

## FILE VERSIONING SYSTEM

### X-Axis: File Version Operations

```yaml
file_create_version: 0x1030          # NEW
file_list_versions: 0x1031           # NEW
file_restore_version: 0x1032         # NEW
file_compare_versions: 0x1033        # NEW
file_delete_version: 0x1034          # NEW
```

### File Version Schema

```xml
<schema id="file_version">
  <field name="id" type="uuid" required="true"/>
  <field name="file_id" type="uuid" required="true"/>
  <field name="version_number" type="integer" required="true"/>
  <field name="storage_path" type="string" required="true"/>
  <field name="size_bytes" type="integer"/>
  <field name="hash" type="string"/> <!-- SHA-256 -->
  <field name="uploaded_by" type="uuid"/>
  <field name="uploaded_at" type="timestamp"/>
  <field name="change_description" type="string"/>
  <field name="is_current" type="boolean"/>
</schema>
```

### Workflow Example: Create File Version

```xml
<workflow id="file_create_version">
  <entry p="file" x="create_version" node="validate_upload"/>
  
  <nodes>
    <node id="validate_upload" kind="transform">
      <validate>
        <field name="file_id" type="uuid" required="true"/>
        <field name="file_data" type="binary" required="true"/>
        <field name="change_description" type="string"/>
      </validate>
    </node>
    
    <node id="load_current_file" kind="external" op="0x1001">
      <load file_id="$input.file_id"/>
    </node>
    
    <node id="calculate_version_number" kind="external" op="0x1031">
      <query>
        <filter field="file_id" value="$input.file_id"/>
        <sort by="version_number" direction="desc"/>
        <limit value="1"/>
      </query>
      <compute>
        <var name="next_version" value="$latest_version.version_number + 1"/>
      </compute>
    </node>
    
    <node id="compute_hash" kind="transform">
      <compute>
        <var name="file_hash" value="sha256($input.file_data)"/>
      </compute>
    </node>
    
    <node id="check_duplicate" kind="auth">
      <!-- Don't create version if hash matches current -->
      <require>
        <ne left="$file_hash" right="$current_file.hash"/>
      </require>
    </node>
    
    <node id="upload_to_storage" kind="external" op="0x0900">
      <storage>
        <path value="files/{{file_id}}/v{{next_version}}"/>
        <data value="$input.file_data"/>
      </storage>
    </node>
    
    <node id="create_version_record" kind="external" op="0x0910">
      <event>
        <type>file.version_created</type>
        <data>
          <field name="file_id" value="$input.file_id"/>
          <field name="version_number" value="$next_version"/>
          <field name="storage_path" value="$storage_path"/>
          <field name="size_bytes" value="length($input.file_data)"/>
          <field name="hash" value="$file_hash"/>
          <field name="uploaded_by" value="$token.sub"/>
          <field name="change_description" value="$input.change_description"/>
        </data>
      </event>
    </node>
    
    <node id="mark_as_current" kind="external" op="0x1002">
      <!-- Update file record to point to new version -->
      <update file_id="$input.file_id">
        <field name="current_version" value="$next_version"/>
        <field name="updated_at" value="$now"/>
      </update>
    </node>
    
    <node id="render_version_created" kind="render">
      <template ref="file_version_toast">
        <message>Version {{next_version}} created</message>
        <action label="View versions" op="file_list_versions"/>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_upload" to="load_current_file"><when><always/></when></edge>
    <edge from="load_current_file" to="calculate_version_number"><when><always/></when></edge>
    <edge from="calculate_version_number" to="compute_hash"><when><always/></when></edge>
    <edge from="compute_hash" to="check_duplicate"><when><always/></when></edge>
    <edge from="check_duplicate" to="upload_to_storage"><when><always/></when></edge>
    <edge from="upload_to_storage" to="create_version_record"><when><always/></when></edge>
    <edge from="create_version_record" to="mark_as_current"><when><always/></when></edge>
    <edge from="mark_as_current" to="render_version_created"><when><always/></when></edge>
    <edge from="render_version_created" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## FILE SHARING WITH PERMISSIONS

### X-Axis: File Sharing Operations

```yaml
file_share: 0x1040                   # NEW
file_unshare: 0x1041                 # NEW
file_update_permissions: 0x1042      # NEW
file_generate_share_link: 0x1043    # NEW
file_revoke_share_link: 0x1044      # NEW
file_list_shares: 0x1045             # NEW
```

### File Share Schema

```xml
<schema id="file_share">
  <field name="id" type="uuid" required="true"/>
  <field name="file_id" type="uuid" required="true"/>
  <field name="shared_with_type" type="enum" values="user,team,link,portal"/>
  <field name="shared_with_id" type="uuid"/>  <!-- null for link shares -->
  <field name="permission" type="enum" values="view,edit,download"/>
  <field name="expires_at" type="timestamp"/>
  <field name="shared_by" type="uuid"/>
  <field name="created_at" type="timestamp"/>
  <field name="access_count" type="integer"/>
  <field name="last_accessed" type="timestamp"/>
</schema>
```

---

## DOCUMENT APPROVAL WORKFLOWS

### X-Axis: Document Approval Operations

```yaml
document_request_approval: 0x0660   # NEW
document_approve: 0x0661            # NEW
document_reject: 0x0662             # NEW
document_request_changes: 0x0663    # NEW
document_approval_status: 0x0664    # NEW
```

### Document Approval Schema

```xml
<schema id="document_approval">
  <field name="id" type="uuid" required="true"/>
  <field name="document_id" type="uuid" required="true"/>
  <field name="requested_by" type="uuid" required="true"/>
  <field name="approver_id" type="uuid" required="true"/>
  <field name="status" type="enum" values="pending,approved,rejected,changes_requested"/>
  <field name="requested_at" type="timestamp"/>
  <field name="responded_at" type="timestamp"/>
  <field name="comments" type="string"/>
  <field name="changes_requested" type="array"/> <!-- List of change requests -->
</schema>
```

### Workflow Example: Request Document Approval

```xml
<workflow id="document_request_approval">
  <entry p="document" x="request_approval" node="validate_request"/>
  
  <nodes>
    <node id="validate_request" kind="transform">
      <validate>
        <field name="document_id" type="uuid" required="true"/>
        <field name="approver_id" type="uuid" required="true"/>
        <field name="message" type="string"/>
      </validate>
    </node>
    
    <node id="load_document" kind="external" op="0x0601">
      <load document_id="$input.document_id"/>
    </node>
    
    <node id="check_document_state" kind="auth">
      <require>
        <or>
          <eq left="$document.state" right="draft"/>
          <eq left="$document.state" right="in_review"/>
        </or>
      </require>
    </node>
    
    <node id="create_approval_request" kind="external" op="0x0910">
      <event>
        <type>document.approval_requested</type>
        <data>
          <field name="document_id" value="$input.document_id"/>
          <field name="requested_by" value="$token.sub"/>
          <field name="approver_id" value="$input.approver_id"/>
          <field name="status" value="pending"/>
        </data>
      </event>
    </node>
    
    <node id="update_document_state" kind="external" op="0x0602">
      <update document_id="$input.document_id">
        <field name="state" value="in_review"/>
      </update>
    </node>
    
    <node id="notify_approver" kind="external" op="0x0300">
      <notification>
        <recipient value="$input.approver_id"/>
        <template ref="document_approval_request"/>
        <data>
          <field name="document" value="$document"/>
          <field name="requester" value="$token.user"/>
          <field name="message" value="$input.message"/>
        </data>
        <actions>
          <action label="Review Document" op="document_view" params="{id: $document.id}"/>
          <action label="Approve" op="document_approve" params="{approval_id: $approval.id}"/>
          <action label="Request Changes" op="document_request_changes"/>
        </actions>
      </notification>
    </node>
    
    <node id="render_confirmation" kind="render">
      <template ref="approval_request_sent">
        <message>Approval request sent to {{approver.name}}</message>
      </template>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="validate_request" to="load_document"><when><always/></when></edge>
    <edge from="load_document" to="check_document_state"><when><always/></when></edge>
    <edge from="check_document_state" to="create_approval_request"><when><always/></when></edge>
    <edge from="create_approval_request" to="update_document_state"><when><always/></when></edge>
    <edge from="update_document_state" to="notify_approver"><when><always/></when></edge>
    <edge from="notify_approver" to="render_confirmation"><when><always/></when></edge>
    <edge from="render_confirmation" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

### Workflow Example: Approve Document with Review

```xml
<workflow id="document_approve">
  <entry p="document" x="approve" node="load_approval"/>
  
  <nodes>
    <node id="load_approval" kind="external" op="0x0664">
      <load approval_id="$input.approval_id"/>
    </node>
    
    <node id="check_is_approver" kind="auth">
      <require>
        <eq left="$token.sub" right="$approval.approver_id"/>
      </require>
    </node>
    
    <node id="load_document_with_diffs" kind="external" op="0x0601">
      <load document_id="$approval.document_id"/>
      <include field="version_diffs"/>
      <include field="comments"/>
    </node>
    
    <node id="render_approval_interface" kind="render">
      <template ref="document_approval_view">
        <document value="$document"/>
        <show_diffs since_version="$approval.created_at"/>
        <input type="textarea" name="approval_comments" label="Comments (optional)"/>
        <actions>
          <button action="approve" primary="true">Approve</button>
          <button action="request_changes">Request Changes</button>
          <button action="reject" destructive="true">Reject</button>
        </actions>
      </template>
    </node>
    
    <node id="await_decision" kind="signal">
      <await signal="approval_decision"/>
    </node>
    
    <node id="update_approval_status" kind="external" op="0x0910">
      <event>
        <type>document.approved</type>
        <data>
          <field name="approval_id" value="$approval.id"/>
          <field name="document_id" value="$approval.document_id"/>
          <field name="approver_id" value="$token.sub"/>
          <field name="comments" value="$user_decision.comments"/>
          <field name="responded_at" value="$now"/>
        </data>
      </event>
    </node>
    
    <node id="update_document_state" kind="external" op="0x0602">
      <update document_id="$approval.document_id">
        <field name="state" value="approved"/>
        <field name="approved_at" value="$now"/>
        <field name="approved_by" value="$token.sub"/>
      </update>
    </node>
    
    <node id="notify_requester" kind="external" op="0x0300">
      <notification>
        <recipient value="$approval.requested_by"/>
        <template ref="document_approved"/>
        <data>
          <field name="document" value="$document"/>
          <field name="approver" value="$token.user"/>
          <field name="comments" value="$user_decision.comments"/>
        </data>
      </notification>
    </node>
    
    <node id="success" kind="terminal" status="200"/>
  </nodes>
  
  <edges>
    <edge from="load_approval" to="check_is_approver"><when><always/></when></edge>
    <edge from="check_is_approver" to="load_document_with_diffs"><when><always/></when></edge>
    <edge from="load_document_with_diffs" to="render_approval_interface"><when><always/></when></edge>
    <edge from="render_approval_interface" to="await_decision"><when><always/></when></edge>
    <edge from="await_decision" to="update_approval_status"><when><always/></when></edge>
    <edge from="update_approval_status" to="update_document_state"><when><always/></when></edge>
    <edge from="update_document_state" to="notify_requester"><when><always/></when></edge>
    <edge from="notify_requester" to="success"><when><always/></when></edge>
  </edges>
</workflow>
```

---

## DOCUMENT SIGNATURE REQUESTS

### X-Axis: Signature Operations

```yaml
document_request_signature: 0x0670  # NEW
document_sign: 0x0671               # NEW
document_signature_status: 0x0672   # NEW
document_download_signed: 0x0673    # NEW
```

### Signature Request Schema

```xml
<schema id="signature_request">
  <field name="id" type="uuid" required="true"/>
  <field name="document_id" type="uuid" required="true"/>
  <field name="requested_by" type="uuid" required="true"/>
  <field name="signer_id" type="uuid" required="true"/>
  <field name="signature_fields" type="array" required="true"/>
  <field name="status" type="enum" values="pending,signed,declined,expired"/>
  <field name="requested_at" type="timestamp"/>
  <field name="signed_at" type="timestamp"/>
  <field name="expires_at" type="timestamp"/>
  <field name="signed_document_path" type="string"/>
</schema>
```

---

## SUMMARY

This addendum adds **6 major advanced operation categories**:

### Bulk Operations (8 operations):
1. **Task Bulk**: Update, delete, archive, assign, tag, change state, export, create workflow
2. **Email Bulk**: (implied similar pattern)
3. **Contact Bulk**: (implied similar pattern)

### Email Advanced (6 operations):
1. **Snooze**: Snooze, unsnooze, list snoozed with background job scheduling
2. **Scheduled Send**: Schedule, cancel, list scheduled with optimal time AI

### File Management (10 operations):
1. **Versioning**: Create version, list versions, restore, compare, delete with hash deduplication
2. **Sharing**: Share, unshare, update permissions, generate link, revoke link, list shares with expiration

### Document Workflows (9 operations):
1. **Approvals**: Request, approve, reject, request changes, status with diff viewing
2. **Signatures**: Request signature, sign, status, download signed with e-signature integration

### Key Patterns:
- All bulk operations use transaction blocks
- All preview changes before execution
- All append events per item OR single bulk event
- Versioning uses content-addressed storage (SHA-256)
- Sharing supports expiration and access tracking
- Approvals support multi-stage workflows
- Signatures integrate with external providers

**Total New Operations**: 33
- Bulk tasks: 8
- Email snooze/schedule: 6
- File versioning: 5
- File sharing: 6
- Document approvals: 4
- Document signatures: 4
