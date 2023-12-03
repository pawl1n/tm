use std::fmt::Display;

#[derive(Debug)]
pub enum ExamResult {
    Found(usize),
    Unknown,
}

impl Display for ExamResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExamResult::Found(class) => write!(f, "{}", class),
            ExamResult::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct ExamData {
    pub class1: usize,
    pub class2: usize,
    pub result: ExamResult,
}

impl ExamData {
    pub fn new(class1: usize, class2: usize, result: ExamResult) -> Self {
        Self {
            class1,
            class2,
            result,
        }
    }
}