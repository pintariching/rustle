use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{message} {start.0}:{start.1}\n{frame}")]
    Compile {
        code: String,
        message: String,
        start: (usize, usize),
        end: (usize, usize),
        pos: usize,
        filename: String,
        frame: String,
    }
}

impl Error {
    pub fn new(message: String, code: String, start: (usize, usize), end: (usize, usize), pos: usize, filename: String, frame: String) -> Self {
        Error::Compile {
            code,
            message,
            start,
            end,
            pos,
            filename,
            frame,
        }
    }
}