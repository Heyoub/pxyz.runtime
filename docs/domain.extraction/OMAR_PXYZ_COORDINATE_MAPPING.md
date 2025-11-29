# OMAR PXYZ COORDINATE MAPPING

> **Purpose**: Map every business operation to PXYZ coordinates for implementation.  
> **For**: OMAR graph compiler, runtime execution, workflow designers.

**This document shows HOW business logic (from OMAR_BUSINESS_LOGIC_REFERENCE.md) maps to the PXYZ coordinate system.**

---

## Coordinate System Primer

```
P = WHAT  (entity type)
X = HOW   (operation)
Y = WHERE (constraints/context)
Z = WHEN  (temporal)
```

**Every operation**: `/pxyz/{P}/{X}` with Y-context and Z-timestamp

---

## 1. ANALYTICS OPERATIONS

### Query Events
```
P: analytics_event (or specific entity: contact_event, deal_event, etc.)
X: query
Y: {
  entityTypes?: string[],
  timeRange?: { start: ISODateTime, end: ISODateTime },
  eventType?: string,
  userId: ActorId,
  filters?: object
}
Z: query_timestamp
```

### Aggregate Time Series
```
P: analytics_timeseries
X: aggregate
Y: {
  metric: string,
  granularities: ("hour" | "day" | "week" | "month")[],
  aggregationFunction: "sum" | "average" | "min" | "max" | "count",
  fillGaps: boolean,
  timezone: string,
  userId: ActorId
}
Z: { start: ISODateTime, end: ISODateTime }
```

### Generate Summary
```
P: analytics_summary
X: compute
Y: {
  userId: ActorId,
  timeRange?: { start: ISODateTime, end: ISODateTime }
}
Z: query_timestamp
```

### Build Report
```
P: analytics_report
X: create
Y: {
  userId: ActorId,
  name: string,
  chartType: string,
  timeRange: { start: ISODateTime, end: ISODateTime }
}
Z: creation_timestamp
```

---

## 2. APPROVALS OPERATIONS

### Request Approval
```
P: approval
X: create
Y: {
  userId: ActorId (requester),
  entityType: string,
  entityId: UUID,
  approvers: ActorId[],
  type: "document" | "invoice" | "contract" | "expense" | "change_request" | "general",
  requiresAll: boolean,
  deadline?: ISODateTime
}
Z: request_timestamp
```

### Respond to Approval
```
P: approval
X: respond
Y: {
  userId: ActorId (responder),
  approvalId: UUID,
  response: "approved" | "rejected" | "changes_requested",
  comments?: string,
  changes?: Array<{field: string, suggestion: string}>
}
Z: response_timestamp
```

### List Pending Approvals
```
P: approval
X: query
Y: {
  userId: ActorId,
  status: ["pending", "in_review"],
  sortBy: "deadline" | "requested_date"
}
Z: query_timestamp
```

### Send Reminder
```
P: approval
X: remind
Y: {
  userId: ActorId,
  approvalId: UUID
}
Z: reminder_timestamp
```

### Initiate E-Signature
```
P: esignature
X: create
Y: {
  userId: ActorId,
  approvalId: UUID,
  documentId: UUID,
  signers: Array<{userId: ActorId, role: string, order: number}>
}
Z: initiation_timestamp
```

### Get Approval Analytics
```
P: approval
X: analyze
Y: {
  userId: ActorId,
  dateRange?: { start: ISODateTime, end: ISODateTime }
}
Z: query_timestamp
```

---

## 3. BUSINESS TOOLS OPERATIONS

### Register Quick Tool
```
P: quick_tool
X: create
Y: {
  userId: ActorId,
  name: string,
  category: "navigation" | "creation" | "analysis" | "communication" | "automation",
  description?: string,
  icon?: string,
  shortcut?: string,
  url?: string
}
Z: registration_timestamp
```

