# Email Domain - Feature PRD (Path/Stack Agnostic)

## Purpose & Scope

The Email domain unifies all business communication into a single, intelligent inbox experience. It manages multiple email accounts, preserves context across threaded conversations, and intelligently links email interactions to the broader business workflow (contacts, tasks, deals, documents). Unlike traditional email clients that treat messages as isolated events, this domain understands email as part of a continuous business narrative.

## Core Philosophy

Email isn't just messages in a mailbox—it's the primary communication channel where relationships develop, commitments form, and work gets done. The system treats every email thread as a story unfolding over time, automatically connecting messages to people, projects, and outcomes. It eliminates context-switching by bringing email into the same workspace as tasks and contacts, making it effortless to turn conversations into actions.

## Primary Features

### Multi-Account Unified Inbox

#### Account Management

- Connect multiple email accounts (Gmail, Outlook, custom SMTP/IMAP)
- Each account maintains separate identity (from address, signature, branding)
- Unified view shows all accounts in one stream
- Per-account filtering when focusing on specific identity
- Automatic credential management with secure storage
- OAuth integration for major providers (no password storage)
- Custom SMTP/IMAP support for specialized needs

#### Account Switching & Composition

- When composing, select which account sends the message
- Default account inferred from context (replying from received account)
- Quick-switch accounts without leaving compose view
- Signature and footer auto-apply based on sending account

#### Sync & Offline Behavior

- Background sync keeps all accounts up to date
- Configurable sync frequency per account
- Offline queue for composed messages when disconnected
- Conflict resolution for messages modified on multiple devices

### Threading & Conversation Intelligence

#### Smart Threading

The system understands conversation context beyond subject lines:

- Groups related messages even if subject changes mid-thread
- Handles reply-all chains, forward chains, and fragmented discussions
- Surfaces newest message first with expandable history
- Collapses read messages; highlights unread or important
- Preserves inline replies and partial quotes for readability

#### Thread Participants

- Visual indicators show all participants in conversation
- Distinguish sender, recipients, CC'd individuals
- Click any participant to open their contact profile
- Quick-add participants to contact database if not already present
- Identify decision-makers and key stakeholders in thread

#### Thread Metadata & State

- Mark threads as starred, snoozed, archived, or deleted
- Custom tags for categorization ("Billing," "Support," "Urgent")
- Last reply time and participant activity tracking
- Unread/read status per user
- Pin important threads to stay at top of inbox

### Focus Views & Filtering

#### Pre-Built Focus Views

- ### Needs Reply

  - Email Messages where response is expected or overdue

- ### Waiting On Me

  - Threads where user sent last message but no response yet

- ### Follow-Up

  - Snoozed threads resurfacing at scheduled time

- ### Billing/Invoicing

  - Auto-tagged financial discussions

- ### Support/Help

  - Customer service or assistance requests

- ### Today/This Week

  - Time-based prioritization

#### Custom Views

- Build views with any combination of filters
- Save and name for quick access
- Share views with team members
- Subscribe to view changes for notifications

#### Smart Filtering

- Filter by sender, recipient, account, date range
- Tag-based filtering (multiple tags with AND/OR logic)
- Search within thread content, subjects, or attachments
- Attachment type filtering (PDFs, images, spreadsheets)
- Unread status, starred status, snoozed status

#### Search Integration

TheCrown global search provides instant email lookup:

- Natural language: "emails from Sarah about Q4 project"
- Property filters: `from:sarah@acme.com tag:urgent`
- Content search: finds text within message bodies
- Attachment filename search
- Date range queries: `after:2025-01-01 before:2025-02-01`

### Email Composition & Templates

#### Compose Experience

- Distraction-free editor with rich text formatting
- Inline image insertion and attachment support
- Contact auto-complete from system contact database
- Recent recipients and frequent contacts suggested first
- Multiple recipient fields (To, CC, BCC) with visual clarity

#### Tone & Style Selector

AI-powered tone adjustment:

- Formal: professional, business-appropriate language
- Friendly: warm, conversational tone
- Urgent: conveys time-sensitivity without being rude
- Concise: bullets and brevity for busy recipients
- System learns user's writing style and mirrors it

#### Templates & Snippets

