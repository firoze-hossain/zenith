//! Zenith Base relational database & forms viewport rendering.

use crate::draw::{draw_rect_outline, draw_rounded_rect, Surface};
use crate::font::{draw_text_font, Font};
use crate::geom::{Color, Rect};
use zenith_core::base::Database;

pub fn draw_base_view<S: Surface + ?Sized>(s: &mut S, bounds: Rect, db: &Database) {
    let font_reg = Font::regular();
    let font_sm = Font::small();

    // 1. Studio Canvas Background
    s.fill_rect(bounds, Color::rgb(15, 23, 42));

    // 2. Left Schema / Tables Sidebar (w = 120)
    let side_w = 120;
    let side_rect = Rect::new(bounds.x, bounds.y, side_w, bounds.h - 20);
    s.fill_rect(side_rect, Color::rgb(20, 27, 45));
    s.fill_rect(Rect::new(bounds.x + side_w, bounds.y, 1, bounds.h - 20), Color::rgb(51, 65, 85));

    draw_text_font(s, bounds.x + 8, bounds.y + 8, "TABLES & FORMS", Color::rgb(148, 163, 184), &font_sm);

    for (i, table) in db.tables.iter().enumerate() {
        let is_sel = db.active_table_idx == i;
        let t_y = bounds.y + 26 + (i as i32) * 26;
        let t_rect = Rect::new(bounds.x + 6, t_y, side_w - 12, 22);

        if is_sel {
            draw_rounded_rect(s, t_rect, 3, Color::rgb(30, 41, 59));
            s.fill_rect(Rect::new(bounds.x + 6, t_y + 3, 3, 16), Color::rgb(124, 58, 237));
            draw_text_font(s, bounds.x + 14, t_y + 4, &table.name, Color::WHITE, &font_sm);
        } else {
            draw_text_font(s, bounds.x + 14, t_y + 4, &table.name, Color::rgb(203, 213, 225), &font_sm);
        }
    }

    // 3. Main Records Data Sheet View
    let main_x = bounds.x + side_w + 1;
    let main_w = bounds.w - side_w - 1;
    let query_h = 60;
    let table_h = bounds.h - query_h - 44;

    let active_t = db.active_table();

    // Table Header Banner
    let th_y = bounds.y;
    s.fill_rect(Rect::new(main_x, th_y, main_w, 24), Color::rgb(30, 41, 59));
    s.fill_rect(Rect::new(main_x, th_y + 23, main_w, 1), Color::rgb(51, 65, 85));
    let hdr_label = format!("Table: {} ({} records)", active_t.name, active_t.records.len());
    draw_text_font(s, main_x + 10, th_y + 5, &hdr_label, Color::rgb(199, 210, 254), &font_reg);

    // Columns Grid Header
    let col_h = 22;
    let col_w = (main_w - 10) / (active_t.columns.len() as i32).max(1);
    let col_y = th_y + 24;
    s.fill_rect(Rect::new(main_x, col_y, main_w, col_h), Color::rgb(24, 30, 44));
    s.fill_rect(Rect::new(main_x, col_y + col_h - 1, main_w, 1), Color::rgb(51, 65, 85));

    for (c_idx, col) in active_t.columns.iter().enumerate() {
        let cx = main_x + (c_idx as i32) * col_w;
        let c_rect = Rect::new(cx, col_y, col_w, col_h);
        draw_rect_outline(s, c_rect, 1, Color::rgb(51, 65, 85));

        let pk_mark = if col.is_primary_key { "[PK] " } else { "" };
        let col_title = format!("{}{}", pk_mark, col.name);
        draw_text_font(s, cx + 6, col_y + 4, &col_title, Color::rgb(245, 158, 11), &font_sm);
    }

    // Records Rows
    let rows_y = col_y + col_h;
    let row_h = 22;
    for (r_idx, rec) in active_t.records.iter().enumerate() {
        let ry = rows_y + (r_idx as i32) * row_h;
        if ry + row_h > bounds.y + table_h {
            break;
        }

        let is_alt = r_idx % 2 == 1;
        let row_bg = if is_alt { Color::rgb(20, 27, 45) } else { Color::rgb(15, 23, 42) };
        s.fill_rect(Rect::new(main_x, ry, main_w, row_h), row_bg);

        for (c_idx, val) in rec.fields.iter().enumerate() {
            let cx = main_x + (c_idx as i32) * col_w;
            let c_rect = Rect::new(cx, ry, col_w, row_h);
            draw_rect_outline(s, c_rect, 1, Color::rgb(30, 41, 59));
            draw_text_font(s, cx + 6, ry + 4, &val.display(), Color::rgb(226, 232, 240), &font_sm);
        }
    }

    // 4. Record Navigation Strip (y: bounds.y + table_h .. bounds.y + table_h + 24)
    let nav_y = bounds.y + table_h;
    let nav_rect = Rect::new(main_x, nav_y, main_w, 24);
    s.fill_rect(nav_rect, Color::rgb(24, 30, 44));
    s.fill_rect(Rect::new(main_x, nav_y, main_w, 1), Color::rgb(51, 65, 85));

    let nav_str = format!("|<<  <   Record 1 of {}   >  >>|  [+] New Record", active_t.records.len());
    draw_text_font(s, main_x + 12, nav_y + 5, &nav_str, Color::rgb(148, 163, 184), &font_sm);

    // 5. Interactive SQL Query Console (y: bounds.bottom() - query_h - 20)
    let q_y = bounds.bottom() - query_h - 20;
    let q_rect = Rect::new(main_x, q_y, main_w, query_h);
    s.fill_rect(q_rect, Color::rgb(10, 15, 30));
    s.fill_rect(Rect::new(main_x, q_y, main_w, 1), Color::rgb(51, 65, 85));

    draw_text_font(s, main_x + 8, q_y + 6, "SQL Query Editor:", Color::rgb(124, 58, 237), &font_sm);
    let sample_sql = "SELECT CustomerID, CompanyName, TotalSpend FROM Customers WHERE Country = 'USA';";
    draw_text_font(s, main_x + 8, q_y + 22, sample_sql, Color::rgb(74, 222, 128), &font_sm);
    draw_text_font(s, main_x + 8, q_y + 38, "[OK] 1 row affected (Execution time: 0.12ms in pure Rust engine)", Color::rgb(148, 163, 184), &font_sm);

    // 6. Bottom Status Bar
    let stat_y = bounds.bottom() - 20;
    let stat_rect = Rect::new(bounds.x, stat_y, bounds.w, 20);
    s.fill_rect(stat_rect, Color::rgb(15, 23, 42));
    s.fill_rect(Rect::new(bounds.x, stat_y, bounds.w, 1), Color::rgb(51, 65, 85));

    let stat_msg = format!("Database: {} | Active Table: {} | Engine: Pure Rust Relational Base", db.name, active_t.name);
    draw_text_font(s, bounds.x + 12, stat_y + 4, &stat_msg, Color::rgb(203, 213, 225), &font_sm);
}
