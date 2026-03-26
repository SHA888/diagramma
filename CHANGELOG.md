# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-03-27

### Fixed

- **Security Vulnerabilities**: Resolved moderate and high severity vulnerabilities in dependencies
  - Fixed picomatch ReDoS vulnerability (GHSA-c2c7-rcm5-vvqj)
  - Fixed brace-expansion process hang vulnerability (GHSA-f886-m6hf-6m8v)
- **CI/CD Improvements**:
  - Made benchmark workflow manual-only to prevent release delays
  - Updated publish workflow to only require Rust CI and TypeScript CI
  - Improved reliability with updated cache actions and toolchain installation

### Changed

- **Dependency Management**: Added pnpm overrides for security patches
- **Release Process**: Streamlined automated publishing without benchmark dependency

## [0.2.0] - 2026-03-24

### Added

- **Flowchart Layout Engine**: Complete hierarchical layout implementation using Sugiyama-style algorithm with layer assignment, barycenter-based crossing minimization, and coordinate assignment with tier capping.
- **Structural Layout Engine**: Tree packing algorithm for containers with recursive layout, padding calculations, and label width estimation.
- **Arrow Routing with Obstacle Avoidance**: Multi-strategy routing system with direct paths, L-bend routing (vertical-first and horizontal-first), and fallback dogleg routing with clearance-based obstacle detection.
- **Layout Benchmarks**: Criterion benchmarks for performance profiling with 10, 50, and 200 node specifications.
- **Comprehensive Test Suite**: Unit tests, snapshot tests, and property-based tests covering all layout algorithms with no-overlap invariants and viewbox containment validation.
- **API Documentation**: Complete documentation with examples for all public layout functions.

### Changed

- **Code Quality Improvements**: Fixed float epsilon precision issues, added comprehensive direction test coverage, cleaned up unused parameters, and added self-referencing edge validation.
- **CI/CD Infrastructure**: Added Rust CI with coverage reporting, TypeScript CI, benchmark workflows, and documentation deployment with GitHub Pages.

### Fixed

- **Edge Cases**: Added validation for self-referencing edges and improved geometric comparisons with appropriate epsilon values.
- **Test Coverage**: Added missing tests for BottomUp and RightLeft layout directions.

## [0.1.0] - 2026-03-21

### Added

- Complete diagram DSL data model in `diagramma-core`: `NodeId`, nodes, edges, containers, elements, color ramps, direction/theme enums, and diagram spec structs/enums (flowchart, structural, illustrative, interactive, `DiagramSpec`).
- Validation + error reporting for duplicate IDs, missing edge refs, container depth limits, and empty specs.
- Serialization + JSON schema support (`schemars`) plus Husky helpers to emit `schemas/diagram-spec.json` for editor tooling.
- Expanded test suite covering serialization round-trips, validation scenarios, and property-based duplicate detection.

### Changed

- Bumped workspace/crate versions to `0.1.0` in preparation for release.

### Fixed

- Ensured publish artifact includes schema files and documentation.

## [0.0.2] - 2026-03-21

### Added

- **Documentation**: `CONTRIBUTING.md` (DCO sign-off), `CODE_OF_CONDUCT.md` (Contributor Covenant), `CHANGELOG.md` (Keep a Changelog format)
- **GitHub templates**: issue templates (bug report, feature request), pull request template
- **CI/CD workflows**: Rust CI (fmt, clippy, check, test), TypeScript CI (typecheck, lint, test), publish workflow (crates.io + npm on tag), Dependabot configuration
- **Tooling configs**: `rustfmt.toml`, `.editorconfig`, `.eslintrc.cjs`, `.prettierrc.json`, `tsconfig.base.json`, `turbo.json`
- **Pre-commit hooks**: Husky-based validation (Rust fmt/lint/check, Prettier format check, TypeScript lint/typecheck)
- **Build orchestration**: Turborepo pipeline for `build`, `test`, `lint`, `typecheck` tasks across workspace

### Changed

- Repository structure cleanup (removed stray extracted folders)
- Upgraded `eslint-plugin-react-hooks` to v7.0.1 for ESLint 9 compatibility
- Added `#[must_use]` attributes to all crate `version()` functions

### Fixed

- Publishing loop in `scripts/publish-reserve.sh` now runs scoped packages independently
- Clippy configuration removed (was causing parser errors; defaults now used)

## [0.0.1] - 2026-03-21

### Added

- Project scaffolding (Cargo workspace + pnpm workspace)
- Dual licensing (MIT OR Apache-2.0)
- Publish automation script for crates + npm packages

[Unreleased]: https://github.com/SHA888/diagramma/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/SHA888/diagramma/releases/tag/v0.0.1
