//! Shape Registry (P-Dimension Types)
//!
//! Mirrors TypeScript ShapeRegistryService for cross-language compatibility.
//! MUST generate identical default projections (All/Active/Recent) as TS.
//!
//! Reference: adherify/server/src/lib/ShapeRegistry.ts

use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::collections::HashMap;

use crate::runtime::pxyz::Pxyz;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeStatus {
    Active,
    Deprecated,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeRegistryOptions {
    pub pxyz: Pxyz,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeProjection {
    pub name: String,
    pub query: String,
    pub index_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredShape {
    pub name: String,
    pub options: ShapeRegistryOptions,
    pub status: ShapeStatus,
    pub projections: Vec<ShapeProjection>,
}

#[derive(Debug, thiserror::Error)]
pub enum ShapeError {
    #[error("shape name cannot be empty")]
    EmptyName,
    #[error("shape name must match ^[a-zA-Z][a-zA-Z0-9_]*$")]
    InvalidName,
    #[error("shape '{0}' not found")]
    NotFound(String),
    #[error("shape '{0}' already registered")]
    AlreadyRegistered(String),
    #[error("cannot remove active shape '{0}' without force flag")]
    CannotRemoveActive(String),
    #[error("validation failed for shape '{0}': {1}")]
    Validation(String, String),
}

pub trait ShapeRegistryLike {
    fn register(&mut self, shape: RegisteredShape) -> Result<(), ShapeError>;
    fn get(&self, name: &str) -> Result<&RegisteredShape, ShapeError>;
    fn validate(&self, name: &str, data: &Json) -> Result<Json, ShapeError>;
}

pub struct ShapeRegistry {
    shapes: HashMap<String, RegisteredShape>,
}

impl ShapeRegistry {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
        }
    }

    /// Validate shape name matches TypeScript rules
    ///
    /// Must match: ^[a-zA-Z][a-zA-Z0-9_]*$
    /// - Starts with letter
    /// - Followed by letters, digits, or underscores
    fn validate_name(name: &str) -> Result<(), ShapeError> {
        if name.trim().is_empty() {
            return Err(ShapeError::EmptyName);
        }

        let mut chars = name.chars();
        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() => {}
            _ => return Err(ShapeError::InvalidName),
        }

        for c in chars {
            if !(c.is_ascii_alphanumeric() || c == '_') {
                return Err(ShapeError::InvalidName);
            }
        }

        Ok(())
    }

    /// Generate default projections matching TypeScript ShapeRegistry
    ///
    /// CRITICAL: Must produce identical projection names and queries as TS.
    ///
    /// TypeScript generates:
    /// - `{ShapeName}All`: SELECT * FROM {ShapeName}
    /// - `{ShapeName}Active`: SELECT * FROM {ShapeName} WHERE status != 'deleted'
    /// - `{ShapeName}Recent`: SELECT * FROM {ShapeName} ORDER BY updatedAt DESC LIMIT 100
    ///
    /// Index fields:
    /// - All: ["id", "createdAt"]
    /// - Active: ["status", "updatedAt"]
    /// - Recent: ["updatedAt"]
    fn generate_default_projections(name: &str) -> Vec<ShapeProjection> {
        vec![
            ShapeProjection {
                name: format!("{name}All"),
                query: format!("SELECT * FROM {name}"),
                index_fields: vec!["id".into(), "createdAt".into()],
            },
            ShapeProjection {
                name: format!("{name}Active"),
                query: format!("SELECT * FROM {name} WHERE status != 'deleted'"),
                index_fields: vec!["status".into(), "updatedAt".into()],
            },
            ShapeProjection {
                name: format!("{name}Recent"),
                query: format!("SELECT * FROM {name} ORDER BY updatedAt DESC LIMIT 100"),
                index_fields: vec!["updatedAt".into()],
            },
        ]
    }
}

impl ShapeRegistryLike for ShapeRegistry {
    fn register(&mut self, mut shape: RegisteredShape) -> Result<(), ShapeError> {
        Self::validate_name(&shape.name)?;

        if self.shapes.contains_key(&shape.name) {
            return Err(ShapeError::AlreadyRegistered(shape.name.clone()));
        }

        // Generate default projections (matches TS behavior)
        shape.projections = Self::generate_default_projections(&shape.name);

        self.shapes.insert(shape.name.clone(), shape);
        Ok(())
    }

    fn get(&self, name: &str) -> Result<&RegisteredShape, ShapeError> {
        self.shapes
            .get(name)
            .ok_or_else(|| ShapeError::NotFound(name.to_string()))
    }

