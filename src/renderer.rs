//! Renderer for the solar system simulation.
use crate::vec3::Vec3;
use crate::body::CelestialBody;
use crate::physics::G;
use macroquad::prelude::*;

/// Draws the info panel for the selected celestial body.
fn draw_body_panel(
    body: &CelestialBody, 
    bodies: &[CelestialBody],
    sun_position: Option<Vec3>,
) {
    draw_text(
        &format!("Selected: {}", body.name),
        screen_width() - 300.0,
        25.0,
        20.0,
        WHITE
    );

    let current_velocity_magnitude = format!(
        "Velocity: {:.2e} m/s",
        body.velocity.compute_magnitude()
    );
    draw_text(
        &current_velocity_magnitude,
        screen_width() - 300.0,
        50.0,
        20.0,
        WHITE
    );

    let distance_from_sun = format!(
        "Distance from Sun: {:.2e} m",
        body.position.subtract(sun_position.unwrap_or(Vec3::new(0.0, 0.0, 0.0))).compute_magnitude()
    );
    draw_text(
        &distance_from_sun,
        screen_width() - 300.0,
        75.0,
        20.0,
        WHITE
    );

    let kinetic_energy = format!(
        "Kinetic Energy: {:.2e} J",
        0.5 * body.mass * body.velocity.compute_magnitude().powi(2)
    );
    draw_text(
        &kinetic_energy,
        screen_width() - 300.0,
        100.0,
        20.0,
        WHITE
    );

    let potential_energy = if let Some(sun_pos) = sun_position {
        format!(
            "Potential Energy: {:.2e} J",
            -G * body.mass * bodies.iter().find(|b| b.name == "Sun").unwrap().mass
            / body.position.subtract(sun_pos).compute_magnitude()
        )
    } else {
        "Potential Energy: N/A".to_string()
    };
    draw_text(
        &potential_energy,
        screen_width() - 300.0,
        125.0,
        20.0,
        WHITE
    );

    let orbital_period = match body.name.as_str() {
        "Mercury" => "87.97 days",
        "Venus" => "224.70 days",
        "Earth" => "365.25 days",
        "Mars" => "686.97 days",
        "Jupiter" => "11.86 years",
        "Saturn" => "29.46 years",
        "Uranus" => "84.02 years",
        "Neptune" => "164.77 years",
        "Halley's Comet" => "75.91 years",
        _ => "N/A",
    };
    draw_text(
        &format!("Orbital Period: {}", orbital_period),
        screen_width() - 300.0,
        150.0,
        20.0,
        WHITE
    );

    let eccentricity = match body.name.as_str() {
        "Mercury" => "0.2056",
        "Venus" => "0.0068",
        "Earth" => "0.0167",
        "Mars" => "0.0934",
        "Jupiter" => "0.0489",
        "Saturn" => "0.0565",
        "Uranus" => "0.0463",
        "Neptune" => "0.0097",
        "Halley's Comet" => "0.9671",
        _ => "N/A",
    };
    draw_text(
        &format!("Eccentricity: {}", eccentricity),
        screen_width() - 300.0,
        175.0,
        20.0,
        WHITE
    );
}

/// Renders all celestial bodies to the screen using scaled simulation coordinates.
pub fn render_solar_system(
    bodies: &[CelestialBody],
    zoom: f32,
    speed_label: &str,
    body_selected: &mut Option<String>,
    paused: bool,
    current_energy: f64,
    energy_drift_percentage: f64
) {
    let scale: f32 = zoom / 4.5e12_f32;

    let active_speed = format!("Active: {speed_label}");
    let paused_label = if paused { "PAUSED" } else { "" };
    draw_text(
        "[1] 1x [2] 2x [3] 4x [4] 8x | Scroll to zoom in/out",
        12.0,
        25.0,
        20.0,
        WHITE
    );
    draw_text(&active_speed, 12.0, 50.0, 20.0, WHITE);
    draw_text(&paused_label, 12.0, 75.0, 20.0, WHITE);

    let (mx, my) = mouse_position();

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

        if is_mouse_button_pressed(MouseButton::Left) {
            let dx = mx - origin_x;
            let dy = my - origin_y;
            if (dx.powi(2) + dy.powi(2)).sqrt() < radius {
                *body_selected = Some(body.name.clone());
            }
        }

        draw_text(&body.name, origin_x, origin_y - 10.0, 10.0, WHITE);
        draw_circle(origin_x, origin_y, radius, color);
    }

    let sun_position = bodies.iter()
        .find(|b| b.name == "Sun")
        .map(|b| b.position);

    if let Some(name) = body_selected.as_deref() {
        draw_body_panel(
            &bodies.iter().find(|b| b.name == name).unwrap(),
            bodies,
            sun_position,
        );
    }

    let total_energy_label = format!(
        "Total Energy: {:.2e} J",
        current_energy
    );
    draw_text(
        &total_energy_label,
        12.0,
        screen_height() - 50.0,
        20.0,
        WHITE
    );

    let energy_drift_label = format!(
        "Energy Drift: {:.2e}%",
        energy_drift_percentage
    );
    draw_text(
        &energy_drift_label,
        12.0,
        screen_height() - 25.0,
        20.0,
        WHITE
    );
}