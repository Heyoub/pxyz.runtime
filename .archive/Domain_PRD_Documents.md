# Documents Domain - Feature PRD (Path/Stack Agnostic)

## Purpose & Scope

The Documents domain provides a smart canvas for creating, editing, and managing formal business content. It goes beyond traditional document editors by understanding context, morphing its interface based on intent, and intelligently linking to the broader business workflow. The FluidDoc system adapts from simple text to spreadsheets, presentations, or code—all within one unified experience.

## Core Philosophy

Documents aren't static files—they're living artifacts that evolve with your work. A document might start as meeting notes, morph into a proposal, embed a pricing table that expands into a full spreadsheet, then generate an invoice. The system removes the friction of "what tool do I use?" by providing one intelligent canvas that transforms based on what you're trying to accomplish.

## Primary Features

### FluidDoc Smart Canvas

#### Adaptive Interface

The document editor morphs to match intent:

- **Empty State**: Clean blinking cursor; minimal UI; no assumptions
- **Text Mode**: Standard rich text editing when you start typing
- **Table Detection**: Type `| Header | Header |` and it becomes a structured table
- **Spreadsheet Expansion**: Click table icon; expands into full Excel-like interface
- **Presentation Mode**: Switch view to slide-based layout
- **Code Mode**: Syntax highlighting for technical documentation
- **Canvas/Whiteboard Mode**: Freeform spatial layout with blocks

#### Context-Aware Linkification

As you type, the system recognizes patterns:

- **Phone numbers**: Become tap-to-call/SMS links
- **Email addresses**: Become mail-to links
- **@mentions**: Link to contact profiles
- **#tags**: Create document tags
- **{placeholders}**: Pull live data (e.g., {Q4 revenue})
- **URLs**: Auto-preview with metadata cards

#### Intelligent Blocks

Documents compose from smart blocks:

- Text blocks: Paragraphs, headings, lists
- Table blocks: Structured data with formulas
- Image blocks: Photos, diagrams, embedded media
- Code blocks: Syntax-highlighted snippets
- Embed blocks: External content (videos, maps, widgets)
- Data blocks: Charts, graphs, live-updated metrics
- Template blocks: Reusable content modules

### Multiple Document Modes

#### Standard Text Editor

Default writing experience:

- Rich text formatting (bold, italic, headings, lists)
- Markdown shortcuts for speed
- Focus mode hides all UI chrome
- Word count and reading time estimates
- Spell check and grammar suggestions
- AI writing assistance

#### Spreadsheet/Table Mode

Embedded data power:

- Formulas and calculations
- Cell formatting and styles
- Sort and filter
- Pivot tables for analysis
- Charts and visualizations
- Export to CSV/Excel

#### Presentation Mode

Slide-based layout:

- Convert sections to slides automatically
- Speaker notes and presenter view
- Transition animations (subtle, professional)
- Full-screen presentation mode
- PDF export for distribution

#### Code/Technical Mode

Developer-friendly documentation:

- Syntax highlighting for 20+ languages
- Line numbers and code folding
- Diff view for version comparisons
- Copy code with one click
- API documentation templates

#### Canvas/Whiteboard Mode

Spatial thinking:

- Drag blocks anywhere on infinite canvas
- Visual connections between ideas
- Embed images, links, notes
- Zoom in/out for detail vs. overview
- Export as image or PDF

### Document Creation & Templates

#### Quick Creation

- Crown command: "new document" or "doc"
- Template gallery with previews
- Blank document for freeform
- AI-generated starting point from prompt

#### Template Library

Pre-built structures for common needs:

- **Proposals**: Client proposal with pricing, timeline, terms
- **Contracts**: Service agreements, NDAs, SOWs
- **Reports**: Status reports, performance summaries
- **Meeting Agendas**: Structured meeting planning
- **Project Briefs**: Scope, stakeholders, deliverables
- **Invoices**: Formal billing documents
- **Onboarding Guides**: Client welcome packets
- **SOPs**: Standard operating procedures

