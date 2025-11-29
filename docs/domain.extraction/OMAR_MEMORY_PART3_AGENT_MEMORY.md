# OMAR Memory System Part 3: Agent Memory Service - PXYZ Extraction

> **Custom Research**: Multi-granularity aggregation windows with coordinate-space search  
> **NOT Langchain**: Original architecture for interaction memory with temporal indexing  
> **Service**: AgentMemoryService

---

## OVERVIEW: MEMORY AGGREGATION WINDOWS

Your Agent Memory Service treats **interaction tracking as multi-granularity time windows**:

- **PXYZ Pattern #18**: Single interaction affects multiple aggregation windows simultaneously
- **Hourly, daily, monthly aggregations** as coordinate projections
- **Peak activity detection** via density calculation in coordinate space
- **Relevance scoring** with boost factors (recency, sentiment, priority)

This is **NOT** a simple log store - it's a **temporal coordinate-addressable interaction database**.

---

## PART 1: CORE INNOVATION - MEMORY AGGREGATION WINDOWS

### The Coordinate-Space Pattern

```typescript
/**
 * PXYZ Pattern #18: Memory Aggregation Windows
 * 
 * Problem: Interaction data needs aggregation across multiple time granularities
 * Overlap: Single interaction affects hourly, daily, AND monthly windows
 * 
 * Coordinate Space: Time windows for memory interaction aggregation
 */
interface MemoryAggregationWindow {
  windowType: "hourly" | "daily" | "monthly";
  timeSpan: [start: Date, end: Date];      // Window boundaries
  interactionCount: number;                  // Interactions in window
  priority: number;                          // hourly=3, daily=2, monthly=1
}

interface InteractionCoordinate {
  interactionId: UUID;
  timestamp: Date;                           // Z-coordinate
  userId: ActorId;                           // User isolation
  relevanceScore: number;                    // Calculated score
  metadata: {
    tags?: string[];
    priority?: "urgent" | "high" | "medium" | "low";
    sentiment?: "positive" | "neutral" | "negative";
  };
}
```

### Multi-Granularity Aggregation

```typescript
// Single interaction lands in 3 windows simultaneously
const interaction = {
  interactionId: "uuid-123",
  timestamp: new Date("2025-11-29T14:30:00Z"),
  userId: "user-456",
  relevanceScore: 85
};

const windows = createAggregationWindows(interaction.timestamp, [
  "hourly",   // 2025-11-29 14:00-15:00
  "daily",    // 2025-11-29 00:00 - 2025-11-30 00:00
  "monthly"   // 2025-11-01 00:00 - 2025-12-01 00:00
]);

// Interaction affects all 3 windows
windows.forEach(window => {
  if (interactionInWindow(interaction, window)) {
    window.interactionCount++;
  }
});
```

---

## P-Axis: Agent Memory Entities

```xml
<schema id="interaction_log">
  <field name="id" type="uuid" required="true"/> <!-- Log ID -->
  <field name="user_id" type="uuid" required="true"/>
  <field name="timestamp" type="timestamp" required="true"/> <!-- Z-coordinate -->
  <field name="interaction_type" type="enum" values="user_input,assistant_response,tool_execution,error,system"/>
  
  <!-- Search fields for relevance -->
  <field name="search_fields" type="object">
    <field name="user_input" type="string"/>
    <field name="assistant_response" type="string"/>
    <field name="current_task" type="string"/>
    <field name="active_files" type="array"/>
    <field name="tools_used" type="array"/>
  </field>
  
  <!-- Context -->
  <field name="project_path" type="string"/>
  <field name="session_id" type="uuid"/>
  
  <!-- Metadata for boosts -->
  <field name="metadata" type="object">
    <field name="tags" type="array"/>
    <field name="priority" type="enum" values="urgent,high,medium,low"/>
    <field name="sentiment" type="enum" values="positive,neutral,negative"/>
    <field name="response_time" type="number"/> <!-- Milliseconds -->
    <field name="satisfaction" type="number"/> <!-- 0.0-1.0 -->
    <field name="tool_usage_complexity" type="number"/>
  </field>
  
  <field name="pxyz" type="object" required="true"/>
</schema>

<schema id="memory_aggregation_window">
  <field name="window_type" type="enum" values="hourly,daily,monthly" required="true"/>
  <field name="time_span_start" type="timestamp" required="true"/>
  <field name="time_span_end" type="timestamp" required="true"/>
  <field name="interaction_count" type="number" required="true"/>
  <field name="priority" type="number" required="true"/> <!-- 3/2/1 -->
  <field name="interactions" type="array"/> <!-- InteractionCoordinate[] -->
</schema>

<schema id="interaction_coordinate">
  <field name="interaction_id" type="uuid" required="true"/>
  <field name="timestamp" type="timestamp" required="true"/>
  <field name="user_id" type="uuid" required="true"/>
  <field name="relevance_score" type="number" required="true"/>
  <field name="metadata" type="object"/>
</schema>

<schema id="peak_activity_window">
  <field name="window" type="object" required="true"/> <!-- MemoryAggregationWindow -->
  <field name="density" type="number" required="true"/> <!-- Interactions per hour -->
  <field name="rank" type="number"/> <!-- Top-N ranking -->
</schema>

<schema id="interaction_search_result">
  <field name="interaction" type="object" required="true"/>
  <field name="relevance_score" type="number" required="true"/>
  <field name="boosted_score" type="number" required="true"/>
  <field name="matched_fields" type="array" required="true"/>
  <field name="highlights" type="object"/> <!-- Field → matched text -->
</schema>

<schema id="memory_analytics">
  <field name="user_id" type="uuid" required="true"/>
  <field name="time_period" type="object" required="true"/>
  <field name="metrics" type="object" required="true">
    <field name="interaction_frequency" type="number"/>
    <field name="response_time" type="number"/>
    <field name="satisfaction" type="number"/>
    <field name="tool_usage_patterns" type="object"/>
    <field name="project_engagement" type="object"/>
    <field name="task_complexity" type="number"/>
  </field>
  <field name="insights" type="array"/> <!-- Generated patterns -->
</schema>
```