- Save common message patterns as templates
- Insert saved snippets with keyboard shortcuts
- Variable placeholders for personalization (name, company, project)
- AI suggests relevant templates based on context
- Team-shared templates for consistent communication

#### Smart Suggestions

AI Copilot assists during composition:

- "You mentioned a deadline but didn't specify a date"
- "Sarah prefers bullet-point emails—would you like me to reformat?"
- "This message feels terse—soften the tone?"
- Auto-complete sentences based on past messages
- Suggest attachments if context implies need

#### Scheduling & Send Later

- Schedule messages to send at optimal times
- AI suggests best send times based on recipient timezone and past response patterns
- Edit or cancel scheduled sends before delivery
- Recurring scheduled messages for regular check-ins

### Email Actions & Workflows

#### Quick Actions Panel

Every email thread has context-aware actions:

- Reply, Reply All, Forward with one click
- Create task from email (captures thread context)
- Link to existing contact or create new contact
- Add to deal or workflow
- Convert email to note or document
- Snooze until specific date/time
- Archive, delete, mark unread

#### Snooze & Reminders

- Snooze threads to reappear at chosen time
- Options: later today, tomorrow, next week, custom
- Snoozed threads hide from inbox but resurface on schedule
- Reminder notifications when snoozed thread returns

#### Archiving & Deletion

- Archive removes from active inbox but preserves searchability
- Soft-delete with recovery window before permanent removal
- Bulk archive/delete for inbox cleanup
- Undo immediately after action

#### Tagging & Organization

- Apply tags to threads for categorization
- Tags auto-suggest based on content and context
- Nested tag hierarchies (Projects > Client Work > Acme Corp)
- Tag-based automation (auto-tag emails from specific senders)

### Integration with Business Workflow

#### Contact Linking

- Emails automatically link to sender/recipient contact records
- View full email history with contact from contact detail page
- Create contacts on-the-fly from email participants
- Enrich contact profiles with information from email signatures

#### Task Creation from Email

- Convert email into actionable task with one click
- Task inherits email content as description
- Due date and owner assignable during creation
- Email thread remains linked to task for reference
- Completing task can auto-send reply or follow-up

#### Deal & Workflow Association

- Link emails to active deals or projects
- Emails appear in deal timeline and context
- Workflow steps can trigger automated emails
- Email responses advance workflow state

#### Document Generation

- Save email as document for formal record-keeping
- Convert email thread to meeting notes or summary document
- Extract attachments and associate with contacts or projects
- Generate proposals or contracts from email discussions

#### Calendar Integration

- Propose meeting times in email; sync to calendar
- Calendar invites sent via email with RSVP tracking
- Email reminders for upcoming meetings pulled from calendar

### AI-Powered Email Intelligence

#### Automatic Summarization

- Long threads summarized into key points
- Commitments and action items extracted automatically
- Sentiment analysis shows tone shifts through conversation
- "Catch me up" button generates executive summary

#### Smart Prioritization

- Emails ranked by urgency, importance, and context
- VIP contacts surface to top
- Threads with deadlines or commitments prioritized
- Low-priority marketing/newsletters de-emphasized

#### Commitment Tracking

- AI detects commitments ("I'll send the report by Friday")
- Automatically creates reminder or task for follow-up
- Flags broken commitments or missed deadlines
- Surfaces commitments from others for accountability

#### Response Suggestions

- AI drafts quick replies based on thread context
- Three-option quick responses for simple queries ("Yes," "No," "Let me check")
- Full draft generation for complex replies
- Learn from user edits to improve future drafts

#### Follow-Up Detection

- AI identifies threads needing follow-up
- Suggests follow-up timing based on prior patterns
- Drafts gentle reminders referencing previous conversation
- Escalation suggestions if repeated follow-ups ignored

#### Tone & Sentiment Analysis

- Detect frustration, confusion, urgency in received emails
- Alert user to potentially negative sentiment
- Suggest appropriate response tone
- Track sentiment trends with specific contacts

### Collaborative Inbox Features

#### Shared Inboxes

- Team accounts accessible to multiple users (e.g., support@, sales@)
- Assign conversations to specific team members
- Status indicators (unassigned, in progress, resolved)
- Internal notes and comments on threads (not visible to sender)

#### Delegation & Handoff

- Forward conversations with context notes to colleagues
- Transfer ownership of thread to another team member
- Preserve history and continuity through handoffs

