# Domain Stragglers & AI-Specific Tools (Path/Stack Agnostic)

This document captures missing fidelity details and domain-specific AI capabilities that weren't fully covered in the 12 core domain PRDs. These are extracted from original design documents and represent the final layer of granularity.

---

## GLOBAL UX STRAGGLERS

### Font & Typography Intentionality

#### Font System - Screen Optimization

- Multiple font families optimized for screen reading (not print)
- Dyslexia-friendly fonts as first-class option (OpenDyslexic, Lexie Readable)
- Neurotypical-friendly fonts with high legibility
- System-wide font switching without breaking layouts
- Relative sizing only (em, rem, %) - NO pixels or fixed units
- Line height and letter spacing optimized per font family

#### Typography - Hierarchy System

- Clear heading/body/caption scales
- Consistent rhythm across all surfaces
- Support for RTL languages from day one
- Font loading strategy prevents layout shift

### Theme System Architecture

#### Theme Management - Centralized

- Single CSS custom properties file controls all colors
- Swap entire theme by changing one file reference
- NO chasing million CSS files to change themes

#### Theme Variants - Built-In

- Light (default): clean, bright, professional
- Dark: true dark mode (not gray), OLED-friendly
- High Contrast: WCAG AAA compliant for accessibility
- Muted: reduced saturation for long work sessions
- Dyslexia Mode: pairs high-contrast with dyslexia fonts

#### Theme - Switching System

- Per-user preference saved
- Instant toggle (no page reload)
- Respects OS preference as default
- Smooth transition animations (no jarring flash)

### 2-3 Click Principle Enforcement

#### Interaction Auditing

- System tracks clicks-to-outcome for common actions
- Alerts when any workflow exceeds 3 clicks
- Dashboard for admins showing click-path heatmaps
- AI suggests UX improvements when patterns emerge

---

## CONTACTS DOMAIN STRAGGLERS

### Contacts: Domain-Specific AI Tools

#### Relationship Health AI

- **Health Score Calculator**: Analyzes recency, frequency, sentiment, commitments to generate health score (0-100)
- **Cooling Detection**: Alerts when engagement patterns decline (e.g., "Sarah typically replies in 2 days; it's been 8")
- **Re-Engagement Suggester**: Proposes personalized outreach based on past successful patterns
- **Ghosting Predictor**: Flags relationships at risk of going dormant based on communication gaps

#### Stakeholder Intelligence

- **Decision-Maker Detector**: Analyzes email CC patterns and meeting attendees to identify key decision-makers
- **Org Chart Inference**: Builds organizational hierarchy from email signatures and mentions
- **Missing Role Identifier**: Suggests gaps in stakeholder coverage ("You haven't connected with their CFO")
- **Champion Detector**: Identifies internal advocates based on sentiment and engagement

#### Communication Optimization

- **Best Contact Time Predictor**: Learns when contacts typically respond fastest
- **Channel Preference Detector**: Identifies if contact prefers email, phone, or meetings
- **Tone Matcher**: Adjusts suggested communication tone based on contact's communication style
- **Response Time Estimator**: Predicts when contact likely to respond based on patterns

### Missing Features

#### Contact Enrichment Pipeline

- Automatically enriches contacts from email signatures (title, company, phone)
- LinkedIn profile linking (if integration enabled)
- Company firmographic data population (size, industry, location)
- Duplicate detection across email aliases and name variations
- Merge conflicts resolution with field-by-field selection

#### Relationship Timeline Intelligence

- Visual timeline shows relationship arc (warming, stable, cooling)
- Milestone markers (first contact, deal closed, last interaction)
- Sentiment trend line overlaying timeline
- Commitment tracking with fulfillment indicators

---

## EMAIL DOMAIN STRAGGLERS

### Email: Domain-Specific AI Tools

#### Email Thread Intelligence

- Commitment Extractor: Parses emails for commitments ("I'll send by Friday") and creates tracking
- Sentiment Tracker: Monitors tone shifts in conversation threads
- Urgency Detector: Flags emails with time-sensitive language or deadlines
- Question Identifier: Highlights unanswered questions across thread
- Decision Tracker: Identifies when decisions were made in thread

#### Smart Email Composition

- Tone Calibrator: Adjusts draft tone (professional → friendly → urgent) with one click
- Brevity Optimizer: Suggests shorter alternatives for wordy emails
- Clarity Checker: Flags ambiguous language or missing context
- Subject Line Generator: Proposes subject lines based on email body
- Follow-Up Predictor: Suggests optimal follow-up timing based on recipient patterns

