//! `AivoGuard` library crate (foundation stage).
//!
//! This crate currently exists only to anchor the Cargo workspace so the
//! repository remains buildable, testable, and CI-ready during bootstrap.
//!
//! Product scope, Gate 0, Gate 1, and economic implementation are **not**
//! present here. Domain modules must not be invented ahead of frozen
//! specifications. See `docs/design/SPECIFICATION-POLICY.md`.

#![forbid(unsafe_code)]

#[cfg(test)]
mod foundation_tests {
    #[test]
    fn workspace_anchor_package_identity() {
        // Smoke test: confirms the foundation crate links and exposes the
        // expected package identity under `cargo test`. No product behavior
        // is asserted.
        assert_eq!(env!("CARGO_PKG_NAME"), "aivoguard");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.0.0");
    }
}
