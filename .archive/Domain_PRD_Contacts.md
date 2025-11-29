# Contacts Domain - Feature PRD (Path/Stack Agnostic)

## Purpose & Scope

The Contacts domain serves as the relationship intelligence center of the system. It manages all people and organizations the business interacts with, tracking relationship health, interaction history, and evolution through various lifecycle stages. Unlike traditional static contact lists, this domain treats relationships as living, evolving entities that can transform based on engagement patterns and business context.

## Core Philosophy

Contacts aren't just database records—they're relationship stories. The system captures not just "who" someone is, but the narrative of your business relationship: how you met, what you've discussed, what commitments exist, and where the relationship is heading. Every interaction adds a chapter to this story, making it easy to pick up conversations months later without losing context.

## Primary Features

### Relationship Profiles

**Individual Contacts**
Each person in the system maintains a rich profile that grows organically through interactions:

- Identity basics (name, email, phone) captured with minimal friction
- Role and company associations that update automatically as relationships evolve
- Relationship history timeline showing all emails, notes, tasks, and documents linked to this person
- Momentum indicators showing engagement health (active, warming, cooling, dormant)
- Smart tags auto-applied based on content analysis and manual curation
- Preferred communication methods and timezone awareness

**Organization Profiles**
Companies exist as network hubs connecting related individuals:

- Organizational hierarchy showing who reports to whom (when relevant)
- Aggregate relationship health across all contacts at this organization
- Shared history timeline combining interactions with all individuals
- Industry, size, and other firmographic data captured passively through interactions
- Deal and project history tied to the organization
- Team access controls for multi-user workspaces

**Relationship Mapping**
The system automatically infers and visualizes connection patterns:

- Stakeholder networks within organizations (who knows whom)
- Decision-maker identification based on email patterns and mentions
- Champion detection (internal advocates for your business)
- Missing role identification ("You haven't connected with their CFO yet")
- Introduction suggestions ("Would you like me to draft an intro email to their CEO?")

### Lifecycle Evolution & State Management

Contacts progress through natural lifecycle stages without rigid pipelines:

#### Lead Stage

- Initial entry point from any source (form submission, email exchange, note mention, manual creation)
- Minimal data capture—system doesn't force unnecessary fields
- Auto-tagging based on source and initial interaction content
- Quick actions: promote to prospect, schedule first touch, add to nurture sequence

#### Prospect Stage

- Active exploration phase with multiple touchpoints
- Opportunity tracking and qualification criteria
- Activity timeline showing engagement frequency and sentiment
- Risk indicators if engagement drops or stalls
- Transition triggers: schedule demo, send proposal, move to negotiation

#### Customer/Client Stage

- Active business relationship with ongoing work
- Project and workflow association showing what you're delivering
- Invoice history and payment patterns
- Health scoring based on satisfaction signals and retention risk
- Upsell and cross-sell opportunity detection

#### Partner/Vendor Stage

- Alternative path for non-customer relationships
- Different interaction patterns and expectations
- Collaboration history and joint projects
- Performance tracking for vendors

#### Dormant/Archive Stage

- Relationships that have gone quiet or concluded
- Preservation of full history for future reactivation
- Periodic "check-in" suggestions to revive relationships
- Clean separation from active workflow without data loss

**Circular Logic**: Any contact can move between stages in any direction. A dormant client can reactivate, a customer can become a referral partner, a lead can skip stages entirely if context warrants. The system never enforces artificial progression rules.

### Interaction Timeline & History

Every contact maintains a chronological stream of all related activity:

### Timeline Components

