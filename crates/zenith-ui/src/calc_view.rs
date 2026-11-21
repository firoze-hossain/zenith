//! Zenith Calc spreadsheet grid viewport rendering.

use crate::draw::{draw_rect_outline, draw_rounded_rect, Surface};
use crate::font::{draw_text_font, Font};
use crate::geom::{Color, Rect};
use zenith_core::calc::{CellAddress, CellValue, Worksheet};

pub fn draw_calc_view<S: Surface + ?Sized>(s: &mut S, bounds: Rect, sheet: &Worksheet) {
    let font_reg = Font::regular();
    let font_sm = Font::small();

    // 1. Top Formula Bar (y: bounds.y .. bounds.y + 26)
    let fb_h = 26;
    let fb_rect = Rect::new(bounds.x, bounds.y, bounds.w, fb_h);
    s.fill_rect(fb_rect, Color::rgb(24, 30, 44));
    s.fill_rect(Rect::new(bounds.x, bounds.y + fb_h - 1, bounds.w, 1), Color::rgb(51, 65, 85));

    // Name Box (Active cell e.g. "A1", "C4")
    let nb_w = 48;
    let nb_rect = Rect::new(bounds.x + 6, bounds.y + 3, nb_w, 20);
    draw_rounded_rect(s, nb_rect, 2, Color::rgb(15, 23, 42));
    draw_rect_outline(s, nb_rect, 1, Color::rgb(71, 85, 105));
    let cell_name = sheet.active_cell.to_string();
    draw_text_font(s, nb_rect.x + 6, nb_rect.y + 4, &cell_name, Color::WHITE, &font_sm);

    // fx glyph
    let fx_x = nb_rect.right() + 8;
    draw_text_font(s, fx_x, bounds.y + 5, "fx", Color::rgb(56, 189, 248), &font_reg);

    // Formula Input Bar
    let fi_x = fx_x + 22;
    let fi_w = bounds.right() - fi_x - 10;
    let fi_rect = Rect::new(fi_x, bounds.y + 3, fi_w, 20);
    draw_rounded_rect(s, fi_rect, 2, Color::rgb(15, 23, 42));
    draw_rect_outline(s, fi_rect, 1, Color::rgb(71, 85, 105));

    // Get active cell formula or display string
    let formula_str = if let Some(cell) = sheet.get_cell(sheet.active_cell) {
        match cell.value {
            CellValue::Formula(ref f) => f.clone(),
            _ => cell.calculated.display_string(),
        }
    } else {
        String::new()
    };
    draw_text_font(s, fi_rect.x + 8, fi_rect.y + 4, &formula_str, Color::WHITE, &font_sm);

    // 2. Spreadsheet Grid Area (y: bounds.y + fb_h .. bounds.bottom() - 24)
    let grid_y = bounds.y + fb_h;
    let row_header_w = 40;
    let col_header_h = 20;
    let cell_w = 75;
    let cell_h = 20;

    let cols_visible = ((bounds.w - row_header_w) / cell_w).max(1) as usize;
    let rows_visible = ((bounds.h - fb_h - 48) / cell_h).max(1) as usize;

    // Corner box (intersection of row & col headers)
    let corner_rect = Rect::new(bounds.x, grid_y, row_header_w, col_header_h);
    s.fill_rect(corner_rect, Color::rgb(30, 41, 59));
    draw_rect_outline(s, corner_rect, 1, Color::rgb(51, 65, 85));

    // Column Header strip (A, B, C, D...)
    for c in 0..cols_visible {
        let cx = bounds.x + row_header_w + (c as i32) * cell_w;
        let c_rect = Rect::new(cx, grid_y, cell_w, col_header_h);
        let is_active_col = sheet.active_cell.col == c;
        let bg = if is_active_col { Color::rgb(51, 65, 85) } else { Color::rgb(30, 41, 59) };
        s.fill_rect(c_rect, bg);
        draw_rect_outline(s, c_rect, 1, Color::rgb(51, 65, 85));
        let c_name = CellAddress::col_name(c);
        draw_text_font(s, cx + cell_w / 2 - 4, grid_y + 4, &c_name, Color::WHITE, &font_sm);
    }

    // Row Header strip and Cell Matrix
    for r in 0..rows_visible {
        let ry = grid_y + col_header_h + (r as i32) * cell_h;

        // Row header box (1, 2, 3...)
        let is_active_row = sheet.active_cell.row == r;
        let rh_rect = Rect::new(bounds.x, ry, row_header_w, cell_h);
        let rh_bg = if is_active_row { Color::rgb(51, 65, 85) } else { Color::rgb(30, 41, 59) };
        s.fill_rect(rh_rect, rh_bg);
        draw_rect_outline(s, rh_rect, 1, Color::rgb(51, 65, 85));
        let r_str = format!("{}", r + 1);
        draw_text_font(s, bounds.x + 8, ry + 4, &r_str, Color::rgb(203, 213, 225), &font_sm);

        // Cells in row
        for c in 0..cols_visible {
            let cx = bounds.x + row_header_w + (c as i32) * cell_w;
            let cell_rect = Rect::new(cx, ry, cell_w, cell_h);

            let is_sel = sheet.active_cell.col == c && sheet.active_cell.row == r;
            let cell_addr = CellAddress::new(c, r);

            // Cell background
            let cell_bg = if is_sel { Color::rgb(240, 249, 255) } else { Color::WHITE };
            s.fill_rect(cell_rect, cell_bg);
            draw_rect_outline(s, cell_rect, 1, Color::rgb(226, 232, 240));

            // Cell value text
            if let Some(cell) = sheet.get_cell(cell_addr) {
                let disp = cell.calculated.display_string();
                let txt_col = match cell.calculated {
                    CellValue::Error(_) => Color::rgb(220, 38, 38),
                    CellValue::Currency(_) => Color::rgb(16, 185, 129),
                    _ => Color::rgb(15, 23, 42),
                };
                draw_text_font(s, cx + 6, ry + 4, &disp, txt_col, &font_sm);
            }

            // Active cell cursor highlight with corner drag box
            if is_sel {
                draw_rect_outline(s, cell_rect, 2, Color::rgb(14, 165, 233));
                let handle_rect = Rect::new(cell_rect.right() - 4, cell_rect.bottom() - 4, 4, 4);
                s.fill_rect(handle_rect, Color::rgb(14, 165, 233));
            }
        }
    }

    // 3. Sheet Tabs Bar (y: bounds.bottom() - 44 .. bounds.bottom() - 20)
    let tabs_y = bounds.bottom() - 44;
    let tabs_rect = Rect::new(bounds.x, tabs_y, bounds.w, 24);
    s.fill_rect(tabs_rect, Color::rgb(20, 27, 45));
    s.fill_rect(Rect::new(bounds.x, tabs_y, bounds.w, 1), Color::rgb(51, 65, 85));

    let sheet_names = ["Sheet1 (Active)", "Budget_Q4", "Employees", "+"];
    let mut tx = bounds.x + 8;
    for (idx, &s_name) in sheet_names.iter().enumerate() {
        let is_act = idx == 0;
        let t_w = (s_name.len() as i32 * 6) + 16;
        let t_rect = Rect::new(tx, tabs_y + 2, t_w, 20);
        let bg = if is_act { Color::rgb(30, 41, 59) } else { Color::rgb(15, 23, 42) };
        draw_rounded_rect(s, t_rect, 3, bg);
        if is_act {
            s.fill_rect(Rect::new(tx + 2, tabs_y + 20, t_w - 4, 2), Color::rgb(16, 185, 129));
        }
        draw_text_font(s, tx + 6, tabs_y + 5, s_name, if is_act { Color::WHITE } else { Color::rgb(148, 163, 184) }, &font_sm);
        tx += t_w + 6;
    }

    // 4. Bottom Status / Telemetry Bar
    let stat_y = bounds.bottom() - 20;
    let stat_rect = Rect::new(bounds.x, stat_y, bounds.w, 20);
    s.fill_rect(stat_rect, Color::rgb(15, 23, 42));
    s.fill_rect(Rect::new(bounds.x, stat_y, bounds.w, 1), Color::rgb(51, 65, 85));

    let stat_msg = "Ready | Average: 166.67 | Count: 3 | Sum: 500.00 | Zoom: 100%";
    draw_text_font(s, bounds.x + 12, stat_y + 4, stat_msg, Color::rgb(74, 222, 128), &font_sm);
}
