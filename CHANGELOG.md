# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial documentation scaffold (`ARCHITECTURE.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, per-package READMEs)
- Name reservation publish script (`scripts/publish-reserve.sh`)
- Crate stubs: `diagramma-core`, `diagramma-layout`, `diagramma-svg`
- Package stubs: `diagramma`, `@diagramma/wasm`, `@diagramma/theme`, `@diagramma/react`, `@diagramma/bridge`

### Changed

- Repository structure cleanup (removed stray extracted folders)

### Fixed

- Publishing loop in `scripts/publish-reserve.sh` now runs scoped packages independently

## [0.0.1] - 2026-03-21

### Added

- Project scaffolding (Cargo workspace + pnpm workspace)
- Dual licensing (MIT OR Apache-2.0)
- Publish automation script for crates + npm packages

[Unreleased]: https://github.com/SHA888/diagramma/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/SHA888/diagramma/releases/tag/v0.0.1
