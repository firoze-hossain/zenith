//! Zenith Present slide deck studio viewport rendering.

use crate::draw::{draw_rect_outline, draw_rounded_rect, Surface};
use crate::font::{draw_text_font, Font};
use crate::geom::{Color, Rect};
use zenith_core::present::{Presentation, SlideShape};

pub fn draw_present_view<S: Surface + ?Sized>(s: &mut S, bounds: Rect, pres: &Presentation) {
    let font_reg = Font::regular();
    let font_sm = Font::small();

    // 1. Studio Canvas Background
    s.fill_rect(bounds, Color::rgb(15, 23, 42));

    // 2. Left Filmstrip Thumbnail Sidebar (w = 110)
    let side_w = 110;
    let side_rect = Rect::new(bounds.x, bounds.y, side_w, bounds.h - 20);
    s.fill_rect(side_rect, Color::rgb(20, 27, 45));
    s.fill_rect(Rect::new(bounds.x + side_w, bounds.y, 1, bounds.h - 20), Color::rgb(51, 65, 85));

    // Draw slide thumbnails
    for (i, slide) in pres.slides.iter().enumerate() {
        let th_y = bounds.y + 10 + (i as i32) * 62;
        if th_y + 54 > bounds.bottom() - 24 {
            break;
        }

        let is_cur = pres.current_slide == i;
        let th_rect = Rect::new(bounds.x + 8, th_y, side_w - 16, 52);

        // Slide number
        let num_str = format!("{}", i + 1);
        draw_text_font(s, bounds.x + 2, th_y + 18, &num_str, Color::rgb(148, 163, 184), &font_sm);

        // Mini card
        s.fill_rect(th_rect, Color::rgb(241, 245, 249));
        let border_col = if is_cur { Color::rgb(245, 158, 11) } else { Color::rgb(71, 85, 105) };
        let border_w = if is_cur { 2 } else { 1 };
        draw_rect_outline(s, th_rect, border_w, border_col);

        // Title preview
        let preview_title = if slide.title.len() > 10 { &slide.title[..10] } else { &slide.title };
        draw_text_font(s, th_rect.x + 4, th_rect.y + 6, preview_title, Color::rgb(15, 23, 42), &font_sm);
    }

    // 3. Center Slide Canvas (16:9 widescreen presentation stage)
    let center_x = bounds.x + side_w + 1;
    let center_w = bounds.w - side_w - 1;
    let notes_h = 70;
    let stage_h = bounds.h - notes_h - 20;

    let slide_w = (center_w - 60).min(560).max(280);
    let slide_h = (slide_w * 9) / 16;
    let slide_x = center_x + (center_w - slide_w) / 2;
    let slide_y = bounds.y + (stage_h - slide_h) / 2;
    let slide_rect = Rect::new(slide_x, slide_y, slide_w, slide_h);

    // Drop shadow
    s.fill_rect(Rect::new(slide_x + 6, slide_y + 6, slide_w, slide_h), Color::rgba(0, 0, 0, 100));
    s.fill_rect(Rect::new(slide_x + 3, slide_y + 3, slide_w, slide_h), Color::rgba(0, 0, 0, 60));

    // Slide background
    draw_rounded_rect(s, slide_rect, 4, Color::WHITE);
    draw_rect_outline(s, slide_rect, 1, Color::rgb(203, 213, 225));

    // Render active slide content
    let cur_slide = pres.active_slide();
    let margin_x = slide_x + 28;
    let mut cy = slide_y + 24;

    // Slide Title banner
    draw_text_font(s, margin_x, cy, &cur_slide.title, Color::rgb(15, 23, 42), &font_reg);
    cy += 24;

    if !cur_slide.subtitle.is_empty() {
        draw_text_font(s, margin_x, cy, &cur_slide.subtitle, Color::rgb(217, 119, 6), &font_sm);
        cy += 18;
    }

    // Divider line
    s.fill_rect(Rect::new(margin_x, cy, slide_w - 56, 2), Color::rgb(245, 158, 11));
    cy += 16;

    // Slide shapes
    for shape in &cur_slide.shapes {
        match shape {
            SlideShape::BulletList { items, .. } => {
                for item in items {
                    s.fill_rect(Rect::new(margin_x + 4, cy + 4, 4, 4), Color::rgb(217, 119, 6));
                    draw_text_font(s, margin_x + 14, cy, item, Color::rgb(30, 41, 59), &font_sm);
                    cy += 18;
                }
            }
            SlideShape::CardBox { title, body, .. } => {
                let card_rect = Rect::new(margin_x, cy, slide_w - 56, 36);
                s.fill_rect(card_rect, Color::rgb(248, 250, 252));
                draw_rect_outline(s, card_rect, 1, Color::rgb(203, 213, 225));
                draw_text_font(s, card_rect.x + 8, card_rect.y + 6, title, Color::rgb(15, 23, 42), &font_sm);
                draw_text_font(s, card_rect.x + 8, card_rect.y + 20, body, Color::rgb(100, 116, 139), &font_sm);
                cy += 42;
            }
            _ => {}
        }
    }

    // 4. Bottom Speaker Notes Panel
    let notes_y = bounds.y + stage_h;
    let notes_rect = Rect::new(center_x, notes_y, center_w, notes_h);
    s.fill_rect(notes_rect, Color::rgb(20, 27, 45));
    s.fill_rect(Rect::new(center_x, notes_y, center_w, 1), Color::rgb(51, 65, 85));

    draw_text_font(s, center_x + 12, notes_y + 8, "Speaker Notes:", Color::rgb(245, 158, 11), &font_sm);
    let note_text = if cur_slide.speaker_notes.is_empty() {
        "Click to add presenter speech cues, audience talking points, and Q&A references."
    } else {
        &cur_slide.speaker_notes
    };
    draw_text_font(s, center_x + 12, notes_y + 26, note_text, Color::rgb(148, 163, 184), &font_sm);

    // 5. Bottom Status Bar
    let stat_y = bounds.bottom() - 20;
    let stat_rect = Rect::new(bounds.x, stat_y, bounds.w, 20);
    s.fill_rect(stat_rect, Color::rgb(15, 23, 42));
    s.fill_rect(Rect::new(bounds.x, stat_y, bounds.w, 1), Color::rgb(51, 65, 85));

    let stat_str = format!(
        "Slide {} of {} | Layout: Title & Content | 16:9 Widescreen | Transitions: Fade",
        pres.current_slide + 1,
        pres.slides.len()
    );
    draw_text_font(s, bounds.x + 12, stat_y + 4, &stat_str, Color::rgb(203, 213, 225), &font_sm);
}
