# Client Portal, Notifications & Comms Domains - Feature PRD (Path/Stack Agnostic)

## CLIENT PORTAL DOMAIN

### Purpose & Scope

The Client Portal provides an external-facing, mobile-optimized interface where clients interact with your business. It feels like a consumer app (think iPhone, not enterprise software)—clean, intuitive, and idiotproofed. Clients access project status, approve deliverables, upload files, message your team, and view invoices without navigating complex interfaces.

### Core Philosophy

The portal isn't a dumbed-down version of your internal system—it's a purpose-built client experience. Every surface is designed for clarity and confidence. Clients shouldn't feel lost or confused; they should feel informed and empowered. Strategic friction prompts confirm critical actions and enforce best practices without blocking momentum.

### Primary Features

**Portal Home / Dashboard**
Client lands on personalized, mobile-optimized home:

- **Welcome Message**: Personalized greeting ("Hi Sarah, here's what's happening with your Website Redesign project")
- **Status Summary**: Current phase, progress percentage, upcoming milestones
- **Action Items**: What requires client attention (approve design, upload logo, review invoice)
- **Recent Activity**: Timeline of recent updates from your team
- **Quick Links**: Jump to Messages, Documents, Invoices, Support

**Project Status & Progress**
Clients see transparent project visibility:

- **Phase View**: Visual progress through project phases (Discovery → Design → Build → Launch)
- **Milestone Tracking**: Key deliverables with completion status
- **Timeline**: Expected completion dates with confidence indicators
- **Activity Feed**: Updates from your team with timestamps
- **Team Members**: Who's working on what with contact info

**Secure Messaging**
Slack-like threaded conversations with your team:

- **Channels per Project**: Each project has dedicated message thread
- **File Attachments**: Share files inline with messages
- **Notifications**: Email alerts for new messages
- **Read Receipts**: See when team read your message
- **@Mentions**: Tag specific team members
- **Mobile Push**: Real-time notifications on mobile devices

**Document Library**
Client access to shared files and deliverables:

- **Two Tabs**: "Sent by Team" and "Upload Documents"
- **Preview**: View PDFs, images, documents without downloading
- **Download**: One-click download of any file
- **Upload**: Drag-and-drop file uploads with progress bars
- **Approval Workflow**: Approve/request changes on deliverables
- **Version History**: See document revisions with timestamps

**Invoices & Payments**
Self-service billing access:

- **Invoice List**: All invoices (paid, unpaid, overdue) with status badges
- **Invoice Detail**: Line items, payment terms, total due
- **One-Click Payment**: Integrated Stripe/PayPal for instant payment
- **Payment History**: Receipt log with downloadable PDFs
- **Payment Plan**: Request payment plan for large invoices
- **Auto-Pay Option**: Save payment method for recurring invoices

**Support & Help**
Client self-service resources:

- **Knowledge Base**: FAQ articles organized by topic
- **Ticket System**: Submit support requests with priority levels
- **Ticket Tracking**: View open and closed tickets with responses
- **Live Chat** (if enabled): Real-time chat with support team
- **Contact Directory**: Team member contact cards with roles

**Settings & Profile**
Client manages their portal experience:

- **Notification Preferences**: Email frequency, mobile push settings
- **Password & Security**: Change password, two-factor auth
- **Team Members**: Add additional users from their organization
- **Billing Information**: Update payment methods and addresses

### Portal UX Principles

- **Mobile-First Design**

- Every screen optimized for phone and tablet
- Bottom-focused navigation for thumb reachability
- Large tap targets (minimum 44px)
- Swipe gestures for common actions
- Responsive layouts collapse gracefully

- **Idiotproofing**

- One primary action per screen (clear what to do next)
- Minimal choices to reduce decision fatigue
- Progress indicators reassure during multi-step flows
- Helpful microcopy explains "why" at friction points
- Errors prevented rather than corrected

- **Strategic Friction for Compliance**

- Gentle confirmation prompts for critical actions:

- "You're about to approve the final design. This will move to development. Confirm?"
- "Uploading contracts? Make sure signatures are visible and pages complete."
- Prevents accidental approvals and incomplete uploads
- Friction is informative, not bureaucratic

- **Zero Clutter**

- Clean, white-space-heavy layouts
- Hide complexity; show only what's needed now
- Progressive disclosure for advanced options
- Icons and visual cues over text labels
- Consistent navigation patterns

- **Personalization**

- Client name usage throughout ("Sarah's Projects")
- Tailored recommendations based on client activity
- Custom branding (your logo, colors) for white-label feel

