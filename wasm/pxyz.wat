(module
  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; PXYZ RUNTIME - The Complete WAT
  ;; ═══════════════════════════════════════════════════════════════════════════
  ;;
  ;; This is THE runtime. ~600 lines. Zero dependencies.
  ;; 
  ;; What it does:
  ;;   1. Loads graph.bin into memory
  ;;   2. Looks up entry point by (P, X) hash
  ;;   3. Traverses nodes, evaluating predicates on edges
  ;;   4. Calls host for all IO (external nodes)
  ;;   5. Enforces safety limits (visited count, predicate steps, call depth)
  ;;
  ;; What it delegates to JS host:
  ;;   - All IO operations (0x0100-0x09FF)
  ;;   - Variable resolution from Y-context
  ;;   - String operations (contains, matches, etc.)
  ;;   - Logging and audit trail
  ;;   - Confirmation status checks
  ;;
  ;; ═══════════════════════════════════════════════════════════════════════════

  ;; ───────────────────────────────────────────────────────────────────────────
  ;; IMPORTS FROM HOST (JS)
  ;; ───────────────────────────────────────────────────────────────────────────
  
  (import "io" "call" (func $io_call (param i32 i32 i32) (result i32)))
  ;; io_call(op_code, payload_ptr, payload_len) -> result_ptr
  
  (import "io" "resolve_var" (func $io_resolve_var (param i32 i32) (result i64)))
  ;; resolve_var(path_ptr, path_len) -> i64 (type << 32 | value)
  
  (import "io" "str_contains" (func $io_str_contains (param i32 i32 i32 i32) (result i32)))
  (import "io" "str_matches" (func $io_str_matches (param i32 i32 i32 i32) (result i32)))
  (import "io" "str_starts_with" (func $io_str_starts_with (param i32 i32 i32 i32) (result i32)))
  (import "io" "str_ends_with" (func $io_str_ends_with (param i32 i32 i32 i32) (result i32)))
  
  (import "io" "is_confirmed" (func $io_is_confirmed (param i32) (result i32)))
  ;; is_confirmed(entity_id) -> 0 or 1
  
  (import "io" "is_human" (func $io_is_human) (result i32))
  ;; is_human() -> 0 or 1 (checks Y-context actor type)
  
  (import "io" "log" (func $io_log (param i32 i32 i32)))
  ;; log(level, msg_ptr, msg_len)
  
  (import "io" "emit_event" (func $io_emit_event (param i32 i32 i32)))
  ;; emit_event(kind, node_id, data_ptr) - for EventBus

  ;; Merge/CRDT host functions (Y-constraint operations for conflict resolution)
  (import "io" "get_timestamp" (func $io_get_timestamp (param i32) (result i64)))
  ;; get_timestamp(value_ref) -> i64 timestamp (for LWW merge policies)

  (import "io" "is_flagged" (func $io_is_flagged (param i32) (result i32)))
  ;; is_flagged(value_ref) -> 0 or 1 (for human review flags)

  (import "io" "get_origin" (func $io_get_origin (param i32) (result i32)))
  ;; get_origin(value_ref) -> string_offset (author/origin id)

  (import "io" "vclock_dominates" (func $io_vclock_dominates (param i32 i32) (result i32)))
  ;; vclock_dominates(a_ref, b_ref) -> 1 if a's vclock dominates b's

  (import "io" "get_merge_field" (func $io_get_merge_field (param i32 i32 i32) (result i64)))
  ;; get_merge_field(selector, path_ptr, path_len) -> i64 (type << 32 | value)
  ;; selector: 0=a, 1=b, 2=candidate
  
  ;; ───────────────────────────────────────────────────────────────────────────
  ;; MEMORY
  ;; ───────────────────────────────────────────────────────────────────────────
  
  (memory (export "memory") 16) ;; 1MB = 16 pages
  
  ;; Layout:
  ;; 0x000000 - 0x00FFFF : Graph data (loaded from graph.bin)
  ;; 0x010000 - 0x01FFFF : Visited bitmap + traversal state
  ;; 0x020000 - 0x02FFFF : IO buffer (for payloads)
  ;; 0x030000 - 0x03FFFF : Predicate VM stack + scratch

  ;; ───────────────────────────────────────────────────────────────────────────
  ;; GLOBALS - Execution State
  ;; ───────────────────────────────────────────────────────────────────────────
  
  (global $graph_loaded (mut i32) (i32.const 0))
  (global $current_node (mut i32) (i32.const 0))
  (global $visited_count (mut i32) (i32.const 0))
  (global $last_error (mut i32) (i32.const 0))
  (global $trace_mode (mut i32) (i32.const 0))
  
  ;; Predicate VM state
  (global $pred_sp (mut i32) (i32.const 0))        ;; stack pointer
  (global $pred_steps (mut i32) (i32.const 0))     ;; step counter
  (global $pred_call_depth (mut i32) (i32.const 0)) ;; nested predicate calls

  ;; ─── ENERGY TRACKING (Physics Integration) ───
  ;; Based on Horowitz 2014: DRAM = 6400× register operation
  ;; Our bounded execution limits ARE energy budgets
  (global $energy_spent (mut i64) (i64.const 0))   ;; total energy units spent
  (global $energy_budget (mut i64) (i64.const 1000000)) ;; MAX_VISITED × 1000

  ;; ───────────────────────────────────────────────────────────────────────────
  ;; CONSTANTS
  ;; ───────────────────────────────────────────────────────────────────────────
  
  ;; Memory regions
  (global $GRAPH_BASE i32 (i32.const 0x000000))
  (global $VISITED_BASE i32 (i32.const 0x010000))
  (global $IO_BASE i32 (i32.const 0x020000))
  (global $STACK_BASE i32 (i32.const 0x030000))
  
  ;; Safety limits
  (global $MAX_VISITED i32 (i32.const 1000))
  (global $MAX_PRED_STEPS i32 (i32.const 256))
  (global $MAX_PRED_DEPTH i32 (i32.const 4))
  (global $MAX_STACK i32 (i32.const 16))
  
  ;; Binary format - Header offsets
  (global $HDR_MAGIC i32 (i32.const 0x00))
  (global $HDR_VERSION_MAJOR i32 (i32.const 0x04))
  (global $HDR_VERSION_MINOR i32 (i32.const 0x06))
  (global $HDR_NODE_COUNT i32 (i32.const 0x08))
  (global $HDR_EDGE_COUNT i32 (i32.const 0x0C))
  (global $HDR_PRED_COUNT i32 (i32.const 0x10))
  (global $HDR_STRING_SIZE i32 (i32.const 0x14))
  (global $HDR_ENTRY_COUNT i32 (i32.const 0x18))
  (global $HDR_NODES_OFF i32 (i32.const 0x40))
  (global $HDR_EDGES_OFF i32 (i32.const 0x44))
  (global $HDR_PREDS_OFF i32 (i32.const 0x48))
  (global $HDR_STRINGS_OFF i32 (i32.const 0x4C))
  (global $HDR_ENTRIES_OFF i32 (i32.const 0x50))
  
  ;; Node kinds
  (global $KIND_TRANSFORM i32 (i32.const 0))
  (global $KIND_EXTERNAL i32 (i32.const 1))
  (global $KIND_RENDER i32 (i32.const 2))
  (global $KIND_SIGNAL i32 (i32.const 3))
  (global $KIND_AUTH i32 (i32.const 4))
  (global $KIND_TERMINAL i32 (i32.const 5))
  (global $KIND_ERROR i32 (i32.const 6))
  
  ;; Node flags (byte at offset 5 in node entry)
  (global $FLAG_ASYNC i32 (i32.const 0x01))
  (global $FLAG_REQUIRES_AUTH i32 (i32.const 0x02))
  (global $FLAG_HAS_SIDE_EFFECTS i32 (i32.const 0x04))
  (global $FLAG_IRREVERSIBLE i32 (i32.const 0x08))
  (global $FLAG_REQUIRES_HUMAN i32 (i32.const 0x10))
  (global $FLAG_CACHEABLE i32 (i32.const 0x20))
  
  ;; Edge flags (u16 at offset 10 in edge entry)
  (global $EDGE_PARALLEL i32 (i32.const 0x0001))
  (global $EDGE_FALLBACK i32 (i32.const 0x0002))
  (global $EDGE_ERROR i32 (i32.const 0x0004))
  
  ;; Predicate opcodes
  (global $OP_NOOP i32 (i32.const 0x00))
  (global $OP_PUSH_INT i32 (i32.const 0x01))
  (global $OP_PUSH_STR i32 (i32.const 0x02))
  (global $OP_LOAD_VAR i32 (i32.const 0x03))
  (global $OP_LOAD_FIELD i32 (i32.const 0x04))
  (global $OP_EQ i32 (i32.const 0x10))
  (global $OP_NEQ i32 (i32.const 0x11))
  (global $OP_GT i32 (i32.const 0x12))
  (global $OP_GTE i32 (i32.const 0x13))
  (global $OP_LT i32 (i32.const 0x14))
  (global $OP_LTE i32 (i32.const 0x15))
  (global $OP_AND i32 (i32.const 0x20))
  (global $OP_OR i32 (i32.const 0x21))
  (global $OP_NOT i32 (i32.const 0x22))
  (global $OP_CONTAINS i32 (i32.const 0x30))
  (global $OP_MATCHES i32 (i32.const 0x31))
  (global $OP_STARTS_WITH i32 (i32.const 0x32))
  (global $OP_ENDS_WITH i32 (i32.const 0x33))
  (global $OP_LEN i32 (i32.const 0x40))
  (global $OP_GET i32 (i32.const 0x41))
  (global $OP_IS_NULL i32 (i32.const 0x42))
  (global $OP_IS_DEFINED i32 (i32.const 0x43))
  (global $OP_IS_CONFIRMED i32 (i32.const 0x44))

  ;; Merge/CRDT opcodes (Y-constraint operations for conflict resolution)
  (global $OP_TIMESTAMP i32 (i32.const 0x50))    ;; pop value ref, push i64 timestamp
  (global $OP_IS_FLAGGED i32 (i32.const 0x51))   ;; pop value ref, push 1 if flagged for review
  (global $OP_ORIGIN i32 (i32.const 0x52))       ;; pop value ref, push origin/author id
  (global $OP_VCLOCK_GT i32 (i32.const 0x53))    ;; pop 2 value refs, push 1 if first dominates second
  (global $OP_MERGE_FIELD i32 (i32.const 0x54))  ;; + 1 byte selector, load merge context field

  (global $OP_CALL_PRED i32 (i32.const 0xF0))
  (global $OP_RET i32 (i32.const 0xFF))
  
  ;; Error codes
  (global $ERR_OK i32 (i32.const 0))
  (global $ERR_NOT_LOADED i32 (i32.const -1))
  (global $ERR_NO_ENTRY i32 (i32.const -2))
  (global $ERR_DEPTH_EXCEEDED i32 (i32.const -3))
  (global $ERR_PRED_STEPS_EXCEEDED i32 (i32.const -4))
  (global $ERR_PRED_DEPTH_EXCEEDED i32 (i32.const -5))
  (global $ERR_STACK_OVERFLOW i32 (i32.const -6))
  (global $ERR_STACK_UNDERFLOW i32 (i32.const -7))
  (global $ERR_INVALID_OPCODE i32 (i32.const -8))
  (global $ERR_CYCLE_DETECTED i32 (i32.const -9))
  (global $ERR_INVALID_MAGIC i32 (i32.const -100))
  (global $ERR_VERSION_MISMATCH i32 (i32.const -101))
  (global $ERR_AUTH_FAILED i32 (i32.const -403))
  (global $ERR_ACTOR_BLOCKED i32 (i32.const -405))
  (global $ERR_IO_FAILED i32 (i32.const -500))
  
  ;; Event kinds (for EventBus)
  (global $EVT_GRAPH_LOADED i32 (i32.const 1))
  (global $EVT_TRAVERSAL_START i32 (i32.const 2))
  (global $EVT_TRAVERSAL_END i32 (i32.const 3))
  (global $EVT_NODE_ENTER i32 (i32.const 4))
  (global $EVT_NODE_EXIT i32 (i32.const 5))
  (global $EVT_EDGE_TAKEN i32 (i32.const 6))
  (global $EVT_PRED_EVAL i32 (i32.const 7))
  (global $EVT_IO_CALL i32 (i32.const 8))
  (global $EVT_FUSE_TRIP i32 (i32.const 9))
  (global $EVT_AUTH_FAIL i32 (i32.const 10))
  (global $EVT_ERROR i32 (i32.const 11))

  ;; Energy costs (Horowitz 2014 scale: register = 1, DRAM = 6400)
  (global $ENERGY_STACK_OP i64 (i64.const 1))      ;; push/pop
  (global $ENERGY_COMPARE i64 (i64.const 1))       ;; predicate comparison
  (global $ENERGY_NODE_VISIT i64 (i64.const 100))  ;; visit a node (L2 access)
  (global $ENERGY_EDGE_EVAL i64 (i64.const 50))    ;; evaluate edge predicate
  (global $ENERGY_IO_CALL i64 (i64.const 100000))  ;; external I/O (DRAM + bus)

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; EXPORTS
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  (func (export "get_version") (result i32)
    (i32.const 1000)) ;; 1.0.0
  
  (func (export "set_trace") (param $mode i32)
    (global.set $trace_mode (local.get $mode)))
  
  (func (export "get_last_error") (result i32)
    (global.get $last_error))
  
  (func (export "get_graph_ptr") (result i32)
    (global.get $GRAPH_BASE))

  ;; ─── ENERGY TRACKING EXPORTS ───

  (func (export "get_energy_spent") (result i64)
    (global.get $energy_spent))

  (func (export "get_energy_budget") (result i64)
    (global.get $energy_budget))

  (func (export "get_energy_efficiency") (result f64)
    ;; Returns 0.0-1.0 (1.0 = used nothing, 0.0 = exhausted budget)
    (if (result f64) (i64.eqz (global.get $energy_budget))
      (then (f64.const 1.0))
      (else
        (f64.sub (f64.const 1.0)
          (f64.div
            (f64.convert_i64_u (global.get $energy_spent))
            (f64.convert_i64_u (global.get $energy_budget)))))))

  ;; Internal: spend energy (called during traversal)
  (func $spend_energy (param $amount i64)
    (global.set $energy_spent
      (i64.add (global.get $energy_spent) (local.get $amount))))

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; GRAPH LOADING
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  (func (export "load_graph") (param $size i32) (result i32)
    (local $magic i32)
    (local $version_major i32)
    
    ;; Validate magic number "PXYZ" = 0x504E5958
    (local.set $magic (i32.load (global.get $GRAPH_BASE)))
    (if (i32.ne (local.get $magic) (i32.const 0x504E5958))
      (then
        (global.set $last_error (global.get $ERR_INVALID_MAGIC))
        (return (i32.const -1))))
    
    ;; Validate version (major must be 1)
    (local.set $version_major 
      (i32.load16_u (i32.add (global.get $GRAPH_BASE) (global.get $HDR_VERSION_MAJOR))))
    (if (i32.ne (local.get $version_major) (i32.const 1))
      (then
        (global.set $last_error (global.get $ERR_VERSION_MISMATCH))
        (return (i32.const -1))))
    
    ;; Mark loaded, clear state
    (global.set $graph_loaded (i32.const 1))
    (global.set $visited_count (i32.const 0))
    (global.set $last_error (global.get $ERR_OK))
    (call $clear_visited)
    
    ;; Emit event
    (if (global.get $trace_mode)
      (then (call $io_emit_event (global.get $EVT_GRAPH_LOADED) (i32.const 0) (i32.const 0))))
    
    (i32.const 0))

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; ENTRY POINT LOOKUP
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  ;; FNV-1a hash for (P, X) lookup
  (func $hash_px (param $p_ptr i32) (param $p_len i32) 
                 (param $x_ptr i32) (param $x_len i32) (result i32)
    (local $hash i32)
    (local $i i32)
    (local $byte i32)
    
    (local.set $hash (i32.const 0x811c9dc5)) ;; FNV offset basis
    
    ;; Hash P
    (local.set $i (i32.const 0))
    (block $break_p
      (loop $loop_p
        (br_if $break_p (i32.ge_u (local.get $i) (local.get $p_len)))
        (local.set $byte (i32.load8_u (i32.add (local.get $p_ptr) (local.get $i))))
        (local.set $hash (i32.xor (local.get $hash) (local.get $byte)))
        (local.set $hash (i32.mul (local.get $hash) (i32.const 0x01000193)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop_p)))
    
    ;; Separator
    (local.set $hash (i32.xor (local.get $hash) (i32.const 0xFF)))
    (local.set $hash (i32.mul (local.get $hash) (i32.const 0x01000193)))
    
    ;; Hash X
    (local.set $i (i32.const 0))
    (block $break_x
      (loop $loop_x
        (br_if $break_x (i32.ge_u (local.get $i) (local.get $x_len)))
        (local.set $byte (i32.load8_u (i32.add (local.get $x_ptr) (local.get $i))))
        (local.set $hash (i32.xor (local.get $hash) (local.get $byte)))
        (local.set $hash (i32.mul (local.get $hash) (i32.const 0x01000193)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop_x)))
    
    (local.get $hash))
  
  ;; Find entry point node ID by (P, X) hash
  (func $find_entry (param $px_hash i32) (result i32)
    (local $entries_off i32)
    (local $entry_count i32)
    (local $i i32)
    (local $entry_ptr i32)
    (local $stored_hash i32)
    
    (local.set $entries_off 
      (i32.load (i32.add (global.get $GRAPH_BASE) (global.get $HDR_ENTRIES_OFF))))
    (local.set $entry_count 
      (i32.load (i32.add (global.get $GRAPH_BASE) (global.get $HDR_ENTRY_COUNT))))
    
    (local.set $i (i32.const 0))
    (block $break
      (loop $loop
        (br_if $break (i32.ge_u (local.get $i) (local.get $entry_count)))
        
        ;; Entry is 8 bytes: [px_hash:4][node_id:4]
        (local.set $entry_ptr 
          (i32.add (global.get $GRAPH_BASE)
            (i32.add (local.get $entries_off)
              (i32.mul (local.get $i) (i32.const 8)))))
        
        (local.set $stored_hash (i32.load (local.get $entry_ptr)))
        (if (i32.eq (local.get $stored_hash) (local.get $px_hash))
          (then
            (return (i32.load (i32.add (local.get $entry_ptr) (i32.const 4))))))
        
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    
    (i32.const -1)) ;; not found

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; VISITED TRACKING
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  (func $clear_visited
    (local $i i32)
    (local.set $i (i32.const 0))
    (block $break
      (loop $loop
        (br_if $break (i32.ge_u (local.get $i) (i32.const 128))) ;; 1024 bits = 128 bytes
        (i32.store8 (i32.add (global.get $VISITED_BASE) (local.get $i)) (i32.const 0))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (global.set $visited_count (i32.const 0))
    (global.set $energy_spent (i64.const 0))) ;; Reset energy tracking
  
  (func $is_visited (param $node_id i32) (result i32)
    (local $byte_idx i32)
    (local $bit_idx i32)
    (local $byte_val i32)
    
    (local.set $byte_idx (i32.shr_u (local.get $node_id) (i32.const 3)))
    (local.set $bit_idx (i32.and (local.get $node_id) (i32.const 7)))
    (local.set $byte_val 
      (i32.load8_u (i32.add (global.get $VISITED_BASE) (local.get $byte_idx))))
    
    (i32.and 
      (i32.shr_u (local.get $byte_val) (local.get $bit_idx))
      (i32.const 1)))
  
  (func $mark_visited (param $node_id i32)
    (local $byte_idx i32)
    (local $bit_idx i32)
    (local $byte_val i32)
    
    (local.set $byte_idx (i32.shr_u (local.get $node_id) (i32.const 3)))
    (local.set $bit_idx (i32.and (local.get $node_id) (i32.const 7)))
    (local.set $byte_val 
      (i32.load8_u (i32.add (global.get $VISITED_BASE) (local.get $byte_idx))))
    
    (i32.store8 
      (i32.add (global.get $VISITED_BASE) (local.get $byte_idx))
      (i32.or (local.get $byte_val) (i32.shl (i32.const 1) (local.get $bit_idx))))
    
    (global.set $visited_count (i32.add (global.get $visited_count) (i32.const 1))))

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; NODE ACCESS
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  ;; Get pointer to node entry (16 bytes each)
  (func $get_node_ptr (param $node_id i32) (result i32)
    (local $nodes_off i32)
    (local.set $nodes_off 
      (i32.load (i32.add (global.get $GRAPH_BASE) (global.get $HDR_NODES_OFF))))
    (i32.add (global.get $GRAPH_BASE)
      (i32.add (local.get $nodes_off)
        (i32.mul (local.get $node_id) (i32.const 16)))))
  
  ;; Node entry layout (16 bytes):
  ;; [0:4] id, [4:1] kind, [5:1] flags, [6:2] op_code, 
  ;; [8:4] data_offset, [12:2] edge_start, [14:2] edge_count
  
  (func $get_node_kind (param $node_ptr i32) (result i32)
    (i32.load8_u (i32.add (local.get $node_ptr) (i32.const 4))))
  
  (func $get_node_flags (param $node_ptr i32) (result i32)
    (i32.load8_u (i32.add (local.get $node_ptr) (i32.const 5))))
  
  (func $get_node_op (param $node_ptr i32) (result i32)
    (i32.load16_u (i32.add (local.get $node_ptr) (i32.const 6))))
  
  (func $get_node_edge_start (param $node_ptr i32) (result i32)
    (i32.load16_u (i32.add (local.get $node_ptr) (i32.const 12))))
  
  (func $get_node_edge_count (param $node_ptr i32) (result i32)
    (i32.load16_u (i32.add (local.get $node_ptr) (i32.const 14))))

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; EDGE ACCESS
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  ;; Get pointer to edge entry (12 bytes each)
  (func $get_edge_ptr (param $edge_idx i32) (result i32)
    (local $edges_off i32)
    (local.set $edges_off 
      (i32.load (i32.add (global.get $GRAPH_BASE) (global.get $HDR_EDGES_OFF))))
    (i32.add (global.get $GRAPH_BASE)
      (i32.add (local.get $edges_off)
        (i32.mul (local.get $edge_idx) (i32.const 12)))))
  
  ;; Edge entry layout (12 bytes):
  ;; [0:4] target_node, [4:4] predicate_id, [8:2] weight, [10:2] flags
  
  (func $get_edge_target (param $edge_ptr i32) (result i32)
    (i32.load (local.get $edge_ptr)))
  
  (func $get_edge_predicate (param $edge_ptr i32) (result i32)
    (i32.load (i32.add (local.get $edge_ptr) (i32.const 4))))
  
  (func $get_edge_flags (param $edge_ptr i32) (result i32)
    (i32.load16_u (i32.add (local.get $edge_ptr) (i32.const 10))))

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; ACTOR CHECK (Human vs Agent)
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  (func $check_actor_allowed (param $node_ptr i32) (result i32)
    (local $flags i32)
    
    (local.set $flags (call $get_node_flags (local.get $node_ptr)))
    
    ;; If REQUIRES_HUMAN flag is set, check with host
    (if (i32.and (local.get $flags) (global.get $FLAG_REQUIRES_HUMAN))
      (then
        (if (i32.eqz (call $io_is_human))
          (then (return (i32.const 0)))))) ;; blocked
    
    (i32.const 1)) ;; allowed

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; ERROR EDGE LOOKUP
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  (func $find_error_edge (param $node_id i32) (result i32)
    (local $node_ptr i32)
    (local $edge_start i32)
    (local $edge_count i32)
    (local $i i32)
    (local $edge_ptr i32)
    (local $edge_flags i32)
    
    (local.set $node_ptr (call $get_node_ptr (local.get $node_id)))
    (local.set $edge_start (call $get_node_edge_start (local.get $node_ptr)))
    (local.set $edge_count (call $get_node_edge_count (local.get $node_ptr)))
    
    (local.set $i (i32.const 0))
    (block $break
      (loop $loop
        (br_if $break (i32.ge_u (local.get $i) (local.get $edge_count)))
        
        (local.set $edge_ptr 
          (call $get_edge_ptr (i32.add (local.get $edge_start) (local.get $i))))
        (local.set $edge_flags (call $get_edge_flags (local.get $edge_ptr)))
        
        ;; Check for ERROR flag
        (if (i32.and (local.get $edge_flags) (global.get $EDGE_ERROR))
          (then
            (return (call $get_edge_target (local.get $edge_ptr)))))
        
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    
    (i32.const -1)) ;; no error edge found

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; PREDICATE VM - Stack Operations
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  (func $stack_push (param $val i32) (result i32)
    (if (i32.ge_u (global.get $pred_sp) (global.get $MAX_STACK))
      (then (return (global.get $ERR_STACK_OVERFLOW))))
    
    (i32.store 
      (i32.add (global.get $STACK_BASE) (i32.mul (global.get $pred_sp) (i32.const 4)))
      (local.get $val))
    (global.set $pred_sp (i32.add (global.get $pred_sp) (i32.const 1)))
    (i32.const 0))
  
  (func $stack_pop (result i32)
    (if (i32.le_s (global.get $pred_sp) (i32.const 0))
      (then (return (i32.const 0)))) ;; underflow returns 0
    
    (global.set $pred_sp (i32.sub (global.get $pred_sp) (i32.const 1)))
    (i32.load 
      (i32.add (global.get $STACK_BASE) (i32.mul (global.get $pred_sp) (i32.const 4)))))
  
  (func $stack_peek (result i32)
    (if (i32.le_s (global.get $pred_sp) (i32.const 0))
      (then (return (i32.const 0))))
    
    (i32.load 
      (i32.add (global.get $STACK_BASE) 
        (i32.mul (i32.sub (global.get $pred_sp) (i32.const 1)) (i32.const 4)))))

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; PREDICATE VM - Evaluation
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  ;; Get predicate bytecode pointer
  (func $get_predicate_ptr (param $pred_id i32) (result i32)
    (local $preds_off i32)
    (local $i i32)
    (local $ptr i32)
    (local $len i32)
    
    (local.set $preds_off 
      (i32.load (i32.add (global.get $GRAPH_BASE) (global.get $HDR_PREDS_OFF))))
    (local.set $ptr (i32.add (global.get $GRAPH_BASE) (local.get $preds_off)))
    
    ;; Walk through predicates (each has 2-byte length prefix)
    (local.set $i (i32.const 0))
    (block $break
      (loop $loop
        (br_if $break (i32.ge_u (local.get $i) (local.get $pred_id)))
        (local.set $len (i32.load16_u (local.get $ptr)))
        (local.set $ptr (i32.add (local.get $ptr) (i32.add (i32.const 2) (local.get $len))))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    
    (local.get $ptr))
  
  ;; Evaluate predicate bytecode
  (func $eval_predicate (param $pred_id i32) (result i32)
    (local $ptr i32)
    (local $len i32)
    (local $end i32)
    (local $op i32)
    (local $a i32)
    (local $b i32)
    (local $str_off i32)
    
    ;; Predicate ID 0 = always true
    (if (i32.eqz (local.get $pred_id))
      (then (return (i32.const 1))))
    
    ;; Check call depth
    (if (i32.ge_u (global.get $pred_call_depth) (global.get $MAX_PRED_DEPTH))
      (then
        (global.set $last_error (global.get $ERR_PRED_DEPTH_EXCEEDED))
        (return (i32.const 0))))
    
    (global.set $pred_call_depth (i32.add (global.get $pred_call_depth) (i32.const 1)))
    
    ;; Reset stack and step counter for this evaluation
    (global.set $pred_sp (i32.const 0))
    (global.set $pred_steps (i32.const 0))
    
    ;; Get bytecode
    (local.set $ptr (call $get_predicate_ptr (local.get $pred_id)))
    (local.set $len (i32.load16_u (local.get $ptr)))
    (local.set $ptr (i32.add (local.get $ptr) (i32.const 2))) ;; skip length
    (local.set $end (i32.add (local.get $ptr) (local.get $len)))
    
    ;; Execute bytecode
    (block $done
      (loop $exec
        (br_if $done (i32.ge_u (local.get $ptr) (local.get $end)))
        
        ;; Check step limit
        (global.set $pred_steps (i32.add (global.get $pred_steps) (i32.const 1)))
        (if (i32.gt_u (global.get $pred_steps) (global.get $MAX_PRED_STEPS))
          (then
            (global.set $last_error (global.get $ERR_PRED_STEPS_EXCEEDED))
            (global.set $pred_call_depth (i32.sub (global.get $pred_call_depth) (i32.const 1)))
            (return (i32.const 0))))
        
        (local.set $op (i32.load8_u (local.get $ptr)))
        (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))
        
        ;; NOOP
        (if (i32.eq (local.get $op) (global.get $OP_NOOP))
          (then (br $exec)))
        
        ;; PUSH_INT: push next 4 bytes as i32
        (if (i32.eq (local.get $op) (global.get $OP_PUSH_INT))
          (then
            (drop (call $stack_push (i32.load (local.get $ptr))))
            (local.set $ptr (i32.add (local.get $ptr) (i32.const 4)))
            (br $exec)))
        
        ;; LOAD_VAR: resolve variable from Y-context
        (if (i32.eq (local.get $op) (global.get $OP_LOAD_VAR))
          (then
            (local.set $str_off (i32.load (local.get $ptr)))
            (local.set $ptr (i32.add (local.get $ptr) (i32.const 4)))
            ;; Call host to resolve, push low 32 bits
            (drop (call $stack_push 
              (i32.wrap_i64 (call $io_resolve_var 
                (i32.add (global.get $GRAPH_BASE) 
                  (i32.add (i32.load (i32.add (global.get $GRAPH_BASE) (global.get $HDR_STRINGS_OFF)))
                    (local.get $str_off)))
                (i32.const 64))))) ;; TODO: actual string length
            (br $exec)))
        
        ;; EQ: pop two, push (a == b)
        (if (i32.eq (local.get $op) (global.get $OP_EQ))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.eq (local.get $a) (local.get $b))))
            (br $exec)))
        
        ;; NEQ
        (if (i32.eq (local.get $op) (global.get $OP_NEQ))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.ne (local.get $a) (local.get $b))))
            (br $exec)))
        
        ;; GT
        (if (i32.eq (local.get $op) (global.get $OP_GT))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.gt_s (local.get $a) (local.get $b))))
            (br $exec)))
        
        ;; GTE
        (if (i32.eq (local.get $op) (global.get $OP_GTE))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.ge_s (local.get $a) (local.get $b))))
            (br $exec)))
        
        ;; LT
        (if (i32.eq (local.get $op) (global.get $OP_LT))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.lt_s (local.get $a) (local.get $b))))
            (br $exec)))
        
        ;; LTE
        (if (i32.eq (local.get $op) (global.get $OP_LTE))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.le_s (local.get $a) (local.get $b))))
            (br $exec)))
        
        ;; AND
        (if (i32.eq (local.get $op) (global.get $OP_AND))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.and (local.get $a) (local.get $b))))
            (br $exec)))
        
        ;; OR
        (if (i32.eq (local.get $op) (global.get $OP_OR))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.or (local.get $a) (local.get $b))))
            (br $exec)))
        
        ;; NOT
        (if (i32.eq (local.get $op) (global.get $OP_NOT))
          (then
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.eqz (local.get $a))))
            (br $exec)))
        
        ;; IS_NULL
        (if (i32.eq (local.get $op) (global.get $OP_IS_NULL))
          (then
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.eqz (local.get $a))))
            (br $exec)))
        
        ;; IS_DEFINED (opposite of IS_NULL)
        (if (i32.eq (local.get $op) (global.get $OP_IS_DEFINED))
          (then
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.ne (local.get $a) (i32.const 0))))
            (br $exec)))
        
        ;; IS_CONFIRMED - delegate to host
        (if (i32.eq (local.get $op) (global.get $OP_IS_CONFIRMED))
          (then
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (call $io_is_confirmed (local.get $a))))
            (br $exec)))

        ;; ─── MERGE/CRDT OPCODES (Y-constraint operations) ───

        ;; TIMESTAMP - get timestamp of value (for LWW merge policies)
        (if (i32.eq (local.get $op) (global.get $OP_TIMESTAMP))
          (then
            (local.set $a (call $stack_pop))
            ;; Push low 32 bits of timestamp (seconds since epoch fits in i32)
            (drop (call $stack_push (i32.wrap_i64 (call $io_get_timestamp (local.get $a)))))
            (br $exec)))

        ;; IS_FLAGGED - check if value is flagged for human review
        (if (i32.eq (local.get $op) (global.get $OP_IS_FLAGGED))
          (then
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (call $io_is_flagged (local.get $a))))
            (br $exec)))

        ;; ORIGIN - get origin/author of value
        (if (i32.eq (local.get $op) (global.get $OP_ORIGIN))
          (then
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (call $io_get_origin (local.get $a))))
            (br $exec)))

        ;; VCLOCK_GT - check if first vclock dominates second
        (if (i32.eq (local.get $op) (global.get $OP_VCLOCK_GT))
          (then
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (call $io_vclock_dominates (local.get $a) (local.get $b))))
            (br $exec)))

        ;; MERGE_FIELD - load field from merge context ($a, $b, or $candidate)
        (if (i32.eq (local.get $op) (global.get $OP_MERGE_FIELD))
          (then
            ;; Read selector byte (0=a, 1=b, 2=candidate)
            (local.set $a (i32.load8_u (local.get $ptr)))
            (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))
            ;; Read string offset for field path
            (local.set $str_off (i32.load (local.get $ptr)))
            (local.set $ptr (i32.add (local.get $ptr) (i32.const 4)))
            ;; Call host to get field value
            (drop (call $stack_push
              (i32.wrap_i64 (call $io_get_merge_field
                (local.get $a)
                (i32.add (global.get $GRAPH_BASE)
                  (i32.add (i32.load (i32.add (global.get $GRAPH_BASE) (global.get $HDR_STRINGS_OFF)))
                    (local.get $str_off)))
                (i32.const 64)))))
            (br $exec)))

        ;; ─── END MERGE/CRDT OPCODES ───

        ;; CONTAINS - delegate to host
        (if (i32.eq (local.get $op) (global.get $OP_CONTAINS))
          (then
            ;; TODO: implement with io_str_contains
            (local.set $b (call $stack_pop))
            (local.set $a (call $stack_pop))
            (drop (call $stack_push (i32.const 0))) ;; placeholder
            (br $exec)))
        
        ;; CALL_PRED: nested predicate call
        (if (i32.eq (local.get $op) (global.get $OP_CALL_PRED))
          (then
            (local.set $a (i32.load16_u (local.get $ptr)))
            (local.set $ptr (i32.add (local.get $ptr) (i32.const 2)))
            (drop (call $stack_push (call $eval_predicate (local.get $a))))
            (br $exec)))
        
        ;; RET: return top of stack
        (if (i32.eq (local.get $op) (global.get $OP_RET))
          (then
            (global.set $pred_call_depth (i32.sub (global.get $pred_call_depth) (i32.const 1)))
            (return (call $stack_pop))))
        
        ;; Unknown opcode
        (global.set $last_error (global.get $ERR_INVALID_OPCODE))
        (global.set $pred_call_depth (i32.sub (global.get $pred_call_depth) (i32.const 1)))
        (return (i32.const 0))))
    
    ;; End of bytecode - return top of stack (or 1 if empty)
    (global.set $pred_call_depth (i32.sub (global.get $pred_call_depth) (i32.const 1)))
    (if (result i32) (i32.gt_s (global.get $pred_sp) (i32.const 0))
      (then (call $stack_pop))
      (else (i32.const 1))))

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; NODE EXECUTION
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  (func $execute_node (param $node_id i32) (result i32)
    (local $node_ptr i32)
    (local $kind i32)
    (local $flags i32)
    (local $op_code i32)
    (local $result i32)
    
    (local.set $node_ptr (call $get_node_ptr (local.get $node_id)))
    (local.set $kind (call $get_node_kind (local.get $node_ptr)))
    (local.set $flags (call $get_node_flags (local.get $node_ptr)))
    (local.set $op_code (call $get_node_op (local.get $node_ptr)))
    
    ;; Emit node enter event
    (if (global.get $trace_mode)
      (then (call $io_emit_event (global.get $EVT_NODE_ENTER) (local.get $node_id) (i32.const 0))))

    ;; Track energy for node visit
    (call $spend_energy (global.get $ENERGY_NODE_VISIT))

    ;; Check actor permission
    (if (i32.eqz (call $check_actor_allowed (local.get $node_ptr)))
      (then
        (global.set $last_error (global.get $ERR_ACTOR_BLOCKED))
        (if (global.get $trace_mode)
          (then (call $io_emit_event (global.get $EVT_AUTH_FAIL) (local.get $node_id) (i32.const 0))))
        (return (i32.const -1))))
    
    ;; Execute based on kind
    (if (i32.eq (local.get $kind) (global.get $KIND_TRANSFORM))
      (then
        ;; Transform: just pass through (validation happens in predicate)
        (local.set $result (i32.const 0))))
    
    (if (i32.eq (local.get $kind) (global.get $KIND_EXTERNAL))
      (then
        ;; External: call host IO (EXPENSIVE - track energy)
        (call $spend_energy (global.get $ENERGY_IO_CALL))
        (if (global.get $trace_mode)
          (then (call $io_emit_event (global.get $EVT_IO_CALL) (local.get $node_id) (local.get $op_code))))
        (local.set $result (call $io_call (local.get $op_code) (global.get $IO_BASE) (i32.const 0)))))
    
    (if (i32.eq (local.get $kind) (global.get $KIND_RENDER))
      (then
        ;; Render: call host with template data
        (local.set $result (call $io_call (i32.const 0xF000) (global.get $IO_BASE) (i32.const 0)))))
    
    (if (i32.eq (local.get $kind) (global.get $KIND_SIGNAL))
      (then
        ;; Signal: update Datastar signals via host
        (local.set $result (call $io_call (i32.const 0xF001) (global.get $IO_BASE) (i32.const 0)))))
    
    (if (i32.eq (local.get $kind) (global.get $KIND_AUTH))
      (then
        ;; Auth: evaluate predicate (stored in data_offset as predicate ID)
        ;; The predicate result IS the auth result
        (local.set $result (i32.const 0)))) ;; predicate checked on edge
    
    (if (i32.eq (local.get $kind) (global.get $KIND_TERMINAL))
      (then
        ;; Terminal: return status code from op_code field
        (local.set $result (local.get $op_code))))
    
    (if (i32.eq (local.get $kind) (global.get $KIND_ERROR))
      (then
        ;; Error: set error state, continue to allow recovery
        (global.set $last_error (local.get $op_code))
        (local.set $result (i32.const 0))))
    
    ;; Emit node exit event
    (if (global.get $trace_mode)
      (then (call $io_emit_event (global.get $EVT_NODE_EXIT) (local.get $node_id) (local.get $result))))
    
    (local.get $result))

  ;; ═══════════════════════════════════════════════════════════════════════════
  ;; GRAPH TRAVERSAL
  ;; ═══════════════════════════════════════════════════════════════════════════
  
  (func (export "execute") (param $p_ptr i32) (param $p_len i32)
                           (param $x_ptr i32) (param $x_len i32) (result i32)
    (local $px_hash i32)
    (local $entry_node i32)
    (local $current i32)
    (local $node_ptr i32)
    (local $kind i32)
    (local $edge_start i32)
    (local $edge_count i32)
    (local $i i32)
    (local $edge_ptr i32)
    (local $pred_id i32)
    (local $target i32)
    (local $exec_result i32)
    (local $found_next i32)
    (local $error_target i32)
    
    ;; Check graph loaded
    (if (i32.eqz (global.get $graph_loaded))
      (then
        (global.set $last_error (global.get $ERR_NOT_LOADED))
        (return (global.get $ERR_NOT_LOADED))))
    
    ;; Hash (P, X) and find entry
    (local.set $px_hash (call $hash_px 
      (local.get $p_ptr) (local.get $p_len)
      (local.get $x_ptr) (local.get $x_len)))
    (local.set $entry_node (call $find_entry (local.get $px_hash)))
    
    (if (i32.lt_s (local.get $entry_node) (i32.const 0))
      (then
        (global.set $last_error (global.get $ERR_NO_ENTRY))
        (return (global.get $ERR_NO_ENTRY))))
    
    ;; Clear state
    (call $clear_visited)
    (global.set $last_error (global.get $ERR_OK))
    (global.set $pred_call_depth (i32.const 0))
    
    ;; Emit traversal start
    (if (global.get $trace_mode)
      (then (call $io_emit_event (global.get $EVT_TRAVERSAL_START) (local.get $entry_node) (i32.const 0))))
    
    ;; Traversal loop
    (local.set $current (local.get $entry_node))
    (block $exit
      (loop $traverse
        ;; Check visited limit
        (if (i32.ge_u (global.get $visited_count) (global.get $MAX_VISITED))
          (then
            (global.set $last_error (global.get $ERR_DEPTH_EXCEEDED))
            (if (global.get $trace_mode)
              (then (call $io_emit_event (global.get $EVT_FUSE_TRIP) (local.get $current) (i32.const 0))))
            (br $exit)))
        
        ;; Check cycle
        (if (call $is_visited (local.get $current))
          (then
            (global.set $last_error (global.get $ERR_CYCLE_DETECTED))
            (br $exit)))
        
        (call $mark_visited (local.get $current))
        (global.set $current_node (local.get $current))
        
        ;; Execute node
        (local.set $exec_result (call $execute_node (local.get $current)))
        
        ;; Check for IO failure
        (if (i32.lt_s (local.get $exec_result) (i32.const 0))
          (then
            ;; Try error edge
            (local.set $error_target (call $find_error_edge (local.get $current)))
            (if (i32.ge_s (local.get $error_target) (i32.const 0))
              (then
                (local.set $current (local.get $error_target))
                (br $traverse))
              (else
                (global.set $last_error (global.get $ERR_IO_FAILED))
                (br $exit)))))
        
        ;; Check if terminal
        (local.set $node_ptr (call $get_node_ptr (local.get $current)))
        (local.set $kind (call $get_node_kind (local.get $node_ptr)))
        (if (i32.eq (local.get $kind) (global.get $KIND_TERMINAL))
          (then
            (if (global.get $trace_mode)
              (then (call $io_emit_event (global.get $EVT_TRAVERSAL_END) (local.get $current) (local.get $exec_result))))
            (return (local.get $exec_result))))
        
        ;; Find next node via edges
        (local.set $edge_start (call $get_node_edge_start (local.get $node_ptr)))
        (local.set $edge_count (call $get_node_edge_count (local.get $node_ptr)))
        (local.set $found_next (i32.const 0))
        
        (local.set $i (i32.const 0))
        (block $found
          (loop $edges
            (br_if $found (i32.ge_u (local.get $i) (local.get $edge_count)))
            
            (local.set $edge_ptr 
              (call $get_edge_ptr (i32.add (local.get $edge_start) (local.get $i))))
            (local.set $pred_id (call $get_edge_predicate (local.get $edge_ptr)))
            
            ;; Track energy for edge evaluation
            (call $spend_energy (global.get $ENERGY_EDGE_EVAL))

            ;; Emit predicate eval event
            (if (global.get $trace_mode)
              (then (call $io_emit_event (global.get $EVT_PRED_EVAL) (local.get $pred_id) (i32.const 0))))

            ;; Evaluate predicate
            (if (call $eval_predicate (local.get $pred_id))
              (then
                (local.set $target (call $get_edge_target (local.get $edge_ptr)))
                
                ;; Emit edge taken event
                (if (global.get $trace_mode)
                  (then (call $io_emit_event (global.get $EVT_EDGE_TAKEN) (local.get $current) (local.get $target))))
                
                (local.set $current (local.get $target))
                (local.set $found_next (i32.const 1))
                (br $found)))
            
            (local.set $i (i32.add (local.get $i) (i32.const 1)))
            (br $edges)))
        
        ;; No edge matched - traversal ends
        (if (i32.eqz (local.get $found_next))
          (then
            (if (global.get $trace_mode)
              (then (call $io_emit_event (global.get $EVT_TRAVERSAL_END) (local.get $current) (i32.const 0))))
            (br $exit)))
        
        (br $traverse)))
    
    (global.get $last_error))
)
