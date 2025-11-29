# OMAR RAG SYSTEM EXTRACTION

> **Mission**: Extract TypeScript RAG implementation into OMAR's PXYZ coordinate system.

---

## 📋 EXTRACTION INVENTORY

### Files Analyzed
| File | Lines | Extractable Workflows | Pure Functions | IO Operations |
|------|-------|----------------------|----------------|---------------|
| HybridSearch.ts | ~350 | hybrid_search | mergeResults | 3 vector ops |
| Vectors.ts | ~280 | embedding_generation | 8 utilities | 1 embed op |
| ChunkingPipeline.ts | ? | document_chunking | splitting logic | 2 ops |
| RagChain.ts | ? | rag_query | context building | 3 ops |
| CAGEngine.ts | ? | context_generation | template selection | 1 op |
| BusinessRAGSystem.ts | ? | rag_orchestration | orchestration | 4 ops |
| rules.ts | ~400 | - | 20+ pure functions | - |
| config.json | ~180 | - | - | config data |

---

## 🎯 PXYZ COORDINATE MAPPING

### P (Entities)
```typescript
// Core RAG Entities
- document        // Input documents
- chunk           // Split text segments
- embedding       // Vector representations
- query           // User questions
- search_result   // Retrieved chunks
- context         // Aggregated context
- answer          // Generated response
- cache_entry     // Cached embeddings
```

### X (Operations)
```typescript
// Document Processing
- chunk           // Split document into segments
- embed           // Generate vector embedding
- index           // Store in vector DB

// Search & Retrieval
- search          // Hybrid search (dense + sparse + graph)
- retrieve        // Get top-k chunks
- rerank          // Multi-stage scoring

// Generation
- generate        // LLM completion
- format          // Structure answer
- stream          // SSE streaming response
```

### Y (Constraints)
```typescript
// Chunking
chunk_size: 512
overlap: 50
min_chunk_length: 100

// Embeddings
embedding_dimension: 384
model: "all-MiniLM-L6-v2"
normalize: true

// Search
weights: {
  dense: 0.6,
  sparse: 0.2, 
  graph: 0.2,
  bias: 0.0
}
similarity_threshold: 0.7
max_results: 10

// Cache
cache_enabled: true
cache_ttl_ms: 86400000
cache_max_size: 10000
```

### Z (Events)
```typescript
- document.uploaded
- document.chunked
- chunk.embedded
- vector.indexed
- search.started
- search.completed
- rag.query.received
- rag.answer.generated
- cache.hit
- cache.miss
- cache.evicted
```

---

## 📦 WORKFLOW EXTRACTION

### 1. HYBRID SEARCH WORKFLOW

**Source**: `HybridSearch.ts` → `createHybridSearchService()`

