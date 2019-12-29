[![Crates.io](https://img.shields.io/crates/v/tynm.svg)](https://crates.io/crates/tynm)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/azriel91/tynm?branch=master&svg=true)](https://ci.appveyor.com/project/azriel91/tynm/branch/master)
[![Build Status](https://travis-ci.org/azriel91/tynm.svg?branch=master)](https://travis-ci.org/azriel91/tynm)
[![Coverage Status](https://codecov.io/gh/azriel91/tynm/branch/master/graph/badge.svg)](https://codecov.io/gh/azriel91/tynm)

# Tynm -- Type Name

Returns type names with a specifiable number of module segments as a `String`.

```rust
// === std library === //
assert_eq!(
    std::any::type_name::<Option<String>>(),
    "core::option::Option<alloc::string::String>",
);

// === tynm === //
// Simple type name:
assert_eq!(tynm::type_name::<Option<String>>(), "Option<String>",);

// Type name with 1 module segment, starting from the most significant module.
assert_eq!(
    tynm::type_namem::<Option<String>>(1),
    "core::..::Option<alloc::..::String>",
);

// Type name with 1 module segment, starting from the least significant module.
assert_eq!(
    tynm::type_namen::<Option<String>>(1),
    "..::option::Option<..::string::String>",
);

// Type name with 1 module segment from both the most and least significant modules.
#[rustfmt::skip]
mod rust_out { pub mod two { pub mod three { pub struct Struct; } } }
assert_eq!(
    tynm::type_namemn::<rust_out::two::three::Struct>(1, 1),
    "rust_out::..::three::Struct",
);
```

## Motivation

The [`std::any::type_name`] function stabilized in Rust 1.38 returns the fully qualified type
name with all module segments. This can be difficult to read in error messages, especially for
type-parameterized types.

Often, the simple type name is more readable, and enough to distinguish the type referenced in
an error.

[`std::any::type_name`]: https://doc.rust-lang.org/std/any/fn.type_name.html

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
