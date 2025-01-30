#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pdbanalyze {
    pub adtype: String,
    pub dnumber: String,
    pub itype: String,
    pub gtype: String,
    pub ntype: String,
    pub atype: String,
    pub btype: String,
    pub ctype: String,
    pub utype: String,
    pub attype: String,
    pub lttype: String,
    pub uchar: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pdbsequence {
    pub adtype: String,
    pub dnumber: String,
    pub gtype: String,
    pub ntype: String,
    pub atype: String,
    pub btype: String,
    pub ctype: String,
    pub utype: String,
    pub attype: String,
    pub lttype: String,
    pub uchar: String,
}


#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Endchain {
    pub tab0: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Comprativedrive {
    pub chain: String,
    pub residue: usize,
    pub coordinate1: f32,
    pub coordinate2: f32,
    pub coordinate3: f32,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct EuclideanWrite{
    pub tab1: f32,
    pub tab2: f32,
    pub tab3: f32,
    pub tab4: f32,
    pub tab5: f32,
    pub tab6: f32,
    pub tab7: f32,
}
