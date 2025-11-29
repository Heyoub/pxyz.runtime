//! WASM Host Environment
//!
//! Implements the 3 host functions that GraphIR WASM bytecode can import:
//! 1. host_emit_event - Emit compliance/audit events
//! 2. host_validate_shape - Validate data against registered shapes
//! 3. host_constraint_hash - Compute canonical YCtx hash
//!
//! # JSON Envelopes
//!
//! All host calls use JSON for simplicity and cross-language compatibility.
//!
//! ## host_emit_event
//! ```json
//! {
//!   "pxyz": {"P": "LoanApplication", "X": "create", "Y": "ltv_basic", "Z": "2025-01-01T00:00:00Z"},
//!   "payload": { ... }
//! }
//! ```
//!
//! ## host_validate_shape
//! ```json
//! {
//!   "shapeName": "LoanApplication",
//!   "data": { "loanAmount": 500000, "ltv": 0.80 }
//! }
//! ```
//!
//! ## host_constraint_hash
//! ```json
//! {
//!   "yctx": { "name": "ltv_basic", "maxLtv": 0.97 }
//! }
//! ```

use serde_json::Value as Json;
use crate::runtime::{ConstraintHash, hash_yctx, ShapeRegistryLike, Pxyz};

#[derive(Debug, thiserror::Error)]
pub enum HostError {
    #[error("json decode failed: {0}")]
    JsonDecode(String),
    #[error("shape error: {0}")]
    Shape(#[from] crate::runtime::shape::ShapeError),
    #[error("event sink error: {0}")]
    EventSink(String),
}

/// Event sink trait for compliance/audit events
///
/// Implementations might write to:
/// - Kafka topic
/// - Database table
/// - File log
/// - In-memory buffer (tests)
pub trait EventSink {
    fn emit(&mut self, coord: Pxyz, payload: Json) -> Result<(), HostError>;
}

/// Host environment providing runtime services to WASM
///
/// This is the bridge between GraphIR bytecode and the CRM runtime.
/// All GraphIR side effects flow through these 3 methods.
pub struct HostEnv<R: ShapeRegistryLike, E: EventSink> {
    pub shapes: R,
    pub events: E,
}

impl<R: ShapeRegistryLike, E: EventSink> HostEnv<R, E> {
    pub fn new(shapes: R, events: E) -> Self {
        Self { shapes, events }
    }

    /// Implements host_emit_event
    ///
    /// JSON contract: `{ "pxyz": {...}, "payload": {...} }`
    ///
    /// # Errors
    /// - JsonDecode if JSON is malformed or missing required fields
    /// - EventSink if event emission fails
    pub fn host_emit_event_json(&mut self, json: &str) -> Result<(), HostError> {
        let v: Json =
            serde_json::from_str(json).map_err(|e| HostError::JsonDecode(e.to_string()))?;

        let pxyz = v
            .get("pxyz")
            .ok_or_else(|| HostError::JsonDecode("missing pxyz field".into()))?;

        let payload = v
            .get("payload")
            .cloned()
            .unwrap_or(Json::Null);

        let coord = Pxyz::from_json(pxyz)
            .map_err(|e| HostError::JsonDecode(format!("invalid pxyz: {e}")))?;

        self.events.emit(coord, payload)
    }

    /// Implements host_validate_shape
    ///
    /// JSON contract: `{ "shapeName": "...", "data": {...} }`
    ///
    /// # Errors
    /// - JsonDecode if JSON is malformed or missing shapeName
    /// - Shape if shape doesn't exist or validation fails
    pub fn host_validate_shape_json(&self, json: &str) -> Result<Json, HostError> {
        let v: Json =
            serde_json::from_str(json).map_err(|e| HostError::JsonDecode(e.to_string()))?;

        let name = v
            .get("shapeName")
            .and_then(|v| v.as_str())
            .ok_or_else(|| HostError::JsonDecode("missing shapeName field".into()))?;

        let data = v
            .get("data")
            .cloned()
            .unwrap_or(Json::Null);

        Ok(self.shapes.validate(name, &data)?)
    }

