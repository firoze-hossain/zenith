//! Zenith Cloud Office: Web browser-integrated document & spreadsheet editor
//! synthesizing Google Docs and Google Sheets inside Aero Browser.

use crate::draw::{draw_rect_outline, draw_rounded_rect, Surface};
use crate::font::{draw_text_font, Font};
use crate::geom::{Color, Rect};

pub fn draw_cloud_office_view<S: Surface + ?Sized>(
    s: &mut S,
    bounds: Rect,
    doc_title: &str,
    is_spreadsheet: bool,
) {
    let font_reg = Font::regular();
    let font_sm = Font::small();

    // 1. Cloud Web App Header (y: bounds.y .. bounds.y + 36)
    let hdr_h = 36;
    let hdr_rect = Rect::new(bounds.x, bounds.y, bounds.w, hdr_h);
    s.fill_rect(hdr_rect, Color::rgb(24, 30, 44));
    s.fill_rect(Rect::new(bounds.x, bounds.y + hdr_h - 1, bounds.w, 1), Color::rgb(51, 65, 85));

    // Cloud Logo Pill
    let logo_x = bounds.x + 8;
    let logo_bg = if is_spreadsheet { Color::rgb(16, 185, 129) } else { Color::rgb(37, 99, 235) };
    draw_rounded_rect(s, Rect::new(logo_x, bounds.y + 6, 24, 24), 4, logo_bg);
    let logo_char = if is_spreadsheet { "X" } else { "W" };
    draw_text_font(s, logo_x + 8, bounds.y + 10, logo_char, Color::WHITE, &font_reg);

    // Document Title Pill & Cloud status
    let title_x = logo_x + 32;
    draw_text_font(s, title_x, bounds.y + 6, doc_title, Color::WHITE, &font_reg);
    let cloud_status = "✓ Saved to Nova Cloud Drive";
    draw_text_font(s, title_x, bounds.y + 20, cloud_status, Color::rgb(74, 222, 128), &font_sm);

    // Right Action Controls: [History] [Comments] [Share] [Avatars]
    let btn_share_x = bounds.right() - 84;
    let btn_share = Rect::new(btn_share_x, bounds.y + 6, 76, 24);
    draw_rounded_rect(s, btn_share, 12, Color::rgb(37, 99, 235));
    draw_text_font(s, btn_share.x + 12, btn_share.y + 5, "Share", Color::WHITE, &font_sm);

    let btn_hist_x = btn_share_x - 70;
    let btn_hist = Rect::new(btn_hist_x, bounds.y + 6, 64, 24);
    draw_rounded_rect(s, btn_hist, 4, Color::rgb(30, 41, 59));
    draw_text_font(s, btn_hist.x + 8, btn_hist.y + 5, "History", Color::rgb(203, 213, 225), &font_sm);

    // Collaborator Avatars (Google Docs style live presence)
    let ava_x = btn_hist_x - 56;
    draw_rounded_rect(s, Rect::new(ava_x, bounds.y + 6, 24, 24), 12, Color::rgb(245, 158, 11));
    draw_text_font(s, ava_x + 6, bounds.y + 10, "JD", Color::BLACK, &font_sm);

    draw_rounded_rect(s, Rect::new(ava_x + 20, bounds.y + 6, 24, 24), 12, Color::rgb(14, 165, 233));
    draw_text_font(s, ava_x + 26, bounds.y + 10, "FH", Color::WHITE, &font_sm);

    // 2. Google Docs / Sheets Menu Bar (y: bounds.y + 36 .. bounds.y + 58)
    let menu_y = bounds.y + hdr_h;
    let menu_h = 22;
    s.fill_rect(Rect::new(bounds.x, menu_y, bounds.w, menu_h), Color::rgb(30, 41, 59));
    s.fill_rect(Rect::new(bounds.x, menu_y + menu_h - 1, bounds.w, 1), Color::rgb(51, 65, 85));

    let menus = ["File", "Edit", "View", "Insert", "Format", "Tools", "Extensions", "Help"];
    let mut mx = bounds.x + 12;
    for &m in &menus {
        draw_text_font(s, mx, menu_y + 4, m, Color::rgb(203, 213, 225), &font_sm);
        mx += (m.len() as i32 * 6) + 16;
    }

    // 3. Document or Spreadsheet Canvas View
    let canvas_y = menu_y + menu_h;
    let canvas_h = bounds.bottom() - canvas_y;
    let canvas_rect = Rect::new(bounds.x, canvas_y, bounds.w, canvas_h);
    s.fill_rect(canvas_rect, Color::rgb(15, 23, 42));

    if is_spreadsheet {
        // Render Google Sheets grid preview
        let cell_w = 80;
        let cell_h = 22;
        let cols = (bounds.w / cell_w).max(1) as usize;
        let rows = (canvas_h / cell_h).max(1) as usize;

        for r in 0..rows {
            let ry = canvas_y + (r as i32) * cell_h;
            for c in 0..cols {
                let cx = bounds.x + (c as i32) * cell_w;
                let c_rect = Rect::new(cx, ry, cell_w, cell_h);
                let bg = if r == 0 { Color::rgb(30, 41, 59) } else { Color::WHITE };
                s.fill_rect(c_rect, bg);
                draw_rect_outline(s, c_rect, 1, Color::rgb(226, 232, 240));

                if r == 0 {
                    let col_letter = ((b'A' + (c as u8 % 26)) as char).to_string();
                    draw_text_font(s, cx + cell_w / 2 - 4, ry + 4, &col_letter, Color::WHITE, &font_sm);
                } else if r == 1 {
                    let sample = match c {
                        0 => "Q1 Revenue",
                        1 => "$142,500",
                        2 => "Active",
                        _ => "",
                    };
                    draw_text_font(s, cx + 6, ry + 4, sample, Color::rgb(15, 23, 42), &font_sm);
                }
            }
        }
    } else {
        // Render Google Docs page preview
        let page_w = (bounds.w - 60).min(520).max(280);
        let page_h = canvas_h - 24;
        let page_x = bounds.x + (bounds.w - page_w) / 2;
        let page_y = canvas_y + 12;
        let page_rect = Rect::new(page_x, page_y, page_w, page_h);

        // Drop shadow
        s.fill_rect(Rect::new(page_x + 4, page_y + 4, page_w, page_h), Color::rgba(0, 0, 0, 80));
        draw_rounded_rect(s, page_rect, 4, Color::WHITE);
        draw_rect_outline(s, page_rect, 1, Color::rgb(203, 213, 225));

        // Sample text
        let margin_x = page_x + 28;
        draw_text_font(s, margin_x, page_y + 24, doc_title, Color::rgb(15, 23, 42), &font_reg);
        s.fill_rect(Rect::new(margin_x, page_y + 46, page_w - 56, 1), Color::rgb(226, 232, 240));
        draw_text_font(s, margin_x, page_y + 56, "Real-time collaborative editing in Aero Web Browser.", Color::rgb(51, 65, 85), &font_sm);
        draw_text_font(s, margin_x, page_y + 74, "* Multi-user concurrent presence with live cursors.", Color::rgb(37, 99, 235), &font_sm);
        draw_text_font(s, margin_x, page_y + 92, "* Direct import/export to .docx, .xlsx, and .pdf.", Color::rgb(16, 185, 129), &font_sm);
    }
}
