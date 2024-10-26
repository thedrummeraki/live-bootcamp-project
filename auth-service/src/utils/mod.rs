use std::sync::Arc;

use tokio::sync::RwLock;

pub mod auth;
pub mod constants;

/// Objects that use the Default trait will be able to initialize
/// a "thread-safe" of themselves, wrapping the default instance into
/// an `Arc` and `RwLock` (ie: thread-safe read-write mutex protected
/// smart pointer).
pub trait ThreadSafe {
    fn thread_safe() -> Arc<RwLock<Self>>;
}

impl<T: Default + Send + Sync> ThreadSafe for T {
    fn thread_safe() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::default()))
    }
}
