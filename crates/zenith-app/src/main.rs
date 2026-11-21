//! Zenith Office Suite CLI & Headless Snapshot Exporter.

pub mod png;
pub mod sample;

use std::env;
use std::fs;
use std::path::Path;
use zenith_core::suite::AppModule;
use zenith_ui::cloud_view::draw_cloud_office_view;
use zenith_ui::draw::BufferSurface32;
use zenith_ui::geom::Rect;
use zenith_ui::view::ZenithView;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut view = ZenithView::new();
    view.suite = sample::create_sample_suite();

    if args.len() > 1 {
        match args[1].as_str() {
            "--export-view-png" | "-e" => {
                let out_dir = args.get(2).cloned().unwrap_or_else(|| {
                    "/home/firoze/.gemini/antigravity/brain/7192285d-9f72-46e9-869e-0c53ffc319de".to_string()
                });
                export_all_snapshots(&out_dir);
                return;
            }
            "--writer" | "-w" => {
                view.set_module(AppModule::Writer);
                println!("Zenith Writer active: {}", view.suite.writer_doc.title);
            }
            "--calc" | "-c" => {
                view.set_module(AppModule::Calc);
                println!("Zenith Calc active: {}", view.suite.calc_workbook.title);
            }
            "--present" | "-p" => {
                view.set_module(AppModule::Present);
                println!("Zenith Present active: {}", view.suite.present_deck.title);
            }
            "--base" | "-b" => {
                view.set_module(AppModule::Base);
                println!("Zenith Base active: {}", view.suite.base_db.name);
            }
            "--info" | "-i" => {
                print_info(&view);
                return;
            }
            "--version" | "-v" => {
                println!("Zenith Office Suite v1.0.0 (Pure Rust 1.99.1)");
                println!("Synthesizes Microsoft Office (Word, Excel, PowerPoint, Access) & LibreOffice");
                return;
            }
            _ => {
                let file = &args[1];
                if file.ends_with(".xlsx") || file.ends_with(".ods") || file.ends_with(".csv") {
                    view.set_module(AppModule::Calc);
                } else if file.ends_with(".pptx") || file.ends_with(".odp") {
                    view.set_module(AppModule::Present);
                } else if file.ends_with(".accdb") || file.ends_with(".odb") {
                    view.set_module(AppModule::Base);
                } else {
                    view.set_module(AppModule::Writer);
                }
                println!("Opening '{}' in {}", file, view.suite.active_module.name());
            }
        }
    } else {
        print_help();
    }
}

fn print_help() {
    println!("Zenith Office Suite 1.0.0 (Pure Rust 1.99.1)");
    println!("Usage: zenith [options | file.docx | file.xlsx | file.pptx | file.accdb]");
    println!();
    println!("Options:");
    println!("  --writer, -w          Launch Zenith Writer (Word/Docs)");
    println!("  --calc, -c            Launch Zenith Calc (Excel/Sheets)");
    println!("  --present, -p         Launch Zenith Present (PowerPoint/Slides)");
    println!("  --base, -b            Launch Zenith Base (Access/Database)");
    println!("  --export-view-png [d] Render and save high-resolution visual PNG snapshots");
    println!("  --info, -i            Print suite statistics and loaded document telemetry");
    println!("  --version, -v         Display suite version");
}

fn print_info(view: &ZenithView) {
    println!("=== Zenith Office Suite Telemetry ===");
    println!("Active Module: {}", view.suite.active_module.name());
    println!("Writer Document: '{}' ({} words, {} est. pages)",
        view.suite.writer_doc.title,
        view.suite.writer_doc.total_words(),
        view.suite.writer_doc.estimated_pages()
    );
    println!("Calc Workbook: '{}' ({} sheets)",
        view.suite.calc_workbook.title,
        view.suite.calc_workbook.sheets.len()
    );
    println!("Present Presentation: '{}' ({} slides)",
        view.suite.present_deck.title,
        view.suite.present_deck.slides.len()
    );
    println!("Base Database: '{}' ({} tables)",
        view.suite.base_db.name,
        view.suite.base_db.tables.len()
    );
    println!("Typography Engine: 11 Microsoft & LibreOffice font families loaded");
}