#### Thread Summarization Modes

- Executive Summary: 2-3 sentence distillation of long thread
- Action Items Only: Extracts commitments and next steps
- Decision Log: Highlights decisions made and rationale
- Sentiment Report: Shows tone evolution through conversation
- Participant Contributions: Summarizes who said what

### Missing Features -- Email

#### Email Health & Deliverability

- Deliverability Dashboard: SPF/DKIM/DMARC status per sending domain
- Bounce Rate Tracking: Alerts when bounce rate exceeds threshold
- Spam Score Checker: Pre-send spam score analysis
- Warm-Up Suggestions: Gradual send volume increase for new domains
- Send Window Optimizer: Suggests best send times based on recipient timezone and open patterns

#### Advanced Threading

- Handle split threads (conversation branched into multiple threads)
- Merge related threads that should be connected
- Detect thread hijacking (subject changed mid-conversation)
- Preserve threading even when subject line changes

---

## TASKS DOMAIN STRAGGLERS

### Tasks: Domain-Specific AI Tools

#### Task Intelligence

- Blocker Detector: Analyzes task descriptions for implicit blockers or dependencies
- Duration Estimator: Predicts task duration based on similar past tasks
- Owner Suggester: Recommends best person to assign based on skills and availability
- Subtask Generator: Breaks complex tasks into logical steps automatically
- Completion Predictor: Estimates likelihood of on-time completion based on progress

#### Workload Management AI

- Overload Detector: Warns when user has too many tasks due in short period
- Delegation Suggester: Proposes tasks that could be delegated to reduce load
- Priority Re-Ranker: Adjusts priorities based on changing context and deadlines
- Batch Suggester: Groups similar tasks for efficiency
- Focus Time Calculator: Estimates total focus hours needed for day's tasks

#### Task Evolution AI

- Workflow Promoter: Detects when task scope indicates it should become workflow
- Task Merger: Suggests combining duplicate or overlapping tasks
- Stale Task Identifier: Flags tasks unchanged for extended period
- Zombie Task Detector: Identifies tasks repeatedly rescheduled (never actually done)

### Missing Features -- Tasks

#### Task Templates with Smart Variables

- Template tasks with placeholders: {ClientName}, {DueDate+7days}, {AssignedTo}
- Conditional subtasks based on task category
- Auto-tag rules based on template type
- Team-shared template library with versioning

#### Task Time Blocking

- Reserve calendar time for tasks
- Auto-scheduling based on task estimates and calendar availability
- Pomodoro timer integration for time-boxed work
- Actual vs. estimated time tracking for learning

---

## WORKFLOWS DOMAIN STRAGGLERS

### Workflows: Domain-Specific AI Tools

#### Workflow Generation AI

- Wizard Intelligence: Generates entire workflow from 4-6 question inputs
- Template Matcher: Suggests existing templates based on user description
- Phase Optimizer: Recommends optimal phase breakdown for process
- Duration Predictor: Estimates realistic timeline based on similar workflows
- Resource Allocator: Suggests team member assignments based on skills and availability

#### Workflow Health Monitoring

- Bottleneck Detector: Identifies phases where workflows consistently slow down
- Risk Scorer: Predicts workflow completion probability based on current progress
- Escalation Trigger: Auto-alerts when workflow exceeds duration thresholds
- Parallel Path Suggester: Identifies steps that could run concurrently
- Efficiency Analyzer: Compares workflow performance to historical benchmarks

#### Workflow Optimization AI

- Step Eliminator: Suggests unnecessary steps based on skip patterns
- Automation Opportunity Finder: Flags steps that could be automated
- Template Generator: Converts successful workflow into reusable template
- Phase Merger: Suggests consolidating similar or sequential phases

### Missing Features -- Workflows

#### Workflow Accordion Inbox Pattern

- Top-level category rows (Operations, Pipeline, Work Order, Marketing)
- Click category expands to show active workflows
- Click workflow slides open center panel with phase details
- Horizontal phase blocks (not vertical nested hell)
- Collapsed mode shows: progress %, owner, health status, due date
- Zero navigation overhead—inbox-like familiarity

#### Workflow Branching Logic

- Conditional paths based on step outcomes
- Parallel execution tracks that merge at gates
- Loop-back capabilities for iterative processes
- Skip conditions for optional steps

---

## NOTES DOMAIN STRAGGLERS