#### Team Visibility

- Optionally share personal inbox threads with team
- Activity log shows who viewed, replied, or acted on threads
- Prevent duplicate responses with "Someone is replying" indicator

### Email Security & Privacy

#### Spam & Phishing Protection

- Auto-filtering of obvious spam and phishing attempts
- User training on suspicious messages
- Quarantine for review before permanent deletion
- Allow/block lists for sender management

#### Encryption & Compliance

- End-to-end encryption option for sensitive communications
- Compliance-friendly archiving for legal/regulatory needs
- Retention policies configurable per account or workspace
- Audit trail for all email actions (sent, read, forwarded)

#### Redaction & Sharing

- Redact sensitive information before forwarding or sharing threads
- Preview redacted version before send
- Log all redaction decisions for compliance

#### Access Control

- Restrict email access by user role
- Shared inbox permissions (read, reply, assign)
- Prevent clients from accessing team email accounts

### Mobile & Offline Experience

#### Mobile-Optimized Inbox

- Swipe gestures for quick actions (left: archive, right: snooze)
- Tap-to-call or tap-to-text senders from email
- Bottom-focused UI for thumb reachability
- Collapsed thread views to reduce scrolling

#### Offline Capabilities

- Recently viewed threads cached for offline reading
- Compose drafts offline; sync when reconnected
- Queue send operations and deliver on reconnect
- Clear indicators of offline state and pending actions

#### Push Notifications

- Configurable notifications for new emails
- VIP sender alerts
- Thread activity alerts (new reply in watched thread)
- Quiet hours respect to prevent after-hours interruptions

### Performance & Scalability

#### Inbox Virtualization

- Render only visible emails in list view
- Lazy-load older messages as user scrolls
- Maintain smooth scrolling even with thousands of threads

#### Background Sync

- Incremental sync to minimize bandwidth
- Delta updates for changed threads
- Batch operations for efficiency

#### Search Performance

- Indexed full-text search across all accounts
- Results return sub-second even with large mailboxes
- Progressive loading for large result sets

## User Journeys

### Journey: Unified Inbox First Use

1. User connects Gmail and Outlook accounts via OAuth
2. System syncs both accounts; displays unified inbox
3. Gmail thread and Outlook thread appear intermixed by recency
4. User filters to "Needs Reply" view
5. Two threads surface requiring responses
6. User replies to Gmail thread; system sends from correct account automatically
7. User switches to Outlook for second reply; account selection is contextual

### Journey: Email to Task Conversion

1. User receives email from client with project request
2. Opens email; reads request details
3. Clicks "Create Task" action
4. Task modal pre-fills with email subject and body content
5. User assigns due date and owner; saves
6. Task created and linked to email thread
7. Opening task later shows original email for reference
8. Completing task offers to send reply email to client

### Journey: Thread Summarization

1. User returns from vacation; 47 unread emails
2. Opens first thread with 15 messages from team discussion
3. Clicks "Summarize" button
4. AI generates: "Team debated vendor selection; decided on Vendor B; Mary assigned to draft contract by Friday"
5. User catches up instantly without reading 15 messages
6. Clicks through to full thread for specific details when needed

### Journey: Smart Follow-Up

1. User sent proposal email to prospect two weeks ago
2. No response received
3. AI surfaces notification: "No reply from David—follow up?"
4. User clicks "Draft Follow-Up"
5. AI generates gentle reminder referencing original proposal
6. User edits slightly; sends
7. Prospect replies within hours; opportunity stays alive

### Journey: Commitment Tracking

1. Client emails promising budget approval "by end of week"
2. AI detects commitment and creates automatic reminder
3. Friday arrives; user receives notification
4. User follows up with client to confirm budget status
5. Client apologizes for delay; commits to Monday
6. AI updates reminder; user stays on top of the commitment

## State & Data Model (Conceptual)

### Email Thread

- Unique thread identifier
- Participants (sender, recipients, CC, BCC)
- Subject and subject history (if changed mid-thread)
- Account association (which account received/sent)
- Messages (ordered list of individual emails)
- Tags and categories
- Status flags (read/unread, starred, snoozed, archived)
- Linked objects (contacts, tasks, deals, workflows)
- Timestamps (first message, last message, last read)

### Individual Message