### Portal Security & Access

- **Authentication**

- Email + password with password strength requirements
- Magic link login (passwordless email link)
- Two-factor authentication (SMS or authenticator app)
- Social login (Google, Microsoft) for convenience

- **Permission Scoping**

- Clients see only their own projects and data
- No cross-client data leakage
- Role-based access within client organization (owner vs. viewer)
- Temporary guest access for external reviewers

- **Audit & Compliance**

- All client actions logged (viewed, uploaded, approved, paid)
- Exportable audit trail for compliance
- Data retention policies enforced
- GDPR/CCPA compliant data export and deletion

### Integration Points

- **With Workflows Domain**

- Portal actions advance internal workflows (approval clears gate)
- Workflow milestones trigger portal notifications
- Client uploads stored and linked to workflow steps

- **With Documents Domain**

- Documents published to portal with permission control
- Client-approved documents flow back to internal document library
- Redaction before portal publishing

- **With Invoices Domain**

- Invoices appear in portal automatically when sent
- Payment via portal updates internal invoice status
- Receipt delivery via portal and email

- **With Email Domain**

- Portal messages route through email system
- Client can reply to portal notifications via email
- Email-to-portal threading maintains conversation continuity

- **With Notifications Domain**

- Portal activity generates notifications for internal team
- Client receives notifications for team updates

- **With Files Domain**

- Client file uploads saved to centralized file storage
- Internal team files published to portal selectively

### User Journey: Client Approval Flow

1. Internal team completes website design mockups
2. Publishes to portal: "Website Design v2" document
3. Client "Sarah" receives email notification: "New design ready for review"
4. Sarah opens portal on phone; sees notification badge
5. Taps "Documents" → "Sent by Team"
6. Views design mockup with inline preview
7. Loves it! Taps "Approve" button
8. Strategic friction prompt: "Approving this design moves to development. Ready to proceed?"
9. Sarah confirms approval
10. Internal workflow gate clears; development phase starts
11. Sarah receives confirmation: "Design approved! Development starting next week."
12. Internal team notified; project advances automatically

---

## NOTIFICATIONS DOMAIN

### Purpose & Scope -

The Notifications domain keeps users informed of important events, actions, and changes across the system. It balances urgency with calm, ensuring users stay aware without drowning in alerts. Notifications respect user preferences, quiet hours, and context to deliver the right information at the right time.

### Core Philosophy -

Notifications should be helpful signals, not distractions. The system assumes users are busy and defaults to digest modes rather than real-time bombardment. Critical items get immediate attention; routine updates batch into summaries. Users control frequency, channels, and sensitivity.

### Primary Features -

- **Notification Types**

- **Transient (Toast)**

- Appear briefly in corner; auto-dismiss after 3–5 seconds
- Low-stakes confirmations ("Task marked complete," "File uploaded")
- Non-blocking; user can continue working
- Click to jump to related object

- **Persistent (Shelf)**

- Remain in notification center until acknowledged
- Important events requiring awareness ("Invoice overdue," "Approval needed")
- Badge count on notification icon
- Swipe/click to dismiss or act

- **Contextual (Inline)**

- Appear in relevant UI areas (e.g., overdue task badge in task list)
- Glow states on objects needing attention
- In-context prompts ("This contact hasn't been touched in 60 days")

- **Email Digests**

- Daily or weekly summary of activity
- Grouped by type (tasks, emails, approvals, updates)
- Smart prioritization (urgent first)
- One-click actions from email

- **Mobile Push**

- Real-time alerts on phone
- Critical only by default (mentions, approvals, deadlines)
- Rich notifications with quick actions
- Respect quiet hours

### Notification Triggers -

- **Task-Related**

- Task assigned to you
- Task due soon (configurable threshold)
- Task overdue
- Task blocked or unblocked
- Mentioned in task comment

- **Email-Related**

- New email in focus views (Needs Reply, Waiting On Me)
- Email thread you're watching has update
- High-priority email detected

- **Workflow-Related**

- Workflow step assigned to you
- Workflow milestone reached
- Workflow blocked or at risk
- Approval gate requires your review

- **Contact-Related**

- Contact relationship cooling (no touch in 30+ days)
- Contact mentioned you in communication
- Contact activity needing follow-up

- **Invoice-Related**

- Invoice sent to client
- Invoice viewed by client
- Invoice payment received
- Invoice approaching due date
- Invoice overdue

- **Document-Related**