```xml
<workflow id="hybrid_search">
  <entry p="query" x="search" node="validate_request"/>
  
  <nodes>
    <!-- Stage 1: Validate -->
    <node id="validate_request" kind="transform">
      <schema ref="hybrid_search_request"/>
      <validate>
        <require field="queryEmbedding" type="array"/>
        <require field="queryText" type="string" minLength="1"/>
        <check predicate="valid_embedding_dimension"/>
      </validate>
    </node>
    
    <!-- Stage 2: Dense Vector Search -->
    <node id="dense_search" kind="external" op="0x0700">
      <operation>QDRANT_SEARCH</operation>
      <input>
        <field>queryEmbedding</field>
        <field>limit</field>
      </input>
      <output>denseResults</output>
    </node>
    
    <!-- Stage 3: Sparse Keyword Search -->
    <node id="sparse_search" kind="external" op="0x0600">
      <operation>SPARSE_SEARCH</operation>
      <input>
        <field>queryText</field>
        <field>limit</field>
      </input>
      <output>sparseResults</output>
    </node>
    
    <!-- Stage 4: Graph Proximity Scoring -->
    <node id="graph_scores" kind="external" op="0x0601">
      <operation>GRAPH_PROXIMITY_SCORE</operation>
      <input>
        <field>denseResults</field>
        <field>sparseResults</field>
      </input>
      <output>graphScores</output>
    </node>
    
    <!-- Stage 5: Merge Results -->
    <node id="merge" kind="transform">
      <algorithm>merge_results</algorithm>
      <input>
        <field>denseResults</field>
        <field>sparseResults</field>
      </input>
      <output>combinedResults</output>
    </node>
    
    <!-- Stage 6: Rerank with Weights -->
    <node id="rerank" kind="transform">
      <algorithm>weighted_rerank</algorithm>
      <input>
        <field>combinedResults</field>
        <field>graphScores</field>
        <field>weights</field>
      </input>
      <output>rerankedResults</output>
    </node>
    
    <!-- Stage 7: Limit Results -->
    <node id="limit_results" kind="transform">
      <algorithm>take_top_k</algorithm>
      <input>
        <field>rerankedResults</field>
        <field>limit</field>
      </input>
      <output>finalResults</output>
    </node>
    
    <!-- Stage 8: Persist Search -->
    <node id="persist" kind="external" op="0x0901">
      <operation>STORAGE_SET</operation>
      <collection>hybrid_search_results</collection>
    </node>
    
    <!-- Stage 9: Emit Event -->
    <node id="emit_event" kind="signal">
      <event>search.completed</event>
      <data>
        <field>searchId</field>
        <field>totalResults</field>
        <field>avgScore</field>
      </data>
    </node>
    
    <node id="done" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="validate_request" to="dense_search">
      <when><always/></when>
    </edge>
    <edge from="validate_request" to="sparse_search">
      <when><always/></when>
    </edge>
    <edge from="dense_search" to="graph_scores">
      <when><always/></when>
    </edge>
    <edge from="sparse_search" to="graph_scores">
      <when><always/></when>
    </edge>
    <edge from="graph_scores" to="merge">
      <when><always/></when>
    </edge>
    <edge from="merge" to="rerank">
      <when><always/></when>
    </edge>
    <edge from="rerank" to="limit_results">
      <when><always/></when>
    </edge>
    <edge from="limit_results" to="persist">
      <when><always/></when>
    </edge>
    <edge from="persist" to="emit_event">
      <when><always/></when>
    </edge>
    <edge from="emit_event" to="done">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

---

### 2. EMBEDDING GENERATION WORKFLOW

**Source**: `Vectors.ts` → `generateProductionEmbedding()`

```xml
<workflow id="embedding_generation">
  <entry p="document" x="embed" node="check_cache"/>
  
  <nodes>
    <!-- Stage 1: Cache Lookup -->
    <node id="check_cache" kind="external" op="0x0900">
      <operation>CACHE_GET</operation>
      <input>
        <field>contentHash</field>
      </input>
      <output>cachedEmbedding</output>
    </node>
    
    <!-- Stage 2: Extract Features -->
    <node id="extract_features" kind="transform">
      <algorithm>extract_all_features</algorithm>
      <input>
        <field>content</field>
      </input>
      <output>
        <field>semanticFeatures</field>
        <field>syntacticFeatures</field>
        <field>lexicalFeatures</field>
      </output>
    </node>
    
    <!-- Stage 3: Generate Embedding -->
    <node id="generate_embedding" kind="external" op="0x0710">
      <operation>EMBEDDING_GENERATE</operation>
      <input>
        <field>content</field>
        <field>semanticFeatures</field>
        <field>syntacticFeatures</field>
        <field>lexicalFeatures</field>
      </input>
      <output>rawEmbedding</output>
    </node>
    
    <!-- Stage 4: Normalize -->
    <node id="normalize" kind="external" op="0x0711">
      <operation>EMBEDDING_NORMALIZE</operation>
      <input>
        <field>rawEmbedding</field>
      </input>
      <output>normalizedEmbedding</output>
    </node>
    
    <!-- Stage 5: Cache Result -->
    <node id="cache_result" kind="external" op="0x0901">
      <operation>CACHE_SET</operation>
      <input>
        <field>contentHash</field>
        <field>normalizedEmbedding</field>
        <field>timestamp</field>
      </input>
    </node>
    
    <!-- Stage 6: Cleanup Cache (1% probability) -->
    <node id="cleanup_cache" kind="external" op="0x0902">
      <operation>CACHE_EVICT</operation>
      <input>
        <field>maxSize</field>
        <field>ttl</field>
      </input>
    </node>
    
    <node id="return_cached" kind="terminal"/>
    <node id="return_generated" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="check_cache" to="return_cached">
      <when>
        <ref predicate="cache_hit"/>
      </when>
    </edge>
    <edge from="check_cache" to="extract_features">
      <when>
        <ref predicate="cache_miss"/>
      </when>
    </edge>
    <edge from="extract_features" to="generate_embedding">
      <when><always/></when>
    </edge>
    <edge from="generate_embedding" to="normalize">
      <when><always/></when>
    </edge>
    <edge from="normalize" to="cache_result">
      <when><always/></when>
    </edge>
    <edge from="cache_result" to="cleanup_cache">
      <when>
        <ref predicate="should_cleanup_cache"/>
      </when>
    </edge>
    <edge from="cache_result" to="return_generated">
      <when>
        <not><ref predicate="should_cleanup_cache"/></not>
      </when>
    </edge>
    <edge from="cleanup_cache" to="return_generated">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

---

### 3. DOCUMENT CHUNKING WORKFLOW

**Source**: `ChunkingPipeline.ts` (structure inferred)

