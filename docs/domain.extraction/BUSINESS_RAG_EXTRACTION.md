# BUSINESS RAG SYSTEM - COMPLETE EXTRACTION

> **Source**: BusinessRAGSystem.ts (676 lines)
> **Pattern**: Council of Experts + 4-Domain Intelligence

---

## 🎭 THE COUNCIL SYSTEM

### Council Member Personalities

```xml
<constants>
  <!-- Council Member Contexts -->
  <constant name="COUNCIL_CEO" value="CEO/Owner">
    <context>High-level strategic thinking. Focus on company vision, competitive positioning, and long-term growth. Consider market opportunities, strategic partnerships, and company valuation.</context>
  </constant>
  
  <constant name="COUNCIL_CFO" value="CFO">
    <context>Financial rigor and analytical thinking. Focus on revenue optimization, cost structure, cash flow management. Consider ROI, unit economics, and financial sustainability.</context>
  </constant>
  
  <constant name="COUNCIL_CTO" value="CTO">
    <context>Technical feasibility and implementation. Focus on technology stack, scalability, security. Consider technical debt, architectural decisions, and development velocity.</context>
  </constant>
  
  <constant name="COUNCIL_SALES" value="Sales Director">
    <context>Revenue generation and customer acquisition. Focus on pipeline management, conversion optimization, customer lifetime value maximization.</context>
  </constant>
  
  <constant name="COUNCIL_OPS" value="Operations Manager">
    <context>Operational efficiency and process improvement. Focus on automation opportunities, resource utilization, quality control systems.</context>
  </constant>
</constants>
```

### Business Role Guidance

```xml
<constants>
  <constant name="ROLE_OWNER" value="Owner">
    <guidance>High-level strategic thinking. Focus on company vision, competitive positioning, and long-term growth opportunities.</guidance>
  </constant>
  
  <constant name="ROLE_CFO" value="CFO">
    <guidance>Financial rigor. Focus on ROI, unit economics, cash flow management, and financial sustainability.</guidance>
  </constant>
  
  <constant name="ROLE_CTO" value="CTO">
    <guidance>Technical implementation. Focus on scalability, security, architectural decisions, and development velocity.</guidance>
  </constant>
  
  <constant name="ROLE_SALES" value="Sales Director">
    <guidance>Revenue generation. Focus on pipeline management, conversion optimization, customer lifetime value.</guidance>
  </constant>
  
  <constant name="ROLE_OPS" value="Operations Manager">
    <guidance>Operational efficiency. Focus on automation, resource utilization, quality control.</guidance>
  </constant>
  
  <constant name="ROLE_BD" value="Business Development">
    <guidance>Partnership opportunities. Focus on market expansion, strategic alliances, growth channels.</guidance>
  </constant>
  
  <constant name="ROLE_AM" value="Account Manager">
    <guidance>Customer success. Focus on retention, expansion, relationship building, value delivery.</guidance>
  </constant>
  
  <constant name="ROLE_MARKETING" value="Marketing Director">
    <guidance>Brand positioning. Focus on customer acquisition, marketing funnels, brand differentiation.</guidance>
  </constant>
</constants>
```

### Intent-Specific Guidance

```xml
<constants>
  <constant name="INTENT_MARKET_ANALYSIS" value="Market Analysis">
    <guidance>Focus on competitive landscape, customer needs analysis, market sizing. Use data-driven insights to identify opportunities and threats.</guidance>
  </constant>
  
  <constant name="INTENT_REVENUE_OPT" value="Revenue Optimization">
    <guidance>Analyze customer lifetime value, pricing strategies, revenue stream optimization. Focus on conversion optimization and customer value maximization.</guidance>
  </constant>
  
  <constant name="INTENT_PROCESS_IMPROVEMENT" value="Process Improvement">
    <guidance>Identify bottlenecks, automation opportunities, efficiency gains. Focus on scalable solutions and measurable improvements.</guidance>
  </constant>
  
  <constant name="INTENT_STRATEGIC_PLANNING" value="Strategic Planning">
    <guidance>Balance long-term vision with immediate tactical needs. Focus on resource allocation, competitive positioning, strategic initiative prioritization.</guidance>
  </constant>
  
  <constant name="INTENT_COMPETITIVE_INTEL" value="Competitive Intelligence">
    <guidance>Analyze competitor strengths, weaknesses, strategic moves. Focus on differentiation opportunities and competitive response strategies.</guidance>
  </constant>
</constants>
```

