//! Fluent Ribbon Toolbar for Zenith Office Suite.

use crate::draw::{draw_rect_outline, draw_rounded_rect, Surface};
use crate::font::{draw_text_font, Font};
use crate::geom::{Color, Rect};
use zenith_core::fonts::FontFamily;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RibbonTab {
    Home,
    Insert,
    Layout,
    Formulas,
    SlideShow,
    Data,
    View,
}

impl RibbonTab {
    pub const ALL: [RibbonTab; 7] = [
        RibbonTab::Home,
        RibbonTab::Insert,
        RibbonTab::Layout,
        RibbonTab::Formulas,
        RibbonTab::SlideShow,
        RibbonTab::Data,
        RibbonTab::View,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            RibbonTab::Home => "Home",
            RibbonTab::Insert => "Insert",
            RibbonTab::Layout => "Layout",
            RibbonTab::Formulas => "Formulas",
            RibbonTab::SlideShow => "Slide Show",
            RibbonTab::Data => "Data",
            RibbonTab::View => "View",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RibbonState {
    pub active_tab: RibbonTab,
    pub font_family: FontFamily,
    pub font_size_pt: u8,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub align: u8, // 0 = Left, 1 = Center, 2 = Right, 3 = Justify
    pub font_dropdown_open: bool,
}

impl RibbonState {
    pub fn new() -> Self {
        Self {
            active_tab: RibbonTab::Home,
            font_family: FontFamily::Calibri,
            font_size_pt: 11,
            bold: false,
            italic: false,
            underline: false,
            strike: false,
            align: 0,
            font_dropdown_open: false,
        }
    }
}

impl Default for RibbonState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn draw_ribbon<S: Surface + ?Sized>(
    s: &mut S,
    bounds: Rect,
    state: &RibbonState,
    theme_accent: Color,
) {
    let font_reg = Font::regular();
    let font_sm = Font::small();

    // 1. Ribbon Tab Strip (y: bounds.y .. bounds.y + 24)
    let strip_h = 24;
    s.fill_rect(Rect::new(bounds.x, bounds.y, bounds.w, strip_h), Color::rgb(24, 30, 44));

    let mut tab_x = bounds.x + 8;
    for &tab in &RibbonTab::ALL {
        let is_sel = state.active_tab == tab;
        let t_w = (tab.label().len() as i32 * 8) + 16;
        let t_rect = Rect::new(tab_x, bounds.y + 2, t_w, strip_h - 2);

        if is_sel {
            s.fill_rect(t_rect, Color::rgb(30, 41, 59));
            s.fill_rect(Rect::new(tab_x + 2, bounds.y + strip_h - 2, t_w - 4, 2), theme_accent);
            draw_text_font(s, tab_x + 8, bounds.y + 6, tab.label(), Color::WHITE, &font_sm);
        } else {
            draw_text_font(s, tab_x + 8, bounds.y + 6, tab.label(), Color::rgb(148, 163, 184), &font_sm);
        }
        tab_x += t_w + 4;
    }

    // 2. Ribbon Commands Area (y: bounds.y + 24 .. bounds.bottom())
    let cmd_y = bounds.y + strip_h;
    let cmd_h = bounds.h - strip_h;
    s.fill_rect(Rect::new(bounds.x, cmd_y, bounds.w, cmd_h), Color::rgb(30, 41, 59));
    draw_rect_outline(s, Rect::new(bounds.x, cmd_y, bounds.w, cmd_h), 1, Color::rgb(51, 65, 85));

    // Group 1: Font Family Selector (e.g. Calibri, Times New Roman, Arial...)
    let font_box_w = 140;
    let font_box_h = 22;
    let font_box_x = bounds.x + 10;
    let font_box_y = cmd_y + 6;
    draw_rounded_rect(s, Rect::new(font_box_x, font_box_y, font_box_w, font_box_h), 3, Color::rgb(15, 23, 42));
    draw_rect_outline(s, Rect::new(font_box_x, font_box_y, font_box_w, font_box_h), 1, Color::rgb(71, 85, 105));
    draw_text_font(s, font_box_x + 6, font_box_y + 4, state.font_family.name(), Color::WHITE, &font_sm);
    draw_text_font(s, font_box_x + font_box_w - 14, font_box_y + 4, "v", Color::rgb(148, 163, 184), &font_sm);

    // Group 2: Font Size Box (e.g. 11, 12, 14...)
    let size_box_w = 40;
    let size_box_x = font_box_x + font_box_w + 6;
    draw_rounded_rect(s, Rect::new(size_box_x, font_box_y, size_box_w, font_box_h), 3, Color::rgb(15, 23, 42));
    draw_rect_outline(s, Rect::new(size_box_x, font_box_y, size_box_w, font_box_h), 1, Color::rgb(71, 85, 105));
    let sz_str = format!("{}", state.font_size_pt);
    draw_text_font(s, size_box_x + 8, font_box_y + 4, &sz_str, Color::WHITE, &font_sm);

    // Group 3: B / I / U / S formatting toggle buttons
    let toggles = [
        ("B", state.bold),
        ("I", state.italic),
        ("U", state.underline),
        ("S", state.strike),
    ];
    let mut btn_x = size_box_x + size_box_w + 10;
    for (label, active) in toggles {
        let btn_rect = Rect::new(btn_x, font_box_y, 24, font_box_h);
        let bg = if active { theme_accent } else { Color::rgb(20, 27, 45) };
        draw_rounded_rect(s, btn_rect, 3, bg);
        if active {
            draw_rect_outline(s, btn_rect, 1, Color::WHITE);
        }
        draw_text_font(s, btn_x + 8, font_box_y + 4, label, Color::WHITE, &font_reg);
        btn_x += 28;
    }

    // Group 4: Text Alignments (Left, Center, Right, Justify)
    let aligns = ["L=", "C=", "R=", "J="];
    btn_x += 8;
    for (idx, &label) in aligns.iter().enumerate() {
        let is_sel = state.align == idx as u8;
        let btn_rect = Rect::new(btn_x, font_box_y, 24, font_box_h);
        let bg = if is_sel { Color::rgb(71, 85, 105) } else { Color::rgb(20, 27, 45) };
        draw_rounded_rect(s, btn_rect, 3, bg);
        draw_text_font(s, btn_x + 4, font_box_y + 4, label, if is_sel { Color::WHITE } else { Color::rgb(148, 163, 184) }, &font_sm);
        btn_x += 28;
    }

    // Group 5: Productivity Quick Actions (Table, Formula, Slideshow, SQL)
    btn_x += 12;
    let actions = match state.active_tab {
        RibbonTab::Home => &["+ Table", "+ Image", "Styles", "Find"][..],
        RibbonTab::Insert => &["Table", "Shape", "Chart", "Page Break"][..],
        RibbonTab::Formulas => &["AutoSum", "Average", "Min", "Max"][..],
        RibbonTab::SlideShow => &["From Start", "Current", "Notes", "Loop"][..],
        RibbonTab::Data => &["New Table", "Run SQL", "Filter", "Export"][..],
        _ => &["Ruler", "Grid", "100%", "Dark Mode"][..],
    };

    for &act in actions {
        let a_w = (act.len() as i32 * 6) + 16;
        let act_rect = Rect::new(btn_x, font_box_y, a_w, font_box_h);
        draw_rounded_rect(s, act_rect, 3, Color::rgb(20, 27, 45));
        draw_rect_outline(s, act_rect, 1, Color::rgb(71, 85, 105));
        draw_text_font(s, btn_x + 8, font_box_y + 4, act, Color::rgb(226, 232, 240), &font_sm);
        btn_x += a_w + 6;
    }

    // 3. Dropdown list if open
    if state.font_dropdown_open {
        let dd_w = 160;
        let dd_h = (FontFamily::ALL.len() as i32) * 20 + 8;
        let dd_rect = Rect::new(font_box_x, font_box_y + font_box_h + 2, dd_w, dd_h);
        draw_rounded_rect(s, dd_rect, 4, Color::rgb(15, 23, 42));
        draw_rect_outline(s, dd_rect, 1, Color::rgb(56, 189, 248));

        for (i, &f) in FontFamily::ALL.iter().enumerate() {
            let row_y = dd_rect.y + 4 + (i as i32) * 20;
            let is_cur = state.font_family == f;
            if is_cur {
                s.fill_rect(Rect::new(dd_rect.x + 2, row_y, dd_w - 4, 18), Color::rgb(30, 41, 59));
            }
            draw_text_font(s, dd_rect.x + 8, row_y + 2, f.name(), if is_cur { Color::WHITE } else { Color::rgb(203, 213, 225) }, &font_sm);
        }
    }
}
