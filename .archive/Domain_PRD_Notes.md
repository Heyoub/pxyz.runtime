# Notes Domain - Feature PRD (Path/Stack Agnostic)

## Purpose & Scope

The Notes domain provides a frictionless thought capture system that serves as the entry point for ideas, observations, and working knowledge. It treats notes as living documents that can evolve into more structured objects (tasks, contacts, documents) while preserving the original context. Unlike traditional note-taking apps that trap information in isolated files, this domain understands notes as seeds that grow into actionable business outcomes.

## Core Philosophy

Notes are the raw material of business intelligence. They capture fleeting thoughts, meeting insights, client conversations, and spontaneous ideas before they evaporate. The system makes capture effortless and evolution natural—a note about a client conversation can spawn a task, enrich a contact profile, or transform into a formal document, all while maintaining its existence as reference material.

## Primary Features

### Ultra-Fast Capture

- **Instant Creation**

- Crown command palette: type "note" or "n" and start writing immediately
- Keyboard shortcut from anywhere (Cmd/Ctrl+N)
- Dock quick-capture panel without leaving current view
- Voice-to-note transcription on mobile
- Email-to-note forwarding (send to special address)
- Browser extension for web page clipping

- **Zero Friction Entry**

- No required fields except content
- Title auto-generates from first line or timestamp
- Tags, categories, projects optional and addable later
- Auto-save on every keystroke; no explicit save needed
- Optimistic UI updates immediately

- **Context Inheritance**

- Creating note while viewing contact auto-links to that contact
- Creating note in project view auto-tags with project
- Creating note from email preserves email thread reference
- Current page context suggested but overridable

### Note Types & Formats

- **Markdown-Powered Editor**
Clean, distraction-free writing with formatting support:

- Headings (# ## ###)
- Bold, italic, strikethrough
- Lists (ordered, unordered, checkboxes)
- Links (internal and external)
- Code blocks with syntax highlighting
- Tables
- Blockquotes
- Inline images

**Special Note Types**
System recognizes and enhances certain patterns:

- **Meeting Notes**: Detects date, participants, agenda
- **Call Notes**: Captures who, when, outcomes
- **Idea Notes**: Flagged for later development
- **Decision Logs**: Records why choices were made
- **Quick Captures**: Minimalist single-thought notes

- **Rich Content Embedding**

- Paste images directly into notes
- Embed links with preview cards
- Record voice memos attached to note
- Attach files for reference
- Embed code snippets with formatting

### AI Enhancement & Intelligence

- **Smart Suggestions**
AI Copilot assists during note-taking:

- Suggests titles based on content
- Recommends tags from similar notes
- Identifies mentioned contacts and offers to link
- Detects action items and proposes task creation
- Flags commitments or deadlines

**Content Extraction**
AI parses notes for structured data:

- Email addresses → suggest creating contact
- Phone numbers → add to contact or create new
- Dates and times → offer calendar event or task due date
- URLs → fetch metadata and create previews
- @mentions → link to people or projects

- **Auto-Tagging**

- AI suggests tags based on content analysis
- Learn from user tagging patterns over time
- Apply tags in bulk to related notes
- Tag hierarchies (Projects > Client Work > Acme)

- **Summarization**

- Long notes get AI-generated summaries
- "TL;DR" appears at top with key points
- Expand to read full content
- Summary updates as note evolves

- **Search Enhancement**

- Semantic search finds conceptually related notes
- Natural language queries ("notes about Q4 strategy")
- Related notes surfaced automatically
- Search within note content, tags, and linked objects

### Note Organization

**Flat Structure with Powerful Filtering**
Notes don't live in rigid folders; instead:

- All notes accessible from unified view
- Filter by tags, date, owner, linked objects
- Search finds anything instantly
- Saved views for common filters

**Tagging System**
Flexible categorization:

- Freeform tags (user-created)
- Structured tags (project names, contact names)
- Hierarchical tags (Client Work > Acme Corp > Website)
- Color-coded tags for visual scanning
- Multi-tag support (one note, many tags)

**Pinning & Starring**
Highlight important notes:

- Pin to keep at top of list
- Star for quick access later
- "Working Notes" section for active writing
- Recently edited automatic smart list

**Linking & Relationships**
Notes connect to other objects:

- Link to contacts (person or organization)
- Link to tasks (action items derived from note)
- Link to projects (context for note)
- Link to other notes (creating knowledge graphs)
- Link to documents (note as source material)

### Note Evolution & Transformation

**Inbound Transformations** (How notes are created)

- Manual creation by user
- Voice transcription
- Email forwarding
- Meeting transcription (from calendar integration)
- Web page clipping
- Chat/Slack message forwarding
- Document annotation
- Task completion notes (capture what was done)

**Outbound Transformations** (What notes become)

- Note → Task: Extract action items as standalone tasks
- Note → Contact: Detect person mentioned; create contact profile
- Note → Document: Formalize note into polished document
- Note → Email: Transform note into email draft
- Note → Workflow: Complex note spawns multi-step process
- Note → Meeting Agenda: Structure note into meeting format
- Note → Knowledge Base Article: Publish note as team resource

- **Preservation of Provenance**

- Transformed objects retain link back to original note
- Note itself remains intact and searchable
- Evolution history visible in note metadata
- Bidirectional navigation between note and offspring objects

### Collaborative Notes

- **Shared Notes**

- Share note with team members (read or edit access)
- Real-time collaborative editing (like Google Docs)
- Comment threads on specific sections
- @mentions notify team members
- Version history shows who changed what

- **Permissions**

- Private (only you)
- Shared with specific people
- Team-wide (all workspace members)
- Client-accessible (via portal if appropriate)

- **Commenting**

- Inline comments on paragraphs or selections
- Threaded discussions
- Resolve comment threads when addressed
- Notification when mentioned in comments

- **Activity Log**

- Full history of edits and comments
- Who viewed note and when
- Linked object creation history
- Restoration to prior version

### Note Templates

**Reusable Structures**
Common note patterns saved as templates:

- Meeting notes template (date, attendees, agenda, notes, action items)
- Call notes template (who, when, summary, next steps)
- Project kickoff template (goals, stakeholders, milestones)
- Weekly review template (wins, challenges, priorities)
- Decision log template (context, options, decision, rationale)

- **Template Application**

- Select template when creating note
- Pre-filled structure with placeholders
- Override or customize as needed
- Save successful notes as new templates

- **Team Templates**

- Share templates across workspace
- Admin-curated templates for consistency
- Template library with search and preview

### Quick Capture & Assistant Integration

- **Dock Quick Panel**

- Always-accessible note panel in Dock
- Write quick notes without leaving current work
- Auto-links to active context (contact, task, project)
- Minimizes back to Dock when done
- Notes accessible later from main Notes view

- **Voice Capture**

- Record voice memo via mobile
- AI transcribes to text
- Speaker identification for meeting notes
- Highlight key moments while recording
- Playback with text sync

**AI Quick Capture**
User can dictate to AI Copilot:

- "Remember that Sarah preferred morning meetings"
- AI creates note and tags with contact Sarah
- "Note for later: explore partnership with Vendor B"
- AI files as idea note with relevant tags

### Search & Discoverability

**Unified Search**
Crown search provides instant note lookup:

- Full-text search across all note content
- Tag-based search: `tag:client-work tag:urgent`
- Date range search: `created:last-week`
- Linked object search: `contact:Sarah`
- Combined queries: "notes about Sarah from last month"

**Related Notes Surfacing**
AI suggests related notes:

- When viewing contact, show related notes
- When writing note, show similar existing notes
- Prevent duplication; encourage consolidation
- Discover forgotten context

**Smart Lists**
Dynamic note collections:

- "Recently edited" (last 7 days)
- "Untagged notes" (cleanup list)
- "Notes with tasks" (action-item tracking)
- "Meeting notes this month"
- "Ideas awaiting development"

### Mobile & Offline Experience

- **Mobile Note Capture**

- Voice recording with transcription
- Photo capture with note annotation
- Quick text entry with keyboard shortcuts
- Swipe gestures for actions (archive, tag, share)
- Offline editing with background sync

- **Offline Sync**

- Recently viewed notes cached for offline reading
- Edits queue when offline; sync on reconnect
- Conflict resolution if edited on multiple devices
- Clear offline indicator

- **Widget & Shortcuts**

- Home screen widget for ultra-fast capture
- Share-to-note from other apps
- Siri/Google Assistant voice capture

### Note Archiving & Cleanup

- **Archive System**

- Archive old notes to clean up active view
- Archived notes remain searchable
- Restore from archive anytime
- Bulk archive by date or tag

- **Retention Policies**

- Auto-archive notes after N days of inactivity (configurable)
- Prompt before permanent deletion
- Soft-delete with recovery window
- Compliance-friendly retention for regulated industries

**Note Hygiene**
AI suggests cleanup:

- "You have 47 untagged notes—tag them now?"
- "These 12 notes haven't been viewed in 6 months—archive?"
- Duplicate detection and merge suggestions

## User Journeys

### Journey: Quick Capture During Call

1. User on call with client discussing Q4 priorities
2. Hits keyboard shortcut (Cmd+N) without leaving call
3. Note opens; user types as client talks
4. Mentions "Q4 budget increase" and "new vendor evaluation"
5. Saves note; AI detects "Q4" and "vendor" as tags
6. AI suggests: "Create task: Evaluate vendors for Q4?"
7. User confirms; task created and linked to note
8. Note remains as reference for future conversations

### Journey: Meeting Notes Evolution

1. User attends client kickoff meeting; takes notes
2. Uses meeting notes template: attendees, agenda, discussion, action items
3. During meeting, types action items as checkboxes
4. After meeting, AI offers: "Convert action items to tasks?"
5. User confirms; 4 tasks created and assigned to team
6. Tasks link back to meeting note for context
7. Next week, user reviews note and adds follow-up thoughts
8. Note serves as permanent record of client relationship milestone

### Journey: Idea Capture to Document

1. User has idea for new service offering
2. Opens Dock quick-capture panel; writes rough thoughts
3. Tags with "ideas" and "business-development"
4. Over next week, user adds to note when inspiration strikes
5. Note grows to 3 pages with structure emerging
6. User decides to formalize; clicks "Convert to Document"
7. System creates document, applies business proposal template
8. Original note remains; document links back to it
9. User polishes document; shares with team for feedback

### Journey: Voice Capture on Mobile

1. User driving; remembers important client detail
2. Pulls up voice assistant: "Hey Siri, note for Matter"
3. Speaks: "Remind me that David Chen prefers email over calls and hates long meetings"
4. AI transcribes, creates note, tags with contact "David Chen"
5. Later, user viewing David's contact profile
6. Note appears in context: preference saved as memory tag
7. AI Copilot remembers this when suggesting meeting types with David

### Journey: Collaborative Research Notes

1. User creates note "Q4 Marketing Campaign Ideas"
2. Shares with marketing team (edit access)
3. Team members add ideas and comments in real-time
4. Brainstorming session happens async across week
5. User reviews, highlights top 3 ideas
6. Converts note to workflow: "Q4 Campaign Execution"
7. Workflow phases map to ideas from note
8. Note becomes reference document for campaign

## State & Data Model (Conceptual)

### Note Entity

- Unique identifier
- Title (auto-generated or user-set)
- Content (markdown text)
- Tags (array)
- Owner (creator)
- Shared with (users/teams)
- Linked objects (contacts, tasks, projects, documents)
- Attachments (files, images, voice memos)
- Status (active, archived, deleted)
- Timestamps (created, modified, last viewed)
- Provenance (how created, source context)

### Note Version

- Version number
- Content snapshot
- Timestamp
- Editor (who made changes)
- Change summary (diff)

### Comment

- Comment identifier
- Note reference
- Author
- Content
- Selection/anchor (inline comments)
- Thread parent (for replies)
- Resolved status
- Timestamp

### Template

- Template identifier
- Name
- Content structure (markdown with placeholders)
- Suggested tags
- Visibility (personal, team, system)

## Integration Points

### With Contacts Domain

- Notes link to contacts for context
- Contact detail shows all related notes
- Create contact from email/phone in note
- Memory tags extracted from notes

### With Tasks Domain

- Extract action items from notes as tasks
- Task creation preserves note link
- Completing task offers to add outcome note
- Notes provide context for tasks

### With Email Domain

- Forward emails to create notes
- Notes can be sent as email drafts
- Email threads referenced in notes
- Meeting invites generate note templates

### With Documents Domain

- Formalize notes into polished documents
- Notes serve as document drafts
- Document annotation creates linked notes
- Research notes feed document content

### With Workflows Domain

- Notes capture workflow outcomes and learnings
- Complex notes spawn workflows
- Workflow steps can require note documentation
- Workflow completion generates summary note

### With Calendar Domain

- Meeting events auto-create note templates
- Notes link to calendar events
- Action items from notes sync to calendar tasks

### With AI Copilot

- AI suggests tags, links, and transformations
- Summarizes long notes
- Extracts structured data (contacts, dates, actions)
- Surfaces related notes and context
- Drafts content based on note prompts

## UX Principles Specific to Notes

### Capture Over Organization

Make it trivial to create notes; defer categorization. Getting thoughts out of head is priority one.

### Markdown for Simplicity

Rich text without complexity. Keyboard-friendly formatting that doesn't interrupt flow.

### Evolution Over Silos

Notes aren't dead ends. They naturally transform into tasks, documents, and structured objects.

### Context Over Folders

Flat structure with smart search and filtering beats rigid hierarchies.

### AI as Assistant

AI should enhance notes (suggest tags, extract data) without interfering with writing flow.

### Mobile Parity

Voice capture and quick entry on mobile must be as fast as desktop keyboard shortcuts.

## Edge Cases & Safety Nets

### Accidental Deletion

- Soft-delete with recovery window
- Undo immediately after delete
- Archive vs. delete distinction
- Warn if deleting note with linked objects

### Lost Notes

- "Recently deleted" smart list for recovery
- Search includes archived and deleted notes (with filter)
- Restore with full history intact

### Concurrent Editing

- Real-time collaborative editing for shared notes
- Conflict resolution for offline edits
- Last-write-wins for metadata (tags, title)
- Merge content changes when possible

### Privacy & Sharing

- Private by default
- Explicit sharing required
- Revoke access anytime
- Audit log of who viewed/edited

### Voice Transcription Errors

- Editable transcripts
- Re-listen to recording to verify
- Highlight uncertain transcriptions

## Success Metrics

- Notes created per user per week
- Note-to-task conversion rate
- Note-to-document conversion rate
- Search usage (are notes findable?)
- Voice capture adoption
- Collaborative note usage
- User-reported note utility
- Average time from idea to note capture

## Conclusion

The Notes domain serves as the cognitive scratchpad for the business. By making capture effortless, evolution natural, and discovery instant, it ensures that fleeting thoughts become lasting value. Notes bridge the gap between informal thinking and formal action, creating a continuous flow from idea to outcome while preserving the context that makes information meaningful.
