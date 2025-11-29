# OMAR BUSINESS COUNCIL & PERSONALITY EXTRACTION - INVENTORY

> **Extraction Date**: 2025-11-29  
> **Files Analyzed**: 4 TypeScript files + 3 config.json files  
> **Total Source Code**: ~53KB, ~1,400 lines  
> **Target**: Pure PXYZ coordinate mappings

---

## 📋 FILE INVENTORY

| File | Size | Lines | Type | Purpose |
|------|------|-------|------|---------|
| BusinessCouncil.ts | 21KB | ~477 | Service | Council orchestration & synthesis |
| BusinessPersonality.ts | 14KB | ~300 | Service | Business context & personalization |
| CommunicationStyleManager.ts | 18KB | ~471 | Service | Style analysis & recommendations |
| config.json (council) | 3KB | ~180 | Config | Council member personalities |
| config.json (personality) | 4KB | ~220 | Config | Business context defaults |
| config.json (unified) | 2KB | ~140 | Config | Unified prompt system |

**Total**: ~53KB source, ~1,788 lines

---

## 🎯 HIGH-LEVEL ARCHITECTURE

### The Council System

```
┌─────────────────────────────────────────────────────────────┐
│  BUSINESS COUNCIL - 3-Member Sequential Deliberation        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  INPUT: BusinessQuery + Context                            │
│    ↓                                                        │
│  STEP 1: Operator Deliberates (execution focus)            │
│    ↓                                                        │
│  STEP 2: Strategist Deliberates (ROI/strategy focus)       │
│    ↓                                                        │
│  STEP 3: Signal Deliberates (market/empathy focus)         │
│    ↓                                                        │
│  STEP 4: Synthesis (unified perspective)                   │
│    ↓                                                        │
│  OUTPUT: Council Decision + Actions + Confidence           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Business Personality System

```
┌─────────────────────────────────────────────────────────────┐
│  BUSINESS PERSONALITY - Client Context Management          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Business Context:                                          │
│    - Industry, company size                                 │
│    - Business hours, timezone                               │
│    - Workflow categories                                    │
│                                                             │
│  Communication Style:                                       │
│    - Style type (formal/casual/technical/etc)              │
│    - Characteristics (tone, formality, detail)             │
│    - Preferences (channel, response time)                  │
│                                                             │
│  Interaction History:                                       │
│    - Past interactions                                      │
│    - Detected patterns                                      │
│    - Satisfaction scores                                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Communication Style Manager

```
┌─────────────────────────────────────────────────────────────┐
│  STYLE MANAGER - Heuristic Analysis & Recommendations      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  INPUT: User messages                                       │
│    ↓                                                        │
│  ANALYZE:                                                   │
│    - Tone detection (friendly/authoritative/neutral)       │
│    - Formality analysis (high/medium/low)                  │
│    - Detail level (high/medium/low)                        │
│    ↓                                                        │
│  DETERMINE:                                                 │
│    - Style type                                             │
│    - Confidence score                                       │
│    - Recommendations                                        │
│    ↓                                                        │
│  OUTPUT: Style analysis + recommendations                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔍 EXTRACTION TARGETS

### 1. BUSINESS COUNCIL (BusinessCouncil.ts)

**Entities (P)**:
- `council_member` (operator, strategist, signal)
- `council_deliberation`
- `council_synthesis`
- `business_decision`

**Operations (X)**:
- `council.deliberate` - Full 3-phase council process
- `council.member_deliberate` - Single member reasoning
- `council.synthesize` - Unify perspectives
- `council.make_decision` - Extract actions/risks

**Constraints (Y)**:
- Member configs (temperature, max_tokens, focus areas)
- Reasoning patterns (execution_focused, strategic_focused, empathy_focused)
- Confidence thresholds (0.7-0.8)
- Synthesis modes (consensus, priority-weighted)
- Tool availability per member

**Events (Z)**:
- `council.deliberation_started`
- `council.member_response_received`
- `council.synthesis_completed`
- `council.decision_made`

**Pure Functions** (rules.ts references):
- `buildMemberSystemPrompt(memberConfig, input)`
- `buildMemberContextPrompt(input, history, memberConfig, knowledgeContext)`
- `parseBusinessInsights(statement)`
- `calculateConfidence(statement, memberConfig)`
- `buildWorkflowRecommendation(statement, category, memberConfig)`
- `synthesizeCouncilOutputs(outputs)`
- `makeBusinessDecision(outputs, context)`
- `buildSynthesisPrompt(query, deliberationHistory)`

**IO Operations**:
- `aiAdapter.chat()` - LLM completion (3x per deliberation + 1x synthesis)

**Schemas**:
```typescript
BusinessContextSchema {
  entityType?: string
  entityId?: string
  workflowCategory?: "operations" | "pipeline" | "deals" | "marketing"
  urgency?: "low" | "medium" | "high" | "urgent"
  businessValue?: number
}

