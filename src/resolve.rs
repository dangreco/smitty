// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;

use crate::result::Result;

/// `Resolve<T>` is a trait that defines a method for resolving a value of type `T`.
///
/// This trait is used in `Value` and all of its derived types to resolve their values.
/// By returning a `Result<Cow<'_, T>>`, the trait allows for efficient cloning of values
/// without unnecessary memory allocation.
#[async_trait::async_trait]
pub trait Resolve<T>
where
    T: Clone,
{
    async fn resolve(&self) -> Result<Cow<'_, T>>;
}