---

## X-Axis: Agent Memory Operations

```yaml
# Interaction Logging
memory_log_interaction: 0x3300       # Log interaction
memory_log_user_input: 0x3301        # Log user input
memory_log_assistant_response: 0x3302 # Log assistant response
memory_log_tool_execution: 0x3303    # Log tool usage
memory_log_error: 0x3304             # Log error

# Search Operations
memory_search: 0x3310                # Search interactions
memory_search_semantic: 0x3311       # Semantic vector search
memory_search_by_project: 0x3312     # Project-scoped search
memory_search_by_tags: 0x3313        # Tag-based search
memory_search_by_timerange: 0x3314   # Time-filtered search

# Aggregation Windows
memory_create_windows: 0x3320        # Create aggregation windows
memory_aggregate_interactions: 0x3321 # Aggregate into windows
memory_calculate_density: 0x3322     # Calculate interaction density
memory_find_peak_windows: 0x3323     # Find peak activity periods

# Relevance Scoring
memory_calculate_relevance: 0x3330   # Calculate base relevance
memory_apply_recency_boost: 0x3331   # Apply recency boost
memory_apply_sentiment_boost: 0x3332 # Apply sentiment boost
memory_apply_priority_boost: 0x3333  # Apply priority boost
memory_calculate_final_score: 0x3334 # Calculate final boosted score

# Pattern Detection
memory_detect_tool_patterns: 0x3340  # Detect tool usage patterns
memory_detect_time_patterns: 0x3341  # Detect time-based patterns
memory_detect_project_patterns: 0x3342 # Detect project patterns
memory_detect_collaboration: 0x3343  # Detect collaboration patterns

# Analytics
memory_get_analytics: 0x3350         # Get analytics for user
memory_calculate_metrics: 0x3351     # Calculate metrics
memory_generate_insights: 0x3352     # Generate insights
memory_track_learning: 0x3353        # Track learning progression

# Cleanup
memory_compress_interactions: 0x3360 # Compress old interactions
memory_cleanup_expired: 0x3361       # Clean expired data
memory_archive_old: 0x3362           # Archive to cold storage
```

---

## Y-Axis: Agent Memory Predicates

