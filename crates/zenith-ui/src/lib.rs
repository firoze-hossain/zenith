//! Zenith UI: Software 2D rendering and Fluent Ribbon suite for Zenith Office.

pub mod base_view;
pub mod calc_view;
pub mod cloud_view;
pub mod draw;
pub mod font;
pub mod geom;
pub mod present_view;
pub mod ribbon;
pub mod view;
pub mod writer_view;

pub use draw::{draw_line, draw_rect_outline, draw_rounded_rect, BufferSurface32, Surface};
pub use font::{draw_text_font, Font};
pub use geom::{Color, Point, Rect};
pub use ribbon::{draw_ribbon, RibbonState, RibbonTab};
pub use view::ZenithView;

#[cfg(test)]
mod tests {
    use super::*;
    use zenith_core::suite::AppModule;

    #[test]
    fn test_zenith_view_creation_and_switching() {
        let mut view = ZenithView::new();
        assert_eq!(view.suite.active_module, AppModule::Writer);

        // Click on Calc module pill
        view.handle_click(200, 15, Rect::new(0, 0, 800, 600));
        assert_eq!(view.suite.active_module, AppModule::Calc);

        // Click on Present module pill
        view.handle_click(290, 15, Rect::new(0, 0, 800, 600));
        assert_eq!(view.suite.active_module, AppModule::Present);

        // Click on Base module pill
        view.handle_click(370, 15, Rect::new(0, 0, 800, 600));
        assert_eq!(view.suite.active_module, AppModule::Base);
    }

    #[test]
    fn test_zenith_view_rendering() {
        let view = ZenithView::new();
        let mut surf = BufferSurface32::new(640, 480);
        view.draw(&mut surf, Rect::new(0, 0, 640, 480));

        // Ensure canvas was drawn (not all black)
        let pixel_check = surf.buffer.iter().any(|&p| p != 0xFF000000);
        assert!(pixel_check, "Buffer should contain drawn interface elements");
    }

    #[test]
    fn test_ribbon_toggles() {
        let mut view = ZenithView::new();
        assert!(!view.ribbon.bold);

        // Click Bold button (x: 215, y: 55)
        view.handle_click(215, 55, Rect::new(0, 0, 800, 600));
        assert!(view.ribbon.bold);

        // Click Bold button again
        view.handle_click(215, 55, Rect::new(0, 0, 800, 600));
        assert!(!view.ribbon.bold);
    }
}
