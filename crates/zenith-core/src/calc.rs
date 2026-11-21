//! Zenith Calc: Spreadsheet matrix and formula evaluation engine synthesizing
//! Microsoft Excel, Google Sheets, and LibreOffice Calc in pure Rust.

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellAddress {
    pub col: usize, // 0 = A, 1 = B, 25 = Z, 26 = AA
    pub row: usize, // 0 = 1, 1 = 2, 2 = 3...
}

impl CellAddress {
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }

    /// Converts column number (0-indexed) to letter(s): 0 -> "A", 25 -> "Z", 26 -> "AA"
    pub fn col_name(mut col: usize) -> String {
        let mut name = Vec::new();
        loop {
            let rem = col % 26;
            name.push((b'A' + rem as u8) as char);
            if col < 26 {
                break;
            }
            col = (col / 26) - 1;
        }
        name.into_iter().rev().collect()
    }

    /// Converts 0-indexed (col, row) to Excel address string (e.g. 0, 0 -> "A1")
    pub fn to_string(&self) -> String {
        format!("{}{}", Self::col_name(self.col), self.row + 1)
    }

    /// Parses Excel address string (e.g. "A1", "BC42") into CellAddress
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim().to_uppercase();
        if s.is_empty() {
            return None;
        }

        let mut letters = String::new();
        let mut digits = String::new();
        for ch in s.chars() {
            if ch.is_ascii_alphabetic() {
                if !digits.is_empty() {
                    return None; // letters after digits is invalid
                }
                letters.push(ch);
            } else if ch.is_ascii_digit() {
                digits.push(ch);
            } else {
                return None;
            }
        }

        if letters.is_empty() || digits.is_empty() {
            return None;
        }

        let mut col: usize = 0;
        for ch in letters.chars() {
            col = col * 26 + (ch as usize - 'A' as usize + 1);
        }
        col -= 1;

        let row_1based: usize = digits.parse().ok()?;
        if row_1based == 0 {
            return None;
        }
        Some(Self {
            col,
            row: row_1based - 1,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Formula(String),
    Boolean(bool),
    Currency(f64),
    Percentage(f64),
    Error(String),
}

impl CellValue {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            CellValue::Number(n) => Some(*n),
            CellValue::Currency(n) => Some(*n),
            CellValue::Percentage(n) => Some(*n),
            CellValue::Text(s) => s.trim().parse::<f64>().ok(),
            CellValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    pub fn display_string(&self) -> String {
        match self {
            CellValue::Empty => String::new(),
            CellValue::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e12 {
                    format!("{:.0}", n)
                } else {
                    format!("{:.2}", n)
                }
            }
            CellValue::Text(s) => s.clone(),
            CellValue::Formula(f) => f.clone(),
            CellValue::Boolean(b) => if *b { "TRUE".into() } else { "FALSE".into() },
            CellValue::Currency(n) => format!("${:.2}", n),
            CellValue::Percentage(n) => format!("{:.1}%", n * 100.0),
            CellValue::Error(e) => format!("#ERR: {}", e),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellFormat {
    pub bold: bool,
    pub italic: bool,
    pub align: u8, // 0 = default (text left, num right), 1 = left, 2 = center, 3 = right
    pub bg_color: Option<(u8, u8, u8)>,
}

impl Default for CellFormat {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            align: 0,
            bg_color: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub value: CellValue,
    pub calculated: CellValue,
    pub format: CellFormat,
}

impl Cell {
    pub fn new(value: CellValue) -> Self {
        let calculated = value.clone();
        Self {
            value,
            calculated,
            format: CellFormat::default(),
        }
    }

    pub fn number(n: f64) -> Self {
        Self::new(CellValue::Number(n))
    }

    pub fn text(s: impl Into<String>) -> Self {
        Self::new(CellValue::Text(s.into()))
    }

    pub fn formula(f: impl Into<String>) -> Self {
        Self::new(CellValue::Formula(f.into()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Worksheet {
    pub name: String,
    pub cells: HashMap<CellAddress, Cell>,
    pub active_cell: CellAddress,
    pub column_widths: HashMap<usize, u32>,
    pub row_heights: HashMap<usize, u32>,
}

impl Worksheet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cells: HashMap::new(),
            active_cell: CellAddress::new(0, 0),
            column_widths: HashMap::new(),
            row_heights: HashMap::new(),
        }
    }

    pub fn set_cell(&mut self, addr: CellAddress, cell: Cell) {
        self.cells.insert(addr, cell);
    }

    pub fn set_number(&mut self, col: usize, row: usize, val: f64) {
        self.set_cell(CellAddress::new(col, row), Cell::number(val));
    }

    pub fn set_text(&mut self, col: usize, row: usize, val: impl Into<String>) {
        self.set_cell(CellAddress::new(col, row), Cell::text(val));
    }

    pub fn set_formula(&mut self, col: usize, row: usize, f: impl Into<String>) {
        self.set_cell(CellAddress::new(col, row), Cell::formula(f));
    }

    pub fn get_cell(&self, addr: CellAddress) -> Option<&Cell> {
        self.cells.get(&addr)
    }

    pub fn get_calculated(&self, addr: CellAddress) -> CellValue {
        self.cells
            .get(&addr)
            .map(|c| c.calculated.clone())
            .unwrap_or(CellValue::Empty)
    }

    /// Evaluates all formulas across the worksheet
    pub fn recalculate(&mut self) {
        let addresses: Vec<CellAddress> = self.cells.keys().cloned().collect();
        for addr in addresses {
            if let Some(cell) = self.cells.get(&addr) {
                if let CellValue::Formula(ref f) = cell.value {
                    let formula_text = f.clone();
                    let computed = evaluate_formula(&formula_text, self);
                    if let Some(c) = self.cells.get_mut(&addr) {
                        c.calculated = computed;
                    }
                }
            }
        }
    }
}

/// Evaluates Excel / Sheets formulas like "=SUM(A1:A5)", "=AVERAGE(B1:B4)", "=A1+B1*2"
pub fn evaluate_formula(formula: &str, sheet: &Worksheet) -> CellValue {
    let f = formula.trim();
    let body = if f.starts_with('=') { &f[1..] } else { f }.trim();

    if body.is_empty() {
        return CellValue::Empty;
    }

    let upper = body.to_uppercase();

    // 1. Function evaluation: SUM, AVERAGE, COUNT, MIN, MAX
    if upper.starts_with("SUM(") && upper.ends_with(')') {
        let arg = &body[4..body.len() - 1].trim();
        let nums = resolve_range_numbers(arg, sheet);
        let sum: f64 = nums.iter().sum();
        return CellValue::Number(sum);
    }

    if upper.starts_with("AVERAGE(") && upper.ends_with(')') {
        let arg = &body[8..body.len() - 1].trim();
        let nums = resolve_range_numbers(arg, sheet);
        if nums.is_empty() {
            return CellValue::Number(0.0);
        }
        let avg: f64 = nums.iter().sum::<f64>() / nums.len() as f64;
        return CellValue::Number(avg);
    }

    if upper.starts_with("COUNT(") && upper.ends_with(')') {
        let arg = &body[6..body.len() - 1].trim();
        let nums = resolve_range_numbers(arg, sheet);
        return CellValue::Number(nums.len() as f64);
    }

    if upper.starts_with("MIN(") && upper.ends_with(')') {
        let arg = &body[4..body.len() - 1].trim();
        let nums = resolve_range_numbers(arg, sheet);
        let min = nums.into_iter().fold(f64::INFINITY, f64::min);
        return CellValue::Number(if min.is_infinite() { 0.0 } else { min });
    }

    if upper.starts_with("MAX(") && upper.ends_with(')') {
        let arg = &body[4..body.len() - 1].trim();
        let nums = resolve_range_numbers(arg, sheet);
        let max = nums.into_iter().fold(f64::NEG_INFINITY, f64::max);
        return CellValue::Number(if max.is_infinite() { 0.0 } else { max });
    }

    if upper.starts_with("IF(") && upper.ends_with(')') {
        let inner = &body[3..body.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() >= 2 {
            let cond_str = parts[0];
            let is_true = evaluate_condition(cond_str, sheet);
            let result_str = if is_true { parts[1] } else if parts.len() >= 3 { parts[2] } else { "0" };
            let clean = result_str.trim_matches('"');
            if let Ok(num) = clean.parse::<f64>() {
                return CellValue::Number(num);
            }
            return CellValue::Text(clean.to_string());
        }
    }

    // 2. Simple binary operations: e.g. A1 + B1, A1 * 1.25, A1 - B2
    for op in ['+', '-', '*', '/'] {
        if let Some(idx) = body.find(op) {
            let left_str = &body[..idx].trim();
            let right_str = &body[idx + 1..].trim();
            let left_val = resolve_term_number(left_str, sheet);
            let right_val = resolve_term_number(right_str, sheet);

            let res = match op {
                '+' => left_val + right_val,
                '-' => left_val - right_val,
                '*' => left_val * right_val,
                '/' => {
                    if right_val == 0.0 {
                        return CellValue::Error("DIV/0!".into());
                    }
                    left_val / right_val
                }
                _ => 0.0,
            };
            return CellValue::Number(res);
        }
    }

    // 3. Single cell reference or literal
    if let Some(addr) = CellAddress::parse(body) {
        return sheet.get_calculated(addr);
    }
    if let Ok(num) = body.parse::<f64>() {
        return CellValue::Number(num);
    }

    CellValue::Text(body.to_string())
}

fn resolve_range_numbers(range_str: &str, sheet: &Worksheet) -> Vec<f64> {
    let mut out = Vec::new();
    if let Some(colon) = range_str.find(':') {
        let start_str = &range_str[..colon].trim();
        let end_str = &range_str[colon + 1..].trim();
        if let (Some(a), Some(b)) = (CellAddress::parse(start_str), CellAddress::parse(end_str)) {
            let min_col = a.col.min(b.col);
            let max_col = a.col.max(b.col);
            let min_row = a.row.min(b.row);
            let max_row = a.row.max(b.row);

            for c in min_col..=max_col {
                for r in min_row..=max_row {
                    let addr = CellAddress::new(c, r);
                    if let Some(cell) = sheet.get_cell(addr) {
                        if let Some(num) = cell.calculated.as_number() {
                            out.push(num);
                        }
                    }
                }
            }
        }
    } else if let Some(addr) = CellAddress::parse(range_str) {
        if let Some(cell) = sheet.get_cell(addr) {
            if let Some(num) = cell.calculated.as_number() {
                out.push(num);
            }
        }
    }
    out
}

fn resolve_term_number(term: &str, sheet: &Worksheet) -> f64 {
    if let Some(addr) = CellAddress::parse(term) {
        sheet.get_calculated(addr).as_number().unwrap_or(0.0)
    } else {
        term.parse::<f64>().unwrap_or(0.0)
    }
}

fn evaluate_condition(cond: &str, sheet: &Worksheet) -> bool {
    let operators = [">=", "<=", "!=", "==", ">", "<", "="];
    for op in operators {
        if let Some(idx) = cond.find(op) {
            let left_str = &cond[..idx].trim();
            let right_str = &cond[idx + op.len()..].trim();
            let left_val = resolve_term_number(left_str, sheet);
            let right_val = resolve_term_number(right_str, sheet);

            return match op {
                ">=" => left_val >= right_val,
                "<=" => left_val <= right_val,
                "!=" => (left_val - right_val).abs() > 1e-9,
                "==" | "=" => (left_val - right_val).abs() <= 1e-9,
                ">" => left_val > right_val,
                "<" => left_val < right_val,
                _ => false,
            };
        }
    }
    false
}

#[derive(Clone, Debug, PartialEq)]
pub struct Workbook {
    pub title: String,
    pub sheets: Vec<Worksheet>,
    pub active_sheet_idx: usize,
}

impl Workbook {
    pub fn new(title: impl Into<String>) -> Self {
        let mut wb = Self {
            title: title.into(),
            sheets: Vec::new(),
            active_sheet_idx: 0,
        };
        wb.sheets.push(Worksheet::new("Sheet1"));
        wb
    }

    pub fn active_sheet(&self) -> &Worksheet {
        &self.sheets[self.active_sheet_idx]
    }

    pub fn active_sheet_mut(&mut self) -> &mut Worksheet {
        &mut self.sheets[self.active_sheet_idx]
    }

    pub fn add_sheet(&mut self, name: impl Into<String>) -> usize {
        self.sheets.push(Worksheet::new(name));
        self.sheets.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_address_conversion() {
        assert_eq!(CellAddress::col_name(0), "A");
        assert_eq!(CellAddress::col_name(1), "B");
        assert_eq!(CellAddress::col_name(25), "Z");
        assert_eq!(CellAddress::col_name(26), "AA");

        let addr = CellAddress::new(0, 0);
        assert_eq!(addr.to_string(), "A1");

        let parsed = CellAddress::parse("B12").unwrap();
        assert_eq!(parsed.col, 1);
        assert_eq!(parsed.row, 11);
    }

    #[test]
    fn test_formula_evaluation() {
        let mut ws = Worksheet::new("Financials");
        ws.set_number(0, 0, 100.0); // A1
        ws.set_number(0, 1, 250.0); // A2
        ws.set_number(0, 2, 150.0); // A3

        // SUM formula
        let sum_res = evaluate_formula("=SUM(A1:A3)", &ws);
        assert_eq!(sum_res, CellValue::Number(500.0));

        // AVERAGE formula
        let avg_res = evaluate_formula("=AVERAGE(A1:A3)", &ws);
        assert!((avg_res.as_number().unwrap() - 166.666).abs() < 0.01);

        // Binary operation
        let add_res = evaluate_formula("=A1 + A2", &ws);
        assert_eq!(add_res, CellValue::Number(350.0));

        // IF condition
        let if_res = evaluate_formula("=IF(A1 > 50, \"Exceeded\", \"Below\")", &ws);
        assert_eq!(if_res, CellValue::Text("Exceeded".into()));
    }
}
