use crate::constants::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BavliDaf {
    pub tractate: BavliTractate,
    pub daf_index: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct YerushalmiDaf {
    pub tractate: YerushalmiTractate,
    pub daf_index: i64,
}
pub trait BavliDafTrait {
    fn get_tractate(&self) -> BavliTractate;

    fn get_daf_index(&self) -> i64;
}

pub trait YerushalmiDafTrait {
    fn get_tractate(&self) -> YerushalmiTractate;

    fn get_daf_index(&self) -> i64;
}

impl BavliDafTrait for BavliDaf {
    fn get_tractate(&self) -> BavliTractate {
        self.tractate
    }
    fn get_daf_index(&self) -> i64 {
        self.daf_index
    }
}

impl YerushalmiDafTrait for YerushalmiDaf {
    fn get_tractate(&self) -> YerushalmiTractate {
        self.tractate
    }
    fn get_daf_index(&self) -> i64 {
        self.daf_index
    }
}
