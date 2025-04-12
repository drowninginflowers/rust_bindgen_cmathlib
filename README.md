# rust_bindgen_cmathlib: Learning Rust FFI with C

This is a small educational project I built to teach myself how Rust can interoperate with C through **FFI (Foreign Function Interface)**. It walks through compiling C code, generating Rust bindings automatically, and wrapping unsafe FFI calls in safe, idiomatic Rust functions.

---

## 🎯 Why Though?

I wanted a clean, working example of how Rust talks to C — something minimal, easy to understand, and easy to build on. This covers the basics:

- Compiling C code with [`cc`](https://crates.io/crates/cc)
- Generating bindings from a C header using [`bindgen`](https://crates.io/crates/bindgen)
- Wrapping C functions in safe Rust
- Writing tests to validate the whole flow

---

## 🗂️ Project Layout

```
rust_bindgen_cmathlib/
├── build.rs            # Compiles C and generates bindings
├── cmathlib/           # Tiny C math library (mathlib.c / mathlib.h)
├── src/
│   ├── lib.rs          # Entry point
│   ├── bindings.rs     # Includes bindgen output
│   └── mathlib_rust.rs # Safe Rust wrappers around C functions
├── tests/              # Integration tests for the wrappers
│   └── ffi_tests.rs
├── Cargo.toml
└── README.md
```

---

## ⚙️  How It Works

1. `build.rs` compiles the C code and uses `bindgen` to generate bindings from `mathlib.h`.
2. The generated bindings are written to `OUT_DIR` and included at compile time.
3. Rust wrapper functions in `mathlib_rust.rs` call the unsafe bindings safely.
4. Integration tests confirm everything works.

---

## 🧪 Running the Tests

```bash
cargo test
```
