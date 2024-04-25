use crate::project::Canvas;
use std::{ fs, io, fmt, path::Path };
use serde_json::{ to_string, from_str };


impl Canvas {
    //todo: remove after shifting filing responsibility to pixylene-ui
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        to_string(self)
    }
}

#[derive(Debug)]
pub struct CanvasFile;

impl CanvasFile {
    pub fn read(path: String) -> Result<Canvas, CanvasFileError> {
        use CanvasFileError::{ ReadError, DeserializeError };

        Ok(from_str(
            &fs::read_to_string(&path).map_err(|err| ReadError(path.clone(), err))?
        ).map_err(|err| DeserializeError(path, err))?)
    }

    pub fn write(path: String, canvas: &Canvas) -> Result<(), CanvasFileError> {
        use CanvasFileError::{ WriteError, SerializeError };
        use std::io::Write;

        Ok(fs::File::create(Path::new(&path)).map_err(|err| WriteError(path.clone(), err))?
            .write_all(to_string(canvas).map_err(|err| SerializeError(path.clone(), err))?
                .as_bytes()).map_err(|err| WriteError(path, err))?)
    }
}

#[derive(Debug)]
pub enum CanvasFileError {
    ReadError(String, io::Error),
    WriteError(String, io::Error),
    DeserializeError(String, serde_json::Error),
    SerializeError(String, serde_json::Error),
}

impl fmt::Display for CanvasFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CanvasFileError::*;
        match self {
            ReadError(path, io_error) => write!(
                f,
                "file error reading from '{}':\n{}",
                path,
                io_error,
            ),
            WriteError(path, io_error) => write!(
                f,
                "file error writing to '{}':\n{}",
                path,
                io_error,
            ),
            DeserializeError(path, err) => write!(
                f,
                "deserialization error reading from '{}':\n{}",
                path,
                err,
            ),
            SerializeError(path, err) => write!(
                f,
                "serialization error writing to '{}':\n{}",
                path,
                err,
            ),
        }
    }
}