- Unique message identifier
- Sender and recipients
- Subject
- Body content (HTML and plain text)
- Attachments (files with metadata)
- Send/received timestamps
- Read status per user
- Reply-to and in-reply-to headers for threading

### Account Configuration

- Account identifier
- Provider (Gmail, Outlook, custom)
- Credentials (encrypted)
- Sync settings (frequency, folders)
- Signature and default settings
- Send-from name and address

### Tags & Categories

- Tag name and color
- Hierarchy (parent tags, nested tags)
- Auto-tagging rules
- Usage count and recency

### Filters & Views

- View name
- Filter criteria (sender, tag, date, status)
- Sort order and grouping
- Visibility (personal, shared, team)

## Integration Points

### With Contacts Domain

- Auto-link emails to sender/recipient contacts
- Create contacts from email participants
- View full email history with contact
- Enrich contact profiles from email signatures

### With Tasks Domain

- Convert emails to tasks with one click
- Link tasks to email threads for context
- Task completion can trigger reply emails
- Email-based task creation via special addresses

### With Workflows Domain

- Workflow steps can send automated emails
- Email replies advance workflow state
- Enroll contacts in email sequences from workflows
- Track email opens and clicks within workflow analytics

### With Documents Domain

- Save emails as formal documents
- Extract email attachments and link to document library
- Generate documents from email discussions
- Attach documents to outgoing emails

### With Calendar Domain

- Send meeting invites via email
- RSVP tracking and calendar sync
- Email reminders for upcoming events
- Propose meeting times in email with calendar check

### With Client Portal

- Portal messages route through email system
- Clients can reply to portal notifications via email
- Secure email gateway for client communication
- Email notifications for portal activity

### With AI Copilot

- Thread summarization and catch-up
- Response drafting and tone adjustment
- Commitment detection and tracking
- Follow-up suggestions and timing
- Sentiment analysis and urgency detection

## UX Principles Specific to Email

### Inbox Zero Philosophy

Make it effortless to clear inbox through fast actions, smart defaults, and AI assistance. Users shouldn't feel buried by email.

### Context Over Chrome

Hide email client complexity; surface only what's needed for current action. No overwhelming toolbar or endless options.

### Conversation, Not Messages

Show emails as threads with narrative flow, not isolated messages. Maintain story continuity.

### Action-Oriented

Every email should offer clear next steps—reply, task, snooze, archive. No dead-ends.

### Multi-Account Transparency

Users shouldn't think about which account they're using unless explicitly needed. System handles routing intelligently.

### Mobile Parity

Email must be fully functional on mobile with touch-optimized gestures and offline support.

### AI as Assistant

AI helps catch up, draft replies, and detect important patterns without being intrusive or making decisions for the user.

## Edge Cases & Safety Nets

### Duplicate Prevention

- Detect duplicate sends (same recipient, subject, time)
- Warn before sending message with empty subject or body
- Confirm before sending to large recipient lists

### Send Failures & Retries

- Retry failed sends automatically with exponential backoff
- Notify user of persistent failures with actionable guidance
- Queue sends if account temporarily unreachable

### Conflict Resolution

- Handle messages modified on multiple devices
- Merge read/unread status across devices
- Resolve tag conflicts with last-write-wins

### Attachment Handling

- Warn before sending large attachments
- Offer link-based sharing for oversized files
- Scan attachments for malware before opening

### Privacy & Compliance

- BCC recipients never visible to other recipients
- Redaction tools for sensitive information
- Audit logs for compliance and legal holds
- Retention policies for regulatory requirements

### Access Control and Permissions

- Team members see only assigned or shared threads in collaborative inboxes
- Clients never see internal team emails
- Admin override with audit logging

## Success Metrics

- Time to inbox zero (should decrease with AI assist)
- Email response time (should improve with templates and drafts)
- Task conversion rate (emails turned into actions)
- Follow-up completion rate
- User-reported sentiment (less email overwhelm)
- Engagement with AI suggestions (acceptance rate)
- Multi-account switching frequency (should decrease as system learns)

## Conclusion

The Email domain transforms the traditional inbox into an intelligent communication hub. By unifying accounts, understanding context, and connecting emails to the broader business workflow, it eliminates the friction of email management. Users spend less time in their inbox and more time building relationships and delivering value, with AI assistance ensuring nothing falls through the cracks.
