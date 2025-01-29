#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pdbanalyze {
    pub adtype: String,
    pub dnumber: usize,
    pub itype: String,
    pub gtype: String,
    pub ntype: String,
    pub atype: usize,
    pub btype: usize,
    pub ctype: usize,
    pub utype: usize,
    pub attype: usize,
    pub lttype: usize,
    pub uchar: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Comprativedrive {
    pub chain: String,
    pub residue: usize,
    pub coordinate1: f32,
    pub coordinate2: f32,
    pub coordinate3: f32,
}
