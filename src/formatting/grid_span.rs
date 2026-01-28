use hard_xml::{XmlRead, XmlWrite};

#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:gridSpan")]
pub struct GridSpan {
    #[xml(attr = "w:val", with = "crate::rounded_float")]
    pub val: isize,
}
