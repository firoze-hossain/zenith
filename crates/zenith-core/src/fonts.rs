//! Font metrics and typography engine supporting Microsoft Office & LibreOffice families.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontFamily {
    // Microsoft Office Typography
    Calibri,
    TimesNewRoman,
    Arial,
    SegoeUI,
    Cambria,
    Consolas,
    Georgia,

    // LibreOffice / Linux Typography
    LiberationSans,
    LiberationSerif,
    LiberationMono,
    Ubuntu,
}

impl FontFamily {
    pub const ALL: [FontFamily; 11] = [
        FontFamily::Calibri,
        FontFamily::TimesNewRoman,
        FontFamily::Arial,
        FontFamily::SegoeUI,
        FontFamily::Cambria,
        FontFamily::Consolas,
        FontFamily::Georgia,
        FontFamily::LiberationSans,
        FontFamily::LiberationSerif,
        FontFamily::LiberationMono,
        FontFamily::Ubuntu,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            FontFamily::Calibri => "Calibri",
            FontFamily::TimesNewRoman => "Times New Roman",
            FontFamily::Arial => "Arial",
            FontFamily::SegoeUI => "Segoe UI",
            FontFamily::Cambria => "Cambria",
            FontFamily::Consolas => "Consolas",
            FontFamily::Georgia => "Georgia",
            FontFamily::LiberationSans => "Liberation Sans",
            FontFamily::LiberationSerif => "Liberation Serif",
            FontFamily::LiberationMono => "Liberation Mono",
            FontFamily::Ubuntu => "Ubuntu",
        }
    }

    pub fn is_monospace(&self) -> bool {
        matches!(self, FontFamily::Consolas | FontFamily::LiberationMono)
    }

    pub fn is_serif(&self) -> bool {
        matches!(
            self,
            FontFamily::TimesNewRoman
                | FontFamily::Cambria
                | FontFamily::Georgia
                | FontFamily::LiberationSerif
        )
    }

    pub fn default_size_pt(&self) -> u8 {
        match self {
            FontFamily::Calibri => 11,
            FontFamily::TimesNewRoman => 12,
            FontFamily::Arial => 11,
            FontFamily::SegoeUI => 10,
            FontFamily::Cambria => 12,
            FontFamily::Consolas => 10,
            FontFamily::Georgia => 11,
            FontFamily::LiberationSans => 11,
            FontFamily::LiberationSerif => 12,
            FontFamily::LiberationMono => 10,
            FontFamily::Ubuntu => 11,
        }
    }

    /// Rough character width factor in 1/1000 em
    pub fn avg_char_width_factor(&self) -> u32 {
        if self.is_monospace() {
            600
        } else if self.is_serif() {
            540
        } else {
            520
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let lower = name.to_lowercase();
        if lower.contains("calibri") {
            Some(FontFamily::Calibri)
        } else if lower.contains("times") {
            Some(FontFamily::TimesNewRoman)
        } else if lower.contains("arial") {
            Some(FontFamily::Arial)
        } else if lower.contains("segoe") {
            Some(FontFamily::SegoeUI)
        } else if lower.contains("cambria") {
            Some(FontFamily::Cambria)
        } else if lower.contains("consolas") {
            Some(FontFamily::Consolas)
        } else if lower.contains("georgia") {
            Some(FontFamily::Georgia)
        } else if lower.contains("liberation sans") {
            Some(FontFamily::LiberationSans)
        } else if lower.contains("liberation serif") {
            Some(FontFamily::LiberationSerif)
        } else if lower.contains("liberation mono") {
            Some(FontFamily::LiberationMono)
        } else if lower.contains("ubuntu") {
            Some(FontFamily::Ubuntu)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_family_properties() {
        assert_eq!(FontFamily::Calibri.name(), "Calibri");
        assert_eq!(FontFamily::Calibri.default_size_pt(), 11);
        assert!(!FontFamily::Calibri.is_serif());
        assert!(!FontFamily::Calibri.is_monospace());

        assert!(FontFamily::TimesNewRoman.is_serif());
        assert!(FontFamily::Consolas.is_monospace());
        assert!(FontFamily::LiberationMono.is_monospace());
        assert!(FontFamily::LiberationSerif.is_serif());

        assert_eq!(FontFamily::from_name("times new roman"), Some(FontFamily::TimesNewRoman));
        assert_eq!(FontFamily::from_name("Ubuntu"), Some(FontFamily::Ubuntu));
    }
}