- Document shared with you
- Comment on document you're watching
- Document approval requested
- Document version updated

- **Portal-Related**

- Client uploaded file
- Client sent message
- Client approved deliverable
- Client made payment

- **AI-Suggested**

- Copilot detects pattern needing attention
- Recommended action based on activity
- Risk flag on project or relationship

### Notification Settings & Control -

- **Global Preferences**

- Notification channels (in-app, email, mobile push)
- Digest frequency (real-time, hourly, daily, weekly)
- Quiet hours (no notifications during sleep/off-hours)
- Do Not Disturb mode (pause all except critical)

- **Per-Type Preferences**

- Customize behavior for each notification type
- Example: Tasks real-time, emails daily digest, workflows weekly
- Critical override (some notifications always immediate)

- **Smart Batching**

- Group similar notifications ("5 new tasks assigned to you")
- Consolidate updates ("3 workflows advanced today")
- Reduce noise without losing information

- **Snooze & Defer**

- Snooze notification to reappear later
- "Remind me in 1 hour / tomorrow / next week"
- Snoozed items tracked in notification shelf

### Notification UI -

- **Notification Center (In-App)**

- Panel accessible from Crown or icon click
- Grouped by type or chronological
- Filter by read/unread, type, date
- Mark all as read option
- Notification history (last 30 days)

- **Badge Counts**

- Unread count on notification icon
- Per-view badges (e.g., "3" on Tasks view if 3 overdue)
- Clear indicators without overwhelming

- **Notification Card Design**

- Icon indicating type (task, email, workflow, etc.)
- Short title and description
- Timestamp (relative: "2 hours ago")
- Quick actions (Mark Done, View, Snooze, Dismiss)
- Click card to navigate to source object

### Integration Points -

- **All Domains**
Every domain can generate notifications:

- Tasks: assignments, due dates, completions
- Workflows: step changes, approvals, milestones
- Email: new threads, replies, mentions
- Documents: shares, comments, approvals
- Invoices: sent, paid, overdue
- Portal: client activity and messages
- Contacts: relationship health changes

- **With AI Copilot**

- AI suggests notification tuning based on user behavior
- "You dismiss all email notifications—should we switch to digest?"
- Learn optimal notification timing per user

### User Journey: Notification Flow

1. Client uploads contract to portal at 2 PM
2. System generates notification: "Client uploaded: Service Agreement.pdf"
3. User's settings: Portal notifications = real-time push
4. Mobile push notification appears on phone
5. User taps notification; opens app to portal view
6. Reviews uploaded contract; marks as "Reviewed"
7. Notification dismissed automatically
8. Internal task created: "Review and sign Service Agreement"
9. Task notification sent to legal team member
10. Legal team member receives notification; approves contract
11. Client receives portal notification: "Contract approved! Payment invoice sent."

---

## COMMS (DOCK CHAT) DOMAIN

### Purpose & Scope --

The Comms domain provides a threaded, persistent communication channel between users and the AI Copilot (and optionally team members). It lives in the Dock as an always-accessible sidebar chat, functioning like Slack but with AI intelligence baked in. It serves as both a workspace chat and an audit trail of AI interactions.

### Core Philosophy --

The Dock Chat is your conversation log with the AI assistant and your team. It preserves context across sessions, supports threaded discussions, and maintains a searchable history of decisions and interactions. It's not ephemeral chat—it's a permanent record that helps the AI remember your preferences and the team stay aligned.

### Primary Features --

- **Persistent Chat Interface**

- Dock panel opens chat sidebar (30–40% of screen width)
- Scrollable message history (infinite scroll backward)
- Message composer at bottom with rich text support
- Typing indicators when AI or team member responding
- Collapsible/expandable; doesn't block main work area

- **AI Copilot Thread**
Primary conversation with Matter AI assistant:

- Ask questions: "What tasks are due this week?"
- Request actions: "Draft email to Sarah about Q4 project"
- Get summaries: "Summarize my meeting notes from yesterday"
- Seek advice: "Should I follow up with David or wait?"
- Copilot responds with helpful answers, drafts, and suggestions

- **Threaded Conversations**

- Each major topic or request starts a thread
- Threads collapsible to reduce clutter
- Jump between threads without losing context
- Search within threads for specific exchanges

- **Council Reasoning Visibility** (Optional)
When AI uses "Super Think" mode, show deliberation:

- **Strategist** (blue): "We should prioritize the invoicing flow—it aligns with client expectations."
- **Operator** (gray): "Use existing template; auto-fill client data to save time."
- **Signal** (purple): "Keep tone friendly—client mentioned feeling overwhelmed last week."
- User sees how AI reached conclusion (builds trust)
- Toggle visibility on/off per user preference

- **Team Chat Channels** (Optional)
If enabled, Dock can include team communication:

- Project-specific channels
- Team-wide announcements
- Quick coordination without leaving workflow
- @mentions and notifications
- File sharing in chat

- **Message Types**

- **User Messages**: Questions, commands, thoughts
- **AI Responses**: Answers, drafts, summaries, suggestions
- **System Messages**: Automated updates ("Workflow completed," "Invoice paid")
- **Team Messages**: Colleague replies and coordination

- **AI Interaction History**

- All AI requests and responses logged
- Searchable archive of past conversations
- Filter by date, topic, or intent
- Export for review or compliance

- **Quick Actions from Chat**
AI responses include actionable buttons:

- "Create this task" (from AI task suggestion)
- "Send this email" (from AI draft)
- "Apply this change" (from AI recommendation)
- "Show me more" (expand AI suggestion)

- **Memory Tags & Context**

- AI references prior chat context ("As we discussed last week...")
- Memory tags captured from chat ("Remember: Sarah prefers calls over email")
- User can explicitly tell AI to remember facts
- AI recalls preferences and patterns from chat history

### Chat UX Patterns -

- **Calm, Conversational Interface**

- Messages appear without excessive animation
- Subtle typing indicators (no aggressive bouncing)
- Soft colors for different message types
- Readable font sizes; ample white space

- **Markdown Support**

- AI responses use markdown for formatting
- Code blocks, lists, bold, links supported
- User messages accept markdown shortcuts

- **Inline Previews**

- Links expand to preview cards
- Mentioned contacts show avatar and name
- Referenced tasks/docs show summary
- Click preview to jump to object

- **Voice Input** (Mobile)

- Voice-to-text for hands-free chat
- AI responds audibly (optional text-to-speech)
- Conversation continuity across voice and text

### Integration Points --

**With All Domains**
Chat can reference and act on any object:

- "Show me overdue tasks" → Task list
- "Summarize latest email from Sarah" → Email thread
- "Generate invoice for Acme Corp project" → Invoice creation
- "What's the status of Website Redesign workflow?" → Workflow summary

- **With AI Copilot**

- Primary interface for AI interaction
- Tool execution confirmations appear in chat
- AI explains reasoning and provides transparency
- Feedback (thumbs up/down) captured in chat

- **With Notifications Domain**

- Critical notifications appear as chat messages
- User can dismiss or act on notifications from chat
- Notification settings adjustable via chat command

### User Journey: Dock Chat Interaction

1. User opens Dock chat panel mid-morning
2. Types: "What's on my plate today?"
3. AI responds: "You have 8 tasks due today, 3 overdue, and 2 emails needing replies. Start with overdue tasks?"
4. User: "Yes, show me overdue"
5. AI lists 3 overdue tasks with quick action buttons
6. User clicks "Mark task 1 complete"
7. Task updates; AI confirms: "Task 'Review contract' marked done. 2 overdue remaining."
8. User: "Draft follow-up email to Sarah about Q4 budget"
9. AI (with Council reasoning visible):
   - Strategist: "Mention budget approval timeline—critical for her planning."
   - Operator: "Keep email concise; Sarah's inbox is busy."
   - Signal: "Friendly tone; she appreciated your last check-in."
10. AI generates email draft in chat
11. User reviews; clicks "Send this email"
12. Email sent; chat logs the exchange for future reference
13. User closes chat; continues work; chat context preserved for next session

---

## Cross-Domain Patterns --

- **Unified Experience**

- Portal, Notifications, and Comms work together for seamless communication
- Portal activity generates notifications; notifications appear in Comms
- AI responses in Comms can trigger portal updates

- **Mobile Optimization**
All three domains designed for mobile-first:

- Portal: touch-optimized, swipe gestures
- Notifications: rich push with quick actions
- Comms: voice input, minimal typing

- **AI Integration**

- Portal: AI drafts client-facing updates and messages
- Notifications: AI suggests optimal notification settings
- Comms: AI is primary chat participant with full context

## Conclusion

The Client Portal, Notifications, and Comms domains create a comprehensive communication layer—external clients stay informed via portal, users stay aware via notifications, and AI collaboration happens via Dock chat. Together, they ensure information flows smoothly while respecting attention and focus.