```xml
<workflow id="document_chunking">
  <entry p="document" x="chunk" node="validate_document"/>
  
  <nodes>
    <!-- Stage 1: Validate -->
    <node id="validate_document" kind="transform">
      <schema ref="document_input"/>
      <validate>
        <require field="content" type="string" minLength="1"/>
        <require field="metadata" type="object"/>
      </validate>
    </node>
    
    <!-- Stage 2: Split into Chunks -->
    <node id="split_chunks" kind="transform">
      <algorithm>sliding_window_split</algorithm>
      <input>
        <field>content</field>
        <field>chunkSize</field>
        <field>overlap</field>
      </input>
      <output>rawChunks</output>
    </node>
    
    <!-- Stage 3: Enrich Metadata -->
    <node id="enrich_metadata" kind="transform">
      <algorithm>add_chunk_metadata</algorithm>
      <input>
        <field>rawChunks</field>
        <field>documentMetadata</field>
      </input>
      <output>enrichedChunks</output>
    </node>
    
    <!-- Stage 4: Generate Embeddings (foreach) -->
    <node id="embed_chunks" kind="external" op="0x0710">
      <operation>EMBEDDING_GENERATE_BATCH</operation>
      <input>
        <field>enrichedChunks</field>
      </input>
      <output>embeddedChunks</output>
    </node>
    
    <!-- Stage 5: Index in Vector Store -->
    <node id="index_chunks" kind="external" op="0x0701">
      <operation>QDRANT_INDEX</operation>
      <input>
        <field>embeddedChunks</field>
      </input>
      <output>indexedIds</output>
    </node>
    
    <!-- Stage 6: Persist Chunk Records -->
    <node id="persist_chunks" kind="external" op="0x0901">
      <operation>STORAGE_SET</operation>
      <collection>chunks</collection>
      <input>
        <field>embeddedChunks</field>
      </input>
    </node>
    
    <!-- Stage 7: Emit Event -->
    <node id="emit_event" kind="signal">
      <event>document.chunked</event>
      <data>
        <field>documentId</field>
        <field>chunkCount</field>
        <field>indexedCount</field>
      </data>
    </node>
    
    <node id="done" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="validate_document" to="split_chunks">
      <when><always/></when>
    </edge>
    <edge from="split_chunks" to="enrich_metadata">
      <when><always/></when>
    </edge>
    <edge from="enrich_metadata" to="embed_chunks">
      <when><always/></when>
    </edge>
    <edge from="embed_chunks" to="index_chunks">
      <when><always/></when>
    </edge>
    <edge from="index_chunks" to="persist_chunks">
      <when><always/></when>
    </edge>
    <edge from="persist_chunks" to="emit_event">
      <when><always/></when>
    </edge>
    <edge from="emit_event" to="done">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

---

### 4. RAG QUERY WORKFLOW

**Source**: `RagChain.ts` (structure inferred)

```xml
<workflow id="rag_query">
  <entry p="query" x="rag" node="validate_query"/>
  
  <nodes>
    <!-- Stage 1: Validate -->
    <node id="validate_query" kind="transform">
      <schema ref="rag_query_request"/>
      <validate>
        <require field="question" type="string" minLength="1"/>
        <require field="context" type="object"/>
      </validate>
    </node>
    
    <!-- Stage 2: Generate Query Embedding -->
    <node id="embed_query" kind="external" op="0x0710">
      <operation>EMBEDDING_GENERATE</operation>
      <input>
        <field>question</field>
      </input>
      <output>queryEmbedding</output>
    </node>
    
    <!-- Stage 3: Hybrid Search -->
    <node id="search" kind="external" op="0x0700">
      <operation>HYBRID_SEARCH</operation>
      <input>
        <field>queryEmbedding</field>
        <field>question</field>
        <field>limit</field>
        <field>weights</field>
      </input>
      <output>searchResults</output>
    </node>
    
    <!-- Stage 4: Build Context -->
    <node id="build_context" kind="transform">
      <algorithm>aggregate_chunks</algorithm>
      <input>
        <field>searchResults</field>
        <field>maxTokens</field>
      </input>
      <output>aggregatedContext</output>
    </node>
    
    <!-- Stage 5: Generate Answer -->
    <node id="generate_answer" kind="external" op="0x0800">
      <operation>LLM_COMPLETE</operation>
      <input>
        <field>question</field>
        <field>aggregatedContext</field>
        <field>systemPrompt</field>
      </input>
      <output>rawAnswer</output>
    </node>
    
    <!-- Stage 6: Format Response -->
    <node id="format_response" kind="transform">
      <algorithm>format_rag_response</algorithm>
      <input>
        <field>rawAnswer</field>
        <field>searchResults</field>
        <field>citations</field>
      </input>
      <output>formattedResponse</output>
    </node>
    
    <!-- Stage 7: Persist Query -->
    <node id="persist_query" kind="external" op="0x0901">
      <operation>STORAGE_SET</operation>
      <collection>rag_queries</collection>
      <input>
        <field>queryId</field>
        <field>question</field>
        <field>formattedResponse</field>
        <field>pxyz</field>
      </input>
    </node>
    
    <!-- Stage 8: Emit Event -->
    <node id="emit_event" kind="signal">
      <event>rag.query.completed</event>
      <data>
        <field>queryId</field>
        <field>retrievedChunks</field>
        <field>answerLength</field>
      </data>
    </node>
    
    <node id="done" kind="terminal"/>
  </nodes>
  
  <edges>
    <edge from="validate_query" to="embed_query">
      <when><always/></when>
    </edge>
    <edge from="embed_query" to="search">
      <when><always/></when>
    </edge>
    <edge from="search" to="build_context">
      <when><always/></when>
    </edge>
    <edge from="build_context" to="generate_answer">
      <when><always/></when>
    </edge>
    <edge from="generate_answer" to="format_response">
      <when><always/></when>
    </edge>
    <edge from="format_response" to="persist_query">
      <when><always/></when>
    </edge>
    <edge from="persist_query" to="emit_event">
      <when><always/></when>
    </edge>
    <edge from="emit_event" to="done">
      <when><always/></when>
    </edge>
  </edges>
