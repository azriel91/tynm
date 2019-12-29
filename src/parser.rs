use nom::{
    bytes::complete::{tag, take_while},
    character::complete::char,
    multi::separated_list,
    sequence::{delimited, tuple},
    IResult,
};

use crate::types::TypeName;

pub fn is_alphanumeric_underscore(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

pub fn is_lowercase_alphanumeric_underscore(c: char) -> bool {
    c == '_' || c.is_ascii_digit() || c.is_ascii_lowercase()
}

pub fn module_name(input: &str) -> IResult<&str, &str> {
    if let Some(first_char) = input.chars().next() {
        if first_char.is_ascii_alphabetic() && first_char.is_ascii_lowercase() {
            take_while(is_lowercase_alphanumeric_underscore)(input)
        } else {
            Ok((input, ""))
        }
    } else {
        Ok((input, ""))
    }
}

pub fn module_path(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list(tag("::"), module_name)(input).map(|(input, mut module_names)| {
        module_names.retain(|module_name| !module_name.is_empty());
        (input, module_names)
    })
}

pub fn type_simple_name(input: &str) -> IResult<&str, &str> {
    if let Some(first_char) = input.chars().next() {
        if first_char.is_ascii_alphabetic() && first_char.is_ascii_uppercase() {
            take_while(is_alphanumeric_underscore)(input)
        } else {
            Ok((input, ""))
        }
    } else {
        Ok((input, ""))
    }
}

pub fn type_parameters(input: &str) -> IResult<&str, Vec<TypeName>> {
    if input.starts_with('<') {
        delimited(char('<'), separated_list(tag(", "), type_name), char('>'))(input)
    } else {
        Ok((input, Vec::new()))
    }
}

/// Parses a type name.
pub fn type_name(input: &str) -> IResult<&str, TypeName> {
    tuple((module_path, type_simple_name, type_parameters))(input).map(
        |(s, (module_path, simple_name, type_params))| {
            (
                s,
                TypeName {
                    module_path,
                    simple_name,
                    type_params,
                },
            )
        },
    )
}