BusinessCouncilInputSchema {
  query: string
  businessContext?: BusinessContext
  documentContext?: string
}

BusinessCouncilOutputSchema {
  member: string
  statement: string
  confidence: number
  businessInsights: {
    executionComplexity?: "simple" | "moderate" | "complex"
    roiImpact?: "high" | "medium" | "low"
    marketResonance?: "strong" | "moderate" | "weak"
    recommendedActions?: string[]
  }
  suggestedTool?: any
  workflowRecommendation?: {
    category: string
    nextSteps: string[]
    automationOpportunity: boolean
  }
  routing?: {
    providerId: string
    modelId: string
  }
}
```

---

### 2. BUSINESS PERSONALITY (BusinessPersonality.ts)

**Entities (P)**:
- `business_context`
- `communication_style`
- `interaction_history`

**Operations (X)**:
- `business_context.get`
- `business_context.set`
- `communication_style.determine`
- `interaction_history.get`
- `interaction_history.update`
- `greeting.generate`
- `business_hours.check`
- `contexts.export`
- `personality.get_stats`

**Constraints (Y)**:
- Industry defaults (finance, technology, healthcare)
- Company size defaults (startup, small, medium, enterprise)
- Business hours (timezone, working days, start/end time)
- Communication preferences

**Events (Z)**:
- `business_context.created`
- `business_context.updated`
- `communication_style.created`
- `communication_style.updated`
- `interaction_history.created`
- `interaction_history.updated`

**Pure Functions** (from rules.ts):
- `generatePersonalizedGreeting(context, history, style, clientName)`
- `getTimeOfDayGreeting(timezone)`
- `isWithinBusinessHours(timezone, workingDays, startTime, endTime)`

**IO Operations**:
- Database CRUD (query, create, update)

**Schemas**:
```typescript
CommunicationStyleShape {
  userId: ActorId
  styleType: "technical" | "formal" | "professional" | "casual" | "conversational"
  characteristics: {
    tone: "friendly" | "authoritative" | "neutral" | "enthusiastic"
    formality: "high" | "medium" | "low"
    detail: "high" | "medium" | "low"
    speed: "fast" | "medium" | "slow"
  }
  preferences: {
    responseLength: "brief" | "detailed" | "comprehensive"
    examples: boolean
    visualAids: boolean
  }
  metadata?: Record<string, unknown>
}

BusinessContextShape {
  userId: ActorId
  industry?: string
  companySize?: string
  businessHours?: {
    timezone?: string
    workingDays?: string[]
    startTime?: string
    endTime?: string
  }
  communicationStyleId?: UUID
  metadata?: Record<string, unknown>
}