### Get Quick Tools
```
P: quick_tool
X: query
Y: {
  userId: ActorId,
  category?: string,
  isActive?: boolean
}
Z: query_timestamp
```

### Update Tool Usage
```
P: quick_tool
X: track_usage
Y: {
  userId: ActorId,
  toolId: UUID,
  executionTime?: number,
  success?: boolean
}
Z: usage_timestamp
```

### Create Tool Chain
```
P: tool_chain
X: create
Y: {
  userId: ActorId,
  name: string,
  steps: Array<{toolId: UUID, order: number, config?: object}>
}
Z: creation_timestamp
```

### Execute Tool Chain
```
P: tool_chain
X: execute
Y: {
  userId: ActorId,
  chainId: UUID
}
Z: execution_timestamp
```

### Register Agent
```
P: registered_agent
X: create
Y: {
  userId: ActorId,
  name: string,
  capabilities: string[]
}
Z: registration_timestamp
```

### Get Tool Recommendations
```
P: quick_tool
X: recommend
Y: {
  userId: ActorId
}
Z: query_timestamp
```

---

## 4. CALENDAR OPERATIONS

### Create Event
```
P: calendar_event
X: create
Y: {
  userId: ActorId,
  title: string,
  startTime: ISODateTime,
  endTime: ISODateTime,
  timezone: string,
  eventType: "meeting" | "appointment" | "reminder" | "deadline" | "personal" | "other",
  location?: string,
  attendees?: Array<{email: string, name?: string}>,
  isAllDay?: boolean,
  isRecurring?: boolean,
  recurrencePattern?: object
}
Z: creation_timestamp
```

### Get Events
```
P: calendar_event
X: query
Y: {
  userId: ActorId,
  calendarId?: UUID,
  timeRange?: { start: ISODateTime, end: ISODateTime },
  eventType?: string,
  search?: string
}
Z: query_timestamp
```

### Check Conflicts
```
P: calendar_event
X: check_conflicts
Y: {
  userId: ActorId,
  startTime: ISODateTime,
  endTime: ISODateTime,
  eventId?: UUID (exclude from conflict check),
  attendees?: ActorId[]
}
Z: check_timestamp
```

### Find Optimal Meeting Times
```
P: calendar_event
X: find_slots
Y: {
  userId: ActorId,
  duration: number (minutes),
  attendees: ActorId[],
  timeRange: { start: ISODateTime, end: ISODateTime },
  constraints?: SchedulingConstraints
}
Z: query_timestamp
```

### Schedule Focus Block
```
P: calendar_event
X: schedule_focus
Y: {
  userId: ActorId,
  duration: number (minutes),
  title?: string,
  preferredTime?: "morning" | "afternoon" | "evening"
}
Z: scheduling_timestamp
```

### Create Calendar
```
P: calendar
X: create
Y: {
  userId: ActorId,
  name: string,
  timezone: string,
  isDefault?: boolean
}
Z: creation_timestamp
```

### Share Calendar
```
P: calendar_share
X: create
Y: {
  userId: ActorId (owner),
  calendarId: UUID,
  sharedWith: ActorId,
  permission: "read" | "write" | "admin"
}
Z: share_timestamp
```

### Get Calendar Analytics
```
P: calendar_event
X: analyze_health
Y: {
  userId: ActorId,
  dateRange?: { start: ISODateTime, end: ISODateTime }
}
Z: analysis_timestamp
```

---

## 5. COMMUNICATIONS OPERATIONS

### Send Message
```
P: message
X: create
Y: {
  userId: ActorId (sender),
  recipientId?: ActorId,
  channelId?: UUID,
  content: string,
  messageType: "text" | "image" | "file" | "voice" | "video" | "system",
  priority?: "low" | "medium" | "high" | "urgent"
}
Z: send_timestamp
```

### Get Messages
```
P: message
X: query
Y: {
  userId: ActorId,
  channelId?: UUID,
  threadId?: UUID,
  recipientId?: ActorId,
  status?: "sent" | "delivered" | "read",
  isRead?: boolean
}
Z: query_timestamp
```

