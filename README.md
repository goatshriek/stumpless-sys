[![Linux Builds](https://github.com/goatshriek/stumpless-sys/actions/workflows/linux.yml/badge.svg?branch=latest)](https://github.com/goatshriek/stumpless-sys/actions/workflows/linux.yml)
[![Windows](https://github.com/goatshriek/stumpless-sys/actions/workflows/windows.yml/badge.svg?branch=latest)](https://github.com/goatshriek/stumpless-sys/actions/workflows/windows.yml)
[![Mac](https://github.com/goatshriek/stumpless-sys/actions/workflows/mac.yml/badge.svg?branch=latest)](https://github.com/goatshriek/stumpless-sys/actions/workflows/mac.yml)
[![Apache 2.0 License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Rust Bindings for the Stumpless C API.**

## Stumpless Sys Crate
This crate provides FFI bindings (created via bindgen) of the
[Stumpless](https://github.com/goatshriek/stumpless) library. These bindings are
not very "rusty" though; if you want to use Stumpless from Rust code, check out
the stumpless crate which wraps some key functionality in a more natural
interface.
