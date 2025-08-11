#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Ont {
    pub namemap1: String,
    pub positionmap: usize,
    pub refmap: String,
    pub refmapstart: usize,
    pub refmaplength: usize,
    pub refseq: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Clean {
    pub id: String,
    pub start: Vec<usize>,
    pub end: Vec<usize>,
}