### Create Channel
```
P: channel
X: create
Y: {
  userId: ActorId (creator),
  name: string,
  channelType: "direct" | "group" | "broadcast" | "support" | "announcement",
  participants: ActorId[]
}
Z: creation_timestamp
```

### Get Thread Activity
```
P: message
X: analyze_threads
Y: {
  userId: ActorId,
  constraints?: ThreadActivityConstraints
}
Z: analysis_timestamp
```

### Find Urgent Unread
```
P: message
X: find_urgent
Y: {
  userId: ActorId,
  unreadTimeLimit: number (hours)
}
Z: query_timestamp
```

### Create Template
```
P: message_template
X: create
Y: {
  userId: ActorId,
  name: string,
  category: string,
  template: string,
  variables: Array<{name: string, description: string, defaultValue?: string}>
}
Z: creation_timestamp
```

---

## 6. CONTACTS OPERATIONS

### Create Contact
```
P: contact
X: create
Y: {
  userId: ActorId (owner),
  firstName: string,
  lastName: string,
  email?: string,
  phone?: string,
  company?: string,
  contactType: "lead" | "prospect" | "customer" | "partner" | "vendor",
  source: "website" | "referral" | "cold_outreach" | "event" | "social_media" | "other",
  lifecycleStage?: "awareness" | "interest" | "consideration" | "purchase" | "retention"
}
Z: creation_timestamp
```

### Get Contacts
```
P: contact
X: query
Y: {
  userId: ActorId,
  type?: ContactType,
  status?: ContactStatus,
  source?: ContactSource,
  lifecycleStage?: LifecycleStage,
  search?: string,
  tags?: string[]
}
Z: query_timestamp
```

### Find Duplicates
```
P: contact
X: find_duplicates
Y: {
  userId: ActorId
}
Z: query_timestamp
```

### Merge Contacts
```
P: contact
X: merge
Y: {
  userId: ActorId,
  primaryId: UUID,
  duplicateIds: UUID[]
}
Z: merge_timestamp
```

### Update Lifecycle
```
P: contact
X: update_lifecycle
Y: {
  userId: ActorId,
  contactId: UUID,
  lifecycleStage: LifecycleStage
}
Z: update_timestamp
```

### Log Interaction
```
P: interaction
X: create
Y: {
  userId: ActorId,
  contactId: UUID,
  interactionType: "email" | "call" | "meeting" | "note" | "task",
  subject: string,
  outcome?: "positive" | "neutral" | "negative" | "follow_up_needed"
}
Z: interaction_timestamp
```

### Get Interactions
```
P: interaction
X: query
Y: {
  userId: ActorId,
  contactId: UUID,
  type?: InteractionType,
  dateRange?: { start: ISODateTime, end: ISODateTime }
}
Z: query_timestamp
```

### Create Relationship
```
P: relationship
X: create
Y: {
  userId: ActorId,
  contactId: UUID,
  relatedContactId: UUID,
  relationshipType: "colleague" | "manager" | "subordinate" | "partner" | "vendor" | "customer" | "other",
  strength?: number (0-100)
}
Z: creation_timestamp
```

### Get Relationships
```
P: relationship
X: query
Y: {
  userId: ActorId,
  contactId: UUID,
  depth?: number (degrees of separation)
}
Z: query_timestamp
```

### Enrich Contact
```
P: contact
X: enrich
Y: {
  userId: ActorId,
  contactId: UUID
}
Z: enrichment_timestamp
```

---

## 7. DEALS OPERATIONS

### Create Deal
```
P: deal
X: create
Y: {
  userId: ActorId (owner),
  name: string,
  value: number,
  currency: string,
  stage: "lead" | "qualified" | "proposal" | "negotiation" | "closed_won" | "closed_lost",
  dealType: "new_business" | "expansion" | "renewal" | "upsell",
  expectedCloseDate: ISODateTime,
  probability: number (0-100),
  contactId: UUID,
  company: string
}
Z: creation_timestamp
```

