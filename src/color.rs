use sdl2::pixels::Color;
use colors_transform::{Rgb, Hsl, Color as ColorTransform};
use rand::Rng;

pub fn vary_color(color: Color, variance: i8) -> Color {
    let rgb = Rgb::from(color.r as f32, color.g as f32, color.b as f32);
    let hsl = rgb.to_hsl();

    let mut rng = rand::thread_rng();
    
    let hue = hsl.get_hue().floor();
    let saturation = hsl.get_saturation() + (rng.gen_range(-2 * variance..=0) as f32);
    let lightness = hsl.get_lightness() + (rng.gen_range(-variance..=variance) as f32);

    let new_hsl = Hsl::from(hue, saturation.min(100.).max(0.), lightness.min(100.).max(0.));
    let new_rgb = new_hsl.to_rgb();
    Color::RGB(new_rgb.get_red() as u8, new_rgb.get_green() as u8, new_rgb.get_blue() as u8)
}

pub fn darken_color(color: Color, lightness: f32) -> Color {
    let rgb = Rgb::from(color.r as f32, color.g as f32, color.b as f32);
    let hsl = rgb.to_hsl();

    let hue = hsl.get_hue().floor();
    let saturation = hsl.get_saturation();
    let lightness = lightness.min(hsl.get_lightness());

    let new_hsl = Hsl::from(hue, saturation.min(100.).max(0.), lightness.min(100.).max(0.));
    let new_rgb = new_hsl.to_rgb();
    Color::RGB(new_rgb.get_red() as u8, new_rgb.get_green() as u8, new_rgb.get_blue() as u8)
}

#[allow(dead_code)]
pub fn color_interpolation(a: Color, b: Color, t: f64) -> Color {
    Color {
        r: (a.r as f64 + (b.r as f64 - a.r as f64) * t).floor() as u8,
        g: (a.g as f64 + (b.g as f64 - a.g as f64) * t).floor() as u8,
        b: (a.b as f64 + (b.b as f64 - a.b as f64) * t).floor() as u8,
        a: (a.a as f64 + (b.a as f64 - a.a as f64) * t).floor() as u8,
    }
}