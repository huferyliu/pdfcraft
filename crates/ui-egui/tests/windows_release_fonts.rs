//! Opt-in Windows regression using a release-pinned craft-fonts input without a Hans face.

#![cfg(windows)]

use std::collections::BTreeSet;

use egui::epaint::text::{Fonts, TextOptions};
use egui::{Color32, FontId};
use pdfcraft_ui_egui::theme;

#[test]
fn release_input_without_hans_keeps_chinese_and_other_scripts() {
    if std::env::var_os("PDFCRAFT_TEST_REQUIRE_CJK").is_none_or(|value| value != "1") {
        eprintln!("skipping release-input regression: set PDFCRAFT_TEST_REQUIRE_CJK=1 and CRAFT_FONTS_DIR to the release pin");
        return;
    }
    assert!(!pdfcraft_fonts::ui_japanese_fonts().is_empty(), "the release-pinned Japanese fonts must be embedded");
    assert!(pdfcraft_fonts::ui_chinese_fonts().is_empty(), "this regression requires no embedded Hans face");
    let ids = [FontId::proportional(13.0), FontId::monospace(13.0), theme::medium(13.0), theme::semibold(17.0)];
    let chinese = "工程估价_欢迎编辑导出备份_报告.pdf";
    let mut chars = BTreeSet::new();
    for line in include_str!("../src/i18n/zh-hans.tsv").lines().filter(|line| !line.starts_with('#')) {
        if let Some(translation) = line.split('\t').nth(2) {
            chars.extend(translation.chars().filter(|c| !c.is_ascii() && !c.is_whitespace()));
        }
    }
    let catalog: String = chars.iter().collect();
    let mut embedded = Fonts::new(TextOptions::default(), theme::font_definitions_for(true));
    assert!(!embedded.has_glyphs(&ids[0], chinese), "the fixture must expose the original missing-glyph condition");
    let disabled = std::env::var_os("PDFCRAFT_SYSTEM_FONTS").is_some_and(|value| value == "0");
    for prefer_hans in [true, false, true] {
        let defs = theme::installed_font_definitions(prefer_hans);
        if disabled {
            let embedded = theme::font_definitions_for(prefer_hans);
            assert_eq!(defs.font_data.keys().collect::<Vec<_>>(), embedded.font_data.keys().collect::<Vec<_>>());
            assert_eq!(defs.families, embedded.families);
        }
        let mut fonts = Fonts::new(TextOptions::default(), defs);
        for id in &ids {
            assert!(fonts.has_glyphs(id, "PdfCraft 日本語"), "embedded Latin and Japanese must remain available: {id:?}");
            if disabled {
                assert!(!fonts.has_glyphs(id, chinese), "publication mode must keep installed fonts disabled: {id:?}");
            } else {
                for text in [chinese, "واحد اثنين אבג", catalog.as_str()] {
                    let missing: String = text.chars().filter(|c| !fonts.has_glyphs(id, &c.to_string())).collect();
                    assert!(missing.is_empty(), "prefer_hans={prefer_hans}, {id:?} lacks {missing:?}");
                    let galley = fonts.with_pixels_per_point(2.0).layout_no_wrap(text.to_owned(), id.clone(), Color32::BLACK);
                    assert!(galley.size().x.is_finite() && galley.size().x > 0.0);
                }
                if prefer_hans {
                    let galley = fonts.with_pixels_per_point(2.0).layout_no_wrap("中文欢迎编辑导出备份".to_owned(), id.clone(), Color32::BLACK);
                    let glyphs: Vec<_> = galley.rows.iter().flat_map(|row| &row.glyphs).collect();
                    let first = glyphs.first().expect("Chinese text must have glyphs");
                    assert!(glyphs.iter().all(|glyph| (glyph.font_face_ascent - first.font_face_ascent).abs() < 0.01));
                    assert!(glyphs.iter().all(|glyph| (glyph.font_face_height - first.font_face_height).abs() < 0.01));
                }
            }
        }
    }
    println!("release input: {} catalog glyphs; four families; repeated font-order switch; system fonts disabled={disabled}", chars.len());
}