### Move to Stage
```
P: deal
X: update_stage
Y: {
  userId: ActorId,
  dealId: UUID,
  newStage: DealStage,
  reason?: string
}
Z: stage_change_timestamp
```

### Log Activity
```
P: deal_activity
X: create
Y: {
  userId: ActorId,
  dealId: UUID,
  activityType: "call" | "email" | "meeting" | "proposal_sent" | "contract_sent" | "demo" | "presentation" | "note",
  subject: string,
  nextAction?: string
}
Z: activity_timestamp
```

### Get Pipeline
```
P: deal
X: query_pipeline
Y: {
  userId: ActorId,
  stages?: DealStage[],
  dateRange?: { start: ISODateTime, end: ISODateTime }
}
Z: query_timestamp
```

### Forecast Revenue
```
P: deal
X: forecast
Y: {
  userId: ActorId,
  period: { start: ISODateTime, end: ISODateTime },
  confidenceLevel: "best" | "likely" | "worst" | "committed"
}
Z: forecast_timestamp
```

### Score Deal
```
P: deal
X: score
Y: {
  userId: ActorId,
  dealId: UUID
}
Z: scoring_timestamp
```

### Record Loss
```
P: deal
X: record_loss
Y: {
  userId: ActorId,
  dealId: UUID,
  lossReason: "price" | "features" | "timing" | "competitor" | "budget_cut" | "lost_contact" | "no_decision" | "other",
  competitor?: string,
  notes?: string
}
Z: loss_timestamp
```

---

## 8. DOCUMENTS OPERATIONS

### Create Document
```
P: document
X: create
Y: {
  userId: ActorId (creator),
  name: string,
  documentType: "contract" | "proposal" | "invoice" | "report" | "presentation" | "spreadsheet" | "pdf" | "image" | "other",
  file: File,
  folderId?: UUID,
  accessLevel: "private" | "shared" | "public"
}
Z: creation_timestamp
```

### Create Version
```
P: document
X: create_version
Y: {
  userId: ActorId,
  documentId: UUID,
  file: File,
  notes: string
}
Z: version_timestamp
```

### Share Document
```
P: document_share
X: create
Y: {
  userId: ActorId (sharer),
  documentId: UUID,
  sharedWith: ActorId[],
  permission: "view" | "comment" | "edit",
  expiryDate?: ISODateTime
}
Z: share_timestamp
```

### Add Comment
```
P: document_comment
X: create
Y: {
  userId: ActorId,
  documentId: UUID,
  content: string,
  location: { page?: number, paragraph?: number, selection?: string },
  replyTo?: UUID
}
Z: comment_timestamp
```

### Search Documents
```
P: document
X: search
Y: {
  userId: ActorId,
  query: string,
  documentType?: DocumentType[],
  tags?: string[],
  dateRange?: { start: ISODateTime, end: ISODateTime }
}
Z: search_timestamp
```

### Create Template
```
P: document_template
X: create
Y: {
  userId: ActorId,
  name: string,
  templateType: DocumentType,
  file: File,
  variables: Array<{name: string, description: string, required: boolean, defaultValue?: string}>
}
Z: creation_timestamp
```

### Generate from Template
```
P: document
X: generate
Y: {
  userId: ActorId,
  templateId: UUID,
  variables: Record<string, string>
}
Z: generation_timestamp
```

### Process OCR
```
P: document
X: extract_text
Y: {
  userId: ActorId,
  documentId: UUID,
  language?: string
}
Z: processing_timestamp
```

---

## 9. EMAIL OPERATIONS

### Send Email
```
P: email
X: send
Y: {
  userId: ActorId (sender),
  to: string[],
  cc?: string[],
  bcc?: string[],
  subject: string,
  body: string,
  attachments?: File[],
  priority?: "low" | "normal" | "high" | "urgent"
}
Z: send_timestamp
```