- Email threads (sent, received, and CC'd) with full content searchable
- Meeting notes and call summaries linked to calendar events
- Tasks assigned to or about this contact with completion status
- Documents shared with or created for this contact
- Workflow milestones and project deliverables
- Payment history and invoice status
- Portal interactions if the contact is a client with access
- System-generated insights ("Last contact was 45 days ago—relationship cooling")

### AI-Enhanced Timeline

- Automatic summarization of long thread chains
- Commitment extraction ("They agreed to send budget by Friday")
- Sentiment analysis showing tone shifts over time
- Next-best-action suggestions based on history patterns
- Relationship health scoring with contributing factors explained

### Timeline Filtering & Navigation

- Filter by interaction type (emails only, tasks only, etc.)
- Date range selection for focused review
- Search within timeline for specific topics or keywords
- Jump to key moments (first contact, last invoice, most recent meeting)

### Adaptive Contact Lists & Views

The system presents contacts through multiple lenses optimized for different workflows:

### Momentum View (Default)

Surfaces contacts ranked by relationship health and urgency:

- Recently active relationships needing follow-up
- Cooling relationships at risk of going dormant
- New leads requiring first touch
- Upcoming commitments or deadlines
- Stalled opportunities needing re-engagement

### Segmentation Views

Pre-built and custom segments for targeted work:

- All contacts, leads, prospects, customers, partners
- By industry, company size, or custom tags
- Geographic groupings for location-based work
- Team-based views (my contacts, team contacts, all)
- Smart lists with dynamic membership (e.g., "high-value customers not contacted in 30 days")

### Search & Command Palette Integration

The Crown (global search) provides instant contact lookup:

- Type any name, email, company, or tag to find matches instantly
- Results ranked by relevance and recent activity
- Quick actions available directly from search results
- Jump to contact detail or open related objects (tasks, deals, emails)

### List Interactions

- Multi-select for bulk actions (tag assignment, task creation, export)
- Drag-and-drop into workflows or projects
- Quick-edit inline without opening full detail view
- Export to CSV or send bulk communications

### Contact Detail Experience

Opening a contact reveals a comprehensive yet uncluttered view:

### Layout Philosophy

Theinterface follows the Crown/Main Stage/Dock model:

- Left column: Identity snapshot (photo, name, title, company, contact methods)
- Center area: Interaction timeline taking visual priority
- Right drawer: Context-aware quick actions and AI suggestions
- Bottom Dock: Related objects (open tasks, active deals, recent documents)

### Quick Actions Panel

Context-sensitive suggestions appear based on current state:

- "Draft follow-up email" if last interaction was incoming
- "Schedule call" if timeline shows phone tag
- "Send invoice" if work is complete but unbilled
- "Move to dormant" if relationship has been quiet for extended period
- "Request introduction" if stakeholder mapping shows gaps

### Inline Editing & Evolution

- Click any field to edit without leaving the view
- Add tags, notes, or custom attributes on the fly
- Promote lead to customer with one action (triggers appropriate workflows)
- Link to projects, deals, or workflows via simple selection
- Relationship mapping updates in real-time as you add connections

### Mobile-Optimized Detail View

On mobile devices, the layout adapts:

- Vertical stack with priority on recent activity
- Swipe gestures for quick actions (left: email, right: task)
- Tap-to-call/text using native OS handlers
- Collapsible sections to reduce scrolling

### Content Evolution & Circular Flows

Contacts participate in the system's circular content evolution philosophy:

### Inbound Transformations (How contacts are created)

- Email → Contact: Receiving an email from an unknown address offers to create a contact
- Note → Contact: Typing an email address or phone in a note offers to extract as contact
- Task → Contact: Assigning a task to someone new creates their profile
- Form Submission → Contact: Web form or portal signup auto-creates and enriches
- Import → Contact: CSV import with deduplication and merge suggestions

### Outbound Transformations (What contacts can become)

- Contact → Lead: Flag as potential opportunity and add to pipeline
- Contact → Task: "Create follow-up task for Sarah" instantly links them
- Contact → Deal: Promote lead to active deal with value and stage tracking
- Contact → Workflow: Enroll in onboarding, nurture, or service delivery sequence
- Contact → Archived Note: Convert to lightweight note if relationship concludes

### Bidirectional Linking

- All transformations preserve lineage and provenance
- Objects linking to contacts maintain live references
- Changes to contact data propagate to linked objects where appropriate
- Deleting or archiving contacts prompts handling of linked objects

### AI Copilot Integration

The AI Companion provides intelligent assistance throughout contact workflows:

### Relationship Intelligence

- "Sarah hasn't replied in 2 weeks—would you like me to draft a gentle follow-up?"
- "You last spoke with John's team 6 months ago—might be time to check in"
- "Three people at Acme Corp mentioned budget concerns—flag for discussion?"
- "Contact pattern suggests this is a decision-maker—prioritize for outreach"

### Content Generation

- Draft personalized intro emails using relationship context
- Generate meeting prep briefs summarizing recent activity
- Suggest talking points based on past conversations and current projects
- Create call summaries from voice transcripts with action items extracted

### Proactive Suggestions

- Recommend missing stakeholders based on organizational structure
- Identify cross-sell opportunities from customer interaction patterns
- Flag retention risks from declining engagement metrics
- Propose re-engagement campaigns for dormant relationships

### Memory & Context

- AI maintains memory tags specific to each contact (preferences, quirks, communication style)
- "Remember: Sarah prefers morning calls and likes bullet-point emails"
- Context carried across all interactions with this person
- Tone and formality adjust automatically per relationship

### Search, Filtering & Smart Lists

Finding the right contacts at the right time is effortless:

### Unified Search

The Crown search handles complex contact queries:

- Natural language: "customers in Chicago who haven't paid yet"
- Property filters: `type:customer tag:vip status:active`
- Boolean logic: "tech AND (San Francisco OR remote)"
- Saved searches appear as quick-access pills

### Dynamic Smart Lists

Lists update automatically as contacts match criteria:

- "High-value customers without activity in 30 days"
- "Leads added this week from website forms"
- "All prospects in negotiation stage"
- "Dormant relationships worth reactivating (6+ months quiet, prior revenue >$10K)"

### Filtering & Sorting

Any contact view can be refined:

- By stage, tag, owner, or custom properties
- By interaction recency or frequency
- By deal value or revenue contribution
- By sentiment score or relationship health

### Subscription & Notifications

- Subscribe to smart lists for daily or weekly digests
- Get notified when contacts enter or leave lists
- Set alerts for specific contact activities or threshold events

### Import, Export & Data Management

### Import Flows

- CSV upload with column mapping interface
- Duplicate detection with merge/skip/create options
- Validation and error handling with clear guidance
- Preview import before committing changes
- Undo window for accidental imports

### Export Capabilities

- Export any view or selection to CSV
- Choose fields to include (all, minimal, custom)
- Segment exports for targeted campaigns
- Schedule recurring exports for external systems

### Deduplication & Merging

- Automatic duplicate detection on create/import
- Side-by-side merge UI showing conflicts
- Field-by-field selection for merged record
- History preservation from both source records
- Undo capability if merge was incorrect

### Data Quality Tools

- Identify incomplete profiles (missing email, phone, company)
- Flag suspicious data (invalid email format, placeholder names)
- Suggest enrichment from external sources (when integrated)
- Bulk update tools for mass corrections

### Multi-Tenant & Access Control

### Workspace Scoping

- All contacts belong to a workspace (tenant)
- Strict isolation prevents cross-workspace leakage
- Row-level security enforced at query level

### Role-Based Visibility

- Admins see all workspace contacts
- Team members see contacts they own or are shared with
- Clients (portal users) see only their own profile
- Granular sharing controls for sensitive relationships

### Team Collaboration

- Assign ownership for accountability
- Share contacts with specific team members or groups
- Activity visibility settings (private notes, shared timeline)
- Handoff workflows when ownership transfers

### Performance & Scalability Patterns

### List Virtualization

- Large contact lists render only visible rows
- Infinite scroll or pagination options
- Maintains smooth interaction even with thousands of contacts

### Optimistic Updates

- UI updates immediately on user action
- Background sync with conflict resolution
- Clear indication of pending vs. saved changes

### Offline Support

- Recently viewed contacts cached for offline access
- Queue write operations when offline
- Sync and conflict resolution on reconnect

### Search Performance

- Indexed fields for instant search results
- Debounced queries to reduce server load
- Progressive result loading for large match sets

## User Journeys

### Journey: First Contact Creation

1. User receives email from potential client Jane at Acme Corp
2. Email view shows "Jane Smith" as unknown sender
3. Inline prompt: "Create contact for Jane Smith?"
4. User clicks; system auto-fills email and infers company from domain
5. User adds phone number and role; saves
6. Contact created; email thread now linked to contact record
7. AI suggests: "Would you like to add a follow-up task for Jane?"

### Journey: Relationship Health Check

1. User opens Contacts with default Momentum view
2. System highlights three contacts in "cooling" state (30+ days since last touch)
3. User clicks "David Chen" (previously warm lead)
4. Timeline shows last interaction was a promising email exchange
5. AI suggests: "David expressed interest in Q3 project—time to check in?"
6. User clicks "Draft follow-up"; AI generates contextual email referencing prior conversation
7. User edits, sends; contact status updates to "re-engaged"

### Journey: Stakeholder Mapping

1. User is working with Sarah at TechCo; deal moving slowly
2. Opens Sarah's contact detail
3. Relationship map shows Sarah reports to CTO (not yet in system)
4. AI flags: "Sarah's approval likely requires CTO buy-in"
5. User adds CTO contact via relationship builder
6. System links CTO to Sarah and TechCo organization
7. User requests intro email; AI drafts message asking Sarah to connect them
8. CTO added to deal stakeholders; visibility improved

### Journey: Lead to Customer Conversion

1. User has been nurturing lead "Mark" through email campaign
2. Mark replies ready to proceed
3. User opens Mark's contact; current stage shows "Prospect"
4. Quick action: "Convert to Customer"
5. System prompts: "Start onboarding workflow?"
6. User confirms; contact promoted to Customer stage
7. Onboarding workflow auto-creates tasks and document requests
8. Mark receives portal access for client collaboration

### Journey: Bulk Tag Assignment

1. User attends industry conference; collects 20 new leads
2. Imports leads via CSV
3. Selects all 20 in list view
4. Bulk action: "Add tag: Conference2025"
5. System applies tag to all selected
6. User creates smart list: "tag:Conference2025 AND stage:lead"
7. List auto-updates as these leads evolve
8. User sets up nurture workflow targeting this segment

## State & Data Model (Conceptual)

### Contact Entity

- Unique identifier
- Basic identity (name, email, phone, title)
- Company/organization reference
- Lifecycle stage (lead, prospect, customer, partner, dormant)
- Ownership (primary user responsible)
- Tags (freeform and structured)
- Custom fields (extensible)
- Timestamps (created, modified, last contacted)
- Metadata (source, how created, original lead capture)

### Organization Entity

- Unique identifier
- Company name, domain, industry
- Size, location, firmographics
- Relationship to individuals (one org, many contacts)
- Organizational hierarchy (parent companies, subsidiaries)
- Tags and custom fields
- Aggregate health metrics

### Relationship Metadata

- Connection type (reports to, colleague, stakeholder)
- Strength indicator (close, professional, distant)
- Trust level and decision-making authority
- Communication preferences and patterns
- Memory tags (AI-curated context notes)

### Interaction Log Entry

- Timestamp
- Type (email, call, meeting, task, document)
- Content or summary
- Participants
- Linked objects (tasks, deals, workflows)
- Sentiment score
- Commitments or action items extracted

### Computed Properties (Derived)

- Relationship health score
- Days since last contact
- Engagement frequency
- Sentiment trend
- Momentum indicator
- Revenue contribution (for customers)
- Lifecycle position and stage duration

## Integration Points

### With Email Domain

- Inline contact creation from unknown senders
- Contact enrichment from email signatures
- Email threading preserves contact context
- Auto-linking emails to timeline

### With Tasks Domain

- Create follow-up tasks linked to contacts
- Task assignment to specific individuals
- Task completion updates contact timeline
- Bulk task creation for contact segments

### With Workflows Domain

- Enroll contacts in onboarding or nurture sequences
- Workflow steps trigger contact updates
- Contact stage changes trigger workflows
- Automated follow-up based on contact state

### With Invoices Domain

- Customer contacts linked to invoices
- Payment history visible in contact timeline
- Overdue invoices flag contact health
- Invoice generation pre-fills contact billing info

### With Documents Domain

- Documents shared with specific contacts
- Contracts and proposals linked to contact records
- Signature requests tied to contact workflows
- Document access visible in portal for client contacts

### With Client Portal Domain

- Client contacts receive portal access credentials
- Portal activity feeds into contact timeline
- Secure messaging between client and team
- Document uploads and approvals visible

### With AI Copilot

- Relationship intelligence and health monitoring
- Next-best-action recommendations
- Content generation (emails, call prep, intros)
- Sentiment analysis and commitment tracking
- Memory tag curation and context management

## UX Principles Specific to Contacts

### Progressive Disclosure

Start with minimal data capture; expand as relationship deepens. Don't force users to fill 20 fields for a quick lead entry.

### Context Over Fields

Show relationship story through timeline, not through endless form fields. The narrative matters more than structured data.

### Frictionless Evolution

Moving a contact through lifecycle stages should feel obvious and require minimal clicks. Let context (not process) drive progression.

### Search-First Navigation

Finding contacts should be instant via Crown search. Power users shouldn't need to navigate menus.

### Relationship-Centric Design

Design around "people" not "records." Use conversational language, human photos, and relationship metaphors.

### Mobile Parity

Contact details must be fully accessible and actionable on mobile. Tap-to-call/email, swipe actions, and offline access are table stakes.

### AI as Wingman

The AI should feel like a relationship manager who remembers everything and makes helpful suggestions, never intrusive or presumptuous.

## Edge Cases & Safety Nets

### Duplicate Prevention

- Real-time duplicate detection during creation
- "Similar contacts exist" warning with merge option
- Fuzzy matching on name, email, company

### Relationship Conflicts

- If contact moves companies, preserve history at old org
- Handle transitions gracefully (promotion, job change, acquisition)
- Allow multiple roles at different organizations (consultant, advisor)

### Deletion & Archiving

- Soft-delete with recovery window
- Warn if contact has linked objects (tasks, deals, invoices)
- Option to archive relationships cleanly vs. hard delete

### Privacy & Compliance

- GDPR-compliant data export for contact requests
- Right-to-delete with audit trail
- Consent tracking for marketing communications
- Redaction of sensitive data before sharing

### Access Control Boundaries

- Prevent team members from seeing unshared contacts
- Hide sensitive notes from non-owners
- Portal users see only their own data, never other clients
- Admin override with audit logging

## Success Metrics

- Time to first contact creation (should be under 10 seconds)
- Contact data completeness over time
- Relationship health score trends
- Engagement frequency and recency
- Conversion rates (lead → prospect → customer)
- Dormant relationship reactivation rate
- User adoption of AI suggestions
- Time spent managing contacts (should decrease as AI improves)

## Conclusion

The Contacts domain is the relational memory of the business. It transforms static lists into living relationship intelligence, helping small business operators nurture connections without drowning in data entry or losing context. By combining smart defaults, circular evolution, and AI assistance, it makes relationship management feel natural and effortless rather than like database maintenance.
