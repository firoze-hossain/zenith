# Zenith Office Suite

**Zenith Office Suite** is a sovereign, high-performance office productivity suite implemented in **pure Rust 1.99.1** (zero external C/C++ dependencies). It synthesizes the full power and ergonomics of **Microsoft Office** (Word, Excel, PowerPoint, Access), **Google Docs & Sheets**, and **LibreOffice** (Writer, Calc, Impress, Base) into a unified enterprise desktop application and cloud browser suite.

Zenith is the **default productivity suite preinstalled in NovaOS** (like LibreOffice in Ubuntu) and is also fully integrated into **Aero Web Browser** for collaborative cloud editing.

---

## Architecture & Modular Components

Zenith is organized as a Cargo workspace with strict modular separation:

```
zenith/
├── Cargo.toml
├── crates/
│   ├── zenith-core/               # Document models, formatting, calculation & relational engines
│   │   ├── fonts.rs               # Microsoft (Calibri, Times, Arial, Segoe) & LibreOffice typography
│   │   ├── writer.rs              # Zenith Writer: Paragraphs, runs, tables, headings, metrics
│   │   ├── calc.rs                # Zenith Calc: 2D cell matrix, formula engine (=SUM, =AVERAGE, =IF)
│   │   ├── present.rs             # Zenith Present: 16:9 slides, shapes, thumbnails, speaker notes
│   │   ├── base.rs                # Zenith Base: Relational tables, primary keys, SQL query parser
│   │   └── suite.rs               # Unified suite coordinator & shared clipboard
│   ├── zenith-ui/                 # 2D software rendering & Fluent Ribbon
│   │   ├── geom.rs                # Point, Rect, 32-bit ARGB Color primitives
│   │   ├── draw.rs                # Surface trait, BufferSurface32, lines, rounded rectangles
│   │   ├── font.rs                # Embedded crisp bitmap typography
│   │   ├── ribbon.rs              # Fluent Ribbon toolbar (Home, Insert, Layout, Formulas, Data...)
│   │   ├── writer_view.rs         # Document page canvas with margins, ruler & status telemetry
│   │   ├── calc_view.rs           # Formula bar, row/column headers, active cell highlight
│   │   ├── present_view.rs        # Slide deck filmstrip, presentation stage, notes editor
│   │   ├── base_view.rs           # Database table records sheet, SQL editor & navigation
│   │   ├── cloud_view.rs          # Google Docs/Sheets style web cloud editor for Aero Browser
│   │   └── view.rs                # Master ZenithView workspace with module switcher
│   └── zenith-app/                # Desktop runner, CLI flags, headless PNG snapshot generator
└── novaos/                        # Bare-metal ring-3 executable (ZENITH.ELF, OFFICE.ELF)
```

---

## Features

### 1. Zenith Writer (Microsoft Word / Google Docs / LibreOffice Writer)
- Rich text document modeling with Paragraphs, TextRuns, Headings (H1–H6), Bullet & Numbered lists, Blockquotes, and Code blocks.
- Full formatting: Bold, Italic, Underline, Strikethrough, Custom text color, and Highlighting.
- Scaled 612x792 pt page margins with interactive inch-calibrated margin ruler.
- Live document metrics: Word count, Character count, and Page count estimator.
- Export to Plain Text, Markdown, and Word-compatible structural summaries.

### 2. Zenith Calc (Microsoft Excel / Google Sheets / LibreOffice Calc)
- 2D grid matrix of cells with standard column letters (A, B, C... AA) and row numbers (1, 2, 3...).
- Dynamic formula engine supporting:
  - Aggregate functions: `=SUM(range)`, `=AVERAGE(range)`, `=COUNT(range)`, `=MIN(range)`, `=MAX(range)`
  - Logical & conditional functions: `=IF(condition, true_val, false_val)`
  - Arithmetic operations: `+`, `-`, `*`, `/`, `^` with cell references (e.g., `=A1+B1*2`)
  - Range parsing (e.g. `A1:C10`) with automatic dependency resolution and instant recalculation.
- Interactive Formula Bar (`fx`) with cell coordinate Name Box and active selection highlight.

### 3. Zenith Present (Microsoft PowerPoint / Google Slides / LibreOffice Impress)
- 16:9 widescreen presentation canvas with slide deck layout templates (Title, Title & Content, Two Columns, Comparison, Blank).
- Left-hand filmstrip thumbnail navigator with real-time slide selection.
- Shape engine: Text frames, Card boxes, Bullet lists, and Callouts.
- Presenter speaker notes area and clean slideshow presentation preview.

### 4. Zenith Base (Microsoft Access / LibreOffice Base)
- Relational database schema designer with typed columns: `Integer`, `VarChar`, `Float`, `Date`, `Boolean`, and Primary Key constraints.
- Interactive record navigation strip: `|<<  <  Record X of Y  >  >>|  [+] New`.
- Built-in SQL Query Editor and execution console (`SELECT * FROM Customers WHERE Country = 'USA'`).
- Automated form view mapping table schemas into visual data-entry cards.

### 5. Microsoft & LibreOffice Typography
Native support for standard executive font families:
- **Microsoft Office Fonts**: Calibri, Times New Roman, Arial, Segoe UI, Cambria, Consolas, Georgia.
- **LibreOffice / Linux Fonts**: Liberation Sans, Liberation Serif, Liberation Mono, Ubuntu.
- Font metric calculations with automatic fallback font substitution.

### 6. Aero Web Browser Cloud Integration
- Native cloud editor mode inside **Aero Web Browser** (synthesizing Google Docs & Google Sheets).
- Document title pill with cloud save status: `✓ Saved to Nova Cloud Drive`.
- Google Docs-style live collaborator avatars and presence tags (`[JD]`, `[FH]`).
- Clean browser toolbar menu: `File  Edit  View  Insert  Format  Tools  Help`.

---

## Verification & Testing

```bash
# Run all workspace unit tests
cargo test --workspace

# Export high-resolution PNG visual snapshots
cargo run -p zenith-app -- --export-view-png
```

License: MIT OR Apache-2.0