</workflow>
```

---

## 🔧 IO OPERATION CODES

### Vector Operations (0x07xx)
```typescript
0x0700: QDRANT_SEARCH          // Dense vector similarity search
0x0701: QDRANT_INDEX           // Store embedding in vector DB
0x0702: QDRANT_DELETE          // Remove vector from DB
0x0703: QDRANT_UPDATE          // Update vector metadata
0x0710: EMBEDDING_GENERATE     // Generate single embedding
0x0711: EMBEDDING_NORMALIZE    // L2 normalize vector
0x0712: EMBEDDING_GENERATE_BATCH // Batch embedding generation
```

### Search Operations (0x06xx)
```typescript
0x0600: SPARSE_SEARCH          // Keyword-based BM25 search
0x0601: GRAPH_PROXIMITY_SCORE  // Calculate graph-based relevance
0x0602: HYBRID_SEARCH          // Multi-stage search orchestration
```

### LLM Operations (0x08xx)
```typescript
0x0800: LLM_COMPLETE           // Generate completion
0x0801: LLM_CLASSIFY           // Classify text
0x0802: LLM_STREAM             // Streaming completion
```

### Cache Operations (0x09xx)
```typescript
0x0900: CACHE_GET              // Retrieve from cache
0x0901: CACHE_SET              // Store in cache
0x0902: CACHE_EVICT            // Remove expired entries
0x0903: CACHE_CLEAR            // Clear entire cache
```

---

## 📐 SCHEMAS

```xml
<schemas>
  <!-- Document Chunk -->
  <schema id="chunk">
    <field name="id" type="uuid" required="true"/>
    <field name="content" type="string" required="true"/>
    <field name="embedding" type="array" required="true"/>
    <field name="metadata" type="object" required="true">
      <field name="documentId" type="uuid"/>
      <field name="chunkIndex" type="number"/>
      <field name="timestamp" type="number"/>
      <field name="tags" type="array"/>
      <field name="importance" type="number"/>
    </field>
  </schema>
  
  <!-- Hybrid Search Request -->
  <schema id="hybrid_search_request">
    <field name="queryEmbedding" type="array" required="true"/>
    <field name="queryText" type="string" required="true" minLength="1"/>
    <field name="limit" type="number" default="5"/>
    <field name="weights" type="object">
      <field name="dense" type="number" min="0" max="1" default="0.6"/>
      <field name="sparse" type="number" min="0" max="1" default="0.2"/>
      <field name="graph" type="number" min="0" max="1" default="0.2"/>
      <field name="bias" type="number" min="0" max="1" default="0.0"/>
    </field>
  </schema>
  
  <!-- Search Result -->
  <schema id="search_result">
    <field name="chunk" type="object" required="true"/>
    <field name="score" type="number" required="true"/>
    <field name="metadata" type="object"/>
  </schema>
  
  <!-- RAG Query Request -->
  <schema id="rag_query_request">
    <field name="question" type="string" required="true" minLength="1"/>
    <field name="context" type="object"/>
    <field name="maxChunks" type="number" default="5"/>
    <field name="maxTokens" type="number" default="2000"/>
  </schema>
  
  <!-- Document Input -->
  <schema id="document_input">
    <field name="content" type="string" required="true" minLength="1"/>
    <field name="metadata" type="object" required="true">
      <field name="source" type="string"/>
      <field name="title" type="string"/>
      <field name="author" type="string"/>
      <field name="timestamp" type="number"/>
    </field>
    <field name="chunkSize" type="number" default="512"/>
    <field name="overlap" type="number" default="50"/>
  </schema>
</schemas>
```

---

## 🔍 PREDICATES

```xml
<predicates>
  <!-- Embedding Validation -->
  <predicate id="valid_embedding_dimension">
    <eq left="$embedding.length" right="384"/>
  </predicate>
  
  <predicate id="valid_embedding_values">
    <and>
      <gt left="$embedding.length" right="0"/>
      <not>
        <contains left="$embedding" right="NaN"/>
      </not>
    </and>
  </predicate>
  
  <!-- Cache Predicates -->
  <predicate id="cache_hit">
    <and>
      <not-null left="$cachedEmbedding"/>
      <lt left="$cache.age" right="86400000"/>
    </and>
  </predicate>
  
  <predicate id="cache_miss">
    <not><ref predicate="cache_hit"/></not>
  </predicate>
  
  <predicate id="should_cleanup_cache">
    <lt left="$random" right="0.01"/>
  </predicate>
  
  <!-- Search Predicates -->
  <predicate id="above_threshold">
    <gte left="$score" right="0.7"/>
  </predicate>
  
  <predicate id="has_results">
    <gt left="$results.length" right="0"/>
  </predicate>
  
  <!-- Quality Predicates -->
  <predicate id="high_quality_chunk">
    <and>
      <gte left="$chunk.score" right="0.8"/>
      <gt left="$chunk.content.length" right="100"/>
    </and>
  </predicate>
  
  <predicate id="recent_document">
    <gt left="$metadata.timestamp" right="$now - 2592000000"/>
  </predicate>
</predicates>
```

---

## 🎨 PURE FUNCTIONS → TRANSFORM ALGORITHMS

### From `rules.ts` and `Vectors.ts`

```javascript
// io-browser.ts algorithms section

