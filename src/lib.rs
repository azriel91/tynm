// #![deny(missing_docs, missing_debug_implementations)]

//! Returns type names with a specifiable number of module segments as a `String`.
//!
//! ```rust
//! // === std library === //
//! assert_eq!(
//!     std::any::type_name::<Option<String>>(),
//!     "core::option::Option<alloc::string::String>",
//! );
//!
//! // === tynm === //
//! // Simple type name:
//! assert_eq!(tynm::type_name::<Option<String>>(), "Option<String>",);
//!
//! // Type name with 1 module segment, starting from the most significant module.
//! assert_eq!(
//!     tynm::type_namem::<Option<String>>(1),
//!     "core::..::Option<alloc::..::String>",
//! );
//!
//! // Type name with 1 module segment, starting from the least significant module.
//! assert_eq!(
//!     tynm::type_namen::<Option<String>>(1),
//!     "..::option::Option<..::string::String>",
//! );
//!
//! // Type name with 1 module segment from both the most and least significant modules.
//! #[rustfmt::skip]
//! mod rust_out { pub mod two { pub mod three { pub struct Struct; } } }
//! assert_eq!(
//!     tynm::type_namemn::<rust_out::two::three::Struct>(1, 1),
//!     "rust_out::..::three::Struct",
//! );
//! ```
//!
//! # Motivation
//!
//! The [`std::any::type_name`] function stabilized in Rust 1.38 returns the fully qualified type
//! name with all module segments. This can be difficult to read in error messages, especially for
//! type-parameterized types.
//!
//! Often, the simple type name is more readable, and enough to distinguish the type referenced in
//! an error.
//!
//! [`std::any::type_name`]: https://doc.rust-lang.org/std/any/fn.type_name.html

pub use crate::types::TypeName;

mod parser;
mod types;

/// Returns the simple type name.
///
/// # Type Parameters
///
/// * `T`: Type whose simple type name should be returned.
///
/// # Examples
///
/// ```rust
/// assert_eq!(tynm::type_name::<Option<String>>(), "Option<String>",);
/// ```
pub fn type_name<T>() -> String {
    type_namemn::<T>(0, 0)
}

/// Returns the type name with at most `m` most significant module path segments.
///
/// # Parameters
///
/// * `m`: Number of most significant module path segments to include.
///
/// # Type Parameters
///
/// * `T`: Type whose simple type name should be returned.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///     tynm::type_namem::<Option<String>>(1),
///     "core::..::Option<alloc::..::String>",
/// );
/// ```
pub fn type_namem<T>(m: usize) -> String {
    type_namemn::<T>(m, 0)
}

/// Returns the type name with at most `n` least significant module path segments.
///
/// # Parameters
///
/// * `n`: Number of least significant module path segments to include.
///
/// # Type Parameters
///
/// * `T`: Type whose simple type name should be returned.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///     tynm::type_namen::<Option<String>>(1),
///     "..::option::Option<..::string::String>",
/// );
/// ```
pub fn type_namen<T>(n: usize) -> String {
    type_namemn::<T>(0, n)
}

/// Returns the type name with `m` most significant, and `n` least significant module path segments.
///
/// # Parameters
///
/// * `m`: Number of most significant module path segments to include.
/// * `n`: Number of least significant module path segments to include.
///
/// # Type Parameters
///
/// * `T`: Type whose simple type name should be returned.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///     tynm::type_namen::<Option<String>>(1),
///     "..::option::Option<..::string::String>",
/// );
/// ```
pub fn type_namemn<T>(m: usize, n: usize) -> String {
    let type_name_qualified = std::any::type_name::<T>();

    let type_name = TypeName::from(type_name_qualified);
    type_name.as_str_mn(m, n)
}
