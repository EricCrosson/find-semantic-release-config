use std::{
    io,
    path::{Path, PathBuf},
};

pub enum Error {
    /// Unable to open file
    FileOpenError { source: io::Error, path: PathBuf },

    /// Unable to read file
    FileReadError { source: io::Error, path: PathBuf },

    /// Unable to parse file
    FileParseError {
        source: serde_json::Error,
        path: PathBuf,
    },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::FileOpenError { source, path: _ } => Some(source),
            Error::FileReadError { source, path: _ } => Some(source),
            Error::FileParseError { source, path: _ } => Some(source),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileOpenError { source: _, path } => {
                write!(f, "Unable to open file {:?}", path)
            }
            Error::FileReadError { source: _, path } => {
                write!(f, "Unable to read file {:?}", path)
            }
            Error::FileParseError { source: _, path } => {
                write!(f, "Unable to parse file {:?}", path)
            }
        }
    }
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl Error {
    pub(crate) fn file_open_error(source: io::Error, path: &Path) -> Self {
        Error::FileOpenError {
            source,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_read_error(source: io::Error, path: &Path) -> Self {
        Error::FileReadError {
            source,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_parse_error(source: serde_json::Error, path: &Path) -> Self {
        Error::FileParseError {
            source,
            path: path.to_owned(),
        }
    }
}
