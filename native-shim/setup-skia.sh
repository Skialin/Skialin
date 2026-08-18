#!/usr/bin/env bash
# Regenerates external/skia/out/Release from the tracked args.gn in this
# directory and builds the static libs skialin-sys links against. Needed
# after cloning fresh, after `git submodule update` bumps the skia
# checkout, or any time out/ (gitignored inside the skia submodule) is gone.
#
# Requires depot_tools on PATH (for ninja) and this checkout's own
# bin/gn.exe (depot_tools' gn.bat expects a gclient-managed checkout,
# which this submodule isn't).
set -euo pipefail

cd "$(dirname "$0")/../external/skia"

mkdir -p out/Release
cp ../../native-shim/args.gn out/Release/args.gn

./bin/gn.exe gen out/Release

ninja -C out/Release \
    skia skparagraph skshaper skunicode_core skunicode_icu skcms \
    libpng zlib expat harfbuzz icu pathops