#### AI Document Generation

Copilot creates documents from prompts:

- "Generate a proposal for website redesign project"
- AI pulls context from linked contact, project, pricing history
- Creates structured document with sections
- User reviews, edits, and personalizes

#### Template Customization

- Edit any template to fit your needs
- Save customized versions
- Share templates with team
- Version control for templates

### Collaborative Editing

#### Real-Time Collaboration

- Multiple users edit simultaneously (Google Docs style)
- See cursor positions and selections of collaborators
- Presence indicators show who's viewing/editing
- Changes sync instantly with conflict resolution

#### Comments & Suggestions

- Inline comments on selected text
- Threaded discussions on specific sections
- Suggestion mode: edits require acceptance (like Word Track Changes)
- @mention team members for attention
- Resolve comment threads when addressed

#### Version History

- Auto-save creates checkpoint on every significant change
- Timeline view shows document evolution
- Diff view highlights changes between versions
- Restore to any prior version
- Named checkpoints for milestones

#### Permissions & Sharing

- Private (only you)
- Shared with specific people (view, comment, edit)
- Team-wide access
- Client portal sharing (external, controlled)
- Link-based sharing with expiration
- Redact sensitive sections before sharing

### AI-Powered Document Intelligence

#### Content Generation

- Draft sections based on bullet points
- Expand brief notes into full paragraphs
- Rewrite for tone (professional, friendly, concise)
- Translate to other languages
- Generate summaries and executive briefs

#### Consistency Checking

- Detect inconsistent terminology or names
- Flag missing or contradictory data
- Suggest style improvements
- Check grammar and readability

#### Data Extraction

- Pull information from linked objects (contacts, deals, invoices)
- Auto-populate placeholders with live data
- Update calculations as source data changes
- Detect and warn about stale data

#### Smart Suggestions

- Recommend relevant templates
- Suggest sections based on document type
- Flag incomplete sections (e.g., missing pricing in proposal)
- Offer next steps (e.g., "Send for approval?")

### Document Linking & Context

#### Bidirectional Links

Documents connect to business objects:

- **Contacts**: Who is this document for?
- **Projects**: What work does it support?
- **Tasks**: What actions does it generate?
- **Workflows**: Where does it fit in process?
- **Invoices**: Is it billable or billing-related?

#### Embedded Context

- Document header shows linked objects
- Click links to navigate to related items
- Changes in linked objects can update document
- Document history shows origin (e.g., created from note, generated from workflow)

#### Document References

- Cross-reference other documents
- Embed live sections from other docs
- Version-aware references (update if source doc changes)
- Circular reference detection

### Document Workflows

#### Approval Flows

- Request approval from specific users
- Approval states: pending, approved, changes requested
- Approver sees diffs and context
- Approval gates before sending to clients

#### Signature Requests

- Mark signature fields in document
- Send for e-signature (integrates with DocuSign, etc.)
- Track signature status
- Collect signed copies

#### Review Cycles

- Suggestion mode for reviewers
- Track changes and comments
- Consolidated feedback view
- Accept/reject changes in bulk
- Lock sections after approval

### Document Organization

#### Flat Structure with Smart Filtering

- All documents in unified library
- No forced folder hierarchy
- Tags for flexible categorization
- Link to projects, contacts, workflows for context

#### Document Types

**Document Types**
System recognizes common types:

- Proposals
- Contracts
- Reports
- Meeting Notes
- Onboarding Guides
- Invoices
- SOPs
- Technical Docs

- **Search & Discovery**

- Crown search finds documents instantly
- Full-text search across all content
- Filter by type, tags, owner, linked objects
- Recent and frequently used documents surfaced
- AI suggests related documents

- **Smart Collections**

- "Recently edited by me"
- "Awaiting my approval"
- "Client-facing documents"
- "Drafts (not yet shared)"
- "Documents with @mentions of me"

### Export & Integration

- **Export Formats**

- PDF (high-quality, print-ready)
- Word (.docx)
- Markdown (.md)
- HTML (for web publishing)
- Plain text
- Images (PNG for presentations)