---

## 📦 SCHEMAS

```xml
<schemas>
  <!-- Business Knowledge Chunk -->
  <schema id="business_knowledge">
    <field name="id" type="uuid" required="true"/>
    <field name="title" type="string" required="true"/>
    <field name="content" type="string" required="true"/>
    <field name="category" type="string" required="true"/>
    <field name="priority" type="number" min="1" max="10" required="true"/>
    <field name="businessConcepts" type="array" required="true"/>
    <field name="applicableToRoles" type="array" required="true"/>
    <field name="councilMemberRelevance" type="object" required="true">
      <field name="CEO" type="number" min="0" max="1"/>
      <field name="CFO" type="number" min="0" max="1"/>
      <field name="CTO" type="number" min="0" max="1"/>
      <field name="Sales" type="number" min="0" max="1"/>
      <field name="Operations" type="number" min="0" max="1"/>
    </field>
    <field name="pxyz" type="object" required="true"/>
  </schema>
  
  <!-- Business RAG Request -->
  <schema id="business_rag_request">
    <field name="query" type="string" required="true" minLength="1"/>
    <field name="context" type="object">
      <field name="councilMember" type="string"/>
      <field name="businessRole" type="string"/>
      <field name="industryContext" type="string"/>
      <field name="intent" type="string"/>
    </field>
    <field name="maxChunksPerDomain" type="number" default="3"/>
    <field name="minRelevanceScore" type="number" default="0.5"/>
  </schema>
  
  <!-- Business RAG Result -->
  <schema id="business_rag_result">
    <field name="id" type="uuid" required="true"/>
    <field name="createdAt" type="datetime" required="true"/>
    <field name="updatedAt" type="datetime" required="true"/>
    <field name="businessKnowledge" type="array" required="true"/>
    <field name="marketIntelligence" type="array" required="true"/>
    <field name="operationalInsights" type="array" required="true"/>
    <field name="strategicFrameworks" type="array" required="true"/>
    <field name="relevanceScore" type="number" required="true"/>
    <field name="businessContext" type="object" required="true">
      <field name="applicableConcepts" type="array"/>
      <field name="recommendedActions" type="array"/>
      <field name="riskFactors" type="array"/>
    </field>
  </schema>
</schemas>
```

---

## 🔍 PREDICATES

```xml
<predicates>
  <!-- Council Member Matching -->
  <predicate id="council_member_relevant">
    <gt left="$chunk.councilMemberRelevance[$councilMember]" right="0.5"/>
  </predicate>
  
  <!-- Business Role Matching -->
  <predicate id="business_role_applicable">
    <contains left="$chunk.applicableToRoles" right="$businessRole"/>
  </predicate>
  
  <!-- Priority Filtering -->
  <predicate id="priority_above_threshold">
    <gte left="$chunk.priority" right="7"/>
  </predicate>
  
  <!-- Category Matching -->
  <predicate id="is_business_knowledge">
    <eq left="$chunk.category" right="business_knowledge"/>
  </predicate>
  
  <predicate id="is_market_intelligence">
    <eq left="$chunk.category" right="market_intelligence"/>
  </predicate>
  
  <predicate id="is_operational_insight">
    <eq left="$chunk.category" right="operational_insight"/>
  </predicate>
  
  <predicate id="is_strategic_framework">
    <or>
      <eq left="$chunk.category" right="analytical_framework"/>
      <eq left="$chunk.category" right="strategic_framework"/>
    </or>
  </predicate>
  
  <!-- Context-Based Relevance -->
  <predicate id="has_required_context">
    <and>
      <not-null left="$context.councilMember"/>
      <not-null left="$context.businessRole"/>
    </and>
  </predicate>
  
  <!-- Relevance Scoring -->
  <predicate id="above_min_relevance">
    <gte left="$chunk.relevanceScore" right="$minRelevanceScore"/>
  </predicate>
</predicates>
```

---

## 🎯 MAIN WORKFLOW

