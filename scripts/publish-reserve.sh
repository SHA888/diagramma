#!/usr/bin/env bash
set -euo pipefail

# =============================================================================
# diagramma — name reservation publish script
#
# Prerequisites:
#   1. cargo login          (https://crates.io/settings/tokens)
#   2. npm login            (https://www.npmjs.com)
#   3. Git repo initialized and pushed to github.com/SHA888/diagramma
#
# crates.io requires publishing in dependency order.
# Scoped npm packages require publishConfig.access = "public" (already set).
# =============================================================================

echo "=== Publishing Rust crates to crates.io ==="
echo "Order matters: core → layout → svg"
echo ""

# Step 1: core (no deps)
echo "→ Publishing diagramma-core..."
cargo publish -p diagramma-core
echo "Waiting 30s for crates.io index propagation..."
sleep 30

# Step 2: layout (depends on core)
echo "→ Publishing diagramma-layout..."
cargo publish -p diagramma-layout
echo "Waiting 30s..."
sleep 30

# Step 3: svg (depends on core + layout)
echo "→ Publishing diagramma-svg..."
cargo publish -p diagramma-svg
echo ""

echo "=== Publishing npm packages ==="

# Bare name reservation first
echo "→ Publishing diagramma (bare name)..."
cd packages/diagramma && npm publish && cd ../..

# Scoped packages (order doesn't matter, no inter-deps yet)
for pkg in wasm theme react bridge; do
  echo "→ Publishing @diagramma/$pkg..."
  cd "packages/$pkg" && npm publish --access public && cd ../..
done

echo ""
echo "=== Done ==="
echo "Verify:"
echo "  https://crates.io/crates/diagramma-core"
echo "  https://crates.io/crates/diagramma-layout"
echo "  https://crates.io/crates/diagramma-svg"
echo "  https://www.npmjs.com/package/diagramma"
echo "  https://www.npmjs.com/package/@diagramma/wasm"
echo "  https://www.npmjs.com/package/@diagramma/theme"
echo "  https://www.npmjs.com/package/@diagramma/react"
echo "  https://www.npmjs.com/package/@diagramma/bridge"
