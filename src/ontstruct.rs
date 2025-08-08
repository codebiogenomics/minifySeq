#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ONT {
    pub namemap1: String,
    pub positionmap: usize,
    pub refmap: String,
    pub refmapstart: usize,
    pub refmaplength: usize,
    pub refseq: String,
}
