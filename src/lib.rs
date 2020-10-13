// #![deny(missing_docs, missing_debug_implementations)]

//! Returns type names with a specifiable number of module segments as a `String`.
//!
//! ```rust
//! #[rustfmt::skip]
//! assert_eq!(
//!     std::any::type_name::<Option<String>>(), "core::option::Option<alloc::string::String>"
//! );
//!
//! #[rustfmt::skip]
//! let tuples = vec![
//!     (tynm::type_name::<Option<String>>(),    "Option<String>"),
//!     (tynm::type_namem::<Option<String>>(1),  "core::..::Option<alloc::..::String>"),
//!     (tynm::type_namen::<Option<String>>(1),  "..::option::Option<..::string::String>"),
//!     // 1 segment from most and least significant modules.
//!     (tynm::type_namemn::<rust_out::two::three::Struct>(1, 1), "rust_out::..::three::Struct"),
//!     // traits
//!     (tynm::type_name::<dyn core::fmt::Debug>(), "dyn Debug"),
//! ];
//!
//! tuples
//!     .iter()
//!     .for_each(|(left, right)| assert_eq!(left, right));
//!
//! # #[rustfmt::skip]
//! # mod rust_out { pub mod two { pub mod three { pub struct Struct; } } }
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

pub use crate::types::{TypeName, TypeNameDisplay};

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
pub fn type_name<T>() -> String
where
    T: ?Sized,
{
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
pub fn type_namem<T>(m: usize) -> String
where
    T: ?Sized,
{
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
pub fn type_namen<T>(n: usize) -> String
where
    T: ?Sized,
{
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
pub fn type_namemn<T>(m: usize, n: usize) -> String
where
    T: ?Sized,
{
    let type_name_qualified = std::any::type_name::<T>();

    let type_name = TypeName::from(type_name_qualified);
    type_name.as_str_mn(m, n)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{type_name, type_namem, type_namemn, type_namen, TypeName};

    #[test]
    fn type_name_primitives() {
        assert_eq!(type_name::<usize>(), "usize");
        assert_eq!(type_name::<u8>(), "u8");
        assert_eq!(type_name::<u16>(), "u16");
        assert_eq!(type_name::<u32>(), "u32");
        assert_eq!(type_name::<u64>(), "u64");
        assert_eq!(type_name::<u128>(), "u128");

        assert_eq!(type_name::<isize>(), "isize");
        assert_eq!(type_name::<i8>(), "i8");
        assert_eq!(type_name::<i16>(), "i16");
        assert_eq!(type_name::<i32>(), "i32");
        assert_eq!(type_name::<i64>(), "i64");
        assert_eq!(type_name::<i128>(), "i128");

        assert_eq!(type_name::<f32>(), "f32");
        assert_eq!(type_name::<f64>(), "f64");

        assert_eq!(type_name::<bool>(), "bool");
        assert_eq!(type_name::<char>(), "char");

        assert_eq!(type_name::<HashMap<u32, String>>(), "HashMap<u32, String>");
    }

    #[test]
    fn type_name_array() {
        assert_eq!(type_name::<[u32; 3]>(), "[u32; 3]");
        assert_eq!(type_name::<[Option<String>; 3]>(), "[Option<String>; 3]");
    }

    #[test]
    fn type_name_slice() {
        dbg!(crate::TypeName::from(std::any::type_name::<&[u32]>()));
        assert_eq!(type_name::<&[u32]>(), "&[u32]");
    }

    #[test]
    fn type_name_unit_tuple() {
        assert_eq!(type_name::<()>(), "()");
        assert_eq!(type_name::<(Option<String>,)>(), "(Option<String>,)");
        assert_eq!(
            type_name::<(Option<String>, u32)>(),
            "(Option<String>, u32)"
        );
    }

    #[test]
    fn type_name_reference() {
        assert_eq!(type_name::<&str>(), "&str");

        assert_eq!(type_name::<&Option<String>>(), "&Option<String>");
        assert_eq!(type_name::<Option<&String>>(), "Option<&String>");
    }

    #[test]
    fn type_name_display() {
        use std::sync::atomic::AtomicI8;

        type T<'a> = Box<HashMap<Option<String>, &'a AtomicI8>>;

        let tn: TypeName = TypeName::new::<T>();

        let display = tn.as_display();
        let string = format!("{}", display);
        assert_eq!(type_name::<T>(), string);
    }

    #[test]
    fn type_name_display_mn() {
        use std::sync::atomic::AtomicI8;

        type T<'a> = Box<HashMap<Option<String>, &'a AtomicI8>>;

        let tn: TypeName = TypeName::new::<T>();

        let display = tn.as_display_mn(1, 0);
        let string = format!("{}", display);
        assert_eq!(type_namemn::<T>(1, 0), string);
    }

    #[test]
    fn type_name_usize_mn() {
        assert_eq!(type_namem::<usize>(std::usize::MAX), "::usize");
        assert_eq!(
            type_namemn::<usize>(std::usize::MAX, std::usize::MAX),
            "::usize"
        );
    }

    #[test]
    fn type_name_unsized() {
        assert_eq!(type_name::<dyn core::fmt::Debug>(), "dyn Debug");
    }

    #[test]
    fn type_name_unsized_mn() {
        assert_eq!(type_namem::<dyn core::fmt::Debug>(1), "dyn core::..::Debug");
        assert_eq!(type_namen::<dyn core::fmt::Debug>(1), "dyn ..::fmt::Debug");
        assert_eq!(
            type_namemn::<dyn core::fmt::Debug>(0, 1),
            "dyn ..::fmt::Debug"
        );
    }
}
