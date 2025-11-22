# Macro to CFG

GCC, Clang, and MSVC pre-defined macros, converted to Rust's cfg macros.

# Simple use

- Add `macro2cfg-core` to `[build-dependencies]` (or run `cargo add macro2cfg-core --build`)
- Add `macro2cfg-macros` to `[dependencies]` (or run `cargo add macro2cfg-macros`)
- In your project, add build.rs and add the line `macro2cfg_core::build();`

Now you're good to use.

As of right now, each macro (e.g. `#[__MMX__]`) is just a cfg (e.g. `#[cfg(__MMX__)]`).