- **Import Support**

- Upload Word, PDF, Google Docs
- Import with format preservation
- Convert to FluidDoc for editing
- OCR for scanned PDFs

- **External Tool Integration**

- Open in Microsoft Word/Google Docs
- Sync changes back to system
- Version conflict resolution
- Maintain single source of truth

### Mobile & Offline Experience

- **Mobile Editing**

- Full document editing on phone/tablet
- Voice dictation for drafting
- Touch-optimized formatting toolbar
- Swipe gestures for navigation
- Offline editing with sync

- **Offline Capabilities**

- Recently viewed documents cached
- Edit offline; changes queue for sync
- Conflict resolution on reconnect
- Clear offline indicator

- **Quick Capture**

- Camera integration for document scanning
- Voice memo transcription as document
- Photo annotation and markup

## User Journeys

### Journey: Proposal Creation from Scratch

1. User wins sales call; needs to send proposal quickly
2. Opens Crown; types "new proposal"
3. Selects "Client Proposal" template
4. Template loads with sections: Overview, Scope, Timeline, Pricing, Terms
5. AI prompts: "Link to contact Sarah Chen at Acme Corp?"
6. User confirms; contact linked; AI populates company name throughout
7. User fills Overview and Scope by typing naturally
8. Reaches Pricing section; types table syntax
9. Table expands; user clicks "Spreadsheet Mode"
10. Full Excel-like interface appears; user enters line items and formulas
11. Total auto-calculates; user collapses back to table view
12. Reviews complete proposal; clicks "Request Approval" from manager
13. Manager receives notification; reviews; approves
14. User shares via client portal; PDF auto-generated
15. Sarah receives notification; opens polished proposal

### Journey: FluidDoc Morphing Experience

1. User starts with blank document; just a cursor
2. Types heading: "Q4 Budget Planning"
3. Types normal paragraphs explaining context
4. Types: "| Category | Q3 Actual | Q4 Budget |"
5. System detects table; creates structured table
6. User fills 10 rows of budget data
7. Clicks spreadsheet icon on table
8. Table expands into full-screen spreadsheet mode
9. User adds formulas, conditional formatting, totals
10. Adds chart visualizing budget comparison
11. Collapses spreadsheet back into document
12. Document now has text, table, and embedded chart
13. User shares with team; they see formatted, professional doc

### Journey: Collaborative Document Review

1. User drafts "Vendor Partnership Agreement"
2. Shares with legal team (suggestion mode)
3. Legal reviews; adds inline comments and suggested edits
4. User receives notification; opens document
5. Sees comments highlighted in sidebar
6. Reviews each suggestion: accept, reject, or discuss
7. Replies to comments with clarifications
8. Legal responds; back-and-forth in thread
9. User accepts most suggestions; resolves all comments
10. Document marked "Approved by Legal"
11. User sends to vendor for signature
12. Signature request tracked in document metadata

### Journey: Document from Note Evolution

1. User has detailed note "Client Onboarding Process"
2. Note grown to 5 pages over weeks
3. Realizes it should be formal onboarding guide
4. Clicks "Convert to Document"
5. System creates document; applies SOP template structure
6. Content from note mapped to template sections
7. User adds formatting, images, and polish
8. Document becomes team resource
9. Original note remains; links to document
10. Future updates to document; note preserved as draft history

### Journey: Live Data Integration

1. User creating monthly performance report
2. Types: "Revenue this month: {monthly_revenue}"
3. System recognizes placeholder; pulls live data from invoices
4. Revenue figure appears and updates automatically
5. User types: "Top client: {top_client_by_revenue}"
6. Client name pulled from CRM data
7. Document stays current as data changes
8. User exports PDF; data snapshot at export time

## State & Data Model (Conceptual)

### Document Entity

- Unique identifier
- Title
- Content (structured blocks)
- Document type (proposal, contract, report, etc.)
- Owner (creator)
- Linked objects (contacts, projects, tasks)
- Tags
- Status (draft, in review, approved, published, archived)
- Permissions (who can view/edit)
- Timestamps (created, modified, last viewed)

