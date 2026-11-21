//! Realistic mock enterprise office documents for demonstration and PNG export.

use zenith_core::base::Database;
use zenith_core::calc::{Workbook, Worksheet};
use zenith_core::fonts::FontFamily;
use zenith_core::present::{Presentation, SlideLayout};
use zenith_core::suite::{AppModule, ZenithSuite};
use zenith_core::writer::{DocTable, Document, Paragraph, ParagraphStyle, TextRun};

pub fn create_sample_suite() -> ZenithSuite {
    let mut suite = ZenithSuite::new();

    // 1. Writer Document: "NovaOS Enterprise Strategy & Architecture"
    let mut doc = Document::new("NovaOS Enterprise Strategy & Architecture");
    doc.author = "NovaOS Architecture Board".into();

    let mut p_h1 = Paragraph::new(ParagraphStyle::Title);
    p_h1.add_run(TextRun::new("NovaOS Sovereign Enterprise Platform").with_font(FontFamily::Calibri).bold());
    doc.add_paragraph(p_h1);

    let mut p_lead = Paragraph::new(ParagraphStyle::Subtitle);
    p_lead.add_run(TextRun::new("Sovereignty, Memory Safety & Unified Office Productivity in Pure Rust 1.99.1").with_font(FontFamily::SegoeUI).italic());
    doc.add_paragraph(p_lead);

    let mut p_body = Paragraph::new(ParagraphStyle::Normal);
    p_body.add_run(TextRun::new("The NovaOS desktop environment delivers sovereign computing capabilities, synthesized from the best ergonomics of Microsoft Windows 11, macOS Sequoia, and Ubuntu GNOME, entirely written in safe Rust."));
    doc.add_paragraph(p_body);

    let mut p_list1 = Paragraph::new(ParagraphStyle::BulletList);
    p_list1.add_run(TextRun::new("Zenith Office: Word, Excel, PowerPoint & Access synthesis with zero C/C++ dependencies."));
    doc.add_paragraph(p_list1);

    let mut p_list2 = Paragraph::new(ParagraphStyle::BulletList);
    p_list2.add_run(TextRun::new("Full Microsoft & LibreOffice Typography: Calibri, Times New Roman, Arial, Ubuntu."));
    doc.add_paragraph(p_list2);

    let mut p_list3 = Paragraph::new(ParagraphStyle::BulletList);
    p_list3.add_run(TextRun::new("Deep Cloud Integration: Native collaboration inside Aero Web Browser like Google Docs/Sheets."));
    doc.add_paragraph(p_list3);

    // Add Executive Metrics Table
    let mut table = DocTable::new(3, 4);
    table.set_cell(0, 0, "Quarter");
    table.set_cell(0, 1, "Enterprise Target");
    table.set_cell(0, 2, "Achieved Speed");
    table.set_cell(0, 3, "Memory Footprint");

    table.set_cell(1, 0, "Q1 2026");
    table.set_cell(1, 1, "Fast Ring-3 Boot");
    table.set_cell(1, 2, "142 ms");
    table.set_cell(1, 3, "24 MB");

    table.set_cell(2, 0, "Q2 2026");
    table.set_cell(2, 1, "Office Suite Core");
    table.set_cell(2, 2, "60 FPS VSync");
    table.set_cell(2, 3, "38 MB");
    doc.tables.push(table);

    suite.writer_doc = doc;

    // 2. Calc Workbook: "NovaOS_Quarterly_Financials.xlsx"
    let mut wb = Workbook::new("NovaOS_Quarterly_Financials");
    let mut ws = Worksheet::new("Budget_2026");

    // Column Headers
    ws.set_text(0, 0, "Division");
    ws.set_text(1, 0, "Q1 Budget");
    ws.set_text(2, 0, "Q2 Budget");
    ws.set_text(3, 0, "Total");

    // Row 1: Core Kernel Team
    ws.set_text(0, 1, "Kernel Engineering");
    ws.set_number(1, 1, 120000.0);
    ws.set_number(2, 1, 140000.0);
    ws.set_formula(3, 1, "=B2+C2");

    // Row 2: Userland & Apps Team
    ws.set_text(0, 2, "Desktop Userland");
    ws.set_number(1, 2, 95000.0);
    ws.set_number(2, 2, 110000.0);
    ws.set_formula(3, 2, "=B3+C3");

    // Row 3: Security & Quality
    ws.set_text(0, 3, "Security & Evals");
    ws.set_number(1, 3, 60000.0);
    ws.set_number(2, 3, 75000.0);
    ws.set_formula(3, 3, "=B4+C4");

    // Row 4: Totals
    ws.set_text(0, 4, "Grand Total");
    ws.set_formula(1, 4, "=SUM(B2:B4)");
    ws.set_formula(2, 4, "=SUM(C2:C4)");
    ws.set_formula(3, 4, "=SUM(D2:D4)");

    ws.recalculate();
    wb.sheets[0] = ws;
    suite.calc_workbook = wb;

    // 3. Present Presentation: "NovaOS_Keynote_2026.pptx"
    let mut pres = Presentation::new("NovaOS_Keynote_2026");
    let mut s1 = pres.slides[0].clone();
    s1.title = "NovaOS Enterprise Strategy 2026".into();
    s1.subtitle = "High-Performance Sovereign Desktop Architecture".into();
    pres.slides[0] = s1;

    let s2_idx = pres.add_slide(SlideLayout::TitleAndContent, "Zenith Office Architecture");
    pres.slides[s2_idx] = pres.slides[s2_idx].clone().with_bullets(&[
        "Pure Rust 1.99.1: Zero memory corruption vulnerabilities",
        "Full Microsoft & LibreOffice file format compatibility",
        "Fluent Ribbon toolbar & responsive 32-bit software compositor",
        "Aero Web Browser bridge for instant cloud document editing",
    ]);

    let s3_idx = pres.add_slide(SlideLayout::TwoContent, "Ecosystem Integration");
    pres.slides[s3_idx].add_card("Desktop Standalone", "Runs natively in ring-3 microkernel with sub-millisecond launch speed.", zenith_core::writer::ColorRgb::WHITE);
    pres.slides[s3_idx].add_card("Aero Web Cloud", "Collaborative cloud editor with real-time sync in Aero Browser.", zenith_core::writer::ColorRgb::WHITE);

    suite.present_deck = pres;

    // 4. Base Database: "Enterprise_ERP.accdb"
    suite.base_db = Database::new("Enterprise_ERP");

    suite.active_module = AppModule::Writer;
    suite
}
