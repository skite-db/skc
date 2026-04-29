pub trait ContextExt<T, E> {
    fn with_context<F, S>(self, context_fn: F) -> Result<T, E>
    where
        F: FnOnce() -> S,
        S: Into<String>;
}