const algorithms = {
  /**
   * MERGE RESULTS - Deduplicate by chunk ID
   */
  merge_results: (denseResults, sparseResults) => {
    const seenIds = new Set();
    const merged = [];
    
    for (const item of denseResults) {
      if (!seenIds.has(item.chunk.id)) {
        seenIds.add(item.chunk.id);
        merged.push(item);
      }
    }
    
    for (const item of sparseResults) {
      if (!seenIds.has(item.chunk.id)) {
        seenIds.add(item.chunk.id);
        merged.push(item);
      }
    }
    
    return merged;
  },
  
  /**
   * WEIGHTED RERANK - Multi-stage scoring
   */
  weighted_rerank: (combinedResults, graphScores, weights) => {
    return combinedResults.map((item, idx) => {
      const graphScore = graphScores[idx] || 0;
      const combinedScore =
        weights.dense * item.score +
        weights.sparse * (item.sparseMatch ? 1.0 : 0.0) +
        weights.graph * graphScore +
        weights.bias;
      
      return { ...item, score: combinedScore };
    });
  },
  
  /**
   * TAKE TOP K - Limit and sort
   */
  take_top_k: (results, limit) => {
    return results
      .sort((a, b) => b.score - a.score)
      .slice(0, limit);
  },
  
  /**
   * EXTRACT ALL FEATURES - Semantic + Syntactic + Lexical
   */
  extract_all_features: (content) => {
    return {
      semantic: extractSemanticFeatures(content),
      syntactic: extractSyntacticFeatures(content),
      lexical: extractLexicalFeatures(content)
    };
  },
  
  /**
   * SLIDING WINDOW SPLIT - Chunk with overlap
   */
  sliding_window_split: (content, chunkSize, overlap) => {
    const chunks = [];
    let start = 0;
    
    while (start < content.length) {
      const end = Math.min(start + chunkSize, content.length);
      const chunk = content.slice(start, end);
      
      if (chunk.length >= 100) { // min chunk length
        chunks.push({
          content: chunk,
          start,
          end
        });
      }
      
      start += chunkSize - overlap;
    }
    
    return chunks;
  },
  
  /**
   * ADD CHUNK METADATA - Enrich with document metadata
   */
  add_chunk_metadata: (rawChunks, documentMetadata) => {
    return rawChunks.map((chunk, idx) => ({
      id: generateUUID(),
      content: chunk.content,
      metadata: {
        ...documentMetadata,
        chunkIndex: idx,
        chunkCount: rawChunks.length,
        startOffset: chunk.start,
        endOffset: chunk.end,
        timestamp: Date.now()
      }
    }));
  },
  
  /**
   * AGGREGATE CHUNKS - Build context from results
   */
  aggregate_chunks: (searchResults, maxTokens) => {
    let context = '';
    let tokenCount = 0;
    
    for (const result of searchResults) {
      const chunkTokens = Math.ceil(result.chunk.content.length / 4);
      
      if (tokenCount + chunkTokens > maxTokens) break;
      
      context += `\n\n---\n${result.chunk.content}`;
      tokenCount += chunkTokens;
    }
    
    return { context, chunksUsed: searchResults.length, tokenCount };
  },
  
  /**
   * FORMAT RAG RESPONSE - Structure with citations
   */
  format_rag_response: (rawAnswer, searchResults, citations) => {
    return {
      answer: rawAnswer,
      sources: searchResults.map(r => ({
        id: r.chunk.id,
        content: r.chunk.content,
        score: r.score,
        metadata: r.chunk.metadata
      })),
      citations: citations || [],
      timestamp: Date.now()
    };
  },
  
  /**
   * COSINE SIMILARITY
   */
  cosine_similarity: (vecA, vecB) => {
    if (vecA.length !== vecB.length) return 0;
    
    let dotProduct = 0, normA = 0, normB = 0;
    
    for (let i = 0; i < vecA.length; i++) {
      dotProduct += vecA[i] * vecB[i];
      normA += vecA[i] * vecA[i];
      normB += vecB[i] * vecB[i];
    }
    
    const magnitude = Math.sqrt(normA) * Math.sqrt(normB);
    if (magnitude === 0) return 0;
    
    return Math.max(-1, Math.min(1, dotProduct / magnitude));
  },
  
  /**
   * L2 NORMALIZE
   */
  normalize_l2: (vector) => {
    let norm = 0;
    for (const val of vector) norm += val * val;
    
    if (norm === 0 || !isFinite(norm)) {
      return Array(vector.length).fill(0).map((_, i) => i === 0 ? 1 : 0);
    }
    
    const magnitude = Math.sqrt(norm);
    return vector.map(val => val / magnitude);
  }
};
```

---

## 📊 CONFIGURATION → Y-CONTEXT

```xml
<!-- Y-Context Constants from config.json -->
<constants>
  <!-- Embedding Config -->
  <constant name="EMBEDDING_DIMENSION" value="384"/>
  <constant name="EMBEDDING_MODEL" value="all-MiniLM-L6-v2"/>
  <constant name="EMBEDDING_LOCATION" value="browser"/>
  
  <!-- Search Config -->
  <constant name="MAX_RESULTS_DEV" value="5"/>
  <constant name="MAX_RESULTS_PROD" value="10"/>
  <constant name="SIMILARITY_THRESHOLD" value="0.7"/>
  <constant name="FUZZY_THRESHOLD" value="0.75"/>
  
  <!-- Cache Config -->
  <constant name="CACHE_TTL_MS" value="86400000"/>
  <constant name="CACHE_MAX_SIZE_DEV" value="1000"/>
  <constant name="CACHE_MAX_SIZE_PROD" value="10000"/>
  
  <!-- Chunking Config -->
  <constant name="CHUNK_SIZE" value="512"/>
  <constant name="CHUNK_OVERLAP" value="50"/>
  <constant name="MIN_CHUNK_LENGTH" value="100"/>
  
  <!-- Search Weights -->
  <constant name="WEIGHT_DENSE" value="0.6"/>
  <constant name="WEIGHT_SPARSE" value="0.2"/>
  <constant name="WEIGHT_GRAPH" value="0.2"/>
  <constant name="WEIGHT_BIAS" value="0.0"/>
</constants>
```

---

## 🎯 IO ADAPTER IMPLEMENTATION

### Browser IO Adapter (`io-browser.ts`)

```javascript
// io-browser.ts - RAG operations

