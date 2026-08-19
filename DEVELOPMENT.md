# Building skialin

## Prerequisites

- JDK 25
- Rust (stable)
- Python 3
- A C++ toolchain (MSVC on Windows, clang on macOS/Linux)

## First build

```
git submodule update --init
cd external/skia
python3 tools/git-sync-deps
python3 bin/fetch-gn
python3 bin/fetch-ninja
cd ../..
./gradlew buildSkia
./gradlew test -Pskialin.buildNative=true
```

`buildSkia` regenerates and rebuilds `external/skia/out/Release`; rerun it after
bumping the `external/skia` submodule. Once Skia is built, iterating on the
Rust/Kotlin sides only needs `./gradlew test -Pskialin.buildNative=true`.

## Publishing

`./gradlew publish -Pskialin.buildNative=true` publishes the API jar, sources
jar, and a `natives-<os>-<arch>` classifier jar for the host platform to a
local repository under `build/publishing-repo`.
