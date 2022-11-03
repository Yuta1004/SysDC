use std::fmt;
use std::fmt::{Display, Formatter};

// ソースコード上での位置を表す構造体
#[derive(Debug, Clone)]
pub struct Location {
    filename: Option<String>,
    coord: Option<(i32, i32)>,
}

impl Location {
    pub fn new() -> Location {
        Location {
            filename: None,
            coord: None,
        }
    }

    pub fn with_filename(mut self, filename: String) -> Location {
        self.filename = Some(filename);
        self
    }

    pub fn with_coord(mut self, coord: (i32, i32)) -> Location {
        self.coord = Some(coord);
        self
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (&self.filename, self.coord) {
            (Some(filename), Some((row, col))) => {
                write!(f, "{}:{}:{}", filename, row, col)
            }
            (None, Some((row, col))) => {
                write!(f, "{}:{}", row, col)
            }
            (Some(filename), None) => {
                write!(f, "{}", filename)
            }
            _ => write!(f, "?"),
        }
    }
}
