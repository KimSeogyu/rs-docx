use hard_xml::{XmlRead, XmlWrite};

/// Numbering Id
///
/// ```rust
/// use rs_docx::formatting::*;
///
/// let id = NumberingId::from(42isize);
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:pgMar")]
pub struct PageMargin {
    #[xml(attr = "w:top", with = "crate::rounded_float")]
    pub top: Option<isize>,
    #[xml(attr = "w:right", with = "crate::rounded_float")]
    pub right: Option<isize>,
    #[xml(attr = "w:bottom", with = "crate::rounded_float")]
    pub bottom: Option<isize>,
    #[xml(attr = "w:left", with = "crate::rounded_float")]
    pub left: Option<isize>,
    #[xml(attr = "w:header", with = "crate::rounded_float")]
    pub header: Option<isize>,
    #[xml(attr = "w:footer", with = "crate::rounded_float")]
    pub footer: Option<isize>,
    #[xml(attr = "w:gutter", with = "crate::rounded_float")]
    pub gutter: Option<isize>,
}

// impl<T: Into<isize>> From<T> for NumberingId {
//     fn from(val: T) -> Self {
//         NumberingId { value: val.into() }
//     }
// }

// __xml_test_suites!(
//     NumberingId,
//     NumberingId::from(40isize),
//     r#"<w:numId w:val="40"/>"#,
// );
