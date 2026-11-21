//! Unified Zenith Suite manager coordinating Writer, Calc, Present, and Base.

use crate::base::Database;
use crate::calc::Workbook;
use crate::present::Presentation;
use crate::writer::Document;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppModule {
    Writer = 0,
    Calc = 1,
    Present = 2,
    Base = 3,
}

impl AppModule {
    pub fn name(&self) -> &'static str {
        match self {
            AppModule::Writer => "Zenith Writer (Word)",
            AppModule::Calc => "Zenith Calc (Excel)",
            AppModule::Present => "Zenith Present (PowerPoint)",
            AppModule::Base => "Zenith Base (Access)",
        }
    }

    pub fn short_code(&self) -> &'static str {
        match self {
            AppModule::Writer => "W",
            AppModule::Calc => "X",
            AppModule::Present => "P",
            AppModule::Base => "B",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            AppModule::Writer => ".docx",
            AppModule::Calc => ".xlsx",
            AppModule::Present => ".pptx",
            AppModule::Base => ".accdb",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ZenithSuite {
    pub active_module: AppModule,
    pub writer_doc: Document,
    pub calc_workbook: Workbook,
    pub present_deck: Presentation,
    pub base_db: Database,
    pub shared_clipboard: String,
    pub autosave_enabled: bool,
    pub modified: bool,
}

impl ZenithSuite {
    pub fn new() -> Self {
        Self {
            active_module: AppModule::Writer,
            writer_doc: Document::new("Untitled Document"),
            calc_workbook: Workbook::new("Untitled Workbook"),
            present_deck: Presentation::new("Untitled Presentation"),
            base_db: Database::new("Untitled Database"),
            shared_clipboard: String::new(),
            autosave_enabled: true,
            modified: false,
        }
    }

    pub fn switch_module(&mut self, module: AppModule) {
        self.active_module = module;
    }

    pub fn active_title(&self) -> &str {
        match self.active_module {
            AppModule::Writer => &self.writer_doc.title,
            AppModule::Calc => &self.calc_workbook.title,
            AppModule::Present => &self.present_deck.title,
            AppModule::Base => &self.base_db.name,
        }
    }
}

impl Default for ZenithSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suite_module_switching() {
        let mut suite = ZenithSuite::new();
        assert_eq!(suite.active_module, AppModule::Writer);
        assert_eq!(suite.active_module.short_code(), "W");

        suite.switch_module(AppModule::Calc);
        assert_eq!(suite.active_module, AppModule::Calc);
        assert_eq!(suite.active_module.file_extension(), ".xlsx");

        suite.switch_module(AppModule::Present);
        assert_eq!(suite.active_module, AppModule::Present);

        suite.switch_module(AppModule::Base);
        assert_eq!(suite.active_module, AppModule::Base);
    }
}
