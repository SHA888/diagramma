# Contributing to diagramma

Thank you for helping shape diagramma — a conversation-aware diagram rendering engine. This project is distributed under the MIT OR Apache-2.0 licenses; by contributing you agree that your submissions may be dual licensed under the same terms.

## Ground rules

1. **Be kind.** Assume good intent and keep discussions respectful.
2. **Design for users.** When in doubt, optimize for clarity, accessibility, and interoperability.
3. **Prefer incremental change.** Small, well-tested contributions are easier to review than large refactors.
4. **Document behavior.** Update README files, architecture docs, and CHANGELOG entries whenever behavior changes.
5. **Sign your commits.** Every contribution must include a [Developer Certificate of Origin](https://developercertificate.org/) (DCO) sign-off using `git commit -s`.

## Getting started

### Prerequisites

- Rust `1.85` or newer (matching the workspace `rust-version`)
- `cargo` and `rustup`
- `pnpm` (preferred) or `npm`
- Node.js `18+`
- `wasm-pack` (for WASM bridge development)

### Repository layout

```
crates/       # Rust workspace members (core, layout, svg)
packages/     # TypeScript/JavaScript packages (wasm, theme, react, bridge, etc.)
apps/         # Playground / demo apps
scripts/      # Utility scripts (publishing, tooling)
```

## Contribution workflow

1. **Fork & branch**
   - Fork `github.com/SHA888/diagramma` and clone locally.
   - Create a feature branch (`feature/<short-description>`).
2. **Set up tooling**
   - Run `pnpm install` at the repo root.
   - Build Rust crates with `cargo check`.
   - If touching WASM, run `wasm-pack build` inside `packages/wasm`.
3. **Make focused changes**
   - Keep commits scoped to a single concern.
   - Update or add tests for new behavior.
4. **Lint & test**
   - `cargo fmt --all && cargo clippy --all-targets --all-features`
   - `cargo test --all`
   - `pnpm lint && pnpm test` (once configured)
5. **Document**
   - Update README/ARCHITECTURE/CHANGELOG sections that describe the change.
6. **Sign off commits**
   - Use `git commit -s -m "feat: add X"` so the DCO trailer is added automatically.
7. **Open a pull request**
   - Fill in the PR template, link related issues, and describe testing performed.

## Reporting issues

Use the GitHub issue templates (`Bug report` or `Feature request`). Provide reproduction steps, expected/actual behavior, and environment info. Logs, screenshots, and minimal repro specs are appreciated.

## Coding standards

- **Rust**: Follow `rustfmt` defaults and address all `clippy` warnings unless a justification is documented.
- **TypeScript/React**: Follow the shared ESLint/Prettier configurations (to be introduced in v0.0.2).
- **Tests**: Prefer deterministic tests; use snapshot/property tests where they add coverage.
- **Commit messages**: Conventional style (`feat:`, `fix:`, `docs:`, `chore:` …).

## Security

If you discover a security vulnerability, please email `security+diagramma@kadeding.com` (placeholder) instead of opening a public issue. We will coordinate a disclosure timeline.

## Questions?

Open a GitHub discussion or ping @SHA888. Thanks for contributing! :sparkles:
