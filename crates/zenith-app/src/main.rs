//! Zenith Office Suite CLI & Cross-Platform Desktop GUI Application.

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

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("Zenith Office Suite v1.0.0 (Pure Rust 1.99.1)");
        println!("Synthesizes Microsoft Office (Word, Excel, PowerPoint, Access) & LibreOffice");
        println!("Cross-Platform Desktop: macOS • Linux • Windows • NovaOS Default Office Suite");
        return;
    }

    if args.iter().any(|a| a == "--info" || a == "-i") {
        print_info(&view);
        return;
    }

    if let Some(pos) = args.iter().position(|a| a == "--export-view-png" || a == "-e") {
        let out_dir = args.get(pos + 1).cloned().unwrap_or_else(|| {
            "/home/firoze/.gemini/antigravity/brain/7192285d-9f72-46e9-869e-0c53ffc319de".to_string()
        });
        export_all_snapshots(&out_dir);
        return;
    }

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--writer" | "-w" => {
                view.set_module(AppModule::Writer);
            }
            "--calc" | "-c" => {
                view.set_module(AppModule::Calc);
            }
            "--present" | "-p" => {
                view.set_module(AppModule::Present);
            }
            "--base" | "-b" => {
                view.set_module(AppModule::Base);
            }
            file if !file.starts_with('-') => {
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
            _ => {}
        }
    }

    // Launch Native Cross-Platform Desktop GUI (macOS, Linux, Windows)
    run_desktop_gui(view);
}

fn run_desktop_gui(mut view: ZenithView) {
    let mut sw = 1024;
    let mut sh = 720;

    let mut window = match minifb::Window::new(
        "Zenith Office Suite - Writer, Calc, Present & Base",
        sw,
        sh,
        minifb::WindowOptions {
            resize: true,
            ..minifb::WindowOptions::default()
        },
    ) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("[Zenith] Note: Native window could not be opened ({}). Rendering offscreen...", e);
            let mut surf = BufferSurface32::new(sw as i32, sh as i32);
            view.draw(&mut surf, Rect::new(0, 0, sw as i32, sh as i32));
            let non_zero = surf.buffer.iter().filter(|&&p| p != 0).count();
            println!("[Zenith] Headless offscreen render complete: {} non-zero pixels.", non_zero);
            return;
        }
    };

    window.set_target_fps(60);
    println!("[Zenith] Native cross-platform desktop window created ({}x{}).", sw, sh);

    let mut surf = BufferSurface32::new(sw as i32, sh as i32);
    let mut mouse_down = false;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) && !window.is_key_down(minifb::Key::Q) {
        // 1. Dynamic Window Resizing
        let (cur_w, cur_h) = window.get_size();
        if (cur_w != sw || cur_h != sh) && cur_w >= 320 && cur_h >= 240 {
            sw = cur_w;
            sh = cur_h;
            surf = BufferSurface32::new(sw as i32, sh as i32);
        }

        // 2. Mouse Input Handling
        if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            let px = mx as i32;
            let py = my as i32;
            let is_down = window.get_mouse_down(minifb::MouseButton::Left);

            if is_down && !mouse_down {
                mouse_down = true;
                view.handle_click(px, py, Rect::new(0, 0, sw as i32, sh as i32));
            } else if !is_down && mouse_down {
                mouse_down = false;
            }
        }

        // 3. Keyboard Shortcuts
        for key in window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            match key {
                minifb::Key::W => view.set_module(AppModule::Writer),
                minifb::Key::X | minifb::Key::C => view.set_module(AppModule::Calc),
                minifb::Key::P => view.set_module(AppModule::Present),
                minifb::Key::B => view.set_module(AppModule::Base),
                minifb::Key::A => view.toggle_about(),
                minifb::Key::F => {
                    let next = match view.ribbon.font_family {
                        zenith_core::fonts::FontFamily::Calibri => zenith_core::fonts::FontFamily::TimesNewRoman,
                        zenith_core::fonts::FontFamily::TimesNewRoman => zenith_core::fonts::FontFamily::Arial,
                        zenith_core::fonts::FontFamily::Arial => zenith_core::fonts::FontFamily::SegoeUI,
                        zenith_core::fonts::FontFamily::SegoeUI => zenith_core::fonts::FontFamily::Cambria,
                        zenith_core::fonts::FontFamily::Cambria => zenith_core::fonts::FontFamily::Consolas,
                        zenith_core::fonts::FontFamily::Consolas => zenith_core::fonts::FontFamily::Georgia,
                        zenith_core::fonts::FontFamily::Georgia => zenith_core::fonts::FontFamily::LiberationSans,
                        zenith_core::fonts::FontFamily::LiberationSans => zenith_core::fonts::FontFamily::LiberationSerif,
                        zenith_core::fonts::FontFamily::LiberationSerif => zenith_core::fonts::FontFamily::LiberationMono,
                        zenith_core::fonts::FontFamily::LiberationMono => zenith_core::fonts::FontFamily::Ubuntu,
                        zenith_core::fonts::FontFamily::Ubuntu => zenith_core::fonts::FontFamily::Calibri,
                    };
                    view.set_font(next);
                }
                _ => {}
            }
        }

        // 4. Render & Present
        view.draw(&mut surf, Rect::new(0, 0, sw as i32, sh as i32));
        if let Err(e) = window.update_with_buffer(&surf.buffer, sw, sh) {
            eprintln!("[Zenith] Window update error: {}", e);
            break;
        }
    }

    println!("[Zenith] Session ended cleanly.");
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
        let _ = fs::write(&out_path, png_bytes);
        println!("[+] Generated {}", out_path);
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
        let _ = fs::write(&out_path, png_bytes);
        println!("[+] Generated {}", out_path);
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
        let _ = fs::write(&out_path, png_bytes);
        println!("[+] Generated {}", out_path);
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
        let _ = fs::write(&out_path, png_bytes);
        println!("[+] Generated {}", out_path);
    }

    // 5. Snapshot: Zenith Web Browser Cloud Office Suite
    {
        let mut surf = BufferSurface32::new(width, height);
        draw_cloud_office_view(&mut surf, Rect::new(0, 0, width, height), "Writer Document (Cloud Sync)", false);

        let png_bytes = png::encode_png_32(width as u32, height as u32, &surf.buffer);
        let out_path = format!("{}/view_zenith_browser_docs.png", target_dir);
        let _ = fs::write(&out_path, png_bytes);
        println!("[+] Generated {}", out_path);
    }

    println!("[*] Zenith Office Suite snapshots export complete.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_suite_and_png_export() {
        let view = ZenithView::new();
        assert_eq!(view.suite.active_module, AppModule::Writer);

        let mut surf = BufferSurface32::new(800, 600);
        view.draw(&mut surf, Rect::new(0, 0, 800, 600));

        let non_zero = surf.buffer.iter().filter(|&&p| p != 0).count();
        assert!(non_zero > 1000);
    }
}
