//! Zenith Writer document viewport rendering.

use crate::draw::{draw_rect_outline, draw_rounded_rect, Surface};
use crate::font::{draw_text_font, Font};
use crate::geom::{Color, Rect};
use zenith_core::writer::{Document, ParagraphStyle};

pub fn draw_writer_view<S: Surface + ?Sized>(s: &mut S, bounds: Rect, doc: &Document) {
    let font_reg = Font::regular();
    let font_sm = Font::small();

    // 1. Studio Canvas Background (Dark obsidian workspace)
    s.fill_rect(bounds, Color::rgb(15, 23, 42));

    // 2. Margin Ruler Bar at top
    let ruler_h = 16;
    let ruler_rect = Rect::new(bounds.x, bounds.y, bounds.w, ruler_h);
    s.fill_rect(ruler_rect, Color::rgb(20, 27, 45));
    s.fill_rect(Rect::new(bounds.x, bounds.y + ruler_h - 1, bounds.w, 1), Color::rgb(51, 65, 85));

    // Draw inch tick marks on ruler
    let mut rx = bounds.x + 40;
    while rx < bounds.right() - 40 {
        s.fill_rect(Rect::new(rx, bounds.y + 4, 1, 8), Color::rgb(148, 163, 184));
        rx += 36; // half inch tick
    }

    // 3. Document Paper Sheet (Crisp ivory sheet centered)
    let paper_w = (bounds.w - 80).min(560).max(280);
    let paper_h = bounds.h - ruler_h - 32;
    let paper_x = bounds.x + (bounds.w - paper_w) / 2;
    let paper_y = bounds.y + ruler_h + 8;
    let paper_rect = Rect::new(paper_x, paper_y, paper_w, paper_h);

    // Drop shadow
    s.fill_rect(Rect::new(paper_x + 4, paper_y + 4, paper_w, paper_h), Color::rgba(0, 0, 0, 90));
    s.fill_rect(Rect::new(paper_x + 2, paper_y + 2, paper_w, paper_h), Color::rgba(0, 0, 0, 60));

    // Paper page
    draw_rounded_rect(s, paper_rect, 4, Color::WHITE);
    draw_rect_outline(s, paper_rect, 1, Color::rgb(203, 213, 225));

    // 4. Document Content inside margins
    let margin_x = paper_x + 32;
    let mut cur_y = paper_y + 28;

    // Header title
    draw_text_font(s, margin_x, cur_y, &doc.title, Color::rgb(15, 23, 42), &font_reg);
    cur_y += 24;

    // Subtitle / metadata
    let author_str = format!("Author: {} | Engine: Sovereign Rust 1.99.1", doc.author);
    draw_text_font(s, margin_x, cur_y, &author_str, Color::rgb(100, 116, 139), &font_sm);
    cur_y += 18;

    // Divider line
    s.fill_rect(Rect::new(margin_x, cur_y, paper_w - 64, 1), Color::rgb(226, 232, 240));
    cur_y += 14;

    // Paragraphs rendering
    for p in &doc.paragraphs {
        if cur_y > paper_rect.bottom() - 40 {
            break;
        }

        match p.style {
            ParagraphStyle::Title | ParagraphStyle::Heading1 => {
                cur_y += 8;
                draw_text_font(s, margin_x, cur_y, &p.plain_text(), Color::rgb(30, 41, 59), &font_reg);
                cur_y += 20;
            }
            ParagraphStyle::Heading2 => {
                cur_y += 6;
                draw_text_font(s, margin_x, cur_y, &p.plain_text(), Color::rgb(51, 65, 85), &font_reg);
                cur_y += 18;
            }
            ParagraphStyle::BulletList => {
                s.fill_rect(Rect::new(margin_x + 4, cur_y + 4, 4, 4), Color::rgb(37, 99, 235));
                draw_text_font(s, margin_x + 14, cur_y, &p.plain_text(), Color::rgb(51, 65, 85), &font_sm);
                cur_y += 16;
            }
            ParagraphStyle::Quote => {
                s.fill_rect(Rect::new(margin_x, cur_y, 3, 14), Color::rgb(14, 165, 233));
                draw_text_font(s, margin_x + 8, cur_y, &p.plain_text(), Color::rgb(71, 85, 105), &font_sm);
                cur_y += 18;
            }
            _ => {
                draw_text_font(s, margin_x, cur_y, &p.plain_text(), Color::rgb(51, 65, 85), &font_sm);
                cur_y += 16;
            }
        }
    }

    // Render sample table if present
    if !doc.tables.is_empty() {
        let t = &doc.tables[0];
        let t_y = cur_y + 10;
        let cell_w = (paper_w - 64) / (t.cols as i32).max(1);
        let cell_h = 20;

        for r in 0..t.rows {
            for c in 0..t.cols {
                let cx = margin_x + (c as i32) * cell_w;
                let cy = t_y + (r as i32) * cell_h;
                let c_rect = Rect::new(cx, cy, cell_w, cell_h);
                let bg = if r == 0 { Color::rgb(241, 245, 249) } else { Color::WHITE };
                s.fill_rect(c_rect, bg);
                draw_rect_outline(s, c_rect, 1, Color::rgb(203, 213, 225));
                if let Some(txt) = t.get_cell(r, c) {
                    draw_text_font(s, cx + 6, cy + 4, txt, Color::rgb(30, 41, 59), &font_sm);
                }
            }
        }
    }

    // 5. Bottom Status Bar (y: bounds.bottom() - 20)
    let stat_y = bounds.bottom() - 20;
    let stat_rect = Rect::new(bounds.x, stat_y, bounds.w, 20);
    s.fill_rect(stat_rect, Color::rgb(20, 27, 45));
    s.fill_rect(Rect::new(bounds.x, stat_y, bounds.w, 1), Color::rgb(51, 65, 85));

    let stat_text = format!(
        "Page 1 of {} | Words: {} | Chars: {} | English (US) | 100% Zoom",
        doc.estimated_pages(),
        doc.total_words(),
        doc.total_chars()
    );
    draw_text_font(s, bounds.x + 12, stat_y + 4, &stat_text, Color::rgb(148, 163, 184), &font_sm);
}