    /// Implements host_constraint_hash
    ///
    /// JSON contract: `{ "yctx": {...} }`
    ///
    /// # Errors
    /// - JsonDecode if JSON is malformed or missing yctx
    pub fn host_constraint_hash_json(&self, json: &str) -> Result<ConstraintHash, HostError> {
        let v: Json =
            serde_json::from_str(json).map_err(|e| HostError::JsonDecode(e.to_string()))?;

        let ctx = v
            .get("yctx")
            .cloned()
            .ok_or_else(|| HostError::JsonDecode("missing yctx field".into()))?;

        Ok(hash_yctx(&ctx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::shape::{ShapeRegistry, ShapeRegistryLike, RegisteredShape, ShapeRegistryOptions, ShapeStatus};

    struct TestEventSink {
        events: Vec<(Pxyz, Json)>,
    }

    impl TestEventSink {
        fn new() -> Self {
            Self { events: vec![] }
        }
    }

    impl EventSink for TestEventSink {
        fn emit(&mut self, coord: Pxyz, payload: Json) -> Result<(), HostError> {
            self.events.push((coord, payload));
            Ok(())
        }
    }

    #[test]
    fn test_host_emit_event_json() {
        let shapes = ShapeRegistry::new();
        let mut events = TestEventSink::new();
        let mut host = HostEnv::new(shapes, events);

        let json = r#"{
            "pxyz": {"P": "LoanApplication", "X": "create", "Y": "ltv_basic", "Z": "2025-01-01T00:00:00Z"},
            "payload": {"amount": 500000}
        }"#;

        host.host_emit_event_json(json).unwrap();

        assert_eq!(host.events.events.len(), 1);
        assert_eq!(host.events.events[0].0.p, "LoanApplication");
        assert_eq!(host.events.events[0].1.get("amount").and_then(|v| v.as_i64()), Some(500000));
    }

    #[test]
    fn test_host_emit_event_missing_pxyz() {
        let shapes = ShapeRegistry::new();
        let events = TestEventSink::new();
        let mut host = HostEnv::new(shapes, events);

        let json = r#"{"payload": {"amount": 500000}}"#;

        let result = host.host_emit_event_json(json);
        assert!(matches!(result, Err(HostError::JsonDecode(msg)) if msg.contains("missing pxyz")));
    }

    #[test]
    fn test_host_validate_shape_json() {
        let mut shapes = ShapeRegistry::new();
        shapes.register(RegisteredShape {
            name: "LoanApplication".into(),
            options: ShapeRegistryOptions {
                pxyz: Pxyz::new("LoanApplication", "register", "default", "2025-01-01T00:00:00Z"),
                tags: vec![],
            },
            status: ShapeStatus::Active,
            projections: vec![],
        }).unwrap();

        let events = TestEventSink::new();
        let host = HostEnv::new(shapes, events);

        let json = r#"{
            "shapeName": "LoanApplication",
            "data": {"loanAmount": 500000}
        }"#;

        let result = host.host_validate_shape_json(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_host_validate_shape_not_found() {
        let shapes = ShapeRegistry::new();
        let events = TestEventSink::new();
        let host = HostEnv::new(shapes, events);

        let json = r#"{
            "shapeName": "NonExistent",
            "data": {}
        }"#;

        let result = host.host_validate_shape_json(json);
        assert!(matches!(result, Err(HostError::Shape(_))));
    }

    #[test]
    fn test_host_constraint_hash_json() {
        let shapes = ShapeRegistry::new();
        let events = TestEventSink::new();
        let host = HostEnv::new(shapes, events);

        let json = r#"{
            "yctx": {"name": "ltv_basic", "maxLtv": 0.97}
        }"#;

        let result = host.host_constraint_hash_json(json);
        assert!(result.is_ok());

        let hash = result.unwrap();
        assert_eq!(hash.as_str().len(), 8);
    }

    #[test]
    fn test_host_constraint_hash_missing_yctx() {
        let shapes = ShapeRegistry::new();
        let events = TestEventSink::new();
        let host = HostEnv::new(shapes, events);

        let json = r#"{"other": "field"}"#;

        let result = host.host_constraint_hash_json(json);
        assert!(matches!(result, Err(HostError::JsonDecode(msg)) if msg.contains("missing yctx")));
    }

    #[test]
    fn test_host_constraint_hash_is_deterministic() {
        let shapes = ShapeRegistry::new();
        let events = TestEventSink::new();
        let host = HostEnv::new(shapes, events);

        let json = r#"{"yctx": {"name": "test", "value": 123}}"#;

        let hash1 = host.host_constraint_hash_json(json).unwrap();
        let hash2 = host.host_constraint_hash_json(json).unwrap();

        assert_eq!(hash1, hash2);
    }
}