### Get Emails
```
P: email
X: query
Y: {
  userId: ActorId,
  folder?: "inbox" | "sent" | "draft" | "archived" | "spam",
  isRead?: boolean,
  threadId?: UUID,
  search?: string,
  from?: string,
  dateRange?: { start: ISODateTime, end: ISODateTime }
}
Z: query_timestamp
```

### Create Rule
```
P: email_rule
X: create
Y: {
  userId: ActorId,
  name: string,
  conditions: Array<{field: string, operator: string, value: string}>,
  actions: Array<{type: string, value: string}>,
  priority?: number
}
Z: creation_timestamp
```

### Create Template
```
P: email_template
X: create
Y: {
  userId: ActorId,
  name: string,
  subject: string,
  body: string,
  variables: Array<{name: string, description: string, defaultValue?: string}>
}
Z: creation_timestamp
```

### Track Email
```
P: email
X: track
Y: {
  userId: ActorId,
  emailId: UUID,
  trackOpens: boolean,
  trackClicks: boolean
}
Z: tracking_start_timestamp
```

---

## 10. CROSS-DOMAIN PATTERNS

### Generic Create
```
P: {entity_type}
X: create
Y: {
  userId: ActorId,
  ...entity_data
}
Z: creation_timestamp
```

### Generic Read
```
P: {entity_type}
X: read
Y: {
  userId: ActorId,
  id: UUID
}
Z: read_timestamp
```

### Generic Query
```
P: {entity_type}
X: query
Y: {
  userId: ActorId,
  filters?: object,
  sort?: Array<{field: string, direction: "asc" | "desc"}>,
  limit?: number,
  offset?: number,
  fields?: string[]
}
Z: query_timestamp
```

### Generic Update
```
P: {entity_type}
X: update
Y: {
  userId: ActorId,
  id: UUID,
  changes: object
}
Z: update_timestamp
```

### Generic Delete
```
P: {entity_type}
X: delete
Y: {
  userId: ActorId,
  id: UUID,
  hard?: boolean (true = hard delete, false = soft delete)
}
Z: deletion_timestamp
```

### Bulk Create
```
P: {entity_type}
X: bulk_create
Y: {
  userId: ActorId,
  entities: Array<entity_data>,
  stopOnError?: boolean
}
Z: batch_timestamp
```

### Bulk Update
```
P: {entity_type}
X: bulk_update
Y: {
  userId: ActorId,
  updates: Array<{id: UUID, changes: object}>,
  stopOnError?: boolean
}
Z: batch_timestamp
```

### Bulk Delete
```
P: {entity_type}
X: bulk_delete
Y: {
  userId: ActorId,
  ids: UUID[],
  stopOnError?: boolean,
  hard?: boolean
}
Z: batch_timestamp
```

### Export
```
P: {entity_type}
X: export
Y: {
  userId: ActorId,
  format: "csv" | "json" | "excel",
  filters?: object,
  fields?: string[],
  compress?: boolean
}
Z: export_timestamp
```

### Import
```
P: {entity_type}
X: import
Y: {
  userId: ActorId,
  file: File,
  format: "csv" | "json" | "excel",
  mapping: Record<string, string>,
  conflictHandling: "skip" | "overwrite" | "merge"
}
Z: import_timestamp
```

### Register Webhook
```
P: webhook
X: create
Y: {
  userId: ActorId,
  url: string,
  events: string[],
  secret: string
}
Z: registration_timestamp
```

### Search All
```
P: * (wildcard)
X: search
Y: {
  userId: ActorId,
  query: string,
  entityTypes?: string[],
  limit?: number
}
Z: search_timestamp
```

---

## Y-CONTEXT STANDARD FIELDS

**Every Y-context includes**:

