#!/usr/bin/env bash
# Generate native android/ios directories for the Free Cake mobile app.
# Prerequisites: Node.js 18+, JDK 17+, Android SDK (for android), Xcode (for ios).
#
# This script creates a temporary RN project and copies its native dirs
# into the existing mobile directory, preserving our src/ code.
set -euo pipefail

MOBILE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
TMP_DIR="$(mktemp -d)"

echo "Creating temporary RN 0.76.3 project in $TMP_DIR ..."
npx react-native@0.76.3 init FreeCakeTmp --version 0.76.3 --skip-install --tmp "$TMP_DIR" 2>/dev/null || true

# Copy native directories if generated
if [ -d "$TMP_DIR/FreeCakeTmp/android" ]; then
    cp -r "$TMP_DIR/FreeCakeTmp/android" "$MOBILE_DIR/android"
    echo "Copied android/ directory."
fi
if [ -d "$TMP_DIR/FreeCakeTmp/ios" ]; then
    cp -r "$TMP_DIR/FreeCakeTmp/ios" "$MOBILE_DIR/ios"
    echo "Copied ios/ directory."
fi

# Copy necessary native config files
for f in Gemfile .ruby-version babel.config.js metro.config.js react-native.config.js; do
    if [ -f "$TMP_DIR/FreeCakeTmp/$f" ]; then
        cp "$TMP_DIR/FreeCakeTmp/$f" "$MOBILE_DIR/"
        echo "Copied $f"
    fi
done

rm -rf "$TMP_DIR"
echo "Done. Run 'cd $MOBILE_DIR && npm install && npx pod-install ios' to complete setup."
