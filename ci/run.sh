#!/usr/bin/env sh

set -ex

: "${TARGET?The TARGET environment variable must be set.}"
: "${RUST_VERSION?The RUST_VERSION environment variable must be set.}"

echo "Running tests for target: ${TARGET}"
export RUST_BACKTRACE=1
export RUST_TEST_THREADS=1
export RUST_TEST_NOCAPTURE=1
export CARGO_INCREMENTAL=0
export CARGO_CODEGEN_UNITS=1
export RUSTFLAGS="-C codegen-units=1 "

rustup toolchain install "${RUST_VERSION}" --profile minimal
export RUSTUP_TOOLCHAIN="${RUST_VERSION}"
rustup target add "${TARGET}"

case "${TARGET}" in
    aarch64-apple-ios)
        SDK=iphoneos
        ;;
    *-apple-ios | *-apple-ios-sim)
        SDK=iphonesimulator
        ;;
    *)
        SDK=macosx
        ;;
esac

export SDKROOT
SDKROOT="$(xcrun --sdk "${SDK}" --show-sdk-path)"
xcrun --sdk "${SDK}" --show-sdk-version
if [ -n "${SDK_VERSION:-}" ]; then
    test "$(xcrun --sdk "${SDK}" --show-sdk-version)" = "${SDK_VERSION}"
fi

if [ "${SDK}" = iphonesimulator ] && [ -z "${NORUN:-}" ]; then
    SDKROOT="$(xcrun --sdk macosx --show-sdk-path)" \
        rustc ./ci/deploy_and_run_on_ios_simulator.rs -o ios_cargo_runner --verbose
    CARGO_TARGET_X86_64_APPLE_IOS_RUNNER="$(pwd)/ios_cargo_runner"
    export CARGO_TARGET_X86_64_APPLE_IOS_RUNNER
    export CARGO_TARGET_AARCH64_APPLE_IOS_SIM_RUNNER="${CARGO_TARGET_X86_64_APPLE_IOS_RUNNER}"
fi

# Build w/o std
cargo clean
cargo build --no-default-features --target "${TARGET}" -vv > build_no_std.txt 2>&1 || {
    cat build_no_std.txt
    exit 1
}
cat build_no_std.txt

# Check that the no-std builds are not linked against a libc with default
# features or the std feature enabled:
if grep -Eq -- '--cfg .?feature="(default|std)"' build_no_std.txt; then
    exit 1
fi
# Make sure that the resulting build contains no std symbols
if find target/ -name "*.rlib" -exec nm {} \; | grep "std"; then
    exit 1
fi

# Runs mach's run-time tests:
if [ -z "${NORUN:-}" ]; then
    cargo test --target "${TARGET}" -vv
    cargo test --target "${TARGET}" -vv --features unstable
    cargo test --no-default-features --target "${TARGET}" -vv
fi

# Runs ctest to verify mach's ABI against the system libraries:
if [ -z "${NOCTEST:-}" ]; then
    cargo test --manifest-path mach-test/Cargo.toml --target "${TARGET}" -vv
    cargo test --manifest-path mach-test/Cargo.toml --target "${TARGET}" -vv --features mach2/unstable
    cargo test --no-default-features --manifest-path mach-test/Cargo.toml --target "${TARGET}" -vv
fi
