#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Clean {
    pub id: String,
    pub start: Vec<usize>,
    pub end: Vec<usize>,
}
