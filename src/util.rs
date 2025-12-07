macro_rules! input {
    ($p:literal) => {
        include_str!($p)
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };
}

macro_rules! input_single {
    ($p:literal) => {
        include_str!($p)
            .trim()
            .split(',')
            .collect::<Vec<&str>>()
    };
}

pub(crate) use input;
pub(crate) use input_single;

pub struct Solution {
    pub part_one: String,
    pub part_two: String,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            part_one: "empty".to_string(),
            part_two: "empty".to_string(),
        }
    }
}

pub trait ISolution {
    fn solve(&self) -> Solution;
}
