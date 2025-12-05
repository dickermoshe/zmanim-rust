use crate::constants::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct BavliDaf {
    pub tractate: BavliTractate,
    pub daf_index: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct YerushalmiDaf {
    pub tractate: YerushalmiTractate,
    pub daf_index: i64,
}
