use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
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
                write!(f, "unable to open file {:?}", path)
            }
            Error::FileReadError { source: _, path } => {
                write!(f, "unable to read file {:?}", path)
            }
            Error::FileParseError { source: _, path } => {
                write!(f, "unable to parse file {:?}", path)
            }
        }
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
