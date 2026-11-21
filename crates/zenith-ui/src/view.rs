//! Zenith Studio View: Master composited workspace view with interactive
//! module switcher, ribbon toolbar, and click dispatch.

use crate::base_view::draw_base_view;
use crate::calc_view::draw_calc_view;
use crate::draw::{draw_rect_outline, draw_rounded_rect, Surface};
use crate::font::{draw_text_font, Font};
use crate::geom::{Color, Rect};
use crate::present_view::draw_present_view;
use crate::ribbon::{draw_ribbon, RibbonState, RibbonTab};
use crate::writer_view::draw_writer_view;
use zenith_core::fonts::FontFamily;
use zenith_core::suite::{AppModule, ZenithSuite};

pub struct ZenithView {
    pub suite: ZenithSuite,
    pub ribbon: RibbonState,
    pub show_about: bool,
}

impl ZenithView {
    pub fn new() -> Self {
        Self {
            suite: ZenithSuite::new(),
            ribbon: RibbonState::new(),
            show_about: false,
        }
    }

    pub fn set_module(&mut self, module: AppModule) {
        self.suite.switch_module(module);
    }

    pub fn set_font(&mut self, font: FontFamily) {
        self.ribbon.font_family = font;
    }

    pub fn toggle_about(&mut self) {
        self.show_about = !self.show_about;
    }

    pub fn handle_click(&mut self, px: i32, py: i32, bounds: Rect) {
        let rel_x = px - bounds.x;
        let rel_y = py - bounds.y;

        // 1. If About dialog is open, click anywhere dismisses it
        if self.show_about {
            self.show_about = false;
            return;
        }

        // 2. Top Module Switcher Bar (rel_y: 0..28)
        if rel_y >= 0 && rel_y <= 28 {
            // Check About button '?' on far right
            if rel_x >= bounds.w - 32 && rel_x <= bounds.w - 6 {
                self.show_about = true;
                return;
            }

            // App Module Switchers: [W] Writer, [X] Calc, [P] Present, [B] Base
            // Offset from left ~100px
            let modules = [
                (100, 180, AppModule::Writer),
                (185, 255, AppModule::Calc),
                (260, 345, AppModule::Present),
                (350, 420, AppModule::Base),
            ];
            for (x0, x1, m) in modules {
                if rel_x >= x0 && rel_x <= x1 {
                    self.set_module(m);
                    return;
                }
            }
            return;
        }

        // 3. Ribbon Bar Clicks (rel_y: 28..88)
        if rel_y >= 28 && rel_y <= 88 {
            let ribbon_y = rel_y - 28;

            // Tab strip (ribbon_y: 0..24)
            if ribbon_y <= 24 {
                let tab_idx = (rel_x - 8) / 60;
                if tab_idx >= 0 && (tab_idx as usize) < RibbonTab::ALL.len() {
                    self.ribbon.active_tab = RibbonTab::ALL[tab_idx as usize];
                }
                return;
            }

            // Command area (ribbon_y: 24..60)
            // Font dropdown toggle: rel_x in 10..150
            if rel_x >= 10 && rel_x <= 150 {
                self.ribbon.font_dropdown_open = !self.ribbon.font_dropdown_open;
                return;
            }

            // Font size: rel_x in 156..196
            if rel_x >= 156 && rel_x <= 196 {
                self.ribbon.font_size_pt = if self.ribbon.font_size_pt >= 14 { 11 } else { self.ribbon.font_size_pt + 1 };
                return;
            }

            // B (206..230)
            if rel_x >= 206 && rel_x <= 230 {
                self.ribbon.bold = !self.ribbon.bold;
                return;
            }
            // I (234..258)
            if rel_x >= 234 && rel_x <= 258 {
                self.ribbon.italic = !self.ribbon.italic;
                return;
            }
            // U (262..286)
            if rel_x >= 262 && rel_x <= 286 {
                self.ribbon.underline = !self.ribbon.underline;
                return;
            }
            // S (290..314)
            if rel_x >= 290 && rel_x <= 314 {
                self.ribbon.strike = !self.ribbon.strike;
                return;
            }

            // Alignments: L= (322..346), C= (350..374), R= (378..402), J= (406..430)
            if rel_x >= 322 && rel_x <= 430 {
                let a_idx = ((rel_x - 322) / 28).min(3) as u8;
                self.ribbon.align = a_idx;
                return;
            }
            return;
        }

        // Close dropdown if open and clicked elsewhere
        if self.ribbon.font_dropdown_open {
            self.ribbon.font_dropdown_open = false;
        }
    }

    pub fn draw<S: Surface + ?Sized>(&self, s: &mut S, bounds: Rect) {
        let font_reg = Font::regular();
        let font_sm = Font::small();

        // 1. Top Suite Application Header (h = 28)
        let top_h = 28;
        let top_rect = Rect::new(bounds.x, bounds.y, bounds.w, top_h);
        s.fill_rect(top_rect, Color::rgb(15, 23, 42));
        s.fill_rect(Rect::new(bounds.x, bounds.y + top_h - 1, bounds.w, 1), Color::rgb(51, 65, 85));

        // Brand Emblem
        let emblem_col = match self.suite.active_module {
            AppModule::Writer => Color::rgb(37, 99, 235),  // Word Blue
            AppModule::Calc => Color::rgb(16, 185, 129),   // Excel Green
            AppModule::Present => Color::rgb(245, 158, 11),// PowerPoint Amber/Orange
            AppModule::Base => Color::rgb(124, 58, 237),   // Access Purple
        };
        draw_rounded_rect(s, Rect::new(bounds.x + 8, bounds.y + 4, 20, 20), 4, emblem_col);
        draw_text_font(s, bounds.x + 13, bounds.y + 6, self.suite.active_module.short_code(), Color::WHITE, &font_reg);

        // Suite Brand Name
        draw_text_font(s, bounds.x + 34, bounds.y + 7, "ZENITH", Color::rgb(241, 245, 249), &font_reg);

        // Module Switcher Pills: [W] Writer, [X] Calc, [P] Present, [B] Base
        let modules = [
            ("Writer", AppModule::Writer),
            ("Calc", AppModule::Calc),
            ("Present", AppModule::Present),
            ("Base", AppModule::Base),
        ];

        let mut mx = bounds.x + 100;
        for (m_name, m_mod) in modules {
            let is_sel = self.suite.active_module == m_mod;
            let m_w = (m_name.len() as i32 * 6) + 24;
            let m_rect = Rect::new(mx, bounds.y + 3, m_w, 22);

            let bg = if is_sel { Color::rgb(30, 41, 59) } else { Color::rgb(20, 27, 45) };
            draw_rounded_rect(s, m_rect, 4, bg);
            if is_sel {
                draw_rect_outline(s, m_rect, 1, emblem_col);
            }
            draw_text_font(s, mx + 8, bounds.y + 7, m_name, if is_sel { Color::WHITE } else { Color::rgb(148, 163, 184) }, &font_sm);
            mx += m_w + 6;
        }

        // Active document title pill
        let title_str = format!("{} - {}", self.suite.active_title(), self.suite.active_module.file_extension());
        draw_text_font(s, mx + 16, bounds.y + 7, &title_str, Color::rgb(203, 213, 225), &font_sm);

        // About button '?' on far right
        let btn_ab_x = bounds.right() - 28;
        draw_rounded_rect(s, Rect::new(btn_ab_x, bounds.y + 4, 20, 20), 3, Color::rgba(255, 255, 255, 20));
        draw_text_font(s, btn_ab_x + 6, bounds.y + 6, "?", Color::rgb(203, 213, 225), &font_sm);

        // 2. Fluent Ribbon Bar (h = 60)
        let ribbon_h = 60;
        let ribbon_rect = Rect::new(bounds.x, bounds.y + top_h, bounds.w, ribbon_h);
        draw_ribbon(s, ribbon_rect, &self.ribbon, emblem_col);

        // 3. Main Module Viewport
        let body_y = bounds.y + top_h + ribbon_h;
        let body_h = bounds.bottom() - body_y;
        let body_rect = Rect::new(bounds.x, body_y, bounds.w, body_h);

        match self.suite.active_module {
            AppModule::Writer => draw_writer_view(s, body_rect, &self.suite.writer_doc),
            AppModule::Calc => draw_calc_view(s, body_rect, self.suite.calc_workbook.active_sheet()),
            AppModule::Present => draw_present_view(s, body_rect, &self.suite.present_deck),
            AppModule::Base => draw_base_view(s, body_rect, &self.suite.base_db),
        }

        // 4. Modal About Dialog (if open)
        if self.show_about {
            s.fill_rect(bounds, Color::rgba(0, 0, 0, 160));

            let ab_w = 440.min(bounds.w - 32);
            let ab_h = 250.min(bounds.h - 32);
            let ab_x = bounds.x + (bounds.w - ab_w) / 2;
            let ab_y = bounds.y + (bounds.h - ab_h) / 2;
            let ab_rect = Rect::new(ab_x, ab_y, ab_w, ab_h);

            draw_rounded_rect(s, ab_rect, 8, Color::rgb(15, 23, 42));
            draw_rect_outline(s, ab_rect, 2, Color::rgb(37, 99, 235));

            // Header
            draw_rounded_rect(s, Rect::new(ab_x + 16, ab_y + 14, 26, 26), 5, Color::rgb(37, 99, 235));
            draw_text_font(s, ab_x + 24, ab_y + 17, "Z", Color::WHITE, &font_reg);
            draw_text_font(s, ab_x + 52, ab_y + 14, "ZENITH OFFICE SUITE", Color::WHITE, &font_reg);
            draw_text_font(s, ab_x + 52, ab_y + 28, "v1.0.0 (Pure Rust 1.99.1)", Color::rgb(56, 189, 248), &font_sm);

            draw_rect_outline(s, Rect::new(ab_x + 14, ab_y + 48, ab_w - 28, 1), 1, Color::rgb(48, 54, 61));

            draw_text_font(s, ab_x + 16, ab_y + 58, "Sovereign Enterprise Productivity Suite for NovaOS & Web", Color::rgb(226, 232, 240), &font_sm);
            draw_text_font(s, ab_x + 16, ab_y + 76, "Modules: Zenith Writer, Zenith Calc, Zenith Present, Zenith Base", Color::rgb(148, 163, 184), &font_sm);
            draw_text_font(s, ab_x + 16, ab_y + 98, "Full Microsoft Office & LibreOffice Typography (Calibri, Times, Ubuntu)", Color::rgb(74, 222, 128), &font_sm);
            draw_text_font(s, ab_x + 16, ab_y + 116, "Desktop Standalone + Aero Web Browser Cloud Docs Integration", Color::rgb(245, 158, 11), &font_sm);

            // Dismiss Button
            let btn_close = Rect::new(ab_x + (ab_w - 80) / 2, ab_y + ab_h - 36, 80, 24);
            draw_rounded_rect(s, btn_close, 4, Color::rgb(37, 99, 235));
            draw_text_font(s, btn_close.x + 18, btn_close.y + 5, "Dismiss", Color::WHITE, &font_sm);
        }
    }
}

impl Default for ZenithView {
    fn default() -> Self {
        Self::new()
    }
}
