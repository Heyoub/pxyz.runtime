# OMAR BUSINESS LOGIC REFERENCE

> **Purpose**: Technology-agnostic specification of ALL business logic OMAR must implement.  
> **Source**: Extracted from TypeScript Effects Programs (11,573 lines) into pure functional requirements.  
> **For**: Future agents, developers, and AI systems implementing OMAR in ANY architecture.

**CRITICAL**: This document describes WHAT the system does, not HOW. No TypeScript, no Effect-TS, no implementation details. Pure business requirements that could be implemented in PXYZ/OMAR, traditional backend, or any other architecture.

---

## Table of Contents

1. [Analytics](#1-analytics)
2. [Approvals & E-Signatures](#2-approvals--e-signatures)
3. [Business Tools & Agent Orchestration](#3-business-tools--agent-orchestration)
4. [Calendar & Scheduling](#4-calendar--scheduling)
5. [Communications](#5-communications)
6. [Contacts](#6-contacts)
7. [Deals & Sales](#7-deals--sales)
8. [Documents](#8-documents)
9. [Email](#9-email)
10. [Cross-Domain Patterns](#10-cross-domain-patterns)

---

## 1. ANALYTICS

### 1.1 Time Series Data Aggregation

**Purpose**: Aggregate events across multiple time granularities simultaneously.

#### Time Bucket Granularities
- **Hourly**: 1-hour buckets
- **Daily**: 1-day buckets  
- **Weekly**: 7-day buckets
- **Monthly**: Calendar month buckets

#### Aggregation Functions
- **Sum**: Total all event values in bucket
- **Average**: Mean of all event values
- **Min**: Minimum event value
- **Max**: Maximum event value
- **Count**: Number of events in bucket

#### Time Bucketing Rules

**Multi-Granularity Calculation**:
1. Create buckets for each requested granularity (hour, day, week, month)
2. Each event belongs to exactly ONE bucket per granularity
3. Buckets can overlap across granularities (same event in hourly AND daily bucket)
4. Priority order: hour=4, day=3, week=2, month=1

**Gap Filling**:
- If `fillGaps: true`, insert zero-value buckets for missing time periods
- If `fillGaps: false`, only return buckets with actual events

**Timezone Handling**:
- All bucket boundaries respect specified timezone
- Default timezone: UTC
- Support any IANA timezone (e.g., "America/New_York", "Europe/London")

**Bucket Creation Logic**:
```
For each granularity:
  current = startDate
  while current < endDate:
    if granularity == "hour":
      bucketEnd = current + 1 hour
    if granularity == "day":
      bucketEnd = current + 1 day
    if granularity == "week":
      bucketEnd = current + 7 days
    if granularity == "month":
      bucketEnd = first day of next month
    
    if bucketEnd > endDate:
      bucketEnd = endDate
      create bucket [current, bucketEnd]
      break
    
    create bucket [current, bucketEnd]
    current = bucketEnd
```

### 1.2 Event Querying

**Query Events by Filters**:
- **Entity Types**: Filter by entity type(s) (contact, deal, task, etc.)
- **Time Range**: Start and end timestamps
- **Event Type**: Specific event types only
- **User/Actor**: Events for specific user(s)

**Query Coordinate Pattern**:
```
P = entity type (or "*" for all)
X = operation (or "*" for all)
Y = constraints (or "*" for all)
Z = time range
```

### 1.3 Metrics & Summary Generation

**Summary Statistics** (calculated from event array):
- **Total**: Sum of all numeric values
- **Average**: Mean of values
- **Min**: Minimum value
- **Max**: Maximum value
- **Trend**: "up", "down", or "stable"

**Trend Calculation**:
1. Split events into first half and second half by time
2. Calculate average value for each half
3. If second half > first half * 1.05: trend = "up"
4. If second half < first half * 0.95: trend = "down"
5. Otherwise: trend = "stable"

**Value Extraction Priority** (from event metadata):
1. `event.metadata.value` (if numeric)
2. `event.metadata.count` (if numeric)
3. `event.metadata.amount` (if numeric)
4. Default to 1 if no numeric value found

### 1.4 Trend Analysis

**Calculate Trends Across Time Periods**:
- Input: Array of time series data points (each with timestamp and value)
- Output: Same array with `trend` field added (percentage change from previous point)

**Formula**:
```
For each point (index > 0):
  prevValue = data[index - 1].value
  currentValue = data[index].value
  percentChange = ((currentValue - prevValue) / prevValue) * 100
  point.trend = percentChange
```

**First Point**: trend = 0 (no previous point to compare)

### 1.5 Dimensional Grouping

**Group Events by Dimension**:
- Input: Array of events, dimension key (string)
- Output: Map of dimension value → events with that value

**Grouping Logic**:
```
grouped = {}
For each event:
  key = event.metadata[dimension] OR "unknown"
  if grouped[key] doesn't exist:
    grouped[key] = []
  grouped[key].append(event)
Return grouped
```

### 1.6 Analytics Reports

**Report Generation**:
- **Report ID**: Unique identifier
- **User ID**: Owner of report
- **Name**: Report title
- **Chart Type**: Visualization type (line, bar, pie, etc.)
- **Time Range**: Start and end dates
- **Data Points**: Array of {timestamp, value, label?}
- **Summary**: Calculated statistics (total, average, min, max, trend)
- **Created At**: Timestamp
- **Updated At**: Timestamp

**Chart Types Supported**:
- line
- bar
- pie
- area
- scatter

### 1.7 AI-Powered Time Series Analysis

**LLM Analysis Workflow**:
- Input: Metric name, time range
- Process:
  1. Use AI agent to analyze time series data
  2. Calculate trends, patterns, and anomalies
  3. Generate insights in natural language
- Output: Analysis text + data points

**Agent Configuration**:
- Model: Claude 3.5 Sonnet (or equivalent)
- Max Tokens: 1000
- System Prompt: "Analyze time series data for metric: {metric}. Calculate trends, patterns, and anomalies."

---

## 2. APPROVALS & E-SIGNATURES

### 2.1 Approval Request Creation

**Request Approval Workflow**:

**Inputs**:
- Entity type and ID (what's being approved)
- Approvers array (who needs to approve)
- Approval type: document | invoice | contract | expense | change_request | general
- Optional deadline (ISO datetime)
- Optional message
- User ID (requester)
- Optional `requiresAll` flag (default: true)

**Business Logic**:
1. Validate entity exists
2. Create approval record with status: "pending"
3. Create approval chain: array of {approverId, order, status: "pending", respondedAt: null}
4. Store entity snapshot at request time (title, status, value)
5. Record requested timestamp
6. Send notifications to all approvers

**Output**:
- Approval ID
- Status: "pending"
- Approvers list
- Deadline
- Success message

### 2.2 Approval Response

**Respond to Approval**:

**Inputs**:
- Approval ID
- Response: "approved" | "rejected" | "changes_requested"
- Optional comments
- Optional changes: array of {field, suggestion}
- User ID (responder)

**Business Logic**:

**Validation**:
1. Approval exists
2. User is in approvers list
3. User hasn't already responded

**Update Approval Chain**:
1. Find user's entry in approval chain
2. Set status to response value
3. Set respondedAt to current timestamp
4. Add comments and changes if provided

**Determine Overall Status**:
```
if response == "rejected":
  overall = "rejected"
else if response == "changes_requested":
  overall = "changes_requested"
else if response == "approved":
  if requiresAll == true:
    if all approvers have status "approved":
      overall = "approved"
    else:
      overall = "in_review"
  else:
    overall = "approved" (any one approval suffices)
```

**Record**:
- Last response timestamp
- Updated approval chain

### 2.3 List Pending Approvals

**Get Pending Approvals for User**:

**Inputs**:
- User ID
- Optional sort by: "deadline" | "requested_date"

**Business Logic**:
1. Query all approvals where user is in approvers list
2. Filter to status: "pending" | "in_review"
3. Filter to user's entry in approval chain where status == "pending"
4. Sort by requested criteria (default: oldest first)

**Calculate Metrics**:
- Total pending count
- Overdue count (where deadline < now)

**Output for Each Approval**:
- Approval ID
- Type
- Entity type and ID
- Entity title (from snapshot)
- Requested by user
- Requested at timestamp
- Deadline
- Is overdue? (boolean)
- Message

### 2.4 Approval Reminders

**Send Reminders**:

**Inputs**:
- Approval ID
- User ID (who's sending reminder)

**Business Logic**:
1. Validate approval exists
2. Find approval chain entries with status "pending"
3. Extract approverId for each pending entry
4. Increment reminder count in metadata
5. Update lastReminderAt timestamp
6. Send notification to each pending approver

**Output**:
- Approval ID
- Number of reminders sent
- List of recipient approver IDs
- Total reminder count for this approval
- Success message

### 2.5 E-Signature Workflow

**Initiate E-Signature**:

**Inputs**:
- Approval ID (must be approved first)
- Document ID
- Signers array: [{userId, role, order}]
- User ID (initiator)

**Validation**:
- Approval exists
- Approval status == "approved"

**Business Logic**:
1. Create e-signature record with status: "pending"
2. Create signature chain: [{userId, role, order, status: "pending", signedAt: null}]
3. Record initiated timestamp
4. Link e-signature ID back to approval metadata

**Output**:
- E-signature ID
- Approval ID
- Document ID
- Number of signers
- Status: "pending"
- Success message

### 2.6 Approval Analytics

**Get Analytics**:

**Inputs**:
- User ID
- Optional date range {start, end}

**Metrics Calculated**:
- **Total approvals** (in date range if provided)
- **Approved count** (status == "approved")
- **Rejected count** (status == "rejected")
- **Pending count** (status == "pending" | "in_review")
- **Approval rate**: (approved / total) * 100
- **Average turnaround time**:
  - For each completed approval (approved | rejected)
  - Calculate: (response timestamp - requested timestamp) in hours
  - Average all turnaround times
- **Turnaround in days**: hours / 24
- **By type**: Map of approval type → count
- **Overdue approvals**: Count where deadline < now AND status is pending/in_review

**Output**:
```
{
  totalApprovals: number,
  approved: number,
  rejected: number,
  pending: number,
  approvalRate: percentage,
  avgTurnaroundHours: number,
  avgTurnaroundDays: number,
  byType: { [type]: count },
  overdueApprovals: number
}
```

---

## 3. BUSINESS TOOLS & AGENT ORCHESTRATION

### 3.1 Quick Tools

**Tool Categories**:
- navigation
- creation
- analysis
- communication
- automation

**Register Quick Tool**:

**Inputs**:
- Name
- Description (optional)
- Category
- Icon (optional)
- Keyboard shortcut (optional)
- URL (optional)
- User ID
- Is active (default: true)

**Business Logic**:
1. Create tool record
2. Initialize usage count = 0
3. Store creation timestamp

**Output**: Tool record with ID

**Get User's Tools**:
- Input: User ID
- Output: All tools for user
- Filter: Can filter by category, active status

### 3.2 Tool Chains

**Create Tool Chain**:

**Inputs**:
- Name
- Description (optional)
- Steps array: [{toolId, order, config?}]
- User ID

**Business Logic**:
1. Validate all referenced tool IDs exist
2. Sort steps by order
3. Create chain record

**Output**: Chain ID

**Execute Tool Chain**:
- Input: Chain ID
- Process: Execute each tool in order, passing output of step N to input of step N+1
- Output: Final result from last tool

### 3.3 Tool Usage Analytics

**Update Tool Usage**:

**Inputs**:
- Tool ID
- User ID
- Execution time (milliseconds, optional)
- Success (boolean, optional)

**Business Logic**:
1. Increment tool's usage count
2. Record usage event with timestamp
3. Store execution time and success status

**Create Analytics Record**:
- Tool ID
- User ID
- Execution metrics
- Timestamp

### 3.4 Tool Recommendations

**Get Recommendations**:

**Input**: User ID

**Business Logic**:
1. Get user's tool usage history
2. Calculate most-used tools (sort by usage count, descending)
3. Group tools by category
4. Identify unused tools
5. Return recommendations based on:
   - Popular tools user hasn't tried
   - Tools in categories user actively uses
   - Recently added tools

**Output**:
```
{
  mostUsed: [tools],
  recommended: [tools],
  unused: [tools],
  byCategory: { [category]: [tools] },
  totalTools: number
}
```

### 3.5 Registered Agents

**Register Agent**:

**Inputs**:
- Name
- Capabilities array (strings describing what agent can do)
- User ID
- Metadata (optional)

**Business Logic**:
1. Create agent record
2. Store capabilities
3. Initialize performance metrics

**Output**: Agent ID

**Agent Operations**:
- List agents for user
- Update agent capabilities
- Track agent usage
- Delete agent

---

## 4. CALENDAR & SCHEDULING

### 4.1 Calendar Event Management

**Event Types**:
- meeting
- appointment
- reminder
- deadline
- personal
- other

**Create Calendar Event**:

**Inputs**:
- Title
- Description (optional)
- Start time (ISO datetime)
- End time (ISO datetime)
- Timezone
- Location (optional)
- Event type
- User ID
- Attendees array (optional): [{email, name?, status?}]
- Is all-day (boolean, default: false)
- Is recurring (boolean, default: false)
- Recurrence pattern (if recurring): {frequency, interval, endDate?, occurrences?}
- Tags (optional)

**Business Logic**:
1. Validate times (end > start)
2. Check for conflicts (if desired)
3. Create event record
4. If attendees: create attendee records with status "pending"
5. If recurring: generate occurrences based on pattern

**Output**: Event ID

### 4.2 Recurrence Patterns

**Recurrence Frequencies**:
- daily
- weekly
- monthly
- yearly

**Pattern Definition**:
- **Frequency**: One of the above
- **Interval**: How many units between occurrences (e.g., every 2 weeks)
- **End Date**: When to stop recurring (optional)
- **Occurrences**: Max number of occurrences (optional)

**Recurrence Expansion**:
```
current = event.startTime
occurrences = []
count = 0

while (endDate not reached OR occurrences limit not reached):
  occurrences.append({
    startTime: current,
    endTime: current + event.duration
  })
  count++
  
  if frequency == "daily":
    current += interval days
  if frequency == "weekly":
    current += interval weeks
  if frequency == "monthly":
    current += interval months (same day)
  if frequency == "yearly":
    current += interval years (same date)
  
  if count >= pattern.occurrences:
    break
  if current > pattern.endDate:
    break

Return occurrences
```

### 4.3 Scheduling Conflict Detection

**Event Region** (immutable time coordinate):
- Event ID
- Time span: [startTime, endTime]
- Priority: 0-100 (meeting importance)
- Event type
- User ID
- Is recurring

**Conflict Detection**:
```
For each pair of events (A, B):
  if A.startTime < B.endTime AND A.endTime > B.startTime:
    conflict detected
    overlap_start = max(A.startTime, B.startTime)
    overlap_end = min(A.endTime, B.endTime)
    overlap_minutes = (overlap_end - overlap_start) in minutes
    
    record conflict: {
      region1: A,
      region2: B,
      overlapSpan: [overlap_start, overlap_end],
      overlapMinutes: overlap_minutes
    }
```

**Scheduling Constraints** (rules for resolving conflicts):
- **Priority Order**: Array of event types in order (e.g., [deadline, meeting, appointment, ...])
- **Respect Priority**: Use numeric priority field when types match
- **Allow Partial Overlap**: Can events overlap by N minutes?
- **Partial Overlap Threshold**: Max acceptable overlap in minutes
- **Prefer Longer**: When priorities equal, keep longer event
- **Respect Working Hours**: Only schedule during work hours
- **Working Hours Start/End**: 24-hour format (e.g., 9, 17)

**Default Constraints**:
```
{
  priorityOrder: ["deadline", "meeting", "appointment", "reminder", "personal", "other"],
  respectPriority: true,
  allowPartialOverlap: false,
  partialOverlapThreshold: 0,
  preferLonger: true,
  respectWorkingHours: true,
  workingHoursStart: 9,
  workingHoursEnd: 17
}
```

**Conflict Resolution**:
1. Find all overlapping event pairs
2. For each conflict:
   - If types differ: Use priorityOrder
   - If types same: Use numeric priority if respectPriority=true
   - If priorities equal: Use preferLonger
3. Winning event keeps its slot
4. Losing event must be rescheduled or deleted

### 4.4 Calendar Sharing

**Share Calendar**:

**Inputs**:
- Calendar ID
- Shared with user ID
- Permission: "read" | "write" | "admin"
- User ID (sharer)

**Business Logic**:
1. Validate calendar exists
2. Create share record
3. Grant specified permissions

**Output**: Share ID

**Permissions**:
- **read**: View events only
- **write**: View and create/edit events
- **admin**: Full control including sharing

### 4.5 Find Optimal Meeting Times

**Find Available Slots**:

**Inputs**:
- Duration (minutes)
- Attendees array (user IDs)
- Preferred time range
- Constraints (working hours, etc.)

**Business Logic**:
1. Get all attendees' calendars
2. For each potential time slot in range:
   - Check if slot conflicts with any attendee's events
   - Score slot based on:
     - All attendees free = highest score
     - Within working hours = bonus
     - Not adjacent to other meetings = bonus
     - Earlier in day = slight bonus
3. Sort slots by score descending
4. Return top N slots

**Output**: Array of {start, end, score, reason}

### 4.6 Schedule Focus Blocks

**Auto-Schedule Focus Time**:

**Inputs**:
- Title (default: "Focus Time")
- Duration (minutes)
- Preferred time of day (optional)
- User ID

**Business Logic**:
1. Find optimal meeting times using above algorithm
2. Create event with:
   - Type: "personal"
   - Metadata: {isFocusBlock: true, protected: true}
3. Best slot = earliest high-score slot
4. Create event at best slot

**Output**: Event ID

### 4.7 Calendar Analytics

**Health Metrics**:

**Calculate Health** (typically for past week):

**Metrics**:
- **Total meetings**: Count of meeting events
- **Total meeting hours**: Sum of meeting durations
- **Meeting load percentage**: (meeting hours / working hours) * 100
- **Focus block hours**: Sum of focus block durations
- **Focus time ratio**: (focus hours / total meeting hours) * 100
- **Back-to-back meetings**: Count of meetings with <15min gap before next
- **Health status**:
  - "healthy": meeting load < 40% AND focus ratio > 20%
  - "overloaded": meeting load > 60%
  - "moderate": otherwise

**Recommendations**:
- If meeting load > 50%: "Meeting load is high - consider declining or delegating"
- If focus ratio < 20%: "Low focus time - schedule more focus blocks"
- If back-to-back > 5: "Too many back-to-back meetings - add buffer time"
- If zero meetings: "No meetings scheduled - consider check-ins"

### 4.8 Event Queries

**Get Events**:
- **For Date**: All events on specific date
- **For Interval**: Events between start and end
- **Upcoming**: Next N events from now
- **By Type**: Filter events by type
- **Search**: Text search in title/description

**Group Events**:
- **By Date**: Map of date → events
- **By Type**: Map of type → events
- **By Calendar**: Map of calendar → events

---

## 5. COMMUNICATIONS

### 5.1 Message Thread Windows

**Thread Window** (activity tracking coordinate):
- Thread ID
- Activity span: [firstMessage timestamp, lastMessage timestamp]
- Unread window: [firstUnread timestamp, lastUnread timestamp] (or [null, null])
- Participant count
- Message count
- Priority (highest message priority in thread)

**Message Priorities**:
- low = 1
- medium = 2
- high = 3
- urgent = 4

### 5.2 Thread Activity Classification

**Activity Statuses**:
- **active**: Recent activity (< minActivityGap hours)
- **dormant**: No recent activity (> minActivityGap hours but < archiveInactiveDays)
- **archived**: No activity for archiveInactiveDays+

**Classification Logic**:
```
hoursSinceActivity = (now - thread.lastActivity) in hours
daysSinceActivity = hoursSinceActivity / 24

if daysSinceActivity >= archiveInactiveDays:
  return "archived"
else if hoursSinceActivity >= minActivityGap:
  return "dormant"
else:
  return "active"
```

**Default Constraints**:
```
{
  minActivityGap: 24 hours,
  maxThreadAge: 90 days,
  unreadTimeLimit: 48 hours (mark urgent if unread this long),
  archiveInactiveDays: 30 days
}
```

### 5.3 Urgent Unread Detection

**Find Urgent Unread Threads**:

**Logic**:
1. Filter threads where unread window exists (first unread ≠ null)
2. Calculate hours since first unread = (now - firstUnread) in hours
3. If hours >= unreadTimeLimit: mark urgent
4. Sort by priority descending

**Output**: Array of urgent threads sorted by priority

### 5.4 Message Management

**Message Types**:
- text
- image
- file
- voice
- video
- system

**Message Statuses**:
- draft
- sent
- delivered
- read
- failed

**Send Message**:

**Inputs**:
- Content
- Sender ID
- Recipient ID (optional, for direct messages)
- Channel ID (optional, for channel messages)
- Message type
- Priority (optional)
- Metadata (optional)

**Business Logic**:
1. Validate sender exists
2. Validate recipient or channel exists
3. Create message with status "sent"
4. If channel: notify all channel participants
5. If direct: notify recipient
6. Update thread activity window

**Output**: Message ID

### 5.5 Channel Management

**Channel Types**:
- direct: 1-to-1 conversation
- group: Multiple participants, private
- broadcast: One-to-many, sender posts only
- support: Customer support channel
- announcement: Organization-wide announcements

**Create Channel**:

**Inputs**:
- Name
- Description (optional)
- Channel type
- Created by user ID
- Participants array
- Settings (optional)
- Metadata (optional)

**Business Logic**:
1. Create channel record
2. Add creator to participants if not present
3. Initialize activity metrics

**Output**: Channel ID

### 5.6 Thread Activity Statistics

**Aggregate Statistics**:

**Metrics**:
- **Total threads**: Count
- **Total messages**: Sum across all threads
- **Unread threads**: Threads with unread window
- **Average messages per thread**: Total messages / total threads
- **Most active thread**: Thread with highest message count

### 5.7 Message Templating

**Create Message Template**:

**Inputs**:
- Name
- Category
- Template text (with variables: {{variableName}})
- Variables array: [{name, description, defaultValue?}]
- User ID

**Template Rendering**:
```
For each variable in template:
  replace {{variableName}} with provided value OR defaultValue
Return rendered text
```

### 5.8 Communication Analytics

**Track Metrics**:
- Messages sent by user (over time)
- Messages received by user
- Response time (time between received and replied)
- Most active channels
- Most active threads
- Peak communication hours
- Communication breakdown by type (text, image, file, etc.)

---

## 6. CONTACTS

### 6.1 Contact Management

**Contact Types**:
- lead
- prospect
- customer
- partner
- vendor

**Contact Status**:
- active
- inactive
- archived

**Contact Sources**:
- website
- referral
- cold_outreach
- event
- social_media
- other

**Lifecycle Stages**:
- awareness
- interest
- consideration
- purchase
- retention

**Create Contact**:

**Inputs**:
- First name
- Last name
- Email (optional)
- Phone (optional)
- Company (optional)
- Title (optional)
- Contact type
- Source
- User ID (owner)
- Lifecycle stage (optional)
- Tags (optional)
- Notes (optional)
- Metadata (optional)

**Validation**:
- At least one of email or phone must be provided
- Check for duplicates (same email or phone)

**Business Logic**:
1. Validate inputs
2. Check for duplicate contacts (by email or phone)
3. If duplicate found: return error with existing contact ID
4. Create contact record
5. Set initial lifecycle stage (default: "awareness")

**Output**: Contact ID

### 6.2 Contact Deduplication

**Find Duplicates**:

**Logic**:
1. Query all contacts for user
2. Group by email (where email exists)
3. Group by phone (where phone exists)
4. Any group with 2+ contacts = duplicate set
5. Return duplicate sets

**Output**:
```
{
  byEmail: { [email]: [contact IDs] },
  byPhone: { [phone]: [contact IDs] },
  totalDuplicates: number
}
```

**Merge Contacts**:

**Inputs**:
- Primary contact ID (keep this one)
- Duplicate contact IDs (merge these into primary)
- User ID

**Business Logic**:
1. Validate all contacts exist
2. Move all interactions from duplicates to primary
3. Move all relationships from duplicates to primary
4. Merge tags (union of all tags)
5. Merge custom fields (primary takes precedence, backfill from duplicates)
6. Delete duplicate contacts
7. Emit merge event

**Output**: Merged contact

### 6.3 Interaction Tracking

**Interaction Types**:
- email
- call
- meeting
- note
- task

**Interaction Outcomes**:
- positive
- neutral
- negative
- follow_up_needed

**Log Interaction**:

**Inputs**:
- Contact ID
- User ID
- Interaction type
- Subject
- Content (optional)
- Outcome (optional)
- Scheduled at (optional, for future interactions)
- Completed at (optional)
- Metadata (optional)

**Business Logic**:
1. Validate contact exists
2. Create interaction record
3. Update contact's "last interaction" timestamp
4. If outcome = "follow_up_needed": create task

**Output**: Interaction ID

### 6.4 Contact Relationships

**Relationship Types**:
- colleague
- manager
- subordinate
- partner
- vendor
- customer
- other

**Create Relationship**:

**Inputs**:
- Contact ID
- Related contact ID
- Relationship type
- Strength (0-100, optional)
- Notes (optional)
- User ID

**Business Logic**:
1. Validate both contacts exist
2. Create relationship record
3. Create inverse relationship (if applicable)
   - manager ↔ subordinate
   - colleague ↔ colleague

**Output**: Relationship ID

**Relationship Graph**:
- Get all contacts related to a contact (1 degree)
- Get contacts related to related contacts (2 degrees)
- Find shortest path between two contacts

### 6.5 Contact Lifecycle

**Update Lifecycle Stage**:

**Inputs**:
- Contact ID
- New lifecycle stage
- User ID

**Business Logic**:
1. Validate contact exists
2. Record previous stage
3. Update to new stage
4. Emit lifecycle change event
5. If stage = "purchase": trigger customer onboarding workflow
6. Update lifecycle change timestamp

**Lifecycle Flow**:
```
awareness → interest → consideration → purchase → retention
```

Can move forward or backward in this flow.

### 6.6 Contact Enrichment

**Enrich Contact**:

**Inputs**:
- Contact ID
- User ID

**Business Logic**:
1. Use email or company domain to fetch:
   - LinkedIn profile
   - Company information
   - Social media profiles
   - Professional background
2. Update contact with enriched data
3. Mark as enriched in metadata

**Enriched Fields**:
- LinkedIn URL
- Company size
- Company industry
- Company location
- Job title verification
- Years of experience

### 6.7 Contact Search & Filtering

**Search Contacts**:

**Search Fields**:
- Name (first or last)
- Email
- Phone
- Company
- Tags
- Notes

**Filters**:
- Type (lead, prospect, customer, etc.)
- Status (active, inactive, archived)
- Source (website, referral, etc.)
- Lifecycle stage
- Tags (any of, all of)
- Date range (created, updated)

**Sorting**:
- Name (A-Z, Z-A)
- Created date (newest, oldest)
- Last interaction (most recent, least recent)
- Lifecycle stage

### 6.8 Contact Analytics

**Metrics**:
- **Total contacts** (by type, status, source)
- **New contacts** (this week/month)
- **Lifecycle distribution**: Count per stage
- **Conversion rate**: (purchase stage / total) * 100
- **Avg time in stage**: Average days in each lifecycle stage
- **Top sources**: Contacts by source, sorted by count
- **Interaction frequency**: Average interactions per contact
- **Response rate**: Contacts with response / contacts contacted

---

## 7. DEALS & SALES

### 7.1 Deal Management

**Deal Stages**:
- lead
- qualified
- proposal
- negotiation
- closed_won
- closed_lost

**Deal Types**:
- new_business
- expansion
- renewal
- upsell

**Create Deal**:

**Inputs**:
- Name
- Value (amount)
- Currency (default: USD)
- Stage
- Deal type
- Expected close date
- Probability (0-100)
- Contact ID (primary contact)
- Company name
- User ID (owner)
- Tags (optional)
- Custom fields (optional)

**Business Logic**:
1. Create deal record
2. Initialize stage history (record when deal entered each stage)
3. Calculate weighted value = value * (probability / 100)
4. Set next follow-up date based on stage

**Output**: Deal ID

### 7.2 Deal Stage Progression

**Move Deal to Stage**:

**Inputs**:
- Deal ID
- New stage
- Reason (optional)
- User ID

**Business Logic**:
1. Validate deal exists
2. Record previous stage
3. Update to new stage
4. Record stage change in history with timestamp
5. Calculate stage duration (time in previous stage)
6. Update probability based on stage:
   - lead: 10%
   - qualified: 25%
   - proposal: 50%
   - negotiation: 75%
   - closed_won: 100%
   - closed_lost: 0%
7. Recalculate weighted value
8. If closed_won: trigger won workflow
9. If closed_lost: record loss reason

**Stage Duration Tracking**:
```
{
  stage: "proposal",
  enteredAt: "2025-01-15T10:00:00Z",
  exitedAt: "2025-01-20T14:30:00Z",
  durationDays: 5.19
}
```

### 7.3 Deal Activities

**Activity Types**:
- call
- email
- meeting
- proposal_sent
- contract_sent
- demo
- presentation
- note

**Log Deal Activity**:

**Inputs**:
- Deal ID
- Activity type
- Subject
- Notes (optional)
- Date/time
- Next action (optional)
- User ID

**Business Logic**:
1. Create activity record linked to deal
2. Update deal's "last activity" timestamp
3. If next action specified: create task
4. Increment deal's activity count

### 7.4 Sales Pipeline

**Pipeline Metrics**:

**Calculate Pipeline**:
- **Total value**: Sum of all open deal values
- **Weighted value**: Sum of (value * probability) for open deals
- **Deals by stage**: Count per stage
- **Average deal size**: Total value / deal count
- **Win rate**: closed_won / (closed_won + closed_lost) * 100
- **Average sales cycle**: Average days from created to closed
- **Conversion rate by stage**: What % of deals move from stage N to N+1

**Pipeline Health**:
- **Healthy**: Good mix across stages, moving forward
- **Stalled**: Many deals stuck in one stage too long
- **Top-heavy**: Too many early-stage, few late-stage
- **Weak**: Low total value, low probability deals

### 7.5 Revenue Forecasting

**Forecast Revenue**:

**Input**: Time period (month, quarter, year)

**Logic**:
1. Filter deals with expected close date in period
2. For each deal:
   - If stage = closed_won: forecast = value (100%)
   - If stage != closed_lost: forecast = weighted value
3. Sum all forecasts

**Forecast Categories**:
- **Best case**: Sum of all weighted values
- **Likely**: Sum of weighted values for probability >= 50%
- **Worst case**: Sum of weighted values for probability >= 75%
- **Committed**: Sum of deals in negotiation or later stages

### 7.6 Deal Scoring

**Score Deal** (lead scoring):

**Factors**:
- Company size (employees, revenue)
- Contact seniority
- Industry fit
- Budget indicated
- Timeline urgency
- Engagement level (# activities)
- Source quality

**Scoring Formula**:
```
score = 
  (companySize * 0.2) +
  (seniority * 0.15) +
  (industryFit * 0.15) +
  (budget * 0.25) +
  (timeline * 0.1) +
  (engagement * 0.15)

Scale: 0-100
```

**Score Bands**:
- 80-100: Hot
- 60-79: Warm
- 40-59: Cool
- 0-39: Cold

### 7.7 Deal Loss Analysis

**Record Loss**:

**Inputs**:
- Deal ID
- Loss reason
- Competitor (if applicable)
- Detailed notes
- User ID

**Loss Reasons**:
- price
- features
- timing
- competitor
- budget_cut
- lost_contact
- no_decision
- other

**Business Logic**:
1. Move deal to closed_lost stage
2. Set probability to 0
3. Record loss reason and details
4. Record competitor if applicable
5. Update loss analytics

**Loss Analytics**:
- Total losses by reason
- Total losses by competitor
- Average deal value lost
- Common loss patterns (stage, timeline, etc.)

---

## 8. DOCUMENTS

### 8.1 Document Management

**Document Types**:
- contract
- proposal
- invoice
- report
- presentation
- spreadsheet
- pdf
- image
- other

**Document Status**:
- draft
- in_review
- approved
- published
- archived

**Create Document**:

**Inputs**:
- Name
- Document type
- File (upload or generated)
- User ID (creator)
- Folder ID (optional)
- Tags (optional)
- Access level: private | shared | public
- Metadata (optional)

**Business Logic**:
1. Upload file to storage
2. Extract metadata (file size, MIME type, page count for PDFs)
3. Generate preview/thumbnail
4. Create document record
5. Set initial status: "draft"
6. Index content for search (if text-based)

**Output**: Document ID

### 8.2 Document Versioning

**Create Version**:

**Inputs**:
- Document ID
- New file (upload)
- Version notes
- User ID

**Business Logic**:
1. Archive current version
2. Upload new file
3. Increment version number
4. Record version metadata:
   - Version number
   - Created by
   - Created at
   - Changes summary
5. Keep old versions accessible

**Version History**:
- List all versions
- Compare versions (diff if text-based)
- Revert to previous version
- Download specific version

### 8.3 Document Collaboration

**Share Document**:

**Inputs**:
- Document ID
- Shared with user ID(s)
- Permission: view | comment | edit
- Expiry date (optional)
- User ID (sharer)

**Business Logic**:
1. Validate document exists
2. Create share record for each user
3. Send notification to shared users
4. Track share history

**Permissions**:
- **view**: Read-only access
- **comment**: View + add comments
- **edit**: Full edit access

### 8.4 Document Comments

**Add Comment**:

**Inputs**:
- Document ID
- Content
- Location (page #, paragraph, selection)
- User ID
- Reply to comment ID (optional, for threaded comments)

**Business Logic**:
1. Create comment record
2. Link to document location
3. If reply: link to parent comment
4. Notify document owner
5. Notify mentioned users (@mention)

**Comment Resolution**:
- Mark comment as resolved
- Reopen resolved comment
- Archive old comment threads

### 8.5 Document Search & Indexing

**Index Document**:

**For Text Documents**:
1. Extract full text
2. Tokenize (word boundaries, stemming)
3. Create inverted index: term → document IDs
4. Index metadata (title, tags, author)

**Search Documents**:

**Search Types**:
- Full-text search (search content)
- Metadata search (filter by type, tags, author, date)
- Combined search (text + filters)

**Ranking**:
1. TF-IDF relevance score
2. Boost for title matches
3. Boost for recent documents
4. Boost for frequently accessed

### 8.6 Document Templates

**Create Template**:

**Inputs**:
- Name
- Template type (document type)
- Template file (with placeholders)
- Variables: [{name, description, defaultValue?, required?}]
- User ID

**Generate from Template**:

**Inputs**:
- Template ID
- Variable values: {[variable]: value}
- User ID

**Business Logic**:
1. Load template file
2. Replace all placeholders with provided values
3. If required variable missing: error
4. Generate new document
5. Save as draft

**Placeholder Syntax**:
- `{{variable_name}}` for text replacement
- `{{#if condition}}...{{/if}}` for conditional sections
- `{{#each items}}...{{/each}}` for repeated sections

### 8.7 Document Analytics

**Track Metrics**:
- Views per document
- Downloads per document
- Edit history (who, when, what changed)
- Collaboration activity (comments, shares)
- Storage usage by user/folder
- Most viewed documents
- Most edited documents

### 8.8 Document OCR & Extraction

**Process Scanned Document**:

**Inputs**:
- Document ID (must be image or PDF)
- Language (optional, default: en)

**Business Logic**:
1. Extract text via OCR
2. Store extracted text in metadata
3. Make text searchable
4. Extract structured data if possible (invoices, forms)

**For Forms/Invoices**:
- Extract key fields (date, amount, vendor, etc.)
- Store as structured metadata
- Enable field-based search

---

## 9. EMAIL

### 9.1 Email Management

**Email Types**:
- inbox
- sent
- draft
- archived
- spam

**Email Priority**:
- low
- normal
- high
- urgent

**Send Email**:

**Inputs**:
- To (recipients array)
- CC (optional)
- BCC (optional)
- Subject
- Body (HTML or plain text)
- Attachments (optional)
- Priority (optional)
- User ID (sender)

**Business Logic**:
1. Validate recipients
2. Validate attachments (size limits)
3. Create email record with status "sending"
4. Queue for delivery
5. On success: update status to "sent", record sent timestamp
6. On failure: update status to "failed", record error

**Output**: Email ID

### 9.2 Email Threading

**Thread Detection**:

**Group emails by**:
1. Subject line (normalized - remove "Re:", "Fwd:", etc.)
2. References header (In-Reply-To, References)
3. Participants (same set of people)

**Thread Structure**:
- Thread ID
- Subject
- Participants
- Message count
- First message timestamp
- Last message timestamp
- Unread count

### 9.3 Email Categorization

**Auto-Categorize**:

**Categories**:
- primary (person-to-person)
- social (notifications from social networks)
- promotions (marketing emails)
- updates (automated updates, receipts)
- forums (mailing list, forum posts)

**Classification Logic**:
1. Check sender domain
2. Analyze content patterns
3. Check for unsubscribe links (promotions)
4. Check for transactional patterns (updates)
5. Use ML classifier if available

### 9.4 Email Rules & Filters

**Create Rule**:

**Inputs**:
- Name
- Conditions: [{field, operator, value}]
  - Fields: from, to, subject, body, has_attachment
  - Operators: contains, equals, starts_with, ends_with
- Actions: [{type, value}]
  - Types: move_to_folder, apply_label, mark_read, forward, delete
- User ID

**Rule Execution**:
```
For each incoming email:
  For each rule (in priority order):
    if all conditions match:
      execute all actions
      if rule.stopProcessing: break
```

### 9.5 Email Search

**Search Emails**:

**Search Fields**:
- From
- To/CC/BCC
- Subject
- Body
- Date range
- Has attachment
- Labels/folders
- Is read/unread
- Is starred

**Advanced Operators**:
- `from:user@domain.com`
- `subject:invoice`
- `has:attachment`
- `before:2025-01-01`
- `after:2025-01-01`
- `is:unread`
- `label:important`

### 9.6 Email Templates

**Create Email Template**:

**Inputs**:
- Name
- Subject (with variables)
- Body (with variables)
- Variables: [{name, description, defaultValue?}]
- User ID

**Use Template**:

**Inputs**:
- Template ID
- Variable values
- Recipients

**Business Logic**:
1. Load template
2. Replace variables in subject and body
3. Pre-fill email compose with rendered template
4. User can edit before sending

### 9.7 Email Analytics

**Track Metrics**:
- **Emails sent/received** (over time)
- **Response rate**: Replies / sent * 100
- **Average response time**: Time between received and replied
- **Top senders/recipients**: By email count
- **Busiest hours**: When most emails sent/received
- **Email volume trends**: Daily/weekly/monthly patterns

**Email Health**:
- Inbox zero days (days with 0 unread)
- Average unread count
- Longest unanswered email
- Follow-up needed count

### 9.8 Email Tracking

**Track Email Opens**:
- Embed tracking pixel in sent email
- Record when recipient opens (first open, all opens)
- Record device, location (if available)

**Track Link Clicks**:
- Replace links with tracked redirect URLs
- Record which links clicked
- Record click timestamps

**Tracking Metrics**:
- Open rate: Opened / sent * 100
- Click rate: Clicked / sent * 100
- Click-to-open rate: Clicked / opened * 100

---

## 10. CROSS-DOMAIN PATTERNS

### 10.1 Universal Record Operations

**ALL entities support**:

**Create**:
- Generate unique ID
- Set created timestamp
- Set updated timestamp
- Emit creation event
- Return entity with ID

**Read**:
- Get by ID
- Query with filters
- Paginate results (limit, offset)
- Sort results
- Project fields (select specific fields only)

**Update**:
- Validate entity exists
- Apply changes
- Update timestamp
- Emit update event
- Return updated entity

**Delete**:
- Validate entity exists
- Soft delete (mark as deleted) OR hard delete (remove from DB)
- Emit deletion event
- Return success confirmation

### 10.2 Filtering & Querying

**Filter Operators**:
- equals (==)
- not equals (!=)
- greater than (>)
- less than (<)
- greater or equal (>=)
- less or equal (<=)
- contains (substring or array membership)
- starts with
- ends with
- in (value in array)
- not in (value not in array)

**Compound Filters**:
- AND: All conditions must match
- OR: Any condition must match
- NOT: Negate condition

**Example**:
```
{
  AND: [
    { field: "status", operator: "equals", value: "active" },
    { field: "created", operator: ">", value: "2025-01-01" }
  ]
}
```

### 10.3 Sorting

**Sort Specification**:
- Field name
- Direction: "asc" or "desc"
- Multiple sort fields (priority order)

**Example**:
```
[
  { field: "priority", direction: "desc" },
  { field: "created", direction: "desc" }
]
```

### 10.4 Pagination

**Page-Based**:
- `page`: Page number (1-indexed)
- `pageSize`: Items per page
- Return: {items, total, page, pageSize, totalPages}

**Offset-Based**:
- `limit`: Max items to return
- `offset`: Number of items to skip
- Return: {items, total, limit, offset}

**Cursor-Based** (for large datasets):
- `cursor`: Opaque token from previous response
- `limit`: Max items
- Return: {items, nextCursor, hasMore}

### 10.5 Field Projection

**Select Specific Fields**:
```
fields: ["id", "name", "email"]
```
Returns only requested fields (reduces payload size).

**Exclude Fields**:
```
exclude: ["metadata", "internalNotes"]
```
Returns all fields except specified ones.

### 10.6 Aggregation

**Common Aggregations**:
- **count**: Number of records
- **sum**: Sum of numeric field
- **avg**: Average of numeric field
- **min**: Minimum value
- **max**: Maximum value

**Group By**:
```
{
  groupBy: "status",
  aggregate: { totalValue: "sum" }
}

Result:
{
  "active": { count: 10, totalValue: 5000 },
  "inactive": { count: 3, totalValue: 800 }
}
```

### 10.7 Authorization Patterns

**ALL operations check**:
- User is authenticated
- User has permission for operation
- User can access specific record (ownership or sharing)

**Permission Levels**:
- **read**: View only
- **write**: Create and edit
- **delete**: Delete records
- **admin**: Full control including permissions

**Ownership**:
- Record has `userId` or `ownerId` field
- Owner has full access
- Others have access only if explicitly shared

**Sharing**:
- Records can be shared with specific users
- Share has permission level
- Share can have expiry date

### 10.8 Event Emission

**ALL mutations emit events**:

**Event Structure**:
- Event ID
- Event type (e.g., "contact.created", "deal.stage_changed")
- Entity type
- Entity ID
- Actor ID (who made the change)
- Timestamp
- Previous value (for updates)
- New value (for updates/creates)
- Metadata (extra context)

**Event Uses**:
- Audit trail
- Webhooks
- Analytics
- Real-time updates (websockets)
- Rebuild state from events (event sourcing)

### 10.9 Validation

**Common Validations**:
- **Required fields**: Field must be present and non-empty
- **Type validation**: Field must be correct type (string, number, boolean, etc.)
- **Format validation**: Email format, phone format, URL format, UUID format, datetime format
- **Range validation**: Number within min/max, date within range, string length limits
- **Enum validation**: Value must be one of allowed values
- **Uniqueness**: Field value unique across all records (e.g., email)
- **Reference validation**: Foreign key exists (e.g., contactId references valid contact)

**Validation Errors**:
- Return field name
- Return validation rule that failed
- Return descriptive message
- Return provided value (if safe to expose)

### 10.10 Tagging

**Tags** (cross-domain):
- ANY entity can have tags (array of strings)
- Tags enable categorization, grouping, filtering
- Tags are user-defined (free-form)

**Tag Operations**:
- Add tag to entity
- Remove tag from entity
- List all tags (unique across all entities)
- Filter entities by tag
- Tag autocomplete (suggest existing tags)

### 10.11 Activity Tracking

**Activity Log** (cross-domain):
- Track all user actions
- What: Action type (view, create, update, delete)
- Who: User ID
- When: Timestamp
- Where: Entity type and ID
- Details: Action-specific metadata

**Activity Queries**:
- Recent activity for user
- Recent activity for entity
- Activity by type
- Activity timeline (chronological)

### 10.12 Notifications

**Notification Types**:
- in_app (bell icon, notification center)
- email (send email)
- push (mobile/desktop push notification)
- sms (text message)

**Create Notification**:

**Inputs**:
- Recipient user ID
- Title
- Message
- Type (info, success, warning, error)
- Action URL (optional - what happens when clicked)
- Notification channels (in_app, email, push, sms)

**Business Logic**:
1. Create notification record
2. Mark as unread
3. Deliver to specified channels
4. Track delivery status

**Notification Preferences**:
- User can enable/disable channels per notification type
- User can set quiet hours (no notifications during specified times)

### 10.13 Search

**Universal Search**:
- Search across ALL entity types
- Return grouped results: {contacts: [...], deals: [...], documents: [...]}
- Rank by relevance
- Highlight matching terms

**Search Scope**:
- User's own data
- Shared data user can access
- Public data (if any)

### 10.14 Bulk Operations

**Bulk Create**:
- Input: Array of entities
- Process each create
- Return: Array of created entities (with IDs) + any errors

**Bulk Update**:
- Input: Array of {id, changes}
- Process each update
- Return: Array of updated entities + any errors

**Bulk Delete**:
- Input: Array of IDs
- Process each delete
- Return: Array of deleted IDs + any errors

**Error Handling**:
- Continue on error (process all, report errors at end)
- OR Stop on first error (transaction-like behavior)

### 10.15 Import/Export

**Export**:
- Format: CSV, JSON, Excel
- Scope: All entities or filtered set
- Include: Full data or selected fields
- Compression: Optional ZIP

**Import**:
- Format: CSV, JSON, Excel
- Validation: Validate before import
- Conflict handling: Skip, overwrite, or merge
- Mapping: Map import fields to entity fields
- Preview: Show what will be imported before committing

### 10.16 Webhooks

**Register Webhook**:

**Inputs**:
- URL to call
- Events to subscribe to (e.g., "contact.created", "deal.stage_changed")
- Secret (for signing requests)
- User ID

**Webhook Delivery**:
1. Event occurs
2. Create webhook payload (event data)
3. Sign payload with secret (HMAC)
4. POST to webhook URL
5. Retry on failure (exponential backoff, max 5 attempts)
6. Record delivery status

---

## IMPLEMENTATION NOTES

### Coordinate Addressing (PXYZ)

**Every operation can be expressed as**:
```
P = Entity type (contact, deal, event, etc.)
X = Operation (create, read, update, delete, query, etc.)
Y = Constraints/Context (filters, auth, isolation level, etc.)
Z = Temporal (timestamp, date range)
```

### Pure Functions

**Business logic should be pure functions**:
- Input → Output (deterministic)
- No side effects
- No hidden dependencies
- Easily testable

**Side effects isolated to**:
- Database operations
- External API calls
- File system operations
- Email/notification sending

### Event Sourcing

**State = Projection(Events)**:
- Don't store state directly
- Store events (what happened)
- Rebuild state by replaying events
- Enables:
  - Audit trail
  - Time travel (state at any point in past)
  - Event-driven architecture
  - Debugging (replay events to reproduce bugs)

### Schema Validation

**All inputs validated against schemas**:
- Type checking
- Required field checking
- Format validation
- Business rule validation

**Validation failures**:
- Return clear error messages
- Don't process invalid inputs
- Log validation errors for debugging

### Error Handling

**All operations can fail**:
- Not found errors
- Validation errors
- Permission errors
- Duplicate errors
- External service errors

**Error responses include**:
- Error type/code
- Human-readable message
- Field name (if field-specific error)
- Context (entity ID, operation, etc.)

### Idempotency

**Operations should be idempotent where possible**:
- Same request executed multiple times = same result
- Important for retries
- Use idempotency keys for critical operations

### Pagination Best Practices

**For large datasets**:
- Always paginate (don't return all results)
- Cursor-based pagination for very large sets
- Page-based for smaller sets with stable ordering

### Caching

**Cache frequently accessed data**:
- User profile
- Permissions
- Lookup tables (tags, categories, etc.)
- Recent entities

**Cache invalidation**:
- On update
- On delete
- Time-based expiry

---

## APPENDIX: CONSTANTS & DEFAULTS

### Time Constants
- Hour in milliseconds: 3600000
- Day in milliseconds: 86400000
- Week in milliseconds: 604800000

### Limits
- Max results per query: 1000 (default: 100)
- Max bulk operation size: 1000 items
- Max file upload size: 100MB
- Max attachment size (email): 25MB per file, 25MB total
- Max email recipients: 100

### Defaults
- Default timezone: UTC
- Default currency: USD
- Default page size: 20
- Default working hours: 9:00-17:00
- Default week start: Monday

### Priority Values
- Low: 1
- Medium: 2
- High: 3
- Urgent: 4

### Percentage Thresholds
- Healthy: < 40% (various metrics)
- Moderate: 40-60%
- Overloaded: > 60%

---

**END OF BUSINESS LOGIC REFERENCE**

This document contains EVERY business operation extracted from 11,573 lines of TypeScript across 10 domain files. This is the source of truth for what OMAR must do, independent of how it's implemented.
