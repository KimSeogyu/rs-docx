use crate::__string_enum;
use hard_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:vMerge")]
pub struct VMerge {
    #[xml(attr = "w:val")]
    pub val: Option<VMergeType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VMergeType {
    Restart,
    Continue,
}

__string_enum! {
    VMergeType {
        Restart = "restart",
        Continue = "continue",
    }
}