const ioHandlers = {
  // ───────────────────────────────────────────────────────────
  // VECTOR OPERATIONS (0x07xx)
  // ───────────────────────────────────────────────────────────
  
  0x0700: async (input) => {
    // QDRANT_SEARCH - Dense vector search
    const { queryEmbedding, filter, limit } = input;
    
    // Call Qdrant API (or local vector DB)
    const response = await fetch('https://qdrant-instance/collections/chunks/points/search', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        vector: queryEmbedding,
        filter,
        limit: limit || 10,
        with_payload: true
      })
    });
    
    const data = await response.json();
    return data.result.map(hit => ({
      chunk: hit.payload,
      score: hit.score
    }));
  },
  
  0x0701: async (input) => {
    // QDRANT_INDEX - Store embeddings
    const { chunks } = input;
    
    const points = chunks.map(chunk => ({
      id: chunk.id,
      vector: chunk.embedding,
      payload: {
        content: chunk.content,
        metadata: chunk.metadata
      }
    }));
    
    await fetch('https://qdrant-instance/collections/chunks/points', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ points })
    });
    
    return { indexed: points.length };
  },
  
  0x0710: async (input) => {
    // EMBEDDING_GENERATE - Single embedding
    const { content } = input;
    
    // Use local embedding function
    const embedding = await generateProductionEmbedding(content);
    
    return { embedding };
  },
  
  0x0711: (input) => {
    // EMBEDDING_NORMALIZE - L2 normalize
    const { vector } = input;
    return { normalized: algorithms.normalize_l2(vector) };
  },
  
  0x0712: async (input) => {
    // EMBEDDING_GENERATE_BATCH - Batch embeddings
    const { chunks } = input;
    
    const embeddings = await Promise.all(
      chunks.map(chunk => generateProductionEmbedding(chunk.content))
    );
    
    return { embeddings };
  },
  
  // ───────────────────────────────────────────────────────────
  // SEARCH OPERATIONS (0x06xx)
  // ───────────────────────────────────────────────────────────
  
  0x0600: (input) => {
    // SPARSE_SEARCH - Keyword-based
    const { queryText, limit } = input;
    
    const tokens = queryText.toLowerCase().split(/\s+/).filter(t => t.length > 0);
    
    // Simple BM25-like scoring (stub)
    // In production: use proper BM25 implementation
    const matches = [];
    
    return matches.slice(0, limit);
  },
  
  0x0601: (input) => {
    // GRAPH_PROXIMITY_SCORE
    const { results } = input;
    
    return results.map(result => {
      const metadata = result.chunk.metadata;
      
      let score = 0.5; // Base score
      
      // Recency boost
      if (metadata.timestamp) {
        const ageInDays = (Date.now() - metadata.timestamp) / (1000 * 60 * 60 * 24);
        const recencyBoost = Math.max(0, 1 - ageInDays / 365);
        score += recencyBoost * 0.3;
      }
      
      // Tag/category boost
      if (metadata.tags && metadata.tags.length > 0) {
        score += Math.min(0.2, metadata.tags.length * 0.05);
      }
      
      return Math.min(1, Math.max(0, score));
    });
  },
  
  0x0602: async (input) => {
    // HYBRID_SEARCH - Orchestrate multi-stage search
    // This would call the hybrid_search workflow via graph traversal
    // For now, inline implementation:
    
    const { queryEmbedding, queryText, limit, weights } = input;
    
    // Dense search
    const denseResults = await ioHandlers[0x0700]({ queryEmbedding, limit: limit * 2 });
    
    // Sparse search
    const sparseResults = await ioHandlers[0x0600]({ queryText, limit: limit * 2 });
    
    // Merge
    const merged = algorithms.merge_results(denseResults, sparseResults);
    
    // Graph scores
    const graphScores = await ioHandlers[0x0601]({ results: merged });
    
    // Rerank
    const reranked = algorithms.weighted_rerank(merged, graphScores, weights);
    
    // Limit
    return algorithms.take_top_k(reranked, limit);
  },
  
  // ───────────────────────────────────────────────────────────
  // LLM OPERATIONS (0x08xx)
  // ───────────────────────────────────────────────────────────
  
  0x0800: async (input) => {
    // LLM_COMPLETE - Generate answer
    const { question, context, systemPrompt } = input;
    
    const response = await fetch('https://api.anthropic.com/v1/messages', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': getApiKey()
      },
      body: JSON.stringify({
        model: 'claude-sonnet-4-20250514',
        max_tokens: 1024,
        messages: [{
          role: 'user',
          content: `${systemPrompt}\n\nContext:\n${context}\n\nQuestion: ${question}`
        }]
      })
    });
    
    const data = await response.json();
    return { answer: data.content[0].text };
  },
  
  // ───────────────────────────────────────────────────────────
  // CACHE OPERATIONS (0x09xx)
  // ───────────────────────────────────────────────────────────
  
  0x0900: (input) => {
    // CACHE_GET
    const { key } = input;
    const cached = embeddingCache.get(key);
    
    if (!cached || Date.now() - cached.timestamp > CACHE_TTL_MS) {
      return null;
    }
    
    return cached;
  },
  
  0x0901: (input) => {
    // CACHE_SET
    const { key, value, timestamp } = input;
    embeddingCache.set(key, { value, timestamp: timestamp || Date.now() });
    return { cached: true };
  },
  
  0x0902: () => {
    // CACHE_EVICT - Remove expired entries
    const now = Date.now();
    let evicted = 0;
    
    for (const [key, entry] of embeddingCache.entries()) {
      if (now - entry.timestamp > CACHE_TTL_MS) {
        embeddingCache.delete(key);
        evicted++;
      }
    }
    
    return { evicted };
  }
};
```

---

## 📈 DATASTAR FRONTEND INTEGRATION

### RAG Query Interface

```html
<div data-signals="{
  query: '',
  results: [],
  answer: '',
  loading: false,
  chunks: []
}">
  <!-- Search Input -->
  <div class="search-box">
    <input 
      type="text"
      data-bind:query
      data-on:input.debounce_500ms="$pxyz('query','rag',{question:$query})"
      placeholder="Ask a question..."
    />
    <div data-show="$loading" class="spinner"></div>
  </div>
  
  <!-- Answer Display -->
  <div id="answer" class="answer-card">
    <!-- SSE patches answer here -->
  </div>
  
  <!-- Source Chunks -->
  <div id="sources" class="sources-grid">
    <!-- SSE patches chunks here -->
  </div>
