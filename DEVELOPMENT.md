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
bumping the `external/skia` submodule, or after `native-shim/args.gn` or the
`buildSkia` ninja target list in `build.gradle.kts` change (both happen when
new Skia modules get bound). Once Skia is built, iterating on the Rust/Kotlin
sides only needs `./gradlew test -Pskialin.buildNative=true`.

`cargo build`/`./gradlew test -Pskialin.buildNative=true` fail with
"could not find native static library X" if `external/skia/out/Release`
doesn't have every static lib `rust/skialin-sys/build.rs` links against --
rerun `buildSkia` first.

## Publishing

`./gradlew publish -Pskialin.buildNative=true` publishes the API jar, sources
jar, and a `natives-<os>-<arch>` classifier jar for the host platform to a
local repository under `build/publishing-repo`.
