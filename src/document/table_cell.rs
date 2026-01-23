#![allow(unused_must_use)]
use std::borrow::{Borrow, Cow};

use derive_more::From;
use hard_xml::{XmlRead, XmlWrite};

use crate::{
    __setter, __xml_test_suites, document::Paragraph, document::Table,
    formatting::TableCellProperty,
};

/// Table Cell
///
/// ```rust
/// use rs_docx::document::*;
/// use rs_docx::formatting::*;
///
/// let cell = TableCell::from(Paragraph::default());
///
/// let cell = TableCell::paragraph(Paragraph::default())
///     .property(TableCellProperty::default());
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:tc")]
pub struct TableCell<'a> {
    #[xml(default, child = "w:tcPr")]
    pub property: TableCellProperty,
    #[xml(child = "w:p", child = "w:tbl")]
    pub content: Vec<TableCellContent<'a>>,
}

impl<'a> TableCell<'a> {
    __setter!(property: TableCellProperty);

    pub fn paragraph<T: Into<Paragraph<'a>>>(par: T) -> Self {
        TableCell {
            property: TableCellProperty::default(),
            content: vec![TableCellContent::Paragraph(par.into())],
        }
    }

    pub fn iter_text(&self) -> Box<dyn Iterator<Item = &Cow<'a, str>> + '_> {
        Box::new(self.content.iter().flat_map(|content| match content {
            TableCellContent::Paragraph(p) => {
                Box::new(p.iter_text()) as Box<dyn Iterator<Item = &Cow<'a, str>>>
            }
            TableCellContent::Table(t) => {
                Box::new(t.iter_text()) as Box<dyn Iterator<Item = &Cow<'a, str>>>
            }
        }))
    }

    pub fn iter_text_mut(&mut self) -> Box<dyn Iterator<Item = &mut Cow<'a, str>> + '_> {
        Box::new(self.content.iter_mut().flat_map(|content| match content {
            TableCellContent::Paragraph(p) => {
                Box::new(p.iter_text_mut()) as Box<dyn Iterator<Item = &mut Cow<'a, str>>>
            }
            TableCellContent::Table(t) => {
                Box::new(t.iter_text_mut()) as Box<dyn Iterator<Item = &mut Cow<'a, str>>>
            }
        }))
    }

    pub fn replace_text<'b, I, T, S>(&mut self, dic: T) -> crate::DocxResult<()>
    where
        S: AsRef<str> + 'b,
        T: IntoIterator<Item = I> + Copy,
        I: Borrow<(S, S)>,
    {
        for content in self.content.iter_mut() {
            match content {
                TableCellContent::Paragraph(p) => p.replace_text(dic)?,
                TableCellContent::Table(t) => t.replace_text(dic)?,
            }
        }
        Ok(())
    }
}

impl<'a, T: Into<TableCellContent<'a>>> From<T> for TableCell<'a> {
    fn from(content: T) -> Self {
        TableCell {
            property: TableCellProperty::default(),
            content: vec![content.into()],
        }
    }
}

#[derive(Debug, From, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[allow(clippy::large_enum_variant)]
pub enum TableCellContent<'a> {
    #[xml(tag = "w:p")]
    Paragraph(Paragraph<'a>),
    #[xml(tag = "w:tbl")]
    Table(Table<'a>),
}

__xml_test_suites!(
    TableCell,
    TableCell::paragraph(Paragraph::default()),
    r#"<w:tc><w:tcPr><w:vAlign w:val="top"/></w:tcPr><w:p/></w:tc>"#,
    TableCell::from(Table::default()),
    r#"<w:tc><w:tcPr><w:vAlign w:val="top"/></w:tcPr><w:tbl><w:tblPr/><w:tblGrid/></w:tbl></w:tc>"#,
);