```typescript
{
  // AUTHENTICATION
  userId: ActorId,              // Who is making the request
  token?: string,               // Auth token (optional, depends on auth system)
  
  // AUTHORIZATION (optional)
  requiresPermission?: string,  // Required permission level
  ownershipCheck?: boolean,     // Verify user owns the entity
  
  // ISOLATION (optional)
  isolationLevel?: "read_uncommitted" | "read_committed" | "repeatable_read" | "serializable",
  
  // FILTERING (optional)
  filters?: {
    field: string,
    operator: "==" | "!=" | ">" | "<" | ">=" | "<=" | "contains" | "in" | "not_in",
    value: any
  }[],
  
  // SORTING (optional)
  sort?: {
    field: string,
    direction: "asc" | "desc"
  }[],
  
  // PAGINATION (optional)
  limit?: number,
  offset?: number,
  cursor?: string,
  
  // PROJECTION (optional)
  fields?: string[],           // Select only these fields
  exclude?: string[],          // Exclude these fields
  
  // METADATA
  traceId?: string,            // For distributed tracing
  requestId?: string,          // Unique request identifier
}
```

---

## Z-TEMPORAL PATTERNS

### Single Timestamp
```
Z: "2025-01-20T10:30:00Z"  // Specific moment
```

### Time Range
```
Z: {
  start: "2025-01-01T00:00:00Z",
  end: "2025-01-31T23:59:59Z"
}
```

### Recurring Time
```
Z: {
  pattern: "daily" | "weekly" | "monthly" | "yearly",
  interval: number,
  startDate: ISODateTime,
  endDate?: ISODateTime
}
```

### Time Window
```
Z: {
  windowStart: ISODateTime,
  windowEnd: ISODateTime,
  bucketSize: "hour" | "day" | "week" | "month"
}
```

---

## GRAPH NODE TYPES & OPERATION MAPPING

### External Node Operations (op codes)

**Entity Operations (0x01xx)**:
```
0x0100: ENTITY_CREATE     → X: create
0x0101: ENTITY_READ       → X: read
0x0102: ENTITY_UPDATE     → X: update
0x0103: ENTITY_DELETE     → X: delete
0x0104: ENTITY_LIST       → X: query
0x0105: ENTITY_SEARCH     → X: search
```

**Google Workspace (0x03xx)**:
```
0x0300: GOOGLE_CONTACTS_SEARCH    → X: google_search (P: contact)
0x0301: GOOGLE_CONTACTS_GET       → X: google_read (P: contact)
0x0302: GOOGLE_CONTACTS_CREATE    → X: google_create (P: contact)
0x0310: GOOGLE_CALENDAR_LIST      → X: google_list (P: calendar_event)
0x0320: GOOGLE_DRIVE_SEARCH       → X: google_search (P: document)
0x0330: GOOGLE_GMAIL_SEARCH       → X: google_search (P: email)
```

**Vector/RAG (0x07xx)**:
```
0x0700: QDRANT_SEARCH        → X: vector_search
0x0701: QDRANT_INDEX         → X: vector_index
0x0710: EMBEDDING_GENERATE   → X: generate_embedding
```

**AI (0x08xx)**:
```
0x0800: LLM_COMPLETE         → X: llm_complete
0x0801: LLM_CLASSIFY         → X: llm_classify
0x0810: LOCAL_MODEL_RUN      → X: local_llm_run
```

**Storage (0x09xx)**:
```
0x0900: STORAGE_GET          → X: storage_read
0x0901: STORAGE_SET          → X: storage_write
0x0910: EVENT_LOG_APPEND     → X: event_append
0x0911: EVENT_LOG_QUERY      → X: event_query
```

---

## URL TO PXYZ MAPPING

**REST-style URLs map to PXYZ**:

