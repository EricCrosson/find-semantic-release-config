use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected semantic-release configuration to exist at {path}")]
    ConfigurationFileNotFound { path: PathBuf },

    #[error("Unable to open file {path}")]
    FileOpenError {
        #[source]
        inner: io::Error,
        path: PathBuf,
    },

    #[error("Unable to read file {path}")]
    FileReadError {
        #[source]
        inner: io::Error,
        path: PathBuf,
    },

    #[error("Unable to parse semantic-release configuration file")]
    FileParseError {
        #[source]
        inner: serde_json::Error,
    },

    #[error("Unable to serialize file")]
    FileSerializeError {
        #[source]
        inner: serde_json::Error,
    },

    #[error("Unable to write changes to file {path}")]
    FileWriteError {
        #[source]
        inner: io::Error,
        path: PathBuf,
    },
}

impl Error {
    pub(crate) fn file_open_error(inner: io::Error, path: &Path) -> Error {
        Error::FileOpenError {
            inner,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_read_error(inner: io::Error, path: &Path) -> Error {
        Error::FileReadError {
            inner,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_parse_error(inner: serde_json::Error) -> Error {
        Error::FileParseError { inner }
    }
}