pub fn export_all_snapshots(target_dir: &str) {
    let p = Path::new(target_dir);
    if !p.exists() {
        let _ = fs::create_dir_all(p);
    }

    println!("[*] Exporting Zenith Office high-resolution visual PNG snapshots to: {}", target_dir);

    let width = 1024;
    let height = 640;

    // 1. Snapshot: Zenith Writer
    {
        let mut view = ZenithView::new();
        view.suite = sample::create_sample_suite();
        view.set_module(AppModule::Writer);
        let mut surf = BufferSurface32::new(width, height);
        view.draw(&mut surf, Rect::new(0, 0, width, height));

        let png_bytes = png::encode_png_32(width as u32, height as u32, &surf.buffer);
        let out_path = format!("{}/view_zenith_writer.png", target_dir);
        fs::write(&out_path, png_bytes).expect("Failed to write view_zenith_writer.png");
        println!("  [+] Generated: {}", out_path);
    }

    // 2. Snapshot: Zenith Calc
    {
        let mut view = ZenithView::new();
        view.suite = sample::create_sample_suite();
        view.set_module(AppModule::Calc);
        let mut surf = BufferSurface32::new(width, height);
        view.draw(&mut surf, Rect::new(0, 0, width, height));

        let png_bytes = png::encode_png_32(width as u32, height as u32, &surf.buffer);
        let out_path = format!("{}/view_zenith_calc.png", target_dir);
        fs::write(&out_path, png_bytes).expect("Failed to write view_zenith_calc.png");
        println!("  [+] Generated: {}", out_path);
    }

    // 3. Snapshot: Zenith Present
    {
        let mut view = ZenithView::new();
        view.suite = sample::create_sample_suite();
        view.set_module(AppModule::Present);
        let mut surf = BufferSurface32::new(width, height);
        view.draw(&mut surf, Rect::new(0, 0, width, height));

        let png_bytes = png::encode_png_32(width as u32, height as u32, &surf.buffer);
        let out_path = format!("{}/view_zenith_present.png", target_dir);
        fs::write(&out_path, png_bytes).expect("Failed to write view_zenith_present.png");
        println!("  [+] Generated: {}", out_path);
    }

    // 4. Snapshot: Zenith Base
    {
        let mut view = ZenithView::new();
        view.suite = sample::create_sample_suite();
        view.set_module(AppModule::Base);
        let mut surf = BufferSurface32::new(width, height);
        view.draw(&mut surf, Rect::new(0, 0, width, height));

        let png_bytes = png::encode_png_32(width as u32, height as u32, &surf.buffer);
        let out_path = format!("{}/view_zenith_base.png", target_dir);
        fs::write(&out_path, png_bytes).expect("Failed to write view_zenith_base.png");
        println!("  [+] Generated: {}", out_path);
    }

    // 5. Snapshot: Zenith Cloud Office (Aero Browser Integration)
    {
        let mut surf = BufferSurface32::new(width, height);
        surf.clear(zenith_ui::geom::Color::rgb(15, 23, 42));
        draw_cloud_office_view(
            &mut surf,
            Rect::new(0, 0, width, height),
            "NovaOS Executive Cloud Document",
            false,
        );

        let png_bytes = png::encode_png_32(width as u32, height as u32, &surf.buffer);
        let out_path = format!("{}/view_zenith_browser_docs.png", target_dir);
        fs::write(&out_path, png_bytes).expect("Failed to write view_zenith_browser_docs.png");
        println!("  [+] Generated: {}", out_path);
    }

    println!("[OK] All 5 high-resolution PNG snapshots exported successfully.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_suite_and_png_export() {
        let suite = sample::create_sample_suite();
        assert_eq!(suite.writer_doc.paragraphs.len(), 6);
        assert_eq!(suite.calc_workbook.sheets[0].cells.len(), 20);
        assert_eq!(suite.present_deck.slides.len(), 3);
        assert_eq!(suite.base_db.tables.len(), 2);

        let mut surf = BufferSurface32::new(100, 100);
        surf.put_pixel(50, 50, zenith_ui::geom::Color::rgb(255, 0, 0));
        let png_data = png::encode_png_32(100, 100, &surf.buffer);
        assert!(png_data.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]));
    }
}