```
POST   /contact              → P: contact, X: create
GET    /contact/:id          → P: contact, X: read
PUT    /contact/:id          → P: contact, X: update
DELETE /contact/:id          → P: contact, X: delete
GET    /contact              → P: contact, X: query

POST   /contact/:id/enrich   → P: contact, X: enrich
POST   /contact/merge        → P: contact, X: merge
GET    /contact/duplicates   → P: contact, X: find_duplicates

POST   /deal                 → P: deal, X: create
PUT    /deal/:id/stage       → P: deal, X: update_stage
GET    /deal/pipeline        → P: deal, X: query_pipeline
GET    /deal/forecast        → P: deal, X: forecast

POST   /email/send           → P: email, X: send
GET    /email                → P: email, X: query
POST   /email/rule           → P: email_rule, X: create

POST   /approval             → P: approval, X: create
POST   /approval/:id/respond → P: approval, X: respond
GET    /approval/pending     → P: approval, X: query (with status filter)
```

**Query parameters become Y-context**:
```
GET /contact?type=customer&status=active&search=acme

→ P: contact
  X: query
  Y: {
    filters: {
      type: "customer",
      status: "active",
      search: "acme"
    }
  }
```

---

## PREDICATES & Y-CONSTRAINTS

**Authorization Predicates**:
```xml
<predicate id="is_owner">
  <eq left="$token.userId" right="$entity.userId"/>
</predicate>

<predicate id="can_write">
  <contains left="$token.permissions" right="write"/>
</predicate>

<predicate id="can_access">
  <or>
    <ref predicate="is_owner"/>
    <contains left="$token.permissions" right="admin"/>
  </or>
</predicate>
```

**Business Rule Predicates**:
```xml
<predicate id="is_high_value">
  <gt left="$deal.value" right="10000"/>
</predicate>

<predicate id="needs_approval">
  <and>
    <ref predicate="is_high_value"/>
    <eq left="$deal.stage" right="proposal"/>
  </and>
</predicate>

<predicate id="is_overdue">
  <gt left="$now" right="$entity.deadline"/>
</predicate>
```

**Filter Predicates**:
```xml
<predicate id="active_contacts">
  <eq left="$contact.status" right="active"/>
</predicate>

<predicate id="recent_deals">
  <gt left="$deal.createdAt" right="$constraint.minDate"/>
</predicate>
```

---

## WORKFLOW ENTRY POINTS

**Each workflow has entry points for P+X combinations**:

```xml
<workflow id="contact_management">
  <entry p="contact" x="create" node="validate_contact"/>
  <entry p="contact" x="update" node="validate_update"/>
  <entry p="contact" x="enrich" node="fetch_enrichment"/>
  <entry p="contact" x="merge" node="validate_merge"/>
</workflow>

<workflow id="deal_pipeline">
  <entry p="deal" x="create" node="validate_deal"/>
  <entry p="deal" x="update_stage" node="validate_stage_transition"/>
  <entry p="deal" x="score" node="calculate_score"/>
</workflow>

<workflow id="approval_flow">
  <entry p="approval" x="create" node="validate_approval_request"/>
  <entry p="approval" x="respond" node="validate_responder"/>
  <entry p="approval" x="remind" node="send_reminders"/>
</workflow>
```

---

## IMPLEMENTATION CHECKLIST

**For each business operation**:

1. ✅ Define P (entity type)
2. ✅ Define X (operation name)
3. ✅ Define Y-context schema (required fields, optional fields, predicates)
4. ✅ Define Z-temporal pattern (single timestamp, range, etc.)
5. ✅ Create workflow with entry point
6. ✅ Define validation node (transform)
7. ✅ Define auth node (predicate check)
8. ✅ Define business logic nodes (external calls, transforms)
9. ✅ Define output/render node
10. ✅ Connect with edges (with predicates)
11. ✅ Add to graph.bin compilation
12. ✅ Test with sample inputs
13. ✅ Document in API reference

---

**END OF PXYZ COORDINATE MAPPING**

This document provides the complete coordinate addressing scheme for all business operations. Use this when:
- Designing XML workflows
- Implementing graph traversal
- Building IO adapters
- Creating API endpoints
- Writing business logic tests
