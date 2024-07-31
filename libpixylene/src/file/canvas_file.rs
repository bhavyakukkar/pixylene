use crate::project::Canvas;

use serde_json::{from_str, to_string};
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

impl Canvas {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        to_string(self)
    }
}

#[derive(Debug)]
pub struct CanvasFile;

impl CanvasFile {
    pub fn read(path: &PathBuf) -> Result<Canvas, CanvasFileError> {
        use CanvasFileError::{DeserializeError, ReadError};

        Ok(
            from_str(&fs::read_to_string(&path).map_err(|err| ReadError(path.clone(), err))?)
                .map_err(|err| DeserializeError(path.clone(), err))?,
        )
    }

    pub fn write(path: &PathBuf, canvas: &Canvas) -> Result<(), CanvasFileError> {
        use std::io::Write;
        use CanvasFileError::{SerializeError, WriteError};

        Ok(fs::File::create(Path::new(&path))
            .map_err(|err| WriteError(path.clone(), err))?
            .write_all(
                to_string(canvas)
                    .map_err(|err| SerializeError(path.clone(), err))?
                    .as_bytes(),
            )
            .map_err(|err| WriteError(path.clone(), err))?)
    }
}

#[derive(Debug)]
pub enum CanvasFileError {
    ReadError(PathBuf, io::Error),
    WriteError(PathBuf, io::Error),
    DeserializeError(PathBuf, serde_json::Error),
    SerializeError(PathBuf, serde_json::Error),
}

impl fmt::Display for CanvasFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CanvasFileError::*;
        match self {
            ReadError(path, io_error) => write!(
                f,
                "file error reading from '{}':\n{}",
                path.display(),
                io_error,
            ),
            WriteError(path, io_error) => write!(
                f,
                "file error writing to '{}':\n{}",
                path.display(),
                io_error,
            ),
            DeserializeError(path, err) => write!(
                f,
                "deserialization error reading from '{}':\n{}",
                path.display(),
                err,
            ),
            SerializeError(path, err) => write!(
                f,
                "serialization error writing to '{}':\n{}",
                path.display(),
                err,
            ),
        }
    }
}