</div>
```

### SSE Event Stream

```javascript
// Server-side (Node.js)
app.get('/pxyz/query/rag', async (req, res) => {
  const { question } = req.query;
  
  res.setHeader('Content-Type', 'text/event-stream');
  res.setHeader('Cache-Control', 'no-cache');
  
  // Execute RAG workflow via graph traversal
  const result = await executeWorkflow('rag_query', {
    question,
    context: {},
    maxChunks: 5,
    maxTokens: 2000
  });
  
  // Stream answer
  res.write(`event: datastar-merge-fragments\n`);
  res.write(`data: selector #answer\n`);
  res.write(`data: fragment <div class="answer">${result.answer}</div>\n\n`);
  
  // Stream sources
  res.write(`event: datastar-merge-fragments\n`);
  res.write(`data: selector #sources\n`);
  res.write(`data: fragment ${renderSources(result.sources)}\n\n`);
  
  res.end();
});
```

---

## 🎭 COMPLETE RAG SYSTEM XML

```xml
<?xml version="1.0" encoding="UTF-8"?>
<omar version="1.0">
  <!-- SCHEMAS -->
  <schemas>
    <schema id="chunk">
      <field name="id" type="uuid" required="true"/>
      <field name="content" type="string" required="true"/>
      <field name="embedding" type="array" required="true"/>
      <field name="metadata" type="object" required="true"/>
    </schema>
    
    <schema id="hybrid_search_request">
      <field name="queryEmbedding" type="array" required="true"/>
      <field name="queryText" type="string" required="true" minLength="1"/>
      <field name="limit" type="number" default="5"/>
      <field name="weights" type="object"/>
    </schema>
    
    <schema id="search_result">
      <field name="chunk" type="object" required="true"/>
      <field name="score" type="number" required="true"/>
    </schema>
    
    <schema id="rag_query_request">
      <field name="question" type="string" required="true" minLength="1"/>
      <field name="context" type="object"/>
      <field name="maxChunks" type="number" default="5"/>
      <field name="maxTokens" type="number" default="2000"/>
    </schema>
    
    <schema id="document_input">
      <field name="content" type="string" required="true" minLength="1"/>
      <field name="metadata" type="object" required="true"/>
      <field name="chunkSize" type="number" default="512"/>
      <field name="overlap" type="number" default="50"/>
    </schema>
  </schemas>
  
  <!-- PREDICATES -->
  <predicates>
    <predicate id="valid_embedding_dimension">
      <eq left="$embedding.length" right="384"/>
    </predicate>
    
    <predicate id="cache_hit">
      <and>
        <not-null left="$cachedEmbedding"/>
        <lt left="$cache.age" right="86400000"/>
      </and>
    </predicate>
    
    <predicate id="cache_miss">
      <not><ref predicate="cache_hit"/></not>
    </predicate>
    
    <predicate id="should_cleanup_cache">
      <lt left="$random" right="0.01"/>
    </predicate>
    
    <predicate id="above_threshold">
      <gte left="$score" right="0.7"/>
    </predicate>
    
    <predicate id="has_results">
      <gt left="$results.length" right="0"/>
    </predicate>
  </predicates>
  
  <!-- WORKFLOWS -->
  
  <!-- 1. HYBRID SEARCH -->
  <workflow id="hybrid_search">
    <entry p="query" x="search" node="validate_request"/>
    
    <nodes>
      <node id="validate_request" kind="transform">
        <schema ref="hybrid_search_request"/>
      </node>
      <node id="dense_search" kind="external" op="0x0700"/>
      <node id="sparse_search" kind="external" op="0x0600"/>
      <node id="graph_scores" kind="external" op="0x0601"/>
      <node id="merge" kind="transform"/>
      <node id="rerank" kind="transform"/>
      <node id="limit_results" kind="transform"/>
      <node id="persist" kind="external" op="0x0901"/>
      <node id="emit_event" kind="signal"/>
      <node id="done" kind="terminal"/>
    </nodes>
    
    <edges>
      <edge from="validate_request" to="dense_search"><when><always/></when></edge>
      <edge from="validate_request" to="sparse_search"><when><always/></when></edge>
      <edge from="dense_search" to="graph_scores"><when><always/></when></edge>
      <edge from="sparse_search" to="graph_scores"><when><always/></when></edge>
      <edge from="graph_scores" to="merge"><when><always/></when></edge>
      <edge from="merge" to="rerank"><when><always/></when></edge>
      <edge from="rerank" to="limit_results"><when><always/></when></edge>
      <edge from="limit_results" to="persist"><when><always/></when></edge>
      <edge from="persist" to="emit_event"><when><always/></when></edge>
      <edge from="emit_event" to="done"><when><always/></when></edge>
    </edges>
  </workflow>
  
  <!-- 2. EMBEDDING GENERATION -->
  <workflow id="embedding_generation">
    <entry p="document" x="embed" node="check_cache"/>
    
    <nodes>
      <node id="check_cache" kind="external" op="0x0900"/>
      <node id="extract_features" kind="transform"/>
      <node id="generate_embedding" kind="external" op="0x0710"/>
      <node id="normalize" kind="external" op="0x0711"/>
      <node id="cache_result" kind="external" op="0x0901"/>
      <node id="cleanup_cache" kind="external" op="0x0902"/>
      <node id="return_cached" kind="terminal"/>
      <node id="return_generated" kind="terminal"/>
    </nodes>
    
    <edges>
      <edge from="check_cache" to="return_cached">
        <when><ref predicate="cache_hit"/></when>
      </edge>
      <edge from="check_cache" to="extract_features">
        <when><ref predicate="cache_miss"/></when>
      </edge>
      <edge from="extract_features" to="generate_embedding"><when><always/></when></edge>
      <edge from="generate_embedding" to="normalize"><when><always/></when></edge>
      <edge from="normalize" to="cache_result"><when><always/></when></edge>
      <edge from="cache_result" to="cleanup_cache">
        <when><ref predicate="should_cleanup_cache"/></when>
      </edge>
      <edge from="cache_result" to="return_generated">
        <when><not><ref predicate="should_cleanup_cache"/></not></when>
      </edge>
      <edge from="cleanup_cache" to="return_generated"><when><always/></when></edge>
    </edges>
  </workflow>
  
  <!-- 3. DOCUMENT CHUNKING -->
  <workflow id="document_chunking">
    <entry p="document" x="chunk" node="validate_document"/>
    
    <nodes>
      <node id="validate_document" kind="transform">
        <schema ref="document_input"/>
      </node>
      <node id="split_chunks" kind="transform"/>
      <node id="enrich_metadata" kind="transform"/>
      <node id="embed_chunks" kind="external" op="0x0712"/>
      <node id="index_chunks" kind="external" op="0x0701"/>
      <node id="persist_chunks" kind="external" op="0x0901"/>
      <node id="emit_event" kind="signal"/>
      <node id="done" kind="terminal"/>
    </nodes>
    
    <edges>
      <edge from="validate_document" to="split_chunks"><when><always/></when></edge>
      <edge from="split_chunks" to="enrich_metadata"><when><always/></when></edge>
      <edge from="enrich_metadata" to="embed_chunks"><when><always/></when></edge>
      <edge from="embed_chunks" to="index_chunks"><when><always/></when></edge>
      <edge from="index_chunks" to="persist_chunks"><when><always/></when></edge>
      <edge from="persist_chunks" to="emit_event"><when><always/></when></edge>
      <edge from="emit_event" to="done"><when><always/></when></edge>
    </edges>
  </workflow>
  
  <!-- 4. RAG QUERY -->
  <workflow id="rag_query">
    <entry p="query" x="rag" node="validate_query"/>
    
    <nodes>
      <node id="validate_query" kind="transform">
        <schema ref="rag_query_request"/>
      </node>
      <node id="embed_query" kind="external" op="0x0710"/>
      <node id="search" kind="external" op="0x0602"/>
      <node id="build_context" kind="transform"/>
      <node id="generate_answer" kind="external" op="0x0800"/>
      <node id="format_response" kind="transform"/>
      <node id="persist_query" kind="external" op="0x0901"/>
      <node id="emit_event" kind="signal"/>
      <node id="done" kind="terminal"/>
    </nodes>
    
    <edges>
      <edge from="validate_query" to="embed_query"><when><always/></when></edge>
      <edge from="embed_query" to="search"><when><always/></when></edge>
      <edge from="search" to="build_context"><when><always/></when></edge>
      <edge from="build_context" to="generate_answer"><when><always/></when></edge>
      <edge from="generate_answer" to="format_response"><when><always/></when></edge>
      <edge from="format_response" to="persist_query"><when><always/></when></edge>
      <edge from="persist_query" to="emit_event"><when><always/></when></edge>
      <edge from="emit_event" to="done"><when><always/></when></edge>
    </edges>
  </workflow>
  
  <!-- TEMPLATES -->
  <templates>
    <template id="rag_answer">
      <![CDATA[
      <div class="rag-response">
        <div class="answer">
          <h3>Answer</h3>
          <p>{{answer}}</p>
        </div>
        <div class="sources">
          <h4>Sources</h4>
          {{#each sources}}
          <div class="source-card" data-score="{{score}}">
            <p>{{content}}</p>
            <span class="meta">Score: {{score}} | Chunk: {{metadata.chunkIndex}}</span>
          </div>
          {{/each}}
        </div>
      </div>
      ]]>
    </template>
    
    <template id="search_results">
      <![CDATA[
      <div class="search-results">
        {{#each results}}
        <div class="result-card" data-on:click="$pxyz('chunk','view',{id:'{{chunk.id}}'})">
          <p class="content">{{chunk.content}}</p>
          <div class="meta">
            <span class="score">{{score}}</span>
            <span class="tags">{{chunk.metadata.tags}}</span>
          </div>
        </div>
        {{/each}}
      </div>
      ]]>
    </template>
  </templates>
</omar>
```

---

## 🔬 MIGRATION SUMMARY

### What Gets Eliminated

| TypeScript Component | OMAR Replacement | Lines Saved |
|---------------------|------------------|-------------|
| `HybridSearch.ts` service class | `hybrid_search` workflow | ~300 |
| `Vectors.ts` functions | IO adapter algorithms | ~250 |
| `rules.ts` business logic | Predicates + algorithms | ~350 |
| Effect/Schema validation | XSD + predicate VM | ~200 |
| Type definitions | Schema definitions | ~150 |
| **TOTAL** | | **~1,250 lines** |

### What Remains

| Component | Location | Purpose |
|-----------|----------|---------|
| IO handlers | `io-browser.ts` | Side effects only (~200 lines) |
| Algorithms | `io-browser.ts` | Pure functions (~300 lines) |
| Graph binary | `graph.bin` | Compiled workflows (~5KB) |
| WAT runtime | `pxyz.wat` | Execution engine (~500 lines) |
| **TOTAL** | | **~1,000 lines auditable** |

---

## ✅ NEXT STEPS

1. **Create `rag-workflows.xml`** with all 4 workflows above
2. **Compile to `rag-graph.bin`** using `compiler.ts`
3. **Implement IO handlers** in `io-browser.ts`
4. **Add algorithms** (merge, rerank, chunk, etc.)
5. **Test each workflow** individually with trace mode
6. **Build Datastar frontend** for RAG queries
7. **Deploy** with hot-reloadable graph binaries

---

**The RAG system is now PXYZ-native. State is a lie. Everything is events or constraints. APIs are coordinate queries.**

🎯 Ready to compile?