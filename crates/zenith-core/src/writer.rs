//! Zenith Writer: Rich document authoring engine synthesizing Microsoft Word,
//! Google Docs, and LibreOffice Writer in pure Rust.

use crate::fonts::FontFamily;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParagraphStyle {
    Title,
    Subtitle,
    Heading1,
    Heading2,
    Heading3,
    Normal,
    BulletList,
    NumberList,
    Quote,
    CodeBlock,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRgb {
    pub const BLACK: ColorRgb = ColorRgb { r: 0, g: 0, b: 0 };
    pub const WHITE: ColorRgb = ColorRgb { r: 255, g: 255, b: 255 };
    pub const BLUE: ColorRgb = ColorRgb { r: 37, g: 99, b: 235 };
    pub const RED: ColorRgb = ColorRgb { r: 220, g: 38, b: 38 };
    pub const GRAY: ColorRgb = ColorRgb { r: 100, g: 116, b: 139 };
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextRun {
    pub text: String,
    pub font: FontFamily,
    pub size_pt: u8,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub color: ColorRgb,
    pub highlight: Option<ColorRgb>,
}

impl TextRun {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font: FontFamily::Calibri,
            size_pt: 11,
            bold: false,
            italic: false,
            underline: false,
            strike: false,
            color: ColorRgb::BLACK,
            highlight: None,
        }
    }

    pub fn with_font(mut self, font: FontFamily) -> Self {
        self.font = font;
        self
    }

    pub fn with_size(mut self, size_pt: u8) -> Self {
        self.size_pt = size_pt;
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Paragraph {
    pub style: ParagraphStyle,
    pub alignment: TextAlignment,
    pub line_spacing: f32, // e.g. 1.0, 1.15, 1.5, 2.0
    pub runs: Vec<TextRun>,
}

impl Paragraph {
    pub fn new(style: ParagraphStyle) -> Self {
        Self {
            style,
            alignment: TextAlignment::Left,
            line_spacing: 1.15,
            runs: Vec::new(),
        }
    }

    pub fn with_text(style: ParagraphStyle, text: impl Into<String>) -> Self {
        let mut p = Self::new(style);
        p.runs.push(TextRun::new(text));
        p
    }

    pub fn add_run(&mut self, run: TextRun) {
        self.runs.push(run);
    }

    pub fn plain_text(&self) -> String {
        let mut s = String::new();
        for r in &self.runs {
            s.push_str(&r.text);
        }
        s
    }

    pub fn word_count(&self) -> usize {
        let text = self.plain_text();
        text.split_whitespace().count()
    }

    pub fn char_count(&self) -> usize {
        self.runs.iter().map(|r| r.text.chars().count()).sum()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocTable {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<String>>,
}

impl DocTable {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut cells = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(String::new());
            }
            cells.push(row);
        }
        Self { rows, cols, cells }
    }

    pub fn set_cell(&mut self, row: usize, col: usize, val: impl Into<String>) {
        if row < self.rows && col < self.cols {
            self.cells[row][col] = val.into();
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&str> {
        if row < self.rows && col < self.cols {
            Some(&self.cells[row][col])
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub title: String,
    pub author: String,
    pub page_width_pt: u32,  // 8.5 in * 72 = 612 pt
    pub page_height_pt: u32, // 11 in * 72 = 792 pt
    pub margin_left_pt: u32, // 72 pt = 1 in
    pub margin_right_pt: u32,
    pub margin_top_pt: u32,
    pub margin_bottom_pt: u32,
    pub default_font: FontFamily,
    pub paragraphs: Vec<Paragraph>,
    pub tables: Vec<DocTable>,
}

impl Document {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            author: "NovaOS Administrator".into(),
            page_width_pt: 612,
            page_height_pt: 792,
            margin_left_pt: 72,
            margin_right_pt: 72,
            margin_top_pt: 72,
            margin_bottom_pt: 72,
            default_font: FontFamily::Calibri,
            paragraphs: Vec::new(),
            tables: Vec::new(),
        }
    }

    pub fn add_paragraph(&mut self, p: Paragraph) {
        self.paragraphs.push(p);
    }

    pub fn total_words(&self) -> usize {
        self.paragraphs.iter().map(|p| p.word_count()).sum()
    }

    pub fn total_chars(&self) -> usize {
        self.paragraphs.iter().map(|p| p.char_count()).sum()
    }

    pub fn estimated_pages(&self) -> usize {
        let total_words = self.total_words();
        if total_words == 0 {
            1
        } else {
            ((total_words + 349) / 350).max(1)
        }
    }

    pub fn to_plain_text(&self) -> String {
        let mut out = String::new();
        for p in &self.paragraphs {
            out.push_str(&p.plain_text());
            out.push('\n');
        }
        out
    }

    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        for p in &self.paragraphs {
            match p.style {
                ParagraphStyle::Title => {
                    md.push_str("# ");
                    md.push_str(&p.plain_text());
                    md.push_str("\n\n");
                }
                ParagraphStyle::Subtitle => {
                    md.push_str("### ");
                    md.push_str(&p.plain_text());
                    md.push_str("\n\n");
                }
                ParagraphStyle::Heading1 => {
                    md.push_str("# ");
                    md.push_str(&p.plain_text());
                    md.push_str("\n\n");
                }
                ParagraphStyle::Heading2 => {
                    md.push_str("## ");
                    md.push_str(&p.plain_text());
                    md.push_str("\n\n");
                }
                ParagraphStyle::Heading3 => {
                    md.push_str("### ");
                    md.push_str(&p.plain_text());
                    md.push_str("\n\n");
                }
                ParagraphStyle::BulletList => {
                    md.push_str("* ");
                    md.push_str(&p.plain_text());
                    md.push('\n');
                }
                ParagraphStyle::NumberList => {
                    md.push_str("1. ");
                    md.push_str(&p.plain_text());
                    md.push('\n');
                }
                ParagraphStyle::Quote => {
                    md.push_str("> ");
                    md.push_str(&p.plain_text());
                    md.push_str("\n\n");
                }
                ParagraphStyle::CodeBlock => {
                    md.push_str("```\n");
                    md.push_str(&p.plain_text());
                    md.push_str("\n```\n\n");
                }
                ParagraphStyle::Normal => {
                    md.push_str(&p.plain_text());
                    md.push_str("\n\n");
                }
            }
        }
        md
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation_and_metrics() {
        let mut doc = Document::new("Executive Proposal");
        let mut p1 = Paragraph::new(ParagraphStyle::Title);
        p1.add_run(TextRun::new("NovaOS Enterprise Architecture").with_font(FontFamily::Calibri).bold());
        doc.add_paragraph(p1);

        let p2 = Paragraph::with_text(ParagraphStyle::Normal, "This is a high performance sovereign document written in pure Rust.");
        doc.add_paragraph(p2);

        let p3 = Paragraph::with_text(ParagraphStyle::BulletList, "Pure Rust 1.99.1 microkernel & userspace");
        doc.add_paragraph(p3);

        assert_eq!(doc.paragraphs.len(), 3);
        assert_eq!(doc.total_words(), 20);
        assert_eq!(doc.estimated_pages(), 1);

        let md = doc.to_markdown();
        assert!(md.contains("# NovaOS Enterprise Architecture"));
        assert!(md.contains("* Pure Rust 1.99.1 microkernel"));
    }

    #[test]
    fn test_doc_table() {
        let mut t = DocTable::new(2, 3);
        t.set_cell(0, 0, "Metric");
        t.set_cell(0, 1, "Target");
        t.set_cell(0, 2, "Achieved");
        t.set_cell(1, 0, "Boot Time");
        t.set_cell(1, 1, "< 500ms");
        t.set_cell(1, 2, "180ms");

        assert_eq!(t.get_cell(0, 0), Some("Metric"));
        assert_eq!(t.get_cell(1, 2), Some("180ms"));
    }
}