### Document Block

- Block type (text, table, image, code, embed)
- Content (depends on type)
- Formatting and styles
- Position in document
- Metadata (formulas, links, etc.)

### Document Version/Checkpoint

- Version number or timestamp
- Content snapshot
- Author of changes
- Change summary/diff
- Named checkpoint flag

### Comment

- Comment identifier
- Document and block reference
- Author
- Content
- Thread parent (for replies)
- Resolved status
- Timestamp

### Document Template

- Template identifier
- Name and description
- Structure (blocks and placeholders)
- Suggested tags and type
- Visibility (personal, team, system)
- Version

## Integration Points

### With Contacts Domain

- Documents link to contacts (proposals for clients, contracts with vendors)
- Contact detail shows related documents
- Auto-populate contact info in documents
- Document sharing via contact preferences

### With Tasks Domain

- Document creation/approval can spawn tasks
- Tasks link to deliverable documents
- Document completion updates tasks
- Workflow tasks require document uploads

### With Workflows Domain

- Workflow steps generate documents from templates
- Document approval gates in workflows
- Document status advances workflow
- Workflows track document versions

### With Invoices Domain

- Proposals convert to invoices
- Invoices rendered as formatted documents
- Contract terms inform invoice generation
- Signed contracts trigger billing workflows

### With Notes Domain

- Notes evolve into formal documents
- Document drafts start as notes
- Research notes feed document content
- Document annotations create linked notes

### With Email Domain

- Send documents via email with preview
- Email attachments saved as documents
- Document collaboration via email comments
- Signature requests sent by email

### With Client Portal

- Documents published to portal for clients
- Client uploads saved as documents
- Portal document sharing with redaction
- Approval requests via portal

### With AI Copilot

- AI drafts document sections
- Content suggestions and improvements
- Data extraction and population
- Summarization and expansion
- Tone adjustment and rewriting

## UX Principles Specific to Documents

### FluidDoc Philosophy

One canvas adapts to all needs. Don't force users to choose tools before they know what they're building.

### Progressive Enhancement

Start simple (blank page); add complexity only when needed (tables, spreadsheets, presentations).

### Context Over Chrome

Hide UI complexity until relevant. Empty document shows minimal UI; full spreadsheet shows full toolbar.

### Linkification Everywhere

Make content interactive. Phone numbers, emails, @mentions, data references should all be actionable.

### AI as Co-Author

AI should help draft, improve, and complete documents without taking over creative control.

### Mobile Parity

Document creation and editing must work fully on mobile, not just viewing.

## Edge Cases & Safety Nets

### Version Conflicts

- Real-time sync prevents most conflicts
- Offline edits merge automatically when possible
- Manual conflict resolution for incompatible changes
- Always preserve both versions for user choice

### Data Staleness

- Warn when linked data hasn't updated recently
- Refresh data placeholders on demand
- Snapshot data at export to prevent future changes

### Broken Links

- Detect when linked objects deleted or moved
- Offer repair or unlinking options
- Preserve document even if links break

### Large Documents

- Lazy-load sections for performance
- Paginate or virtualize long docs
- Collapse sections for easier navigation
- Search within document

### Accidental Deletion

- Soft-delete with recovery window
- Warn if document linked to active workflows
- Version history allows restoration
- Undo immediately after delete

## Success Metrics

- Documents created per user per week
- Note-to-document conversion rate
- Template usage and effectiveness
- Collaboration activity (comments, suggestions)
- Approval cycle time
- Export frequency and formats
- AI suggestion acceptance
- Mobile document editing adoption

## Conclusion

The Documents domain transforms static file creation into a dynamic, intelligent authoring experience. By adapting to user intent, linking to business context, and leveraging AI assistance, it makes document work feel natural rather than like operating complex software. The FluidDoc system ensures users spend time thinking and writing, not wrestling with tools.