```xml
<workflow id="business_context_retrieval">
  <entry p="query" x="rag" node="validate_request"/>
  
  <nodes>
    <!-- Stage 1: Validate -->
    <node id="validate_request" kind="transform">
      <schema ref="business_rag_request"/>
      <validate>
        <require field="query" type="string" minLength="1"/>
      </validate>
    </node>
    
    <!-- Stage 2: Enrich Context -->
    <node id="enrich_context" kind="transform">
      <algorithm>enrich_business_context</algorithm>
      <input>
        <field>context</field>
      </input>
      <output>
        <field>councilGuidance</field>
        <field>roleGuidance</field>
        <field>intentGuidance</field>
      </output>
    </node>
    
    <!-- Stage 3: Parallel 4-Domain Retrieval -->
    <node id="retrieve_business_knowledge" kind="external" op="0x0A00">
      <operation>BUSINESS_KNOWLEDGE_QUERY</operation>
      <input>
        <field>query</field>
        <field>councilMember</field>
        <field>businessRole</field>
        <field>limit</field>
      </input>
      <output>businessKnowledge</output>
    </node>
    
    <node id="retrieve_market_intelligence" kind="external" op="0x0A01">
      <operation>MARKET_INTELLIGENCE_QUERY</operation>
      <input>
        <field>query</field>
        <field>industryContext</field>
        <field>limit</field>
      </input>
      <output>marketIntelligence</output>
    </node>
    
    <node id="retrieve_operational_insights" kind="external" op="0x0A02">
      <operation>OPERATIONAL_INSIGHTS_QUERY</operation>
      <input>
        <field>query</field>
        <field>businessRole</field>
        <field>limit</field>
      </input>
      <output>operationalInsights</output>
    </node>
    
    <node id="retrieve_strategic_frameworks" kind="external" op="0x0A03">
      <operation>STRATEGIC_FRAMEWORKS_QUERY</operation>
      <input>
        <field>query</field>
        <field>intent</field>
        <field>limit</field>
      </input>
      <output>strategicFrameworks</output>
    </node>
    
    <!-- Stage 4: Aggregate Results -->
    <node id="aggregate_results" kind="transform">
      <algorithm>aggregate_domain_results</algorithm>
      <input>
        <field>businessKnowledge</field>
        <field>marketIntelligence</field>
        <field>operationalInsights</field>
        <field>strategicFrameworks</field>
      </input>
      <output>allChunks</output>
    </node>
    
    <!-- Stage 5: Calculate Relevance -->
    <node id="calculate_relevance" kind="transform">
      <algorithm>calculate_overall_relevance</algorithm>
      <input>
        <field>query</field>
        <field>context</field>
        <field>allChunks</field>
      </input>
      <output>relevanceScore</output>
    </node>
    
    <!-- Stage 6: Analyze Business Context -->
    <node id="analyze_context" kind="transform">
      <algorithm>analyze_business_context</algorithm>
      <input>
        <field>query</field>
        <field>context</field>
        <field>allChunks</field>
      </input>
      <output>businessContextAnalysis</output>
    </node>
    
    <!-- Stage 7: Build Result -->
    <node id="build_result" kind="transform">
      <algorithm>build_business_rag_result</algorithm>
      <input>
        <field>businessKnowledge</field>
        <field>marketIntelligence</field>
        <field>operationalInsights</field>
        <field>strategicFrameworks</field>
        <field>relevanceScore</field>
        <field>businessContextAnalysis</field>
      </input>
      <output>ragResult</output>
    </node>
    
    <!-- Stage 8: Persist -->
    <node id="persist_result" kind="external" op="0x0901">
      <operation>STORAGE_SET</operation>
      <collection>business_rag_contexts</collection>
      <input>
        <field>retrievalId</field>
        <field>query</field>
        <field>ragResult</field>
        <field>pxyz</field>
      </input>
    </node>
    
    <!-- Stage 9: Emit Event -->
    <node id="emit_event" kind="signal">
      <event>business.rag.context.retrieved</event>
      <data>
        <field>retrievalId</field>
        <field>query</field>
        <field>councilMember</field>
        <field>totalChunks</field>
        <field>relevanceScore</field>
      </data>
    </node>
    
    <node id="done" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="validate_request" to="enrich_context">
      <when><always/></when>
    </edge>
    <edge from="enrich_context" to="retrieve_business_knowledge">
      <when><always/></when>
    </edge>
    <edge from="enrich_context" to="retrieve_market_intelligence">
      <when><always/></when>
    </edge>
    <edge from="enrich_context" to="retrieve_operational_insights">
      <when><always/></when>
    </edge>
    <edge from="enrich_context" to="retrieve_strategic_frameworks">
      <when><always/></when>
    </edge>
    <edge from="retrieve_business_knowledge" to="aggregate_results">
      <when><always/></when>
    </edge>
    <edge from="retrieve_market_intelligence" to="aggregate_results">
      <when><always/></when>
    </edge>
    <edge from="retrieve_operational_insights" to="aggregate_results">
      <when><always/></when>
    </edge>
    <edge from="retrieve_strategic_frameworks" to="aggregate_results">
      <when><always/></when>
    </edge>
    <edge from="aggregate_results" to="calculate_relevance">
      <when><always/></when>
    </edge>
    <edge from="calculate_relevance" to="analyze_context">
      <when><always/></when>
    </edge>
    <edge from="analyze_context" to="build_result">
      <when><always/></when>
    </edge>
    <edge from="build_result" to="persist_result">
      <when><always/></when>
    </edge>
    <edge from="persist_result" to="emit_event">
      <when><always/></when>
    </edge>
    <edge from="emit_event" to="done">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

---

## 🧮 PURE FUNCTIONS / ALGORITHMS

```javascript
// io-browser.ts algorithms section

