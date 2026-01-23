//! Track Changes support
//!
//! Handles w:ins (insertions) and w:del (deletions) elements for revision tracking.

use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

use super::Run;

/// Insertion (w:ins) - Represents inserted content in track changes
///
/// Contains runs of text that were inserted during revision tracking.
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:ins")]
pub struct Insertion<'a> {
    /// Unique identifier for this revision
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
    /// Author of the revision
    #[xml(attr = "w:author")]
    pub author: Option<Cow<'a, str>>,
    /// Date/time of the revision
    #[xml(attr = "w:date")]
    pub date: Option<Cow<'a, str>>,
    /// Runs containing the inserted content
    #[xml(child = "w:r")]
    pub runs: Vec<Run<'a>>,
}

/// Deletion (w:del) - Represents deleted content in track changes
///
/// Contains runs of text that were deleted during revision tracking.
/// The actual deleted text is stored in w:delText elements within the runs.
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:del")]
pub struct Deletion<'a> {
    /// Unique identifier for this revision
    #[xml(attr = "w:id")]
    pub id: Option<Cow<'a, str>>,
    /// Author of the revision
    #[xml(attr = "w:author")]
    pub author: Option<Cow<'a, str>>,
    /// Date/time of the revision
    #[xml(attr = "w:date")]
    pub date: Option<Cow<'a, str>>,
    /// Runs containing the deleted content (with w:delText)
    #[xml(child = "w:r")]
    pub runs: Vec<Run<'a>>,
}

impl<'a> Insertion<'a> {
    /// Extracts all text from the insertion's runs
    pub fn text(&self) -> String {
        self.runs
            .iter()
            .flat_map(|run| run.iter_text())
            .map(|cow| cow.to_string())
            .collect()
    }
}

impl<'a> Deletion<'a> {
    /// Extracts all deleted text from the deletion's runs
    pub fn text(&self) -> String {
        self.runs
            .iter()
            .flat_map(|run| {
                run.content.iter().filter_map(|content| {
                    if let super::RunContent::DelText(del_text) = content {
                        Some(del_text.text.to_string())
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hard_xml::XmlRead;

    #[test]
    fn test_parse_insertion() {
        let xml = r#"<w:ins w:id="1" w:author="test" w:date="2024-01-01T00:00:00Z">
            <w:r><w:t>inserted text</w:t></w:r>
        </w:ins>"#;
        let ins = Insertion::from_str(xml).unwrap();
        assert_eq!(ins.id.as_deref(), Some("1"));
        assert_eq!(ins.author.as_deref(), Some("test"));
        assert_eq!(ins.text(), "inserted text");
    }

    #[test]
    fn test_parse_deletion() {
        let xml = r#"<w:del w:id="1" w:author="test" w:date="2024-01-01T00:00:00Z">
            <w:r><w:delText>deleted text</w:delText></w:r>
        </w:del>"#;
        let del = Deletion::from_str(xml).unwrap();
        assert_eq!(del.id.as_deref(), Some("1"));
        assert_eq!(del.author.as_deref(), Some("test"));
        assert_eq!(del.text(), "deleted text");
    }
}
