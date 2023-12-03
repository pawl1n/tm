use std::fmt::Display;

pub type ExamRealizationResults = (u32, u32, u32);

#[derive(Debug)]
pub enum ExamResult {
    Found(usize, ExamRealizationResults),
    Unknown(ExamRealizationResults),
}

impl Display for ExamResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExamResult::Found(class, results) => write!(f, "{}: {:?}", class, results),
            ExamResult::Unknown(results) => write!(f, "Unknown: {:?}", results),
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
