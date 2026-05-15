//! Renderer for the solar system simulation.
use crate::body::CelestialBody;
use macroquad::prelude::*;

/// Renders all celestial bodies to the screen using scaled simulation coordinates.
pub fn render_solar_system(bodies: &[CelestialBody], zoom: f32, speed_label: &str) {
    let scale: f32 = zoom / 4.5e12_f32;

    let active_speed = format!(
        "Active: {}",
        speed_label
    );
    draw_text(
        "[1] 1x [2] 2x [3] 4x [4] 8x | Scroll to zoom in/out",
        12.0,
        25.0,
        20.0,
        WHITE
    );
    draw_text(&active_speed, 12.0, 50.0, 20.0, WHITE);

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
        let radius = match body.name.as_str() {
            "Sun" => 8.0,
            "Mercury" => 3.0,
            "Venus" => 4.0,
            "Earth" => 4.0,
            "Mars" => 3.5,
            "Jupiter" => 6.0,
            "Saturn" => 5.5,
            "Uranus" => 5.0,
            "Neptune" => 5.0,
            "Halley's Comet" => 2.0,
            _ => 3.0,
        };

        draw_text(&body.name, origin_x, origin_y - 10.0, 10.0, WHITE);
        draw_circle(origin_x, origin_y, radius, color);
    }
}