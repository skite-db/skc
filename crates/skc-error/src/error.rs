#[derive(Debug)]
pub enum SkiteError {
    Any {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        message: String,
    },
}

impl std::fmt::Display for SkiteError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any { source, message } => write!(fmt, "{message} : {source}"),
        }
    }
}