const algorithms = {
  /**
   * ENRICH BUSINESS CONTEXT
   * Add council/role/intent guidance to context
   */
  enrich_business_context: (context) => {
    return {
      ...context,
      councilGuidance: context.councilMember 
        ? getCouncilMemberContext(context.councilMember)
        : null,
      roleGuidance: context.businessRole
        ? getRoleSpecificGuidance(context.businessRole)
        : null,
      intentGuidance: context.intent
        ? getIntentSpecificGuidance(context.intent)
        : null
    };
  },
  
  /**
   * AGGREGATE DOMAIN RESULTS
   * Combine all 4 domains into single array
   */
  aggregate_domain_results: (business, market, operational, strategic) => {
    return [
      ...business,
      ...market,
      ...operational,
      ...strategic
    ];
  },
  
  /**
   * CALCULATE OVERALL RELEVANCE
   * Multi-factor scoring with context boost
   */
  calculate_overall_relevance: (query, context, chunks) => {
    if (chunks.length === 0) return 0;
    
    // Average relevance from chunks
    const avgRelevance = chunks.reduce((sum, chunk) => {
      return sum + (chunk.metadata?.relevance_score || 0);
    }, 0) / chunks.length;
    
    // Context boost
    let contextBoost = 0;
    if (context?.businessRole) contextBoost += 0.1;
    if (context?.industryContext) contextBoost += 0.1;
    if (context?.councilMember) contextBoost += 0.1;
    
    return Math.min(1.0, avgRelevance + contextBoost);
  },
  
  /**
   * ANALYZE BUSINESS CONTEXT
   * Extract concepts, actions, risks from chunks
   */
  analyze_business_context: (query, context, chunks) => {
    const concepts = new Set();
    const actions = [];
    const risks = [];
    
    chunks.forEach(chunk => {
      // Extract business concepts
      if (chunk.metadata?.business_concepts) {
        chunk.metadata.business_concepts.forEach(concept => concepts.add(concept));
      }
      
      // Extract action words
      const content = String(chunk.content || '').toLowerCase();
      if (content.includes('implement') || content.includes('execute')) {
        actions.push('Consider implementation strategies and execution planning');
      }
      if (content.includes('measure') || content.includes('track')) {
        actions.push('Establish measurement and tracking systems');
      }
      if (content.includes('optimize') || content.includes('improve')) {
        actions.push('Identify optimization and improvement opportunities');
      }
      
      // Extract risk indicators
      if (content.includes('risk') || content.includes('challenge')) {
        risks.push('Monitor for potential risks and challenges mentioned in context');
      }
      if (content.includes('resource') || content.includes('constraint')) {
        risks.push('Consider resource constraints and operational limitations');
      }
    });
    
    return {
      applicableConcepts: Array.from(concepts),
      recommendedActions: [...new Set(actions)],
      riskFactors: [...new Set(risks)]
    };
  },
  
  /**
   * BUILD BUSINESS RAG RESULT
   * Construct final result object
   */
  build_business_rag_result: (businessKnowledge, marketIntelligence, operationalInsights, strategicFrameworks, relevanceScore, businessContextAnalysis) => {
    return {
      id: generateUUID(),
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      businessKnowledge,
      marketIntelligence,
      operationalInsights,
      strategicFrameworks,
      relevanceScore,
      businessContext: businessContextAnalysis
    };
  },
  
  /**
   * GET COUNCIL MEMBER CONTEXT
   */
  getCouncilMemberContext: (member) => {
    const contexts = {
      "CEO/Owner": "High-level strategic thinking. Focus on company vision, competitive positioning, and long-term growth. Consider market opportunities, strategic partnerships, and company valuation.",
      "CFO": "Financial rigor and analytical thinking. Focus on revenue optimization, cost structure, cash flow management. Consider ROI, unit economics, and financial sustainability.",
      "CTO": "Technical feasibility and implementation. Focus on technology stack, scalability, security. Consider technical debt, architectural decisions, and development velocity.",
      "Sales Director": "Revenue generation and customer acquisition. Focus on pipeline management, conversion optimization, customer lifetime value maximization.",
      "Operations Manager": "Operational efficiency and process improvement. Focus on automation opportunities, resource utilization, quality control systems."
    };
    return contexts[member] || contexts["CEO/Owner"];
  },
  
  /**
   * GET ROLE SPECIFIC GUIDANCE
   */
  getRoleSpecificGuidance: (role) => {
    const guidance = {
      "Owner": "High-level strategic thinking. Focus on company vision, competitive positioning, and long-term growth opportunities.",
      "CFO": "Financial rigor. Focus on ROI, unit economics, cash flow management, and financial sustainability.",
      "CTO": "Technical implementation. Focus on scalability, security, architectural decisions, and development velocity.",
      "Sales Director": "Revenue generation. Focus on pipeline management, conversion optimization, customer lifetime value.",
      "Operations Manager": "Operational efficiency. Focus on automation, resource utilization, quality control.",
      "Business Development": "Partnership opportunities. Focus on market expansion, strategic alliances, growth channels.",
      "Account Manager": "Customer success. Focus on retention, expansion, relationship building, value delivery.",
      "Marketing Director": "Brand positioning. Focus on customer acquisition, marketing funnels, brand differentiation."
    };
    return guidance[role] || guidance["Owner"];
  },
  
  /**
   * GET INTENT SPECIFIC GUIDANCE
   */
  getIntentSpecificGuidance: (intent) => {
    const guidance = {
      "Market Analysis": "Focus on competitive landscape, customer needs analysis, market sizing. Use data-driven insights to identify opportunities and threats.",
      "Revenue Optimization": "Analyze customer lifetime value, pricing strategies, revenue stream optimization. Focus on conversion optimization and customer value maximization.",
      "Process Improvement": "Identify bottlenecks, automation opportunities, efficiency gains. Focus on scalable solutions and measurable improvements.",
      "Strategic Planning": "Balance long-term vision with immediate tactical needs. Focus on resource allocation, competitive positioning, strategic initiative prioritization.",
      "Competitive Intelligence": "Analyze competitor strengths, weaknesses, strategic moves. Focus on differentiation opportunities and competitive response strategies."
    };
    return guidance[intent] || guidance["Strategic Planning"];
  }
};
```

---

## 🔌 IO OPERATIONS

```javascript
// Business Knowledge Operations (0x0Axx)
const ioHandlers = {
  0x0A00: async (input) => {
    // BUSINESS_KNOWLEDGE_QUERY
    const { query, councilMember, businessRole, limit } = input;
    
    // Query database for business knowledge
    const allKnowledge = await db.query('business_knowledge', {});
    
    // Filter by council member relevance
    let filtered = allKnowledge;
    if (councilMember) {
      filtered = filtered.filter(chunk =>
        chunk.councilMemberRelevance[councilMember] > 0.5
      );
    }
    
    // Filter by business role
    if (businessRole) {
      filtered = filtered.filter(chunk =>
        chunk.applicableToRoles.includes(businessRole)
      );
    }
    
    // Sort by priority and relevance
    filtered = filtered.sort((a, b) => {
      const aRelevance = councilMember ? a.councilMemberRelevance[councilMember] : 0;
      const bRelevance = councilMember ? b.councilMemberRelevance[councilMember] : 0;
      
      if (bRelevance !== aRelevance) return bRelevance - aRelevance;
      return b.priority - a.priority;
    });
    
    return filtered.slice(0, limit || 3);
  },
  
  0x0A01: async (input) => {
    // MARKET_INTELLIGENCE_QUERY
    const { query, industryContext, limit } = input;
    
    const allKnowledge = await db.query('business_knowledge', {});
    const filtered = allKnowledge.filter(chunk =>
      chunk.category === 'market_intelligence'
    );
    
    return filtered.slice(0, limit || 3);
  },
  
  0x0A02: async (input) => {
    // OPERATIONAL_INSIGHTS_QUERY
    const { query, businessRole, limit } = input;
    
    const allKnowledge = await db.query('business_knowledge', {});
    const filtered = allKnowledge.filter(chunk =>
      chunk.category === 'operational_insight'
    );
    
    return filtered.slice(0, limit || 3);
  },
  
  0x0A03: async (input) => {
    // STRATEGIC_FRAMEWORKS_QUERY
    const { query, intent, limit } = input;
    
    const allKnowledge = await db.query('business_knowledge', {});
    const filtered = allKnowledge.filter(chunk =>
      chunk.category === 'analytical_framework' || 
      chunk.category === 'strategic_framework'
    );
    
    return filtered.sort((a, b) => b.priority - a.priority).slice(0, limit || 3);
  }
};
```

---

## 📊 COMPLETE SYSTEM DIAGRAM

```
┌─────────────────────────────────────────────────────────────┐
│  BUSINESS RAG SYSTEM - 4-Domain Parallel Intelligence      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Query + Context                                            │
│       │                                                     │
│       ├─► Enrich Context                                   │
│       │    └─► Council Guidance (5 members)                │
│       │    └─► Role Guidance (8 roles)                     │
│       │    └─► Intent Guidance (5 intents)                 │
│       │                                                     │
│       ├─► PARALLEL RETRIEVAL (Effect.all)                  │
│       │    ├─► Business Knowledge Query (0x0A00)           │
│       │    ├─► Market Intelligence Query (0x0A01)          │
│       │    ├─► Operational Insights Query (0x0A02)         │
│       │    └─► Strategic Frameworks Query (0x0A03)         │
│       │                                                     │
│       ├─► Aggregate Results                                │
│       │    └─► All 4 domains combined                      │
│       │                                                     │
│       ├─► Calculate Relevance                              │
│       │    └─► Multi-factor scoring + context boost        │
│       │                                                     │
│       ├─► Analyze Business Context                         │
│       │    ├─► Extract applicable concepts                 │
│       │    ├─► Identify recommended actions                │
│       │    └─► Flag risk factors                           │
│       │                                                     │
│       ├─► Build Result                                     │
│       │    └─► Complete BusinessRAGResult object           │
│       │                                                     │
│       ├─► Persist + Emit Event                             │
│       │                                                     │
│       └─► Return                                            │
│            └─► 4-domain intelligence + analysis            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## ✅ EXTRACTION CHECKLIST

- [x] 5 Council member contexts
- [x] 8 Business role guidance maps
- [x] 5 Intent-specific guidance maps
- [x] 3 Core schemas (business_knowledge, business_rag_request, business_rag_result)
- [x] 9 Predicates (council/role/category/relevance matching)
- [x] Main workflow (9 stages, 15 edges)
- [x] 7 Pure functions (enrich/aggregate/calculate/analyze/build/get*)
- [x] 4 IO operations (0x0A00-0x0A03)

**Status**: ✅ COMPLETE
**Lines Extracted**: 676 / 676 (100%)
