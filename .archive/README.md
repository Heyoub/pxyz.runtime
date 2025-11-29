# Domain Specifications (PRDs)

This folder contains Product Requirements Documents (PRDs) for all ForgeStack domains. These are path/stack-agnostic specifications that describe the business logic, features, and user experience for each domain.

## Domain PRDs (9 files)

### Core Business Domains
- **[Domain_PRD_Contacts.md](Domain_PRD_Contacts.md)** - Relationship intelligence and contact management
- **[Domain_PRD_Tasks.md](Domain_PRD_Tasks.md)** - Task management with multiple views (Kanban, Timeline, Priority Grid)
- **[Domain_PRD_Workflows.md](Domain_PRD_Workflows.md)** - Workflow orchestration and automation
- **[Domain_PRD_Email.md](Domain_PRD_Email.md)** - Unified inbox and email intelligence

### Content & Knowledge
- **[Domain_PRD_Documents.md](Domain_PRD_Documents.md)** - FluidDoc multi-mode document editing
- **[Domain_PRD_Notes.md](Domain_PRD_Notes.md)** - Voice capture and entity extraction

### Files & Invoicing
- **[Domain_PRD_Files_Invoices_Wizard.md](Domain_PRD_Files_Invoices_Wizard.md)** - File management, invoicing, and wizard flows

### Communication & Portal
- **[Domain_PRD_Portal_Notifications_Comms.md](Domain_PRD_Portal_Notifications_Comms.md)** - Client portal, notifications, and communications

### AI & Tools
- **[Domain_Stragglers_AI_Tools.md](Domain_Stragglers_AI_Tools.md)** - AI services, tools, and integrations

## How to Use These PRDs

### For Developers
1. Read the PRD to understand business requirements
2. Check the corresponding Effects Program in `app/server/logic/programs/`
3. Implement features following PXYZ architecture rules
4. Ensure type safety with branded types from P.ts

### For Product/Design
1. PRDs describe the "what" and "why" (not the "how")
2. Use these to understand user needs and feature scope
3. Update PRDs when requirements change
4. Keep PRDs path/stack-agnostic (no implementation details)

### For AI Agents
When implementing a domain:
1. Read the PRD thoroughly
2. Follow **[../../PXYZ_ARCHITECTURE_RULES.md](../../PXYZ_ARCHITECTURE_RULES.md)**
3. Use type narrowing, NOT type creation
4. Domain logic = pure functions + rules engines
5. Every operation gets PXYZ coordinates

## PRD Structure

Each PRD typically includes:
- **Purpose & Scope** - What problem does this domain solve?
- **Core Philosophy** - Guiding principles
- **Primary Features** - Main functionality
- **User Experience** - How users interact
- **Intelligence & Automation** - AI-powered features
- **Integration Points** - How it connects to other domains

## Implementation Status

To see implementation status for each domain, check:
- Effects Programs: `app/server/logic/programs/*EffectsProgram.ts`
- Frontend Components: `app/src/features/domain/*/`
- Command Handlers: `app/server/io/CommandHandlers.ts`

## Contributing

When updating PRDs:
1. Keep them path/stack-agnostic (no code, no tech stack)
2. Focus on user value and business logic
3. Describe the "what" and "why", not the "how"
4. Update corresponding Effects Program if requirements change
5. Maintain consistency with PXYZ architecture principles
