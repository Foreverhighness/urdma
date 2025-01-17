use std::sync::Arc;

/// verbs provider
pub trait Provider {
    /// Sized because we do not want store a fat pointer
    /// 'static because it will be stored in C side
    type Driver: Sized + 'static;

    /// init context
    ///
    /// guarantee to be called only once
    fn init();

    /// new driver
    fn new() -> Arc<Self::Driver>;
}