InteractionHistoryShape {
  userId: ActorId
  interactions: Array<{
    timestamp: ISODateTime
    type: string
    content: string
    context: Record<string, unknown>
  }>
  patterns: {
    commonTopics: string[]
    preferredChannels: string[]
    responseTime: number
  }
  totalInteractions?: number
  lastInteractionDate?: ISODateTime
  satisfactionScore?: number
}
```

---

### 3. COMMUNICATION STYLE MANAGER (CommunicationStyleManager.ts)

**Entities (P)**:
- `communication_style` (same as BusinessPersonality)
- `style_analysis`

**Operations (X)**:
- `communication_style.set`
- `communication_style.get`
- `style.analyze`
- `style.get_recommendations`
- `style.get_stats`

**Constraints (Y)**:
- Style types: formal, casual, technical, conversational, professional
- Tone types: friendly, authoritative, neutral, enthusiastic
- Formality levels: high, medium, low
- Detail levels: high, medium, low
- Confidence thresholds

**Events (Z)**:
- `communication_style.set`
- `style_analysis.created`
- `communication_style.updated_from_analysis`

**Pure Functions** (from rules.ts):
- `calculateStyleScore(characteristics)`
- `analyzeTone(messages)` - Heuristic tone detection
- `analyzeFormality(messages)` - Heuristic formality detection
- `analyzeDetailLevel(messages)` - Heuristic detail detection
- `determineStyleType(tone, formality, detail)`
- `calculateConfidence(messageCount, tone, formality, detail)`
- `generateRecommendations(style, tone, formality, detail)`

**IO Operations**:
- Database CRUD (query, create, update)

**Schemas**:
```typescript
StyleAnalysisShape {
  userId: ActorId
  sessionId: UUID
  analysis: {
    detectedStyle: "formal" | "casual" | "technical" | "conversational" | "professional"
    confidence: number (0-1)
    characteristics: {
      tone: "friendly" | "authoritative" | "neutral" | "enthusiastic"
      formality: "high" | "medium" | "low"
      detail: "high" | "medium" | "low"
      speed: "fast" | "moderate" | "slow"
    }
    recommendations: string[]
  }
}
```

---

## 📊 EXTRACTION STATISTICS

### Code Distribution

| Component | Lines | Pure Functions | IO Ops | Schemas |
|-----------|-------|---------------|--------|---------|
| BusinessCouncil | ~477 | 8 | 4 | 3 |
| BusinessPersonality | ~300 | 3 | 10 | 3 |
| CommunicationStyle | ~471 | 7 | 7 | 2 |
| **TOTAL** | **~1,248** | **18** | **21** | **8** |

### Operation Code Requirements

Estimated new operation codes needed:

| Domain | Range | Operations | Count |
|--------|-------|------------|-------|
| Council | 0x2000-0x201F | deliberate, synthesize, member_deliberate, make_decision | 4 |
| Business Context | 0x2020-0x202F | get, set, export, stats, check_hours | 5 |
| Communication Style | 0x2030-0x204F | set, get, analyze, recommend, stats | 5 |
| Interaction History | 0x2050-0x205F | get, update, get_stats | 3 |

**Total New Operation Codes**: ~17

---

## 🎯 NEXT STEPS

1. **Extract Council Workflows** (BusinessCouncil.ts)
   - Map 3-phase deliberation to PXYZ
   - Define predicate logic for confidence calculations
   - Create synthesis workflows

2. **Extract Business Personality** (BusinessPersonality.ts)
   - Map context management to PXYZ
   - Define default configurations
   - Create greeting generation workflows

3. **Extract Style Manager** (CommunicationStyleManager.ts)
   - Map heuristic analysis to predicates
   - Define style detection workflows
   - Create recommendation logic

4. **Create Config Data Files**
   - Council member personalities as config data
   - Business context defaults as config data
   - Style analysis rules as config data

5. **Negative Search for Stragglers**
   - Identify missed patterns
   - Find hidden orchestration logic
   - Document edge cases

---

## 🔥 KEY INSIGHTS

### 1. Council is Multi-Step Graph Traversal

The council deliberation is a perfect fit for OMAR:
```
Entry(query) → Operator → Strategist → Signal → Synthesis → Decision
```

Each step is a node with:
- Input validation (transform node)
- AI call (external node)
- Confidence check (auth/predicate node)
- Output generation (render node)

### 2. Business Personality is Pure Configuration

Most of the personality system is:
- **80% config data** (defaults, industry settings, style mappings)
- **15% pure functions** (greeting generation, time checking)
- **5% IO operations** (database CRUD)

This means most of it should be **config.json files**, not workflows!

### 3. Style Analysis is Heuristic Predicates

The style analysis functions are pure heuristics that can be expressed as PXYZ predicates:
```xml
<predicate id="is_friendly_tone">
  <contains left="$message.content" right="thanks"/>
  <contains left="$message.content" right="hello"/>
  <contains left="$message.content" right="please"/>
</predicate>
```

### 4. Rules.ts is Missing!

All the pure functions are in `configs/rules.ts` which wasn't uploaded. These are critical for:
- Prompt building
- Confidence calculation
- Business insight parsing
- Style analysis heuristics

We'll need to either:
- Request rules.ts
- Extract the logic from how it's used in the services
- Define the logic as PXYZ predicates

---

## ⚠️ CRITICAL DEPENDENCIES

These files reference `configs/rules.ts` which contains all the pure business logic:

```typescript
import * as councilRules from "./configs/rules";
import * as personalityRules from "./configs/rules";
```

**Functions we need from rules.ts**:
- Council: `buildMemberSystemPrompt`, `buildMemberContextPrompt`, `parseBusinessInsights`, `calculateConfidence`, `buildWorkflowRecommendation`, `synthesizeCouncilOutputs`, `makeBusinessDecision`, `buildSynthesisPrompt`
- Personality: `generatePersonalizedGreeting`, `getTimeOfDayGreeting`, `isWithinBusinessHours`
- Style: `calculateStyleScore`, `analyzeTone`, `analyzeFormality`, `analyzeDetailLevel`, `determineStyleType`, `calculateConfidence`, `generateRecommendations`

**Next Action**: Either get rules.ts OR reverse-engineer the logic from how it's called.

---

**Status**: ✅ Inventory complete. Ready for systematic extraction.