```xml
<predicates>
  <!-- Relevance Scoring -->
  <predicate id="calculate_base_relevance">
    <compute>
      <var name="score" value="0"/>
      
      <!-- User input match (weight 10) -->
      <when>
        <contains left="$interaction.search_fields.user_input" right="$query_term"/>
      </when>
      <set name="score" value="$score + 10"/>
      
      <!-- Assistant response match (weight 5) -->
      <when>
        <contains left="$interaction.search_fields.assistant_response" right="$query_term"/>
      </when>
      <set name="score" value="$score + 5"/>
      
      <!-- Current task match (weight 8) -->
      <when>
        <contains left="$interaction.search_fields.current_task" right="$query_term"/>
      </when>
      <set name="score" value="$score + 8"/>
      
      <!-- Active files match (weight 3) -->
      <when>
        <contains_any left="$interaction.search_fields.active_files" right="$query_term"/>
      </when>
      <set name="score" value="$score + 3"/>
      
      <!-- Tools used match (weight 6) -->
      <when>
        <contains_any left="$interaction.search_fields.tools_used" right="$query_term"/>
      </when>
      <set name="score" value="$score + 6"/>
      
      <!-- Project path match (weight 4) -->
      <when>
        <contains left="$interaction.project_path" right="$query_term"/>
      </when>
      <set name="score" value="$score + 4"/>
      
      <!-- Tags match (weight 7) -->
      <when>
        <contains_any left="$interaction.metadata.tags" right="$query_term"/>
      </when>
      <set name="score" value="$score + 7"/>
    </compute>
  </predicate>
  
  <!-- Recency Boost -->
  <predicate id="calculate_recency_boost">
    <compute>
      <var name="age_days" value="
        ($now - $interaction.timestamp) / 86400000
      "/>
      <var name="boost" value="
        max(0, $config.boostFactors.recency.maxBoost - 
               ($age_days * $config.boostFactors.recency.decayRate))
      "/>
    </compute>
  </predicate>
  
  <!-- Sentiment Boost -->
  <predicate id="apply_sentiment_multiplier">
    <compute>
      <var name="multiplier" value="
        $config.boostFactors.sentiment[$interaction.metadata.sentiment] ?? 1.0
      "/>
    </compute>
  </predicate>
  
  <!-- Priority Boost -->
  <predicate id="apply_priority_multiplier">
    <compute>
      <var name="multiplier" value="
        $config.boostFactors.priority[$interaction.metadata.priority] ?? 1.0
      "/>
    </compute>
  </predicate>
  
  <!-- Window Containment -->
  <predicate id="interaction_in_window">
    <and>
      <gte left="$interaction.timestamp" right="$window.time_span_start"/>
      <lt left="$interaction.timestamp" right="$window.time_span_end"/>
    </and>
  </predicate>
  
  <!-- Compression Eligibility -->
  <predicate id="should_compress_interaction">
    <and>
      <eq left="$config.compression.enabled" right="true"/>
      <gt left="sizeof($interaction)" right="$config.compression.threshold"/>
    </and>
  </predicate>
  
  <!-- Retention -->
  <predicate id="exceeds_retention_period">
    <gt 
      left="($now - $interaction.timestamp) / 86400000" 
      right="$config.retentionPeriods.interactions"
    />
  </predicate>
</predicates>
```

---

## Z-Axis: Agent Memory Events

```typescript
enum AgentMemoryEventType {
  // Interaction Logging
  INTERACTION_LOGGED = "memory.interaction_logged",
  USER_INPUT_LOGGED = "memory.user_input_logged",
  ASSISTANT_RESPONSE_LOGGED = "memory.assistant_response_logged",
  TOOL_EXECUTION_LOGGED = "memory.tool_execution_logged",
  
  // Search
  MEMORY_SEARCH_EXECUTED = "memory.search_executed",
  MEMORY_SEARCH_NO_RESULTS = "memory.search_no_results",
  
  // Aggregation
  AGGREGATION_WINDOWS_CREATED = "memory.aggregation_windows_created",
  INTERACTIONS_AGGREGATED = "memory.interactions_aggregated",
  PEAK_ACTIVITY_DETECTED = "memory.peak_activity_detected",
  
  // Analytics
  ANALYTICS_GENERATED = "memory.analytics_generated",
  PATTERN_DETECTED = "memory.pattern_detected",
  INSIGHT_GENERATED = "memory.insight_generated",
  
  // Cleanup
  INTERACTIONS_COMPRESSED = "memory.interactions_compressed",
  EXPIRED_DATA_CLEANED = "memory.expired_data_cleaned",
  DATA_ARCHIVED = "memory.data_archived"
}
```

---

## Config-Driven Behavior

```json
{
  "agentMemory": {
    "indexing": {
      "dimensions": ["user", "project", "tags", "time", "type", "sentiment", "priority"],
      "searchFields": [
        "userInput",
        "assistantResponse",
        "currentTask",
        "activeFiles",
        "toolsUsed"
      ],
      "relevanceWeights": {
        "userInput": 10,
        "assistantResponse": 5,
        "currentTask": 8,
        "activeFiles": 3,
        "toolsUsed": 6,
        "projectPath": 4,
        "tags": 7
      },
      "boostFactors": {
        "recency": {
          "maxBoost": 10,
          "decayRate": 0.1
        },
        "sentiment": {
          "positive": 1.2,
          "neutral": 1.0,
          "negative": 0.8
        },
        "priority": {
          "urgent": 2.0,
          "high": 1.5,
          "medium": 1.0,
          "low": 0.7
        }
      }
    },
    "analytics": {
      "trackMetrics": [
        "interactionFrequency",
        "responseTime",
        "satisfaction",
        "toolUsagePatterns",
        "projectEngagement",
        "taskComplexity",
        "errorRecoveryTime",
        "learningProgression"
      ],
      "insightPatterns": [
        "toolUsage",
        "commonTasks",
        "timePatterns",
        "projectPatterns",
        "collaborationPatterns",
        "errorPatterns",
        "learningPatterns",
        "productivityPatterns"
      ],
      "aggregationWindows": {
        "hourly": 24,
        "daily": 30,
        "weekly": 12,
        "monthly": 24
      }
    },
    "storage": {
      "retentionPeriods": {
        "interactions": 365,
        "analytics": 730,
        "patterns": 1095
      },
      "compressionSettings": {
        "enabled": true,
        "threshold": 1000,
        "algorithm": "gzip"
      }
    }
  }
}
```

---

**[Continued in next file due to length...]**