### Notes: Domain-Specific AI Tools

#### Note Intelligence

- Auto-Title Generator: Creates descriptive title from first line or content analysis
- Tag Suggester: Recommends tags based on content and similar notes
- Entity Extractor: Identifies contacts, dates, tasks, projects mentioned in note
- Structure Proposer: Suggests headings and organization for long notes
- Duplicate Detector: Warns when creating note similar to existing ones

#### Note Evolution AI

- Task Identifier: Highlights action items and offers task creation
- Contact Creator: Detects email/phone and offers contact creation
- Document Promoter: Suggests when note should become formal document
- Meeting Note Formatter: Auto-applies meeting note structure when detected
- Knowledge Article Suggester: Flags notes valuable as team knowledge base articles

#### Note Summarization

- TL;DR Generator: Creates 2-3 sentence summary of long notes
- Highlight Extractor: Pulls key points and insights
- Action Item List: Generates checklist of to-dos from note
- Decision Summary: Highlights decisions made and rationale

### Missing Features -- Notes

#### Voice Memo Integration

- Record voice memo attached to note
- AI transcription with speaker identification
- Timestamp navigation (jump to specific moments)
- Voice playback synced with text highlight

#### Note Linking Graph

- Visual graph of note relationships
- Backlinks panel showing notes that reference current note
- Graph navigation to explore knowledge connections
- Orphaned notes detector (notes with no links)

---

## DOCUMENTS (FLUIDDOC) DOMAIN STRAGGLERS

### Domain-Specific AI Tools

#### Canvas AI Generation Modes

- Quick Mode: Rapid draft generation ("Make me a project timeline")
- Detailed Mode: Comprehensive creation with data fusion ("Create proposal for Jane including meeting notes and pricing")
- Transform Mode: Convert existing content ("Turn meeting note into client report")
- Interactive Refinement: Section-by-section regeneration with user guidance

#### Intelligent Morphing Detection

- Intent Recognizer: Detects user intent from typing patterns
- Table Expander: Automatically converts `| Col | Col |` into structured table
- Spreadsheet Promoter: Offers Excel mode when table complexity increases
- Meeting Note Detector: Recognizes meeting structure and applies template
- Presentation Converter: Suggests slide mode when content is outline-structured

#### Context-Aware Linkification

- Phone Number Handler: Converts to tap-to-call/SMS links (uses OS handlers)
- Email Linkifier: Creates mailto: links automatically
- @Mention Linker: Links to contact profiles in real-time
- Data Token Resolver: Pulls live data for {placeholders} like {last_month_revenue}
- URL Previewer: Fetches and displays preview cards for pasted links

#### AI Co-Authoring Tools

- Section Drafter: Generates content for specific sections on demand
- Consistency Checker: Flags inconsistent names, dates, or terminology
- Tone Adjuster: Rewrites sections for different audiences (formal/casual/technical)
- Grammar & Clarity AI: Suggests improvements without being overbearing
- Summary Generator: Creates executive summary from full document

### Missing Features -- Documents

#### Multi-View Transformation

- Same Document, Multiple Views:
  - Internal view: Full context, tools, comments, version history
  - Client portal view: Polished, redacted, professional presentation
  - Team view: Collaboration tools, suggestions, @mentions visible
  - Print view: Optimized layout for PDF export

#### FluidDoc Expansion States

- Table → Spreadsheet: Full Excel-like interface with formulas, charts
- List → Kanban: Task lists transform into board view
- Outline → Presentation: Headings become slide titles
- Paragraph → Canvas: Text becomes spatial blocks for brainstorming

#### Document Intelligence

