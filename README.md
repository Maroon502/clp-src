# Clp-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

Clp-src crate is a *-src crate. This links [Clp] libraries to executable build by cargo, but does not provide Rust bindings. [Clp] build with [CoinUtils] ([CoinUtils-src]), [Osi] ([Osi-src])(Optional) support.

By this package, you don't need to worry about installing Clp in the system, and it's a package for **all platforms**.

Clp (Coin-or linear programming) is an open-source linear programming solver. It is primarily meant to be used as a callable library, but a basic, stand-alone executable version is also available.

## Usage
Just add the following to your `Cargo.toml`:

```toml
[dependencies]
clp-src = "0.2"
```

## Configuration

The following Cargo features are supported:

* `default` to `osiclp` and `clpsolver` feature;
* `osiclp` to build with Osi supported;
* `clpsolver` to build `ClpSolver` lib and crate the api for `Rust`. If you do not use `Clp` directly, you can disable this feature to reduce the build time;

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_COINUTILS_STATIC` to link to CoinUtils statically;
* `CARGO_COINUTILS_SYSTEM` to link to CoinUtils system library;
* `CARGO_OSI_STATIC` to link to Osi statically if `with_osi` feature is enabled;
* `CARGO_OSI_SYSTEM` to link to Osi system library if `with_osi` feature is enabled;
* `CARGO_CLP_STATIC` to link to Clp statically;
* `CARGO_CLP_SYSTEM` to link to Clp system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

## Windows and vcpkg

On Windows, if `${LIB_NAME}_SYSTEM` is set to `1`, `clp-src` will use 
[vcpkg] to find Clp. Before building, you must have the correct Clp 
installed for your target triplet and kind of linking. For instance,
to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install
 `clp` for the `x64-windows` triplet:

```sh
vcpkg install clp --triplet x64-windows
```

To link Clp statically, install `clp` for the `x64-windows-static-md` triplet:

```sh
vcpkg install clp --triplet x64-windows-static-md
```

To link Clp and C Runtime (CRT) statically, install `clp` for the `x64-windows-static` triplet:

```sh
vcpkg install clp --triplet x64-windows-static
```

and build with `+crt-static` option

```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

you can compile it for the other target by providing the `--target` option to 
`cargo build`. 


| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-linux-androideabi`            | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-gnu`              | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[CoinUtils]: https://github.com/coin-or/CoinUtils
[Osi]: https://github.com/coin-or/Osi
[Clp]: https://github.com/coin-or/Clp

[CoinUtils-src]: https://github.com/Maroon502/coinutils-src
[Osi-src]: https://github.com/Maroon502/osi-src

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/clp-src/badge.svg
[documentation-url]: https://docs.rs/clp-src
[package-img]: https://img.shields.io/crates/v/clp-src.svg
[package-url]: https://crates.io/crates/clp-src
[license-img]: https://img.shields.io/crates/l/clp-src.svg
[license-url]: https://github.com/Maroon502/clp-src/blob/master/LICENSE.md