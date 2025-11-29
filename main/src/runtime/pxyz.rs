//! PXYZ Coordinate System
//!
//! Every event, constraint, and operation in the PXYZ system has a 4D coordinate:
//! - P: Entity/Domain (e.g., "LoanApplication", "Contact")
//! - X: Operation (e.g., "create", "update", "search")
//! - Y: Constraint/Policy (e.g., "ltv_basic", "compliance_check")
//! - Z: Timestamp (ISO 8601)

use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pxyz {
    #[serde(rename = "P")]
    pub p: String,
    #[serde(rename = "X")]
    pub x: String,
    #[serde(rename = "Y")]
    pub y: String,
    #[serde(rename = "Z")]
    pub z: String,
}

impl Pxyz {
    pub fn new(p: impl Into<String>, x: impl Into<String>, y: impl Into<String>, z: impl Into<String>) -> Self {
        Self {
            p: p.into(),
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// Parse from JSON object with P,X,Y,Z fields
    pub fn from_json(value: &Json) -> Result<Self, String> {
        serde_json::from_value(value.clone())
            .map_err(|e| format!("invalid pxyz: {}", e))
    }

    /// Serialize to JSON for host calls
    pub fn to_json(&self) -> Json {
        serde_json::to_value(self).expect("pxyz should always serialize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pxyz_json_roundtrip() {
        let coord = Pxyz::new("LoanApplication", "create", "ltv_basic", "2025-01-01T00:00:00Z");
        let json = coord.to_json();
        let parsed = Pxyz::from_json(&json).unwrap();
        assert_eq!(coord, parsed);
    }

    #[test]
    fn test_pxyz_fields_capitalized() {
        let coord = Pxyz::new("Entity", "Op", "Constraint", "2025-01-01T00:00:00Z");
        let json = coord.to_json();

        assert!(json.get("P").is_some(), "should have P field");
        assert!(json.get("X").is_some(), "should have X field");
        assert!(json.get("Y").is_some(), "should have Y field");
        assert!(json.get("Z").is_some(), "should have Z field");
    }
}
