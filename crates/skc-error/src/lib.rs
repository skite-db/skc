pub type Result<T, E = SkcError> = core::result::Result<T, E>; 

#[derive(Debug)]
pub enum SkcError {
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
        context: &'static str,
    },
    Args {
      context: &'static str
    }
}

impl std::fmt::Display for SkcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io {
                source,
                path,
                context,
            } => write!(f, "{context} on '{}' : {source}", path.display()),
        }
    }
}

impl std::error::Error for SkcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
        }
    }
}

