# Files, Invoices & Wizard Domains - Feature PRD (Path/Stack Agnostic)

## FILES DOMAIN

### Files Domain: Purpose & Scope

TheFiles domain provides centralized storage, organization, and intelligent linking of all business files—from client deliverables to internal documents, contracts to invoices. It treats files as first-class objects that integrate seamlessly with contacts, projects, tasks, and workflows.

### Files Domain: Core Philosophy

Files aren't trapped in folder hierarchies—they're living assets connected to business context. Every file knows why it exists, who it's for, and where it fits in your workflow. The system eliminates "where did I save that?" anxiety through smart linking and powerful search.

### Primary Features

#### Universal File Sharing

- Upload any file type: documents, images, videos, spreadsheets, PDFs, audio
- Drag-and-drop anywhere in system
- Email attachments auto-saved
- Mobile photo/document capture
- Cloud import (Dropbox, Google Drive, OneDrive)

#### Smart Tagging & Metadata

- AI suggests tags based on file content and name
- Custom metadata fields per file type
- Version tracking for file iterations
- File relationships (supersedes, relates to, replaces)

#### File Previews

- Inline preview for common types (PDFs, images, videos, docs)
- No download required for quick viewing
- Annotation and markup tools
- Generate thumbnails for visual browsing

#### Search & Discovery

- Full-text search inside PDF and document content
- Filter by type, owner, date, project, contact
- "Orphaned files" smart list for cleanup
- Recently accessed and frequently used files surfaced

#### Sharing & Permissions

- Share files with team members (view, edit)
- Generate shareable links with expiration
- Portal publishing for client access
- Redact sensitive sections before sharing
- Download tracking and audit logs

#### Integration Points

- Tasks: attach deliverables to tasks
- Documents: embed files in FluidDocs
- Workflows: required files at workflow gates
- Invoices: attach receipts and backup documentation
- Email: send files as email attachments
- Portal: publish files for client access

---

## INVOICES DOMAIN

### Invoices Domain: Purpose & Scope

The Invoices domain manages billing, payment tracking, and revenue collection. It auto-generates invoices from completed work, tracks payment status, sends reminders, and integrates with accounting systems—all while maintaining a polished, professional client-facing experience.

### Invoices Domain: Core Philosophy

Invoicing shouldn't be a separate chore disconnected from the work. When tasks complete or workflows finish, billing should happen naturally. The system makes it effortless to get paid while keeping detailed records for financial management and compliance.

### Primary Features -

#### Invoice Generation

- Auto-generate from completed tasks (time tracking + rate)
- Auto-generate from workflow completion (milestone billing)
- Manual creation from template
- Recurring invoices (retainers, subscriptions)
- AI-suggested line items from work history

#### Invoice Structure

- Professional branded templates
- Line items with descriptions, quantities, rates
- Subtotals, taxes, discounts, totals
- Payment terms and due dates
- Notes and payment instructions
- Attachment support (receipts, backup docs)

#### Tax & Compliance

- Configurable tax rates by jurisdiction
- Tax ID and business registration details
- Compliance-ready formatting for audits
- Invoice numbering schemes (auto-generated, custom)
- Retention policies for record-keeping

#### Client-Facing Experience

- Professional PDF generation
- Portal-based invoice viewing (no login required option)
- Online payment integration (Stripe, PayPal, bank transfer)
- One-click payment from invoice view
- Payment confirmation emails

#### Payment Tracking

- Status: Draft, Sent, Viewed, Overdue, Paid, Voided
- Payment received logging
- Partial payment support
- Outstanding balance calculations
- Aging reports (30/60/90 days)

#### Reminders & Follow-Ups

- Auto-reminders for approaching due dates
- Escalating reminders for overdue invoices
- AI-drafted reminder emails (gentle → firm)
- Manual follow-up tracking
- Payment plan negotiation tools

#### Reporting & Analytics

- Revenue by client, project, time period
- Outstanding receivables summary
- Payment velocity (time from invoice to payment)
- Profitability analysis (revenue vs. costs)
- Forecast cash flow based on pipeline

#### Integration Points -

- Tasks: billable work converted to line items
- Workflows: completion triggers invoice generation
- Contacts: client billing information auto-filled
- Documents: invoice rendered as FluidDoc for editing
- Email: send invoices via email with tracking
- Accounting: export to QuickBooks, Xero, FreshBooks
- Portal: client self-service invoice access

### User Journey: Invoice from Workflow

1. User completes "Website Redesign" workflow for Acme Corp
2. System detects billable work order workflow finished
3. Prompts: "Generate invoice for this project?"
4. User confirms; invoice draft created
5. AI pulls line items from completed tasks with time tracked
6. Applies hourly rate from client contract
7. Adds tax based on client jurisdiction
8. User reviews invoice; adjusts one line item description
9. Clicks "Send Invoice"
10. Professional PDF generated and emailed to client
11. Client views invoice in portal; pays online
12. Payment recorded; user notified; invoice marked Paid

---

## WIZARD DOMAIN

### Purpose & Scope

The Wizard domain provides guided, step-by-step experiences for complex setups and recurring processes. It eliminates configuration paralysis by asking smart questions in sequence, then auto-generating the necessary structure (workflows, projects, forms, settings). Wizards make the complex feel simple.

### Core Philosophy

Don't make users build complex structures from scratch. Ask 4–6 thoughtful questions, learn their intent, then generate 80% of the setup automatically. Wizards transform intimidating tasks into conversational interactions that feel helpful, not bureaucratic.

### Primary Features =

#### Wizard Types

