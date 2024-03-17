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