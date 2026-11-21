//! Zenith Core: Sovereign Office Productivity Suite Engine in Pure Rust 1.99.1.
//!
//! Synthesizes Microsoft Office (Word, Excel, PowerPoint, Access), Google Docs/Sheets,
//! and LibreOffice (Writer, Calc, Impress, Base).

pub mod base;
pub mod calc;
pub mod fonts;
pub mod present;
pub mod suite;
pub mod writer;

pub use base::{ColumnDef, Database, FieldType, FieldValue, Record, TableSchema};
pub use calc::{Cell, CellAddress, CellFormat, CellValue, Workbook, Worksheet};
pub use fonts::FontFamily;
pub use present::{Presentation, Slide, SlideLayout, SlideShape};
pub use suite::{AppModule, ZenithSuite};
pub use writer::{ColorRgb, DocTable, Document, Paragraph, ParagraphStyle, TextAlignment, TextRun};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_full_workflow() {
        let mut suite = ZenithSuite::new();

        // 1. Writer: add styled text
        let mut p = Paragraph::new(ParagraphStyle::Heading1);
        p.add_run(TextRun::new("Enterprise Architecture").with_font(FontFamily::Calibri).bold());
        suite.writer_doc.add_paragraph(p);
        assert_eq!(suite.writer_doc.total_words(), 2);

        // 2. Calc: set numbers and formula
        suite.calc_workbook.active_sheet_mut().set_number(0, 0, 50.0);
        suite.calc_workbook.active_sheet_mut().set_number(0, 1, 150.0);
        suite.calc_workbook.active_sheet_mut().set_formula(0, 2, "=SUM(A1:A2)");
        suite.calc_workbook.active_sheet_mut().recalculate();
        assert_eq!(
            suite.calc_workbook.active_sheet().get_calculated(CellAddress::new(0, 2)),
            CellValue::Number(200.0)
        );

        // 3. Present: create slide deck
        let s_idx = suite.present_deck.add_slide(SlideLayout::TitleAndContent, "System Architecture");
        assert_eq!(s_idx, 1);

        // 4. Base: execute SQL
        let recs = suite.base_db.execute_sql("SELECT * FROM Customers").unwrap();
        assert_eq!(recs.len(), 3);
    }
}
