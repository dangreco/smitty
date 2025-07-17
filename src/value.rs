// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;

use crate::lazy::LazyValue;
use crate::resolve::Resolve;
use crate::result::Result;

/// Represents a resolvable value that can be either raw or lazy.
///
/// This enum is used to represent a value that can be either raw or lazy.
/// When the value is raw, it is immediately available.
/// When the value is lazy, it is resolved via some lazy initialization, possibly asynchronously.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum Value<T> {
    Raw(T),
    Lazy(LazyValue<T>),
}

/// Implements the `Resolve` trait for the `Value` enum.
///
/// Raw values are immediately available.
/// Lazy values are resolved asynchronously.
#[async_trait::async_trait]
impl<T> Resolve<T> for Value<T>
where
    T: Clone + Send + Sync,
{
    async fn resolve(&self) -> Result<Cow<'_, T>> {
        match self {
            Value::Raw(value) => Ok(Cow::Borrowed(value)),
            Value::Lazy(value) => value.resolve().await,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::result::Result;

    use super::*;

    #[tokio::test]
    async fn test_raw() -> Result<()> {
        let value = Value::Raw(42);
        assert_eq!(*value.resolve().await?, 42);
        Ok(())
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests_serde {
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn test_serialize_raw() -> Result<()> {
        let value = Value::Raw(42);
        assert_eq!(serde_json::to_string(&value)?, r#"42"#);
        Ok(())
    }

    #[tokio::test]
    async fn test_deserialize_raw() -> Result<()> {
        let s = "42";
        let value: Value<u8> = serde_json::from_str(s)?;
        assert_eq!(*value.resolve().await?, 42);
        Ok(())
    }
}
