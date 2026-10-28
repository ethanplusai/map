#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OPENCODE_DIR="${HOME}/.opencode"
COMMANDS_DIR="${OPENCODE_DIR}/commands"

echo "==> Installing Map commands into ${COMMANDS_DIR}..."
mkdir -p "${COMMANDS_DIR}/map"

# Copy command definitions
cp "${SCRIPT_DIR}/commands/map.md" "${COMMANDS_DIR}/map.md"
cp "${SCRIPT_DIR}/commands/map/"*.md "${COMMANDS_DIR}/map/"

# Ensure the prebuilt binary is executable
if [ -f "${SCRIPT_DIR}/poc-visual-docs/serve" ]; then
  chmod +x "${SCRIPT_DIR}/poc-visual-docs/serve"
fi

echo "==> Map skill and commands successfully installed!"
echo "    Available slash commands in OpenCode:"
echo "      /map"
echo "      /map/init"
echo "      /map/update"
echo "      /map/serve"
