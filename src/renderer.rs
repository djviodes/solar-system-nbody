//! Renderer for the solar system simulation.
use crate::body::CelestialBody;
use macroquad::prelude::*;

/// Renders all celestial bodies to the screen using scaled simulation coordinates.
pub fn render_solar_system(bodies: &[CelestialBody], zoom: f32) {
    let scale: f32 = zoom / 4.5e12_f32;

    for body in bodies {
        let origin_x = (body.position.x as f32 * scale) + screen_width() / 2.0;
        let origin_y = (body.position.y as f32 * scale) + screen_height() / 2.0;
        let color = match body.name.as_str() {
            "Sun" => YELLOW,
            "Mercury" => GRAY,
            "Venus" => ORANGE,
            "Earth" => BLUE,
            "Mars" => RED,
            "Jupiter" => BROWN,
            "Saturn" => LIGHTGRAY,
            "Uranus" => Color::new(0.4, 0.8, 0.9, 1.0),
            "Neptune" => DARKBLUE,
            "Halley's Comet" => WHITE,
            _ => WHITE,
        };

        draw_circle(origin_x, origin_y, 5.0_f32, color);
    }
}