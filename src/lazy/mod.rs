// SPDX-License-Identifier: GPL-3.0-only

use std::borrow::Cow;
use std::sync::Arc;

use futures::future::BoxFuture;

use crate::resolve::Resolve;
use crate::result::Result;

/// `LazyValue<T>` represents a lazily evaluated value that can be resolved asynchronously.
///
/// The simplest (yet least ergonomic) way to create a `LazyValue` is to use the `new` method,
/// which takes a closure that returns a future that resolves to the value.
///
/// Other crate features enable more ergonomic ways to create `LazyValue`s.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[derive(Clone)]
pub enum LazyValue<T> {
    #[cfg_attr(feature = "serde", serde(skip))]
    Async(Arc<dyn Fn() -> BoxFuture<'static, Result<T>> + Send + Sync>),
}

impl<T> LazyValue<T>
where
    T: Send + 'static,
{
    /// Creates a new `LazyValue` from a closure that returns a future that resolves to the value.
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T>> + Send + 'static,
    {
        LazyValue::Async(Arc::new(move || Box::pin(f())))
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LazyValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LazyValue::Async(_) => f
                .debug_tuple("LazyValue::Async")
                .field(&"<closure>")
                .finish(),
        }
    }
}

/// Implements `Resolve` for `LazyValue`.
///
/// Async values are resolved by calling the closure and awaiting the result.
#[async_trait::async_trait]
impl<T> Resolve<T> for LazyValue<T>
where
    T: Clone + Send + Sync,
{
    async fn resolve(&self) -> Result<Cow<'_, T>> {
        match self {
            LazyValue::Async(f) => Ok(Cow::Owned(f().await?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use crate::error::Error;
    use crate::result::Result;

    use super::*;

    #[tokio::test]
    async fn test_lazy_async() -> Result<()> {
        // Check that the closure is called exactly once
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        let value = LazyValue::new(move || {
            let call_count = call_count_clone.clone();
            async move {
                // Here we increment the call count
                call_count.fetch_add(1, Ordering::SeqCst);
                Ok::<_, Error>("Hello, world!".to_string())
            }
        });

        let result = value.resolve().await?;

        assert_eq!(result.as_ref(), "Hello, world!");
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        Ok(())
    }
}
