use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Hill {
    Start(u8),
    End(u8),
    Hill(u8),
}

impl Hill {
    pub fn height(&self) -> u8 {
        match self {
            Hill::Start(h) => *h,
            Hill::End(h) => *h,
            Hill::Hill(h) => *h,
        }
    }

    pub fn can_reach(&self, other: &Hill) -> bool {
        other.height().saturating_sub(self.height()) <= 1
    }
}

impl From<char> for Hill {
    fn from(value: char) -> Self {
        match value {
            'S' => Hill::Start(0),
            'E' => Hill::End(25),
            c if c.is_ascii_lowercase() => Hill::Hill(value as u8 - b'a'),
            _ => unreachable!(),
        }
    }
}

pub type AdjacentHills = [Option<(usize, usize)>; 4];

#[derive(Debug, Clone)]
pub struct AreaMap {
    pub hills: Vec<Vec<Hill>>,
    pub graph: HashMap<(usize, usize), AdjacentHills>,
    pub start_at: (usize, usize),
    pub end_at: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct DownhillMap {
    pub hills: Vec<Vec<Hill>>,
    pub graph: HashMap<(usize, usize), AdjacentHills>,
    pub summit: (usize, usize),
}