#### Onboarding Wizards

- New user workspace setup
- Client onboarding (intake, account setup, first project)
- Team member onboarding (access, training, first tasks)
- New service offering setup (workflow templates, pricing)

#### Project Setup Wizards

- New project initialization (client, scope, team, timeline)
- Campaign planning (goals, audience, channels, calendar)
- Product launch coordination (phases, stakeholders, deliverables)

#### Configuration Wizards

- Workspace settings (branding, permissions, integrations)
- Workflow template builder (steps, owners, automation)
- Form builder (fields, logic, validation)
- Automation rule designer (triggers, conditions, actions)

#### Recurring Process Wizards

- Monthly reporting (data sources, recipients, schedule)
- Quarterly review (goals, retrospective, planning)
- Annual audit (compliance, documentation, approvals)

#### Wizard Step Pattern

Each wizard follows consistent UX:

1. **Welcome/Context**: Explain what wizard does and why
2. **Smart Questions** (3–6 steps): Collect essential info with defaults
3. **Preview**: Show what will be created; allow edits
4. **Confirmation**: Execute with progress indicator
5. **Summary**: Display created objects with links; suggest next steps

#### Question Types

- Text input (names, descriptions)
- Dropdowns (select from options)
- Multi-select (choose multiple items)
- Date pickers
- Participant selection (team members, contacts)
- File uploads
- Conditional questions (based on prior answers)

#### AI-Assisted Wizards

- AI suggests answers based on context and history
- Pre-fills fields from existing data (contacts, projects, templates)
- Detects patterns from prior wizard runs
- Offers to skip questions with high-confidence defaults

#### Wizard Outputs

Wizards generate multiple objects in one flow:

- Workflows with phases and tasks
- Projects with team assignments
- Document templates populated with details
- Calendar events and reminders
- Automation rules configured
- Portal access provisioned
- Notification schedules set

#### Wizard Templates & Customization

- Admins build custom wizards via visual builder
- Wizard templates saved and shared across workspace
- Version control for wizard definitions
- Analytics on wizard completion rates and friction points

#### Progressive Wizards

- Start minimal; add complexity only when needed
- "Advanced Options" expand for power users
- Skip optional steps without breaking flow
- Resume incomplete wizards from sidebar

#### Wizard Guardrails

- Validation at each step prevents bad data
- Helpful error messages with correction guidance
- Can't proceed until required fields complete
- "Why do you ask?" explanations for clarity
- Smart defaults reduce cognitive load

#### Integration Points =

- Workflows: generate entire workflow structures
- Tasks: create task lists from wizard inputs
- Contacts: set up client profiles and relationships
- Documents: populate templates with wizard data
- Portal: configure client portal access and permissions
- Notifications: set up alerts and reminders

### User Journey: Client Onboarding Wizard

1. Sales team closes deal with new client "Acme Corp"
2. User clicks "Onboard New Client" wizard
3. **Step 1**: "Who's your primary contact?" → Select Sarah Chen from contacts
4. **Step 2**: "What services are they purchasing?" → Select "Website Design" (AI suggests based on deal)
5. **Step 3**: "When should we start?" → Pick date; AI suggests 1 week out
6. **Step 4**: "Who's leading this project?" → Assign to team member Dave
7. **Step 5**: "Any special requirements?" → Optional text field; user adds "Rush delivery needed"
8. Preview screen shows: Client contact created, Work order workflow generated (5 phases, 18 tasks), Portal access provisioned, Kickoff meeting scheduled
9. User confirms; wizard executes
10. Summary: "Acme Corp onboarded successfully! Next step: Dave, schedule kickoff call"
11. All objects linked; team notified; client receives welcome email

### User Journey: Workflow Template Builder Wizard

1. Admin wants to standardize "Client Offboarding" process
2. Opens Wizard Builder; selects "Create Workflow Template"
3. **Step 1**: "What's the workflow name?" → "Client Offboarding"
4. **Step 2**: "How many phases?" → Selects 3 (Transition, Documentation, Closure)
5. For each phase, wizard asks: "What steps are needed?"
6. Admin adds steps with descriptions, owners (role-based placeholders), durations
7. **Step 3**: "Any approval gates?" → Adds gate after Documentation phase
8. **Step 4**: "Automation rules?" → Set notification when workflow starts
9. Preview shows full workflow structure as visual diagram
10. Admin confirms; template saved to library
11. Team can now apply "Client Offboarding" template with one click

---

## Cross-Domain Patterns

### Circular Evolution

- Files → Documents: View file in FluidDoc editor
- Invoices → Tasks: Overdue invoice creates follow-up task
- Wizard → Workflow: Wizard generates workflow instances

### AI Enhancement

All three domains benefit from Copilot intelligence:

- Files: auto-tagging, content extraction, duplicate detection
- Invoices: line item suggestions, reminder drafting, payment prediction
- Wizards: answer pre-filling, validation assistance, outcome optimization

### Mobile Experience

- Files: camera capture, quick upload, preview on device
- Invoices: mobile-friendly payment flow, tap-to-pay
- Wizards: touch-optimized step navigation, swipe between steps

### Search & Discoverability

- Unified Crown search finds files, invoices, and wizard history
- Smart lists surface relevant items (unpaid invoices, recent files, incomplete wizards)

## Conclusion

The Files, Invoices, and Wizard domains work together to handle the operational backbone of the business—storing assets, collecting revenue, and guiding complex processes. By integrating deeply with contacts, tasks, and workflows, they ensure that administrative work feels like a natural part of delivering value rather than isolated busywork.
