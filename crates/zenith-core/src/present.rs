//! Zenith Present: Presentation slide deck engine synthesizing
//! Microsoft PowerPoint, Google Slides, and LibreOffice Impress in pure Rust.

use crate::fonts::FontFamily;
use crate::writer::ColorRgb;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlideLayout {
    TitleSlide,
    TitleAndContent,
    SectionHeader,
    TwoContent,
    Comparison,
    TitleOnly,
    Blank,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SlideShape {
    TextBox {
        text: String,
        font: FontFamily,
        size_pt: u8,
        bold: bool,
        color: ColorRgb,
    },
    BulletList {
        items: Vec<String>,
        font: FontFamily,
        size_pt: u8,
        color: ColorRgb,
    },
    CardBox {
        title: String,
        body: String,
        bg_color: ColorRgb,
        border_color: ColorRgb,
    },
    Callout {
        text: String,
        highlight_color: ColorRgb,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Slide {
    pub id: usize,
    pub title: String,
    pub subtitle: String,
    pub layout: SlideLayout,
    pub shapes: Vec<SlideShape>,
    pub speaker_notes: String,
    pub background_color: ColorRgb,
    pub transition: String,
}

impl Slide {
    pub fn new(id: usize, layout: SlideLayout, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            subtitle: String::new(),
            layout,
            shapes: Vec::new(),
            speaker_notes: String::new(),
            background_color: ColorRgb::WHITE,
            transition: "Fade".into(),
        }
    }

    pub fn with_bullets(mut self, bullets: &[&str]) -> Self {
        self.shapes.push(SlideShape::BulletList {
            items: bullets.iter().map(|s| s.to_string()).collect(),
            font: FontFamily::Calibri,
            size_pt: 16,
            color: ColorRgb::BLACK,
        });
        self
    }

    pub fn add_card(&mut self, title: impl Into<String>, body: impl Into<String>, bg: ColorRgb) {
        self.shapes.push(SlideShape::CardBox {
            title: title.into(),
            body: body.into(),
            bg_color: bg,
            border_color: ColorRgb::GRAY,
        });
    }

    pub fn total_shapes(&self) -> usize {
        self.shapes.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Presentation {
    pub title: String,
    pub author: String,
    pub slides: Vec<Slide>,
    pub current_slide: usize,
    pub aspect_ratio_16_9: bool,
}

impl Presentation {
    pub fn new(title: impl Into<String>) -> Self {
        let mut pres = Self {
            title: title.into(),
            author: "NovaOS Administrator".into(),
            slides: Vec::new(),
            current_slide: 0,
            aspect_ratio_16_9: true,
        };

        // Create default title slide
        let mut slide1 = Slide::new(1, SlideLayout::TitleSlide, "NovaOS Enterprise Strategy 2026");
        slide1.subtitle = "High-Performance Sovereign Desktop Architecture".into();
        pres.slides.push(slide1);

        pres
    }

    pub fn add_slide(&mut self, layout: SlideLayout, title: impl Into<String>) -> usize {
        let next_id = self.slides.len() + 1;
        let slide = Slide::new(next_id, layout, title);
        self.slides.push(slide);
        self.slides.len() - 1
    }

    pub fn active_slide(&self) -> &Slide {
        &self.slides[self.current_slide]
    }

    pub fn active_slide_mut(&mut self) -> &mut Slide {
        &mut self.slides[self.current_slide]
    }

    pub fn next_slide(&mut self) -> bool {
        if self.current_slide + 1 < self.slides.len() {
            self.current_slide += 1;
            true
        } else {
            false
        }
    }

    pub fn prev_slide(&mut self) -> bool {
        if self.current_slide > 0 {
            self.current_slide -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_navigation_and_shapes() {
        let mut p = Presentation::new("Enterprise Keynote");
        assert_eq!(p.slides.len(), 1);

        p.add_slide(SlideLayout::TitleAndContent, "Microkernel Advantages");
        p.slides[1] = p.slides[1].clone().with_bullets(&[
            "Dual 32-bit and 64-bit ABI compatibility",
            "Hardware isolation and zero-overhead IPC",
            "Memory safety through pure Rust 1.99.1",
        ]);

        assert_eq!(p.slides.len(), 2);
        assert_eq!(p.current_slide, 0);

        assert!(p.next_slide());
        assert_eq!(p.current_slide, 1);
        assert!(!p.next_slide()); // at end

        assert!(p.prev_slide());
        assert_eq!(p.current_slide, 0);

        let s2 = &p.slides[1];
        assert_eq!(s2.shapes.len(), 1);
    }
}
