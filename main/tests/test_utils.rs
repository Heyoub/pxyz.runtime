//! Test Utilities and DSL
//!
//! Provides macros and helpers for writing concise, readable tests.

/// Test DSL: expect specific diagnostic codes from XML fixtures
///
/// Usage:
/// ```
/// expect_codes!("fixtures/sem004_cycle.xml" => ["SEM004"]);
/// expect_codes!("fixtures/valid.xml" => []); // No diagnostics
/// ```
#[macro_export]
macro_rules! expect_codes {
    ($xml_fixture:literal => [$($code:literal),* $(,)?]) => {{
        let xml = include_str!($xml_fixture);
        let (_ir, _ast, diags) = pxyz::compile_pipeline(xml, false)
            .expect("compile_pipeline should not fail with CompileError");

        let codes: Vec<_> = diags.iter()
            .filter(|d| d.severity == pxyz::Severity::Error)
            .map(|d| d.code.as_str())
            .collect();

        let expected = vec![$($code),*];

        assert_eq!(
            codes, expected,
            "\nFixture: {}\nExpected codes: {:?}\nActual codes: {:?}\nAll diagnostics: {:#?}",
            $xml_fixture, expected, codes, diags
        );
    }};
}

/// Test DSL: expect warnings (non-error diagnostics)
#[macro_export]
macro_rules! expect_warnings {
    ($xml_fixture:literal => [$($code:literal),* $(,)?]) => {{
        let xml = include_str!($xml_fixture);
        let (_ir, _ast, diags) = pxyz::compile_pipeline(xml, false)
            .expect("compile_pipeline should not fail");

        let codes: Vec<_> = diags.iter()
            .filter(|d| d.severity == pxyz::Severity::Warn)
            .map(|d| d.code.as_str())
            .collect();

        let expected = vec![$($code),*];

        assert_eq!(codes, expected, "for {}", $xml_fixture);
    }};
}

/// Assert that two IRs are structurally equivalent
///
/// Ignores ordering and internal IDs that don't affect semantics.
#[allow(dead_code)]
pub fn assert_ir_equivalent(a: &pxyz::compiler::GraphIR, b: &pxyz::compiler::GraphIR) {
    assert_eq!(a.nodes.len(), b.nodes.len(), "Node count mismatch");
    assert_eq!(a.edges.len(), b.edges.len(), "Edge count mismatch");
    assert_eq!(a.entries.len(), b.entries.len(), "Entry count mismatch");

    // Compare node kinds (order-independent) by name
    use std::collections::HashMap;
    let a_kinds: HashMap<_, _> = a.nodes.iter().map(|n| (n.name.clone(), n.kind)).collect();
    let b_kinds: HashMap<_, _> = b.nodes.iter().map(|n| (n.name.clone(), n.kind)).collect();
    assert_eq!(a_kinds, b_kinds, "Node kinds mismatch");
}
