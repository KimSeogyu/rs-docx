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
#[xml(tag = "w:cols")]
pub struct PageCols {
    #[xml(attr = "w:space", with = "crate::rounded_float")]
    pub space: Option<isize>,
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
