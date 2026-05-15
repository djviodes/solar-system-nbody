//! A simple solar system simulation in Rust.
mod vec3;
mod body;
mod physics;
mod simulation;
mod renderer;

use crate::vec3::Vec3;
use crate::body::CelestialBody;
use crate::simulation::simulate_step;
use crate::renderer::render_solar_system;
use crate::physics::compute_total_system_energy;
use macroquad::prelude::*;

fn initialize_bodies() -> Vec<CelestialBody> {
    vec![
        CelestialBody::new(
            "Sun".to_string(),
            1.98841e30_f64,
            6.957e8_f64,
            Vec3::new(-4.588639674035421e8_f64, -8.277742469901335e8_f64, 1.969967502418195e7_f64),
            Vec3::new(1.242505794360611e1_f64, 3.078762484369151e-1_f64, -2.352803314883807e-1_f64)
        ),
        CelestialBody::new(
            "Mercury".to_string(),
            3.302e23_f64,
            2.4394e6_f64,
            Vec3::new(-3.265252087416521e10_f64, -6.204436222997473e10_f64, -2.030280277555656e9_f64),
            Vec3::new(3.331154717745245e4_f64, -2.032288538992848e4_f64, -4.715262224650710e3_f64)
        ),
        CelestialBody::new(
            "Venus".to_string(),
            4.8685e24_f64,
            6.05184e6_f64,
            Vec3::new(1.283698332887579e10_f64, -1.088018897929847e11_f64, -2.230859155295417e9_f64),
            Vec3::new(3.453518141145980e4_f64, 4.156437812065552e3_f64, -1.935089462258685e3_f64)
        ),
        CelestialBody::new(
            "Earth".to_string(),
            5.97219e24_f64,
            6.37101e6_f64,
            Vec3::new(-2.653100241556548e10_f64, 1.439468995740296e11_f64, 1.080681311843544e7_f64),
            Vec3::new(-2.977650610770464e4_f64, -5.395962660572101e3_f64, 1.753836198843395e-1_f64)
        ),
        CelestialBody::new(
            "Mars".to_string(),
            6.4171e23_f64,
            3.38992e6_f64,
            Vec3::new(5.049113049487789e10_f64, -2.083203224890075e11_f64, -5.577837779914886e9_f64),
            Vec3::new(2.445957741742463e4_f64, 7.861473739133970e3_f64, -4.349787415986586e2_f64)
        ),
        CelestialBody::new(
            "Jupiter".to_string(),
            1.89819e27_f64,
            6.9911e7_f64,
            Vec3::new(-2.538782093363155e11_f64, 7.365225315477104e11_f64, 2.626625175033838e9_f64),
            Vec3::new(-1.250761633714600e4_f64, -3.639986644887777e3_f64, 2.949114021371024e2_f64)
        ),
        CelestialBody::new(
            "Saturn".to_string(),
            5.6834e26_f64,
            5.8232e7_f64,
            Vec3::new(1.421819203232436e12_f64, 3.772943733460353e10_f64, -5.726666294257715e10_f64),
            Vec3::new(-7.898651748040556e2_f64, 9.634221220286250e3_f64, -1.355334934147638e2_f64)
        ),
        CelestialBody::new(
            "Uranus".to_string(),
            8.6813e25_f64,
            2.5362e7_f64,
            Vec3::new(1.477614612946379e12_f64, 2.512418267148477e12_f64, -9.811818416460633e9_f64),
            Vec3::new(-5.920365479337566e3_f64, 3.134957028375065e3_f64, 8.857240257287802e1_f64)
        ),
        CelestialBody::new(
            "Neptune".to_string(),
            1.02409e26_f64,
            2.4624e7_f64,
            Vec3::new(4.468346746922093e12_f64, 7.680447071835697e10_f64, -1.045593050619605e11_f64),
            Vec3::new(-1.294614448856751e2_f64, 5.465955556668026e3_f64, -1.100604780101149e2_f64)
        ),
        CelestialBody::new(
            "Halley's Comet".to_string(),
            2.2e14_f64,
            5.5e3_f64,
            Vec3::new(-2.917648124800134e12_f64, 4.102401608732985e12_f64, -1.479112376415818e12_f64),
            Vec3::new(8.956616770535510e2_f64, 3.008332887246409e2_f64, 1.955417717582362e2_f64)
        ),
    ]
}

#[macroquad::main("Solar System")]
async fn main() {
    let mut bodies = initialize_bodies();
    let mut dt = 8.64e4_f64;
    let mut speed_label = "1x";
    let mut zoom: f32 = screen_width();
    let mut body_selected: Option<String> = None;
    let mut paused: bool = false;

    let initial_energy = compute_total_system_energy(&bodies);

    // Main simulation loop
    loop {
        if is_key_pressed(KeyCode::Key1) {
            dt = 8.64e4_f64;
            speed_label = "1x";
        } else if is_key_pressed(KeyCode::Key2) {
            dt = 8.64e4_f64 * 2.0;
            speed_label = "2x";
        } else if is_key_pressed(KeyCode::Key3) {
            dt = 8.64e4_f64 * 4.0;
            speed_label = "4x";
        } else if is_key_pressed(KeyCode::Key4) {
            dt = 8.64e4_f64 * 8.0;
            speed_label = "8x";
        } else if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        };

        if !paused {
            simulate_step(&mut bodies, dt);
        };

        clear_background(BLACK);
        
        let (_, scroll) = mouse_wheel();
        zoom *= 1.0 + scroll * 0.1;

        let current_energy = compute_total_system_energy(&bodies);
        let energy_drift_percentage = (current_energy - initial_energy) / initial_energy;

        render_solar_system(
            &bodies,
            zoom,
            speed_label,
            &mut body_selected,
            paused,
            current_energy,
            energy_drift_percentage
        );

        next_frame().await;
    }
}
