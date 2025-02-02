use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::bundle::{PathRequireMode, RequireMode};

pub type RuleProperties = HashMap<String, RulePropertyValue>;

/// In order to be able to weakly-type the properties of any rule, this enum makes it possible to
/// easily use serde to gather the value associated with a property.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RulePropertyValue {
    Boolean(bool),
    String(String),
    Usize(usize),
    Float(f64),
    StringList(Vec<String>),
    RequireMode(RequireMode),
    None,
}

impl From<bool> for RulePropertyValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<&str> for RulePropertyValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<&String> for RulePropertyValue {
    fn from(value: &String) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for RulePropertyValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<usize> for RulePropertyValue {
    fn from(value: usize) -> Self {
        Self::Usize(value)
    }
}

impl From<f64> for RulePropertyValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<&RequireMode> for RulePropertyValue {
    fn from(value: &RequireMode) -> Self {
        match value {
            RequireMode::Path(mode) => {
                if mode == &PathRequireMode::default() {
                    Self::from("path")
                } else {
                    Self::RequireMode(value.clone())
                }
            }
        }
    }
}

impl<T: Into<RulePropertyValue>> From<Option<T>> for RulePropertyValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Self::None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_true() {
        assert_eq!(
            RulePropertyValue::from(true),
            RulePropertyValue::Boolean(true)
        );
    }

    #[test]
    fn from_false() {
        assert_eq!(
            RulePropertyValue::from(false),
            RulePropertyValue::Boolean(false)
        );
    }

    #[test]
    fn from_string() {
        assert_eq!(
            RulePropertyValue::from(String::from("hello")),
            RulePropertyValue::String(String::from("hello"))
        );
    }

    #[test]
    fn from_string_ref() {
        assert_eq!(
            RulePropertyValue::from(&String::from("hello")),
            RulePropertyValue::String(String::from("hello"))
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            RulePropertyValue::from("hello"),
            RulePropertyValue::String(String::from("hello"))
        );
    }

    #[test]
    fn from_usize() {
        assert_eq!(RulePropertyValue::from(6), RulePropertyValue::Usize(6));
    }

    #[test]
    fn from_float() {
        assert_eq!(RulePropertyValue::from(1.0), RulePropertyValue::Float(1.0));
    }

    #[test]
    fn from_boolean_option_some() {
        let bool = Some(true);
        assert_eq!(
            RulePropertyValue::from(bool),
            RulePropertyValue::Boolean(true)
        );
    }

    #[test]
    fn from_boolean_option_none() {
        let bool: Option<bool> = None;
        assert_eq!(RulePropertyValue::from(bool), RulePropertyValue::None);
    }
}
