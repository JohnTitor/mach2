[![Latest Version]][crates.io] [![docs]][docs.rs]

A Rust interface to the **user-space** API of the Mach 3.0 kernel exposed in
`/usr/include/mach` that underlies macOS and is linked via `libSystem` (and
`libsystem_kernel`).

This library does not expose the **kernel-space** API of the Mach 3.0 kernel
exposed in
`SDK/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach`. 

That is, if you are writing a kernel-resident device drivers or some other
kernel extensions you have to use something else. The user-space kernel API is
often API-incompatible with the kernel space one, and even in the cases where
they match, they are sometimes ABI incompatible such that using this library
would have **undefined behavior**.

# Usage

Add the following to your `Cargo.toml` to conditionally include mach on those
platforms that support it.

```toml
[target.'cfg(any(target_os = "macos", target_os = "ios"))'.dependencies.mach]
version = "0.3"
```

The following crate features are available:

* **deprecated** (disabled by default): exposes deprecated APIs that have been
  removed from the latest versions of the MacOS SDKs. The behavior of using
  these APIs on MacOS versions that do not support them is undefined (hopefully
  a linker error).

# Platform support

The following table describes the current CI set-up:

| Target                  | Min. Rust | XCode           | build | ctest | run |
|-------------------------|-----------|-----------------|-------|-------|-----|
| `x86_64-apple-darwin`   | 1.33.0    | 10.3.0 - 13.1.0 | ✓     | ✓     | ✓   |
| `aarch64-apple-darwin`  | nightly   | 13.1.0          | ✓     | -     | -   |
| `aarch64-apple-ios`     | nightly   | 13.1.0          | ✓     | -     | -   |
| `aarch64-apple-ios-sim` | nightly   | 13.1.0          | ✓     | -     | -   |
| `x86_64-apple-ios`      | nightly   | 13.1.0          | ✓     | -     | -   |

# License

This project is licensed under either of

* A 2-clause BSD License ([LICENSE-BSD](LICENSE-BSD)), or
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `mach` by you, as defined in the Apache-2.0 license, shall be
triple licensed as above, without any additional terms or conditions.

To locally test the library, run:

```
TARGET=x86_64-apple-darwin RUST_VERSION=nightly ./ci/run.sh
```

where you can replace the `TARGET` and `RUST_VERSION` with the target you
want to test (e.g. `x86_64-apple-darwin`) and the Rust version you want to use for
the tests (e.g. `stable`, `1.33.0`, etc.).

[crates.io]: https://crates.io/crates/mach
[Latest Version]: https://img.shields.io/crates/v/mach.svg
[docs]: https://docs.rs/mach/badge.svg
[docs.rs]: https://docs.rs/mach/