- Stale data warnings (linked data hasn't updated recently)
- Broken link detection (linked contact/task deleted)
- Approval status badges (who approved, when, what version)
- Reading time estimates based on content length

---

## FILES DOMAIN STRAGGLERS

### Files: Domain-Specific AI Tools

#### File Intelligence

- Auto-Tagger: Suggests tags from file content analysis (OCR for images/PDFs)
- Duplicate Detector: Flags files that are duplicates or near-duplicates
- Orphan Finder: Identifies files not linked to any contact/project/task
- Sensitivity Detector: Flags potentially sensitive files before sharing
- Version Recognizer: Groups file versions together (Proposal_v1, Proposal_v2)

#### Content Extraction AI

- PDF Text Extractor: Makes PDFs searchable via full-text indexing
- Image OCR: Extracts text from images and scanned documents
- Metadata Enricher: Pulls metadata from files (author, created date, keywords)
- Preview Generator: Creates thumbnails for visual browsing
- File Summarizer: Generates summary of document contents

### Missing Features -- Files

#### File Relationship Mapping

- Files can reference other files (supersedes, replaces, relates to)
- Visual dependency graph for complex file relationships
- "Superseded" badge on older versions
- Download "current version" always gets latest

---

## INVOICES DOMAIN STRAGGLERS

### Invoices: Domain-Specific AI Tools

#### Invoice Intelligence

- Line Item Suggester: Proposes invoice lines from completed tasks with time tracking
- Payment Predictor: Estimates payment timing based on client history
- Tax Calculator: Auto-applies correct tax rates based on client jurisdiction
- Discount Optimizer: Suggests discounts based on client relationship and past deals
- Inconsistency Detector: Flags mismatched rates, quantities, or totals

#### Collection Management AI

- Reminder Drafter: Generates escalating reminder emails (gentle → firm → urgent)
- Payment Plan Suggester: Proposes payment terms for large invoices
- Risk Scorer: Flags invoices at high risk of non-payment
- Follow-Up Scheduler: Optimal timing for overdue invoice follow-ups

### Missing Features -- Invoices

#### Invoice Health Dashboard

- Aging report (30/60/90 days overdue)
- Collection efficiency metrics
- Client payment behavior patterns
- Cash flow forecasting based on outstanding invoices

#### Recurring Invoice Automation

- Auto-generation on schedule (monthly retainers, subscriptions)
- Price adjustment workflows (annual increases)
- Pause/resume recurring invoices
- Prorated invoicing for mid-cycle changes

---

## WIZARD DOMAIN STRAGGLERS

### Wizard: Domain-Specific AI Tools

#### Wizard Intelligence

- Question Pre-Filler: Populates answers from existing data and context
- Smart Skip Logic: Hides irrelevant questions based on prior answers
- Validation Assistant: Friendly error messages with correction guidance
- Outcome Previewer: Shows what will be created before final confirmation
- Template Matcher: Suggests wizard templates based on user goal

#### Wizard Optimization AI

- Friction Detector: Identifies steps where users commonly abandon
- Question Simplifier: Suggests simpler phrasing for confusing questions
- Default Improver: Recommends better defaults based on completion patterns
- Path Optimizer: Suggests question reordering for better flow

### Missing Features -- Wizard

#### Wizard Builder for Admins

- Visual wizard designer with drag-and-drop question builder
- Conditional logic designer (if answer A, show question B)
- Template variable system for dynamic content
- Test mode to preview wizard before publishing
- Analytics dashboard showing completion rates per step

#### Progressive Wizard Pattern

- Start minimal (3 questions); add complexity only when needed
- "Advanced Options" collapsible sections
- Resume incomplete wizards from sidebar shelf
- Save partial progress automatically

---

## CLIENT PORTAL DOMAIN STRAGGLERS

### Client Portal: Domain-Specific AI Tools

#### Portal Intelligence

- Personalization Engine: Tailors portal home based on client activity and preferences
- Nudge Generator: Creates gentle prompts for required client actions
- Activity Predictor: Anticipates client needs based on project phase
- Confusion Detector: Flags when client repeatedly accesses help or support

#### Client Communication AI

- Message Tone Optimizer: Ensures client-facing messages are warm and clear
- Status Update Generator: Auto-creates project status summaries for clients
- FAQ Generator: Builds knowledge base from common client questions
- Response Time Predictor: Estimates when client likely to respond to request

### Missing Features -- Client Portal

#### Portal Onboarding Wizard

- First-login guided tour optimized for clients (not internal users)
- Interactive walkthrough of key features
- Skippable with "remind me later" option
- Contextual help tooltips on first use

#### Portal Analytics for Internal Team

- Client engagement metrics (login frequency, time spent, pages viewed)
- Feature usage heatmaps (what do clients actually use?)
- Confusion signals (repeated help access, support tickets)
- Adoption trends over time

---

## NOTIFICATIONS DOMAIN STRAGGLERS

### Domain-Specific AI Tools -- Notifications

#### Notification Intelligence

- Urgency Classifier: Determines which notifications need immediate attention vs. digest
- Batch Optimizer: Groups related notifications to reduce noise
- Timing Optimizer: Learns best notification times per user
- Channel Selector: Suggests optimal channel (in-app, email, push) per notification type
- Snooze Suggester: Proposes snooze duration based on notification type

#### Notification Tuning AI

- Preference Learner: Adjusts notification settings based on user behavior (dismissals, interactions)
- Noise Detector: Alerts when notification volume exceeds healthy threshold
- Actionability Scorer: Prioritizes notifications user can act on vs. FYI only

### Missing Features -- Notifications

#### Notification Digest Customization

- Build your own digest schedule (e.g., "tasks at 8am, emails at noon, everything else at 5pm")
- Digest preview before sending
- Instant send button for urgent digest delivery
- Digest pause during vacation/OOO

#### Rich Notification Actions

- Quick actions directly in notification (Complete task, Archive email, Approve doc)
- No need to open full app for simple actions
- Optimistic UI updates with background sync

---

## COMMS (DOCK CHAT) DOMAIN STRAGGLERS

### Domain-Specific AI Tools -- Comms

#### Chat Intelligence

- Intent Detector: Identifies user intent from natural language ("show me tasks" vs. "create task")
- Context Carrier: Maintains conversation context across sessions
- Clarification Asker: Prompts for missing info when request is ambiguous
- Memory Tagger: Captures explicit memory statements ("Remember: Sarah prefers calls")
- Suggestion Ranker: Orders AI suggestions by relevance and user patterns

#### Council Visibility Controls

- Reasoning Toggle: Show/hide Council deliberation per user preference
- Persona Highlighting: Color-coded Council member contributions (Strategist/Operator/Signal)
- Depth Indicator: Visual cue when Super Think mode engaged
- Confidence Display: Shows AI confidence levels with explanations

### Missing Features -- Comms

#### Chat History Management

- Thread collapsing to reduce clutter
- Search within chat history
- Export chat log for reference or compliance
- Pin important exchanges
- Jump to date function for long chat histories

#### Voice Interaction

- Voice-to-text input (hands-free)
- Text-to-speech AI responses (optional)
- Voice command shortcuts
- Conversation continuity across voice and text modes

---

## CROSS-DOMAIN AI TOOLS

### Universal AI Capabilities Available to All Domains

#### Semantic Search Engine

- Natural language queries work across all object types
- Understands intent beyond keyword matching
- Returns ranked results with relevance scoring
- Surfaces related objects even if not exact matches

#### Smart Linking Engine

- Detects entity mentions across system (contacts, projects, tasks, documents)
- Auto-creates bidirectional links
- Suggests missing links based on context
- Broken link repair suggestions

#### Predictive Typing & Autocomplete

- Context-aware autocomplete for all text fields
- Learns user's vocabulary and phrasing
- Suggests completions based on similar past entries
- Multi-language support

#### Universal Summarizer

- Summarizes any object type (email thread, document, workflow, note)
- Adjustable summary length (brief, moderate, detailed)
- Highlights key decisions, commitments, and outcomes
- Extracts action items automatically

#### Anomaly Detector

- Flags unusual patterns (e.g., invoice amount drastically different from estimate)
- Alerts on missing expected actions (no follow-up after 2 weeks)
- Detects inconsistencies across linked objects
- Suggests corrections or investigations

#### Data Freshness Tracker

- Warns when linked data becomes stale
- Suggests refreshing or verifying old information
- Highlights outdated relationships or inactive workflows

---

## IMPLEMENTATION GUARDRAILS

### AI Tool Deployment Principles

- All AI features default to "suggest, don't decide"
- User can enable autopilot per tool with confidence thresholds
- Transparency: always show why AI suggested something
- Feedback loops: thumbs up/down to improve suggestions
- Graceful degradation: system works without AI if models unavailable

### Performance Considerations

- AI tools run asynchronously; never block UI
- Cached predictions for common patterns (instant responses)
- Progressive enhancement: basic features work; AI enhances
- Model fallbacks: if primary AI unavailable, use simpler heuristics

### Privacy & Compliance

- AI operations respect data visibility rules (no cross-tenant leakage)
- Sensitive data redaction before external AI calls
- Audit logs for all AI-driven actions
- User control over AI data usage (opt-out options)

---

## CONCLUSION

These stragglers represent the final layer of domain fidelity—the AI intelligence that makes each domain feel magical rather than merely functional. By building these tools as domain-specific enhancements (not generic Copilot features), each part of the system becomes progressively smarter while maintaining its core purpose and clarity.

The key insight: **AI should feel like each domain got smarter, not like a chatbot was bolted onto the side.** Domain AI tools are contextual, specific, and purpose-built for the workflows they enhance.
