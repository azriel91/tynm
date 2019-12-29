use std::fmt::{Error, Write};

use crate::parser;

/// Organizes type name string into distinct parts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeName<'s> {
    /// Module path of this type.
    pub(crate) module_path: Vec<&'s str>,
    /// Simple type name, excluding type parameters.
    pub(crate) simple_name: &'s str,
    /// Type parameters to this type.
    pub(crate) type_params: Vec<TypeName<'s>>,
}

impl<'s> TypeName<'s> {
    /// Returns the module path of the type.
    pub fn module_path(&self) -> &[&'s str] {
        &self.module_path
    }

    /// Returns the simple name of the type, excluding type parameters.
    pub fn simple_name(&self) -> &'s str {
        self.simple_name
    }

    /// Returns the type parameters to the type.
    pub fn type_params(&self) -> &[TypeName<'s>] {
        &self.type_params
    }

    /// Returns the type name string without any module paths.
    ///
    /// This is equivalent to calling `TypeName::as_str_mn(0, 0);`
    pub fn as_str(&self) -> String {
        self.as_str_mn(0, 0)
    }

    /// Returns the type name string with the given number of module segments.
    ///
    /// If the left and right module segments overlap, the overlapping segments will only be printed
    /// printed once.
    ///
    /// # Parameters
    ///
    /// * `m`: Number of module segments to include, beginning from the left (most significant).
    /// * `n`: Number of module segments to include, beginning from the right (least significant).
    pub fn as_str_mn(&self, m: usize, n: usize) -> String {
        let mut buffer = String::with_capacity(128); // TODO: smarter capacity allocation.

        self.write_str(&mut buffer, m, n)
            .unwrap_or_else(|e| panic!("Failed to write `TypeName` as String. Error: `{}`.", e));

        buffer
    }

    /// Writes the type name string to the given buffer.
    ///
    /// If the left and right module segments overlap, the overlapping segments will only be printed
    /// once.
    ///
    /// # Parameters
    ///
    /// * `buffer`: Buffer to write to.
    /// * `m`: Number of module segments to include, beginning from the left (most significant).
    /// * `n`: Number of module segments to include, beginning from the right (least significant).
    pub fn write_str<W>(&self, buffer: &mut W, m: usize, n: usize) -> Result<(), Error>
    where
        W: Write,
    {
        self.write_module_path(buffer, m, n)
            .and_then(|_| self.write_simple_name(buffer))
            .and_then(|_| self.write_type_params(buffer, m, n))
    }

    /// Writes the module path to the given buffer.
    ///
    /// If the left and right module segments overlap, the overlapping segments will only be printed
    /// once.
    ///
    /// # Parameters
    ///
    /// * `buffer`: Buffer to write to.
    /// * `m`: Number of module segments to include, beginning from the left (most significant).
    /// * `n`: Number of module segments to include, beginning from the right (least significant).
    pub fn write_module_path<W>(&self, buffer: &mut W, m: usize, n: usize) -> Result<(), Error>
    where
        W: Write,
    {
        let module_segment_count = m + n;

        if module_segment_count >= self.module_path.len() {
            // Print full module path
            buffer.write_str(&self.module_path.join("::"))?;
        } else {
            // Print leading and trailing module segments
            buffer.write_str(&self.module_path[0..m].join("::"))?;

            if m > 0 {
                buffer.write_str("::")?;
            }

            // If we skipped any module segments, indicate this with `".."`
            if module_segment_count > 0 {
                buffer.write_str("..")?;
            }

            if n > 0 {
                buffer.write_str("::")?;
            }

            let len = self.module_path.len();
            buffer.write_str(&self.module_path[(len - n)..len].join("::"))?;
        }

        if module_segment_count > 0 {
            buffer.write_str("::")?;
        }

        Ok(())
    }

    /// Writes the simple name to the given buffer.
    ///
    /// # Parameters
    ///
    /// * `buffer`: Buffer to write to.
    pub fn write_simple_name<W>(&self, buffer: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        buffer.write_str(self.simple_name)
    }

    /// Writes type parameters to the given buffer.
    ///
    /// If the left and right module segments overlap, the overlapping segments will only be printed
    /// once.
    ///
    /// # Parameters
    ///
    /// * `buffer`: Buffer to write to.
    /// * `m`: Number of module segments to include, beginning from the left (most significant).
    /// * `n`: Number of module segments to include, beginning from the right (least significant).
    pub fn write_type_params<W>(&self, buffer: &mut W, m: usize, n: usize) -> Result<(), Error>
    where
        W: Write,
    {
        if self.type_params.len() > 0 {
            buffer.write_str("<")?;

            if let Some((first, rest)) = self.type_params.split_first() {
                first.write_str(buffer, m, n)?;
                rest.iter().try_for_each(|type_param| {
                    buffer
                        .write_str(", ")
                        .and_then(|_| type_param.write_str(buffer, m, n))
                })?;
            }

            buffer.write_str(">")?;
        }

        Ok(())
    }
}

impl<'s> From<&'s str> for TypeName<'s> {
    fn from(std_type_name: &'s str) -> Self {
        parser::type_name(std_type_name)
            .map(|(_input, type_name)| type_name)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to parse `TypeName` for input string: `{}`. Error: `{:?}`.",
                    std_type_name, e,
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::TypeName;

    macro_rules! type_name_simple {
        () => {{
            TypeName {
                module_path: vec!["tynm", "types", "tests"],
                simple_name: "Simple",
                type_params: Vec::new(),
            }
        }};
    }
    macro_rules! type_name_type_param_single {
        () => {{
            TypeName {
                module_path: vec!["tynm", "types", "tests"],
                simple_name: "TypeParamSingle",
                type_params: vec![type_name_simple!()],
            }
        }};
    }

    #[test]
    fn parse_simple_struct() {
        let expected = type_name_simple!();

        let actual = TypeName::from(std::any::type_name::<Simple>());

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_type_parameterized_struct() {
        let expected = type_name_type_param_single!();

        let actual = TypeName::from(std::any::type_name::<TypeParamSingle<Simple>>());

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_nested_type_parameterized_struct() {
        let expected = TypeName {
            module_path: vec!["tynm", "types", "tests"],
            simple_name: "TypeParamSingle",
            type_params: vec![type_name_type_param_single!()],
        };

        let actual = TypeName::from(std::any::type_name::<
            TypeParamSingle<TypeParamSingle<Simple>>,
        >());

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_multi_type_parameterized_struct() {
        let expected = TypeName {
            module_path: vec!["tynm", "types", "tests"],
            simple_name: "TypeParamDouble",
            type_params: vec![type_name_simple!(), type_name_simple!()],
        };

        let actual = TypeName::from(std::any::type_name::<TypeParamDouble<Simple, Simple>>());

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_nested_multi_type_parameterized_struct() {
        let expected = TypeName {
            module_path: vec!["tynm", "types", "tests"],
            simple_name: "TypeParamDouble",
            type_params: vec![
                type_name_type_param_single!(),
                type_name_type_param_single!(),
            ],
        };

        let actual = TypeName::from(std::any::type_name::<
            TypeParamDouble<TypeParamSingle<Simple>, TypeParamSingle<Simple>>,
        >());

        assert_eq!(expected, actual);
    }

    struct Simple;
    struct TypeParamSingle<T>(T);
    struct TypeParamDouble<T, U>(T, U);
}
