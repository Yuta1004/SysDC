use std::fmt;
use std::fmt::{ Display, Formatter };

// ソースコード上での位置を表す構造体
#[derive(Debug, Clone)]
pub struct Location {
    filename: Option<String>,
    coord: Option<(i32, i32)>
}

impl Location {
    pub fn new() -> Location {
        Location { filename: None, coord: None }
    }

    pub fn new_with_coord(coord: (i32, i32)) -> Location {
        Location { filename: None, coord: Some(coord) }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (&self.filename, self.coord) {
            (Some(filename), Some((row, col))) => {
                write!(f, "{}:{}:{}", filename, row, col)
            },
            (None, Some((row, col))) => {
                write!(f, "{}:{}", row, col)
            },
            (Some(filename), None) => {
                write!(f, "{}", filename)
            },
            _ => write!(f, "?")
        }
    }
}
