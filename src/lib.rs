use std::{pin::Pin, future::Future};

#[cfg(all(not(feature = "local"), not(target_arch = "wasm32")))]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
#[cfg(any(feature = "local", target_arch = "wasm32"))]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;