    fn validate(&self, name: &str, _data: &Json) -> Result<Json, ShapeError> {
        // Verify shape exists
        let _shape = self.get(name)?;

        // TODO: Wire in real schema validation
        // For now, just check shape exists (matches TS stub behavior)
        Ok(Json::Null)
    }
}

impl Default for ShapeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name_accepts_valid() {
        assert!(ShapeRegistry::validate_name("LoanApplication").is_ok());
        assert!(ShapeRegistry::validate_name("Contact").is_ok());
        assert!(ShapeRegistry::validate_name("User_Profile").is_ok());
        assert!(ShapeRegistry::validate_name("A1").is_ok());
    }

    #[test]
    fn test_validate_name_rejects_empty() {
        assert!(matches!(
            ShapeRegistry::validate_name(""),
            Err(ShapeError::EmptyName)
        ));
        assert!(matches!(
            ShapeRegistry::validate_name("   "),
            Err(ShapeError::EmptyName)
        ));
    }

    #[test]
    fn test_validate_name_rejects_invalid() {
        // Must start with letter
        assert!(matches!(
            ShapeRegistry::validate_name("1InvalidStart"),
            Err(ShapeError::InvalidName)
        ));
        assert!(matches!(
            ShapeRegistry::validate_name("_InvalidStart"),
            Err(ShapeError::InvalidName)
        ));

        // No special chars except underscore
        assert!(matches!(
            ShapeRegistry::validate_name("Invalid-Name"),
            Err(ShapeError::InvalidName)
        ));
        assert!(matches!(
            ShapeRegistry::validate_name("Invalid.Name"),
            Err(ShapeError::InvalidName)
        ));
    }

    #[test]
    fn test_generate_default_projections_names() {
        let projections = ShapeRegistry::generate_default_projections("LoanApplication");

        assert_eq!(projections.len(), 3);
        assert_eq!(projections[0].name, "LoanApplicationAll");
        assert_eq!(projections[1].name, "LoanApplicationActive");
        assert_eq!(projections[2].name, "LoanApplicationRecent");
    }

    #[test]
    fn test_generate_default_projections_queries() {
        let projections = ShapeRegistry::generate_default_projections("Contact");

        assert_eq!(projections[0].query, "SELECT * FROM Contact");
        assert_eq!(projections[1].query, "SELECT * FROM Contact WHERE status != 'deleted'");
        assert_eq!(projections[2].query, "SELECT * FROM Contact ORDER BY updatedAt DESC LIMIT 100");
    }

    #[test]
    fn test_generate_default_projections_indexes() {
        let projections = ShapeRegistry::generate_default_projections("User");

        assert_eq!(projections[0].index_fields, vec!["id", "createdAt"]);
        assert_eq!(projections[1].index_fields, vec!["status", "updatedAt"]);
        assert_eq!(projections[2].index_fields, vec!["updatedAt"]);
    }

    #[test]
    fn test_register_creates_projections() {
        let mut registry = ShapeRegistry::new();

        let shape = RegisteredShape {
            name: "LoanApplication".into(),
            options: ShapeRegistryOptions {
                pxyz: Pxyz::new("LoanApplication", "register", "default", "2025-01-01T00:00:00Z"),
                tags: vec![],
            },
            status: ShapeStatus::Active,
            projections: vec![], // Will be filled by register
        };

        registry.register(shape).unwrap();

        let registered = registry.get("LoanApplication").unwrap();
        assert_eq!(registered.projections.len(), 3);
        assert_eq!(registered.projections[0].name, "LoanApplicationAll");
    }

    #[test]
    fn test_register_rejects_duplicate() {
        let mut registry = ShapeRegistry::new();

        let shape = RegisteredShape {
            name: "Contact".into(),
            options: ShapeRegistryOptions {
                pxyz: Pxyz::new("Contact", "register", "default", "2025-01-01T00:00:00Z"),
                tags: vec![],
            },
            status: ShapeStatus::Active,
            projections: vec![],
        };

        registry.register(shape.clone()).unwrap();

        let result = registry.register(shape);
        assert!(matches!(result, Err(ShapeError::AlreadyRegistered(name)) if name == "Contact"));
    }

    #[test]
    fn test_get_returns_not_found() {
        let registry = ShapeRegistry::new();
        let result = registry.get("NonExistent");
        assert!(matches!(result, Err(ShapeError::NotFound(name)) if name == "NonExistent"));
    }

    #[test]
    fn test_validate_requires_shape_exists() {
        let registry = ShapeRegistry::new();
        let data = serde_json::json!({ "test": "value" });

        let result = registry.validate("NonExistent", &data);
        assert!(matches!(result, Err(ShapeError::NotFound(_))));
    }
}
