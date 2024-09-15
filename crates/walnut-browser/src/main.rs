mod canvas;
use canvas::Canvas;
use skia_safe::{
    typeface, Color, Font, FontMgr, FontStyle, Paint, PaintStyle as Style, Point, TextBlob,
    Typeface,
};

use core::str;
use std::fs::File;
use std::io::Write;

use skia_safe::utils::text_utils::Align;

pub const H1_FONT_SIZE: f32 = 56.0;
pub const NORMAL_FONT_SIZE: f32 = 36.0;

fn main() {
    // Get + parse HTML
    let html = /* reqwest::blocking::get("http://info.cern.ch/hypertext/WWW/TheProject.html")
        .unwrap()
        .text()
        .unwrap(); */ include_str!("../../walnut-html/cern.html");
    let dom = walnut_html::parse(&html);

    // Setup canvas and paint
    let mut canvas = Canvas::new(2560, 1280);
    let skia_canvas = canvas.surface.canvas();
    let paint = canvas.paint.clone();

    // Setup fonts
    let font_mgr = FontMgr::new();
    let typeface = font_mgr
        .match_family_style("Arial", FontStyle::normal())
        .unwrap_or_else(|| {
            font_mgr
                .legacy_make_typeface(None, FontStyle::normal())
                .unwrap()
        });
    let h1_font = Font::new(typeface.clone(), H1_FONT_SIZE);
    let normal_font = Font::new(typeface, NORMAL_FONT_SIZE);

    // Paint nodes
    let mut y = 50.0;
    for node in dom.nodes() {
        let node_val = node.value();
        let tag_name = node_val.tag.as_str();
        match tag_name {
            "H1" => {
                let text = &node_val.text;
                skia_canvas.draw_str(text, (50.0, y), &h1_font, &paint);
                y += 50.0;
            }
            "P" | "A" => {
                let text = &node_val.text;
                if text == "The World Wide Web project" {
                    dbg!(&text, &tag_name);
                }
                skia_canvas.draw_str(text, (50.0, y), &normal_font, &paint);
                y += 50.0;
            }
            _ => {}
        }
    }

    let d = canvas.data();
    let mut file = File::create("test.png").unwrap();
    let bytes = d.as_bytes();
    file.write_all(bytes).unwrap();
}
