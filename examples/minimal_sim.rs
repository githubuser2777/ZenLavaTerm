//! Minimal standalone example demonstrating decoupled simulation and ASCII field sampling.

use lavaterm::core::{PhysicsParams, Simulation};

fn main() {
    println!("=== LavaTerm Minimal Simulation Example ===");

    let mut sim = Simulation::new(PhysicsParams::default(), 4, 1234);

    println!("Initial blob states:");
    for (i, blob) in sim.blobs.iter().enumerate() {
        println!(
            "  Blob #{}: pos=({:.2}, {:.2}), radius={:.2}, temp={:.2}",
            i + 1,
            blob.x,
            blob.y,
            blob.radius,
            blob.temperature
        );
    }

    println!("\nStepping simulation forward for 30 ticks...");
    for _ in 0..30 {
        sim.step(0.033);
    }

    println!("Simulation time: {:.2}s", sim.elapsed_time);
    println!("\nSampled 20x10 ASCII Density Grid:");
    for y in (0..10).rev() {
        let py = y as f32 / 10.0;
        let mut row_str = String::new();
        for x in 0..20 {
            let px = x as f32 / 20.0;
            let field = sim.evaluate_field(px, py);
            let ch = if field > 2.0 {
                '@'
            } else if field > 1.0 {
                '#'
            } else if field > 0.5 {
                '*'
            } else if field > 0.2 {
                '.'
            } else {
                ' '
            };
            row_str.push(ch);
        }
        println!("  |{row_str}|");
    }
    println!("=== Example Finished ===");
}
