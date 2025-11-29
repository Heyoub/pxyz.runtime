# OMAR BUSINESS LOGIC REFERENCE (COMPLETE EDITION)

> **Purpose**: Technology-agnostic specification of ALL business logic OMAR must implement.  
> **Source**: Extracted from 21 TypeScript Effects Programs (22,407 lines total).  
> **For**: Future agents, developers, and AI systems implementing OMAR in ANY architecture.

**CRITICAL**: This document describes WHAT the system does, not HOW. No TypeScript, no Effect-TS, no implementation details. Pure business requirements that could be implemented in PXYZ/OMAR, traditional backend, or any other architecture.

---

## Document Statistics

- **Total Files**: 21 domain programs
- **Total Lines**: 22,407 lines of TypeScript
- **Business Domains**: 20 complete domains
- **Operations**: 200+ distinct business operations
- **Coverage**: Complete CRM + Project Management + Customer Support

---

## Table of Contents

**ORIGINAL 9 DOMAINS** (from first extraction):
1. [Analytics](#1-analytics)
2. [Approvals & E-Signatures](#2-approvals--e-signatures)
3. [Business Tools & Agent Orchestration](#3-business-tools--agent-orchestration)
4. [Calendar & Scheduling](#4-calendar--scheduling)
5. [Communications](#5-communications)
6. [Contacts](#6-contacts)
7. [Deals & Sales](#7-deals--sales)
8. [Documents](#8-documents)
9. [Email](#9-email)

**NEW 11 DOMAINS** (from second extraction):
10. [Evolution & Lifecycle](#10-evolution--lifecycle)
11. [Files & Attachments](#11-files--attachments)
12. [Invoices & Billing](#12-invoices--billing)
13. [Notes & Comments](#13-notes--comments)
14. [Notifications](#14-notifications)
15. [Portal & External Access](#15-portal--external-access)
16. [Search & Discovery](#16-search--discovery)
17. [Tasks & Project Management](#17-tasks--project-management)
18. [Tickets & Support](#18-tickets--support)
19. [Wizard & Guided Flows](#19-wizard--guided-flows)
20. [Workflows & Automation](#20-workflows--automation)

**CROSS-DOMAIN**:
21. [Universal Patterns](#21-cross-domain-patterns)

---

## 1-9. ORIGINAL DOMAINS

**See OMAR_BUSINESS_LOGIC_REFERENCE.md for complete details on**:
- Analytics (time series, metrics, reports)
- Approvals & E-Signatures (approval flows, e-sign)
- Business Tools (quick tools, agents, tool chains)
- Calendar & Scheduling (events, conflicts, focus blocks)
- Communications (messages, channels, threads)
- Contacts (CRM, interactions, relationships)
- Deals & Sales (pipeline, forecasting, scoring)
- Documents (versioning, collaboration, OCR)
- Email (send/receive, threading, tracking)

*These are unchanged from the first extraction and remain fully valid.*

---

## 10. EVOLUTION & LIFECYCLE

### 10.1 Entity Lifecycle Tracking

**Purpose**: Track entity state changes over time with full history.

**Lifecycle Events**:
- created
- updated
- status_changed
- stage_changed
- archived
- deleted
- restored

**Record Lifecycle Change**:

**Inputs**:
- Entity type and ID
- Event type
- Previous state (snapshot)
- New state (snapshot)
- Changed by user ID
- Reason/notes (optional)
- Timestamp

**Business Logic**:
1. Capture full entity state before change
2. Apply change
3. Capture full entity state after change
4. Create lifecycle event with both states
5. Calculate diff (which fields changed)
6. Record metadata (IP, user agent, etc.)

**Output**:
```
{
  eventId: UUID,
  entityType: string,
  entityId: UUID,
  eventType: "status_changed" | etc.,
  before: object,
  after: object,
  diff: {field: {from: value, to: value}},
  changedBy: ActorId,
  changedAt: ISODateTime,
  reason: string
}
```

### 10.2 Lifecycle Analytics

**Get Entity History**:
- Input: Entity type and ID
- Output: Chronological list of all lifecycle events
- Shows: Who changed what, when, and why

**Lifecycle Metrics**:
- **Time in stage**: How long entity stayed in each stage
- **Transition frequency**: How often entities move between stages
- **Common paths**: Most frequent stage progression paths
- **Bottlenecks**: Stages where entities get stuck
- **Velocity**: Average time from created to completed

**Lifecycle Visualization**:
- Timeline view (horizontal timeline with events)
- Sankey diagram (flow between stages)
- Funnel view (progression through stages)

### 10.3 Automated Lifecycle Rules

**Define Lifecycle Rule**:

**Inputs**:
- Name
- Entity type
- Trigger condition (e.g., "when status = 'approved'")
- Actions array:
  - Move to stage
  - Send notification
  - Create task
  - Update field
  - Call webhook
- User ID (rule creator)

**Rule Execution**:
```
When entity state changes:
  For each active rule:
    if trigger condition matches:
      execute all actions
      log rule execution
```

---

## 11. FILES & ATTACHMENTS

### 11.1 File Management

**File Types Supported**:
- Documents (PDF, DOC, DOCX, TXT, MD)
- Images (JPG, PNG, GIF, SVG, WEBP)
- Spreadsheets (XLS, XLSX, CSV)
- Presentations (PPT, PPTX)
- Archives (ZIP, RAR, TAR, GZ)
- Video (MP4, MOV, AVI)
- Audio (MP3, WAV, M4A)
- Other (any MIME type)

**Upload File**:

**Inputs**:
- File (binary data or stream)
- Filename
- MIME type
- User ID (uploader)
- Linked entity type and ID (optional)
- Description (optional)
- Tags (optional)
- Access level: private | shared | public

**Business Logic**:
1. Validate file type (check against allowed types)
2. Validate file size (max 100MB by default)
3. Scan for viruses/malware
4. Generate unique file ID
5. Store file (cloud storage or file system)
6. Extract metadata:
   - File size
   - Dimensions (for images/video)
   - Duration (for audio/video)
   - Page count (for documents)
   - EXIF data (for images)
7. Generate preview/thumbnail (if applicable)
8. Create file record in database

**Output**:
```
{
  fileId: UUID,
  filename: string,
  mimeType: string,
  size: number (bytes),
  url: string (access URL),
  thumbnailUrl: string (optional),
  metadata: object,
  uploadedBy: ActorId,
  uploadedAt: ISODateTime
}
```

### 11.2 File Attachments

**Attach File to Entity**:

**Inputs**:
- File ID
- Entity type and ID
- Attachment type: primary | supporting | thumbnail | etc.
- User ID

**Business Logic**:
1. Validate file exists
2. Validate entity exists
3. Check user has permission to modify entity
4. Create attachment link
5. Update entity metadata (attachment count, etc.)

**Output**: Attachment ID

**Get Attachments**:
- Input: Entity type and ID
- Output: Array of attached files with metadata
- Sort: By upload date (newest first)
- Filter: By file type, attachment type

### 11.3 File Operations

**Download File**:
- Input: File ID, user ID
- Validation: User has permission to access
- Output: File stream or download URL
- Track: Download count, last downloaded timestamp

**Delete File**:
- Input: File ID, user ID
- Validation: User is owner or has delete permission
- Options: Soft delete (mark as deleted) or hard delete (remove from storage)
- Cascade: Remove attachments to entities

**Update File**:
- Input: File ID, new file data, user ID
- Creates: New version (if versioning enabled)
- Preserves: Original file as version

### 11.4 File Search

**Search Files**:

**Search Fields**:
- Filename
- File content (text extraction)
- Description
- Tags
- Uploader
- MIME type
- Size range
- Upload date range
- Linked entity

**Full-Text Search** (for supported types):
- Extract text from: PDF, DOC, DOCX, TXT, MD, CSV
- Index content
- Search with highlighting

### 11.5 File Sharing

**Share File**:

**Inputs**:
- File ID
- Share with user ID(s) or make public
- Permission: view | download | edit
- Expiry date (optional)
- Password protection (optional)
- User ID (sharer)

**Business Logic**:
1. Create share link
2. Set permissions
3. Set expiry if provided
4. Generate secure token for access
5. Send notification to shared users

**Access Shared File**:
- Input: Share token
- Validation: Token valid, not expired, password correct (if set)
- Output: File access

---

## 12. INVOICES & BILLING

### 12.1 Invoice Management

**Invoice Statuses**:
- draft
- sent
- viewed (customer opened invoice)
- paid
- partially_paid
- overdue
- cancelled
- refunded

**Create Invoice**:

**Inputs**:
- Customer ID (contact or company)
- Line items array: [{description, quantity, unit_price, tax_rate, discount?}]
- Issue date
- Due date
- Currency (default: USD)
- Tax settings
- Payment terms
- Notes (optional)
- User ID (creator)

**Business Logic**:
1. Generate invoice number (auto-increment or custom format)
2. Calculate line item totals
3. Calculate subtotal (sum of line items)
4. Calculate tax (based on tax rates)
5. Calculate discounts (if any)
6. Calculate total amount
7. Set status: "draft"
8. Create invoice record

**Output**: Invoice ID

**Calculated Fields**:
```
{
  subtotal: sum of (quantity * unit_price) for all line items,
  tax_total: sum of (line_item_total * tax_rate) for all line items,
  discount_total: sum of all discounts,
  total: subtotal + tax_total - discount_total
}
```

### 12.2 Invoice Operations

**Send Invoice**:

**Inputs**:
- Invoice ID
- Send to email(s)
- CC email(s) (optional)
- Include payment link
- User ID

**Business Logic**:
1. Validate invoice exists and status != "cancelled"
2. Generate PDF invoice
3. Create payment link (if requested)
4. Send email with PDF attachment
5. Update status: "sent"
6. Record sent timestamp
7. Track email opens (if tracking enabled)

**Record Payment**:

**Inputs**:
- Invoice ID
- Amount paid
- Payment method (credit_card, bank_transfer, check, cash, other)
- Payment date
- Transaction ID (optional)
- Notes (optional)
- User ID

**Business Logic**:
1. Validate invoice exists
2. Validate amount > 0
3. Create payment record
4. Add payment to invoice
5. Calculate remaining balance: total - sum(payments)
6. Update status:
   - If remaining == 0: "paid"
   - If remaining > 0: "partially_paid"
7. If fully paid: record paid_at timestamp
8. Send payment confirmation email

**Void Invoice**:
- Input: Invoice ID, reason, user ID
- Updates: Status = "cancelled", records void reason and timestamp
- Preserves: Original invoice data for audit

**Refund Invoice**:
- Input: Invoice ID, refund amount, reason, user ID
- Creates: Credit note or refund record
- Updates: Status based on refund amount (partial vs. full)

### 12.3 Recurring Invoices

**Create Recurring Invoice Template**:

**Inputs**:
- Customer ID
- Line items (same structure as regular invoice)
- Recurrence pattern: {frequency, interval, start_date, end_date?}
- Auto-send: boolean (send automatically or create draft)
- User ID

**Recurrence Frequencies**:
- daily
- weekly
- monthly (same day each month)
- yearly (same date each year)

**Business Logic**:
1. Create template record
2. Set up scheduled task for recurrence
3. On each occurrence date:
   - Generate new invoice from template
   - Update dates (issue date, due date)
   - If auto-send: send invoice
   - Else: create as draft

### 12.4 Invoice Analytics

**Metrics**:
- **Total invoiced**: Sum of all invoice totals
- **Total paid**: Sum of all payments
- **Outstanding balance**: Total invoiced - total paid
- **Overdue amount**: Sum of overdue invoices
- **Average payment time**: Average days from issue to payment
- **Payment rate**: Paid invoices / total invoices * 100
- **By customer**: Metrics grouped by customer
- **By period**: Metrics over time (monthly, quarterly, yearly)

**Aging Report**:
- Current (due date not passed)
- 1-30 days overdue
- 31-60 days overdue
- 61-90 days overdue
- 90+ days overdue

---

## 13. NOTES & COMMENTS

### 13.1 Note Management

**Note Types**:
- text (plain text or rich text)
- checklist (to-do items)
- code (syntax-highlighted code)
- table (structured data)
- drawing (canvas/whiteboard)

**Create Note**:

**Inputs**:
- Title
- Content (text, HTML, or structured data depending on type)
- Note type
- User ID (author)
- Linked entity type and ID (optional)
- Tags (optional)
- Is private: boolean (default: false)
- Parent note ID (optional, for nested notes)

**Business Logic**:
1. Create note record
2. If rich text: sanitize HTML
3. If linked to entity: create link
4. If has parent: create parent-child relationship
5. Index content for search
6. Set created/updated timestamps

**Output**: Note ID

### 13.2 Note Operations

**Update Note**:
- Input: Note ID, changes, user ID
- Validation: User is author or has edit permission
- Updates: Content, tags, etc.
- Preserves: Original creation info
- Optional: Create version (if versioning enabled)

**Delete Note**:
- Input: Note ID, user ID
- Validation: User is author or has delete permission
- Options: Soft delete (mark deleted) or hard delete
- Cascade: Delete child notes (if any)

**Pin Note**:
- Input: Note ID, user ID
- Makes note appear at top of note list
- User-specific (each user can pin different notes)

**Archive Note**:
- Input: Note ID, user ID
- Moves note to archive (hidden from main view)
- Can be restored later

### 13.3 Comments

**Comments vs. Notes**:
- Notes: Standalone content
- Comments: Always attached to an entity (conversation threads)

**Add Comment**:

**Inputs**:
- Entity type and ID (what is being commented on)
- Content
- User ID (commenter)
- Parent comment ID (optional, for threaded replies)
- Mentions array (optional): ActorId[] (notify these users)

**Business Logic**:
1. Validate entity exists
2. Create comment record
3. Link to entity
4. If parent comment: create reply relationship
5. Extract mentions (users mentioned with @username)
6. Send notifications to:
   - Mentioned users
   - Entity owner
   - Other commenters on same entity (if they've opted in)
7. Update entity's last activity timestamp

**Output**: Comment ID

**Comment Threads**:
- Top-level comments (parent_id = null)
- Replies (parent_id = comment_id)
- Can nest replies (threaded conversations)
- Sort: By timestamp (newest first) or by votes

### 13.4 Reactions & Votes

**React to Note/Comment**:

**Inputs**:
- Target ID (note or comment ID)
- Reaction type: 👍 | 👎 | ❤️ | 😄 | 😮 | 🎉 | etc.
- User ID

**Business Logic**:
1. Check if user already reacted
2. If yes: Remove previous reaction, add new one (or toggle off if same)
3. If no: Add reaction
4. Update reaction counts

**Vote on Comment**:
- Upvote: +1 score
- Downvote: -1 score
- Score = upvotes - downvotes
- Use for sorting (highest score first)

### 13.5 Note Search

**Search Notes**:

**Search Fields**:
- Title
- Content (full-text search)
- Tags
- Author
- Created date range
- Linked entity
- Note type

**Filters**:
- My notes only
- Shared with me
- Pinned notes
- Archived notes
- By tag

---

## 14. NOTIFICATIONS

### 14.1 Notification Management

**Notification Types**:
- info (informational message)
- warning (caution message)
- error (error message)
- success (success confirmation)
- reminder (time-based reminder)

**Notification Channels**:
- email (send email)
- push (browser/mobile push notification)
- sms (text message)
- in_app (notification center in app)
- webhook (HTTP POST to external URL)

**Notification Statuses**:
- pending (queued for delivery)
- sent (delivered to channel)
- delivered (confirmed delivery)
- failed (delivery failed)
- read (user read notification)

**Notification Priorities**:
- low (batch with other notifications)
- medium (send soon)
- high (send immediately)
- urgent (send immediately + all channels)

**Create Notification**:

**Inputs**:
- User ID (recipient)
- Title
- Message
- Type
- Priority (optional, default: medium)
- Channels array (which channels to use)
- Action URL (optional - where to go when clicked)
- Action data (optional - context for action)
- Schedule for later (optional timestamp)
- Expires at (optional timestamp)

**Business Logic**:
1. Create notification record
2. Set status: "pending"
3. If scheduled for later: queue for scheduled delivery
4. Else: queue for immediate delivery
5. For each channel:
   - Check user preferences (has user enabled this channel?)
   - Check quiet hours (is it during user's quiet hours?)
   - If checks pass: queue delivery
6. Track notification creation

**Output**: Notification ID

### 14.2 Notification Delivery

**Delivery Process** (per channel):

**Email**:
1. Render email template
2. Send via SMTP or email service
3. Track delivery status
4. Track opens (tracking pixel)
5. Track clicks (tracked links)

**Push**:
1. Format push payload
2. Send to push service (FCM, APNs, web push)
3. Track delivery confirmation

**SMS**:
1. Format SMS message (160 char limit or split)
2. Send via SMS gateway (Twilio, etc.)
3. Track delivery receipt

**In-App**:
1. Store in notification center
2. If user online: emit real-time event (WebSocket)
3. Show badge count (unread notifications)

**Webhook**:
1. Format HTTP POST payload
2. Send to webhook URL
3. Retry on failure (exponential backoff, max 5 attempts)
4. Record response

### 14.3 Notification Preferences

**Set User Preferences**:

**Inputs**:
- User ID
- Preferences object:
  ```
  {
    channels: {
      email: {enabled: true, types: ["all"] or ["reminder", "warning"]},
      push: {enabled: true, types: ["urgent"]},
      sms: {enabled: false, types: []},
      in_app: {enabled: true, types: ["all"]}
    },
    quiet_hours: {
      enabled: true,
      start: "22:00",
      end: "08:00",
      timezone: "America/New_York"
    },
    frequency: {
      email: "immediate" | "hourly_digest" | "daily_digest",
      push: "immediate"
    }
  }
  ```

**Business Logic**:
1. Validate preferences structure
2. Update user notification preferences
3. Apply immediately to pending notifications

**Digest Delivery** (if enabled):
- Hourly digest: Group notifications from past hour, send one email
- Daily digest: Group notifications from past day, send one email
- Include: Count per type, summary of each notification

### 14.4 Notification Actions

**Mark as Read**:
- Input: Notification ID, user ID
- Updates: Status = "read", read_at timestamp
- Decrements: Unread badge count

**Mark All as Read**:
- Input: User ID
- Updates: All unread notifications for user
- Resets: Badge count to 0

**Delete Notification**:
- Input: Notification ID, user ID
- Soft delete: Hide from user's notification center
- Preserves: Notification in DB for audit

**Snooze Notification**:
- Input: Notification ID, snooze until timestamp
- Hides notification until snooze time
- Reappears at snooze time

### 14.5 Notification Analytics

**Metrics**:
- **Total sent**: Count by channel, by type
- **Delivery rate**: Delivered / sent * 100 (by channel)
- **Read rate**: Read / delivered * 100 (in-app)
- **Open rate**: Opened / delivered * 100 (email)
- **Click rate**: Clicked / delivered * 100 (email, push)
- **Response time**: Average time from sent to read
- **Unsubscribe rate**: Unsubscribes / sent * 100

**By Notification Type**:
- Which types have highest read/open rates
- Which types are most often ignored
- Which types lead to actions

---

## 15. PORTAL & EXTERNAL ACCESS

### 15.1 Customer/Partner Portal

**Purpose**: Give external users (customers, partners, vendors) access to specific data.

**Portal Types**:
- customer_portal (customers view their data)
- partner_portal (partners collaborate)
- vendor_portal (vendors submit invoices, etc.)
- public_portal (public-facing forms, resources)

**Create Portal**:

**Inputs**:
- Portal name
- Portal type
- Subdomain (e.g., support.company.com)
- Theme/branding (logo, colors)
- Enabled features: [tickets, invoices, documents, etc.]
- User ID (portal creator)

**Output**: Portal ID

### 15.2 Portal Users

**Invite Portal User**:

**Inputs**:
- Portal ID
- Email
- Name
- Role: viewer | editor | admin
- Linked entity ID (e.g., customer ID)
- Send invite: boolean
- User ID (inviter)

**Business Logic**:
1. Create portal user account (separate from internal users)
2. Set temporary password
3. Link to portal and entity
4. If send_invite: email invitation with portal URL and temp password
5. User must reset password on first login

**Portal User Permissions**:
- Controlled by portal type and role
- Example (customer portal):
  - View: Own tickets, invoices, documents
  - Edit: Update ticket descriptions, upload attachments
  - Cannot: View other customers' data, create invoices

### 15.3 Portal Access

**Login to Portal**:
- Input: Email, password
- Validation: Credentials match, account active, portal enabled
- Creates: Session token
- Tracks: Login timestamp, IP address
- Returns: Session token + portal config

**Portal Data Access**:
- Filter ALL queries by portal user's linked entity
- E.g., customer can only see their tickets, not all tickets
- Enforced at API level (Y-context constraints)

**Portal Views**:
- Dashboard (metrics, recent activity)
- Tickets (submit, view status, add comments)
- Invoices (view, download PDF, pay)
- Documents (view shared documents)
- Knowledge base (search articles)

### 15.4 Portal Customization

**Portal Settings**:
- Custom domain (e.g., support.company.com)
- Logo upload
- Color scheme (primary, secondary, accent)
- Custom CSS (advanced)
- Header/footer content
- Welcome message
- Terms & conditions
- Privacy policy

**Portal Features** (enable/disable):
- Ticket submission
- Invoice viewing
- Document access
- Live chat
- Knowledge base
- Community forum

---

## 16. SEARCH & DISCOVERY

### 16.1 Universal Search

**Purpose**: Search across ALL entity types with single query.

**Search Everywhere**:

**Inputs**:
- Query (text)
- User ID
- Filters (optional):
  - Entity types to search (or all)
  - Date range
  - Created by user
  - Tags
- Limit (default: 20)

**Business Logic**:
1. Tokenize query (split into terms)
2. For each enabled entity type:
   - Search in: title, name, description, content, tags
   - Apply user permissions (only show accessible entities)
   - Rank by relevance (TF-IDF or BM25)
3. Merge results from all entity types
4. Sort by relevance score (descending)
5. Group by entity type for UI

**Output**:
```
{
  total: number,
  results: [
    {
      entityType: "contact",
      entityId: UUID,
      title: string,
      snippet: string (excerpt with highlighted query terms),
      score: number,
      url: string
    },
    ...
  ],
  facets: {
    byEntityType: {contact: 5, deal: 3, document: 8},
    byDate: {"2025-01": 10, "2025-02": 6},
    byTag: {urgent: 4, customer: 7}
  }
}
```

### 16.2 Advanced Search

**Structured Search**:

**Inputs**:
- Entity type
- Field filters: [{field, operator, value}]
  - Operators: equals, not_equals, contains, starts_with, gt, lt, gte, lte, in, not_in
- Full-text query (optional)
- Sort by field(s)
- Limit, offset

**Examples**:
```
Entity: deal
Filters:
  - status in ["qualified", "proposal"]
  - value >= 10000
  - expected_close_date <= "2025-12-31"
  - owner_id = current_user
Sort: value desc
```

### 16.3 Saved Searches

**Save Search**:

**Inputs**:
- Name
- Entity type
- Search criteria (filters, query, sort)
- User ID
- Share with team (boolean)

**Business Logic**:
1. Create saved search record
2. Store search criteria as JSON
3. If shared: other team members can use it

**Use Saved Search**:
- Input: Saved search ID
- Execute: Run the saved search criteria
- Returns: Current results (always fresh data)

**Smart Folders**:
- Saved searches that appear as folders in UI
- Example: "High-value deals closing this month"
- Dynamically update as data changes

### 16.4 Search Analytics

**Track Metrics**:
- Most searched terms
- Search term → result count (identify searches with no results)
- Search → click-through rate (did user click a result?)
- Entity types most searched
- Searches per user

**Use Cases**:
- Improve search ranking algorithms
- Identify missing content (searches with no results)
- Understand user behavior

---

## 17. TASKS & PROJECT MANAGEMENT

### 17.1 Task Management

**Task Statuses**:
- pending (not started)
- in_progress (actively working)
- completed (done)
- cancelled (not doing)
- on_hold (paused)

**Task Priorities**:
- low
- medium
- high
- urgent

**Create Task**:

**Inputs**:
- Title
- Description (optional)
- Status (default: pending)
- Priority (default: medium)
- Assigned to user ID (optional)
- Created by user ID
- Due date (optional)
- Estimated hours (optional)
- Linked entity type and ID (optional)
- Tags (optional)
- Parent task ID (optional, for subtasks)

**Business Logic**:
1. Create task record
2. If assigned: send notification to assignee
3. If has parent: create parent-child relationship
4. If due date: schedule reminder (1 day before, 1 hour before)
5. Calculate "earliest start" based on dependencies (if any)

**Output**: Task ID

### 17.2 Task Dependencies

**Dependency Types**:
- **finish_to_start** (task B starts after task A finishes)
- **start_to_start** (task B starts when task A starts)
- **finish_to_finish** (task B finishes when task A finishes)
- **start_to_finish** (task B finishes when task A starts) [rare]

**Add Dependency**:

**Inputs**:
- Task ID
- Depends on task ID
- Dependency type
- User ID

**Business Logic**:
1. Validate both tasks exist
2. Check for circular dependencies (cannot create cycles)
3. If cycle detected: return error
4. Create dependency record
5. Recalculate earliest start times for dependent tasks

**Circular Dependency Detection**:
```
Use depth-first search (DFS):
  visited = set()
  rec_stack = set()
  
  For each task:
    if task in rec_stack: CYCLE DETECTED
    if task in visited: skip
    
    visited.add(task)
    rec_stack.add(task)
    
    For each dependency:
      recurse(dependency)
    
    rec_stack.remove(task)
```

**Earliest Start Time Calculation**:
```
For finish_to_start:
  task.earliest_start = max(dependency.finish_time) for all dependencies

For start_to_start:
  task.earliest_start = max(dependency.start_time) for all dependencies
```

### 17.3 Task Templates

**Create Task Template**:

**Inputs**:
- Name
- Description (optional)
- Title (task title template)
- Priority
- Estimated hours
- Tags
- Subtasks array (optional): [{title, description, estimated_hours}]
- User ID

**Use Template**:
- Input: Template ID, context variables
- Creates: Task + subtasks from template
- Substitutes: Variables in title/description (e.g., {{customer_name}})

**Example**:
```
Template: "Onboard New Customer"
Subtasks:
  1. Send welcome email
  2. Schedule kickoff call
  3. Create project folder
  4. Add to CRM

When used:
  Creates parent task "Onboard New Customer: Acme Corp"
  Creates 4 subtasks
```

### 17.4 Task Scheduling

**Auto-Schedule Tasks**:

**Inputs**:
- Task IDs (tasks to schedule)
- Assignee availability (calendar integration)
- Start date (earliest)
- End date (deadline)

**Business Logic**:
1. Get all task dependencies
2. Calculate critical path (longest dependency chain)
3. For each task in topological order:
   - Find earliest available time slot
   - Consider: Dependencies, assignee availability, estimated hours
   - Assign start and end times
4. If cannot fit all tasks before deadline: flag as "at risk"

**Output**:
```
{
  schedule: [
    {taskId, assignee, startTime, endTime},
    ...
  ],
  criticalPath: [task IDs in critical path],
  atRisk: boolean,
  slack: number (days of buffer before deadline)
}
```

### 17.5 Task Analytics

**Metrics**:
- **Total tasks**: By status, by priority, by assignee
- **Completion rate**: Completed / total * 100
- **Average time to complete**: Mean time from created to completed
- **Overdue tasks**: Count of tasks past due date
- **On-time completion rate**: Completed by due date / total completed * 100
- **Workload by assignee**: Tasks per assignee, hours per assignee
- **Burndown chart**: Completed tasks over time vs. planned

**Task Velocity**:
- Tasks completed per week
- Used for sprint planning and forecasting

---

## 18. TICKETS & SUPPORT

### 18.1 Ticket Management

**Ticket Types**:
- bug (software defect)
- feature_request (enhancement)
- question (how-to, help)
- incident (service disruption)
- task (action item)
- other

**Ticket Statuses**:
- open (new ticket)
- in_progress (being worked on)
- waiting_on_customer (awaiting customer response)
- waiting_on_internal (awaiting internal team)
- resolved (fixed, answered)
- closed (resolved and confirmed)
- reopened (closed ticket reopened)

**Ticket Priorities**:
- low
- medium
- high
- urgent
- critical

**Create Ticket**:

**Inputs**:
- Subject
- Description
- Type
- Priority (default: medium)
- Customer ID (contact who reported)
- Assigned to user ID (optional)
- Tags (optional)
- Attachments (optional)
- Source: email | portal | phone | chat | internal
- User ID (ticket creator)

**Business Logic**:
1. Generate ticket number (auto-increment: TICK-0001)
2. Set status: "open"
3. If assigned: notify assignee
4. If from email: link to original email
5. Apply SLA (based on priority):
   - Critical: 1 hour first response, 4 hours resolution
   - High: 4 hours first response, 24 hours resolution
   - Medium: 8 hours first response, 3 days resolution
   - Low: 24 hours first response, 7 days resolution
6. Create first response reminder
7. Send confirmation to customer

**Output**: Ticket ID

### 18.2 Ticket Assignment

**Auto-Assignment Rules**:

**Round-robin**:
- Distribute tickets evenly among available agents
- Track: Last assigned agent, assign to next in rotation

**Load-balancing**:
- Assign to agent with fewest open tickets
- Consider: Agent capacity, current workload

**Skills-based**:
- Match ticket tags/type to agent skills
- E.g., "billing" tickets → agents with billing skill

**Priority-based**:
- High-priority tickets → senior agents
- Low-priority → junior agents

**Manual Assignment**:
- Manager assigns specific ticket to specific agent
- Overrides auto-assignment rules

### 18.3 Ticket Updates

**Add Update/Comment**:
- Input: Ticket ID, content, user ID, is_internal (boolean)
- If is_internal: Only visible to support team
- If public: Visible to customer, sends notification
- Updates: Last activity timestamp
- Resets: First response timer (if first response)

**Change Status**:
- Input: Ticket ID, new status, user ID
- Validates: Status transition is allowed
- If status = "resolved": prompt for resolution notes
- If status = "closed": record closed_at timestamp
- Notifies: Customer and assigned agent

**Escalate Ticket**:
- Input: Ticket ID, escalate to user/team, reason
- Changes: Priority (often increases), assigned agent (to senior/manager)
- Notifies: Escalated party
- Tracks: Escalation reason and timestamp

### 18.4 SLA Management

**SLA Metrics**:
- **First response time**: Time from created to first agent response
- **Resolution time**: Time from created to resolved
- **Customer satisfaction**: Rating from customer (1-5 stars)

**SLA Breaches**:
- Track when ticket exceeds SLA target
- Escalate automatically if breached
- Report: SLA breach rate by priority, by agent, by type

**SLA Pauses**:
- Pause timer when status = "waiting_on_customer"
- Resume when customer responds
- Ensures fair SLA measurement

### 18.5 Ticket Analytics

**Metrics**:
- **Total tickets**: By type, by priority, by status, by source
- **Open tickets**: Current open count
- **Resolution rate**: Resolved / total * 100
- **Average resolution time**: Mean time to resolve
- **First response time**: Mean time to first response
- **Reopened tickets**: Count and % of reopened tickets
- **Customer satisfaction**: Average rating
- **Tickets by agent**: Volume, resolution time, CSAT per agent
- **Peak times**: When most tickets are created (by hour, day)

**Trends**:
- Ticket volume over time
- Resolution time trends (improving or degrading?)
- Common issues (most frequent types/tags)

---

## 19. WIZARD & GUIDED FLOWS

### 19.1 Wizard Definition

**Purpose**: Guide users through multi-step processes with validation at each step.

**Create Wizard**:

**Inputs**:
- Name
- Description
- Steps array: [
    {
      id: string,
      title: string,
      description: string,
      fields: [{name, type, required, validation, defaultValue?}],
      conditional_on: {field, value} (optional - show step only if condition met)
    }
  ]
- Completion action (what happens when wizard finishes)
- User ID (wizard creator)

**Business Logic**:
1. Define wizard structure
2. Store as template
3. Can be reused for multiple executions

**Example Wizards**:
- "Onboard New Customer" (steps: company info, contact info, product selection, contract)
- "Create Project" (steps: project details, team members, budget, timeline)
- "Report a Bug" (steps: description, environment, steps to reproduce, severity)

### 19.2 Wizard Execution

**Start Wizard**:
- Input: Wizard ID, user ID
- Creates: Wizard session (tracks progress)
- Returns: First step

**Complete Step**:

**Inputs**:
- Session ID
- Step ID
- Field values: {field_name: value}
- User ID

**Business Logic**:
1. Validate all required fields present
2. Validate field values (type, format, range, etc.)
3. If validation fails: return errors, stay on step
4. If validation passes:
   - Save step data
   - Mark step as completed
   - Calculate next step:
     - If current step has conditional next steps: evaluate conditions
     - Else: go to next step in array
   - If no more steps: wizard complete
5. Return next step or completion

**Go Back**:
- Allow user to go back to previous steps
- Preserve data entered
- Can edit previous steps

**Save Progress**:
- Auto-save after each step
- User can exit wizard and resume later
- Session expires after 30 days of inactivity

### 19.3 Wizard Completion

**On Completion**:
- Execute completion action (e.g., create entity with all collected data)
- Send confirmation email
- Redirect user to result (e.g., newly created project page)
- Mark session as completed

**Completion Actions**:
- Create entity (contact, deal, project, etc.)
- Send email notification
- Trigger workflow
- Create tasks
- Update existing entity
- Redirect to URL

### 19.4 Wizard Analytics

**Track Metrics**:
- **Started vs. completed**: Conversion rate
- **Abandonment by step**: Which steps do users quit on?
- **Time per step**: Which steps take longest?
- **Error rate by field**: Which fields cause most validation errors?
- **Completion time**: Average time from start to finish

**Use for**:
- Identify friction points in wizard
- Simplify or remove problematic steps
- Improve field validation messaging

---

## 20. WORKFLOWS & AUTOMATION

### 20.1 Workflow Engine

**Purpose**: Automate business processes with triggers, conditions, and actions.

**Workflow Components**:
- **Trigger**: What starts the workflow
- **Conditions**: When to execute (optional)
- **Actions**: What to do

**Trigger Types**:
- **Entity created**: When entity of type X is created
- **Entity updated**: When entity field changes
- **Entity deleted**: When entity is deleted
- **Schedule**: Run on schedule (cron pattern)
- **Webhook**: External HTTP request
- **Manual**: User manually triggers

**Create Workflow**:

**Inputs**:
- Name
- Description
- Trigger: {type, entity_type?, field?, schedule?}
- Conditions array (optional): [{field, operator, value}]
- Actions array: [
    {
      type: string,
      config: object
    }
  ]
- Is active: boolean
- User ID

**Example**:
```
Name: "Notify sales when high-value deal created"
Trigger: Entity created, type = "deal"
Conditions:
  - value >= 50000
Actions:
  - Send email to sales@company.com
  - Create task "Follow up with {{deal.company}}"
  - Post to Slack channel "high-value-deals"
```

### 20.2 Workflow Actions

**Action Types**:

**Send Email**:
- Config: {to, subject, body, template_id?}

**Send Notification**:
- Config: {user_id, title, message, channel}

**Create Entity**:
- Config: {entity_type, data}

**Update Entity**:
- Config: {entity_type, entity_id, changes}

**Delete Entity**:
- Config: {entity_type, entity_id}

**Create Task**:
- Config: {title, description, assigned_to, due_date}

**HTTP Request**:
- Config: {url, method, headers, body}
- Use for: Webhook calls, API integrations

**Delay/Wait**:
- Config: {duration: "1 hour" | "3 days" | etc.}
- Pauses workflow execution

**If/Else Branch**:
- Config: {condition, then_actions, else_actions}
- Conditional logic within workflow

**Loop**:
- Config: {over: array, actions}
- Repeat actions for each item

### 20.3 Workflow Execution

**Trigger Workflow**:
1. Event occurs (entity created, etc.)
2. Find all active workflows with matching trigger
3. For each workflow:
   - Evaluate conditions
   - If conditions pass: queue for execution
4. Execute actions in sequence
5. If action fails: log error, optionally continue or stop
6. Record execution log

**Execution Log**:
```
{
  workflowId: UUID,
  executionId: UUID,
  triggeredAt: ISODateTime,
  triggeredBy: {type: "entity_created", entity_id: UUID},
  status: "running" | "completed" | "failed",
  actions: [
    {actionId, status, startedAt, completedAt, result, error?}
  ]
}
```

### 20.4 Workflow Templates

**Pre-built Workflows**:
- "Welcome new customer" (send email, create task, notify sales)
- "Escalate overdue invoice" (if invoice overdue by 7 days, email customer and create task)
- "Assign new ticket" (use round-robin or load-balancing)
- "Sync to external CRM" (when contact created, POST to external API)

**Use Template**:
- Select template
- Customize: Change recipients, timing, actions
- Activate

### 20.5 Workflow Analytics

**Metrics**:
- **Executions**: Total workflow runs
- **Success rate**: Completed / total * 100
- **Failure rate**: Failed / total * 100
- **Average execution time**: Mean duration
- **By workflow**: Metrics per workflow
- **By action**: Which actions fail most often

**Optimize**:
- Disable workflows with low success rate
- Fix failing actions
- Remove unused workflows

---

## 21. CROSS-DOMAIN PATTERNS

### 21.1 Universal Operations

**All entities support** (unchanged from first extraction):
- Create, Read, Update, Delete (CRUD)
- Query with filters, sorting, pagination
- Field projection
- Bulk operations
- Import/Export
- Tagging
- Activity tracking
- Notifications
- Search
- Webhooks

*See OMAR_BUSINESS_LOGIC_REFERENCE.md sections 10.1-10.16 for complete details.*

### 21.2 Relationship Patterns

**Entity Relationships**:
- One-to-one (user → profile)
- One-to-many (customer → invoices)
- Many-to-many (tasks ↔ tags)
- Parent-child (task → subtasks)
- Dependencies (task → depends_on → tasks)

**Linking Entities**:
- Foreign keys (entity_id references another entity)
- Junction tables (for many-to-many)
- Polymorphic links (attachment can link to any entity type)

### 21.3 State Machine Pattern

**Many entities follow state machines**:

**Examples**:
- Ticket: open → in_progress → resolved → closed
- Deal: lead → qualified → proposal → negotiation → closed_won/lost
- Task: pending → in_progress → completed/cancelled
- Invoice: draft → sent → paid/overdue/cancelled

**Rules**:
- Define valid transitions (open → in_progress is valid, open → closed is not)
- Validate transitions before allowing
- Record state changes in lifecycle history
- Trigger workflows on state changes

### 21.4 Permission Patterns

**Permission Levels** (applies to all entities):
- **read**: View entity
- **write**: Create and edit entity
- **delete**: Delete entity
- **admin**: All permissions + manage permissions
- **share**: Can share entity with others

**Permission Checks**:
```
For every operation:
  if user is owner: allow
  else if entity is shared with user:
    check granted permission level
    if has required permission: allow
  else if entity is public:
    allow read operations only
  else: deny
```

### 21.5 Audit Trail

**Every mutation generates audit event**:
```
{
  eventId: UUID,
  eventType: "created" | "updated" | "deleted",
  entityType: string,
  entityId: UUID,
  userId: ActorId,
  timestamp: ISODateTime,
  changes: {
    before: object,
    after: object,
    diff: {field: {from, to}}
  },
  metadata: {
    ip: string,
    userAgent: string,
    source: "web" | "api" | "mobile"
  }
}
```

**Audit Queries**:
- Get history for entity
- Get activity for user
- Get activity for time period
- Get all changes to specific field

---

## APPENDIX: DOMAIN STATISTICS

### Files and Line Counts

**Original 10 Files** (11,573 lines):
1. AnalyticsEffectsProgram.ts - 597 lines
2. ApprovalsEffectsProgram.ts - 546 lines
3. BusinessEffectsProgram.ts - 613 lines
4. CalendarEffectsProgram.ts - 1,703 lines
5. CommsEffectsProgram.ts - 1,628 lines
6. ContactsEffectsProgram.ts - 1,744 lines
7. DealsEffectsProgram.ts - 683 lines
8. DocumentsEffectsProgram.ts - 999 lines
9. EmailEffectsProgram.ts - 1,725 lines
10. EffectsProgramHelpers.ts - 1,335 lines

**New 11 Files** (10,834 lines):
11. EvolutionEffectsProgram.ts - 397 lines
12. FilesEffectsProgram.ts - 1,085 lines
13. InvoicesEffectsProgram.ts - 848 lines
14. NotesEffectsProgram.ts - 652 lines
15. NotificationsEffectsProgram.ts - 1,311 lines
16. PortalEffectsProgram.ts - 1,004 lines
17. SearchEffectsProgram.ts - 698 lines
18. TasksEffectsProgram.ts - 1,623 lines
19. TicketsEffectsProgram.ts - 789 lines
20. WizardEffectsProgram.ts - 1,104 lines
21. WorkflowsEffectsProgram.ts - 1,323 lines

**Total**: 21 files, 22,407 lines

### Constants & Limits

- Max file upload: 100MB
- Max email recipients: 100
- Max bulk operation size: 1000 items
- Max search results: 1000 (default: 100)
- Default page size: 20
- Session timeout: 30 days inactivity (wizards)
- SLA timers:
  - Critical: 1h response, 4h resolution
  - High: 4h response, 24h resolution
  - Medium: 8h response, 3 days resolution
  - Low: 24h response, 7 days resolution

---

**END OF COMPLETE BUSINESS LOGIC REFERENCE**

This document now contains EVERY business operation from all 21 TypeScript files (22,407 lines). This is the complete, canonical source of truth for OMAR implementation.
