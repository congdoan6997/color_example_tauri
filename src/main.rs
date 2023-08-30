// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

use enterpolation::{linear::ConstEquidistantLinear, Curve};
use palette::{FromColor, Lch, LinSrgb, Srgb};

#[tauri::command]
fn generate_gradient(r: u8, g: u8, b: u8) -> Vec<Vec<u8>> {
    println!("{} {} {}", r, g, b);
    let my_rgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    // let lin_rbg = my_rgb.into_linear();
    let lch_rgb = Lch::from_color(my_rgb);
    // dbg!(lin_rbg);
    // let lin_rgb1 = LinSrgb::from_color(Lch::new(1.0, lch_rgb.chroma, lch_rgb.hue));

    let gradient = ConstEquidistantLinear::<f32, _, 3>::equidistant_unchecked([
        // LinSrgb::new(0.00, 0.05, 0.20),
        // LinSrgb::new(0.70, 0.10, 0.20),
        // LinSrgb::new(0.95, 0.90, 0.30),
        LinSrgb::from_color(Lch::new(1.0, lch_rgb.chroma, lch_rgb.hue)),
        my_rgb.into_linear(),
        LinSrgb::from_color(Lch::new(128.0, lch_rgb.chroma, lch_rgb.hue)),
    ]);

    let taken_colors: Vec<_> = gradient
        .take(10)
        .map(|color| {
            let (r, g, b) = Srgb::from_color(color).into_components();
            vec![(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
        })
        .collect();
    taken_colors
    // dbg!(taken_colors);
    // dbg!(my_rgb);
}

fn main() {
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![generate_gradient])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
