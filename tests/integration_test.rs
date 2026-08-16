//! Integration tests for LavaTerm.

use lavaterm::{
    config::Config,
    core::{PhysicsParams, Simulation},
    render::{
        rasterize_simulation, BlockRenderer, BrailleRenderer, ColorPalette, HalfBlockRenderer,
        Renderer, VirtualFramebuffer,
    },
};

#[test]
fn test_end_to_end_simulation_and_rasterization() {
    let config = Config::default();
    let physics = PhysicsParams {
        gravity: config.simulation.gravity,
        buoyancy: config.simulation.buoyancy,
        viscosity: config.simulation.viscosity,
        noise: 0.0,
        thermal_transfer_rate: 0.4,
    };

    let mut sim = Simulation::new(physics, 8, 42);
    let palette = ColorPalette::from(config.palette);
    let mut fb = VirtualFramebuffer::new(40, 20, palette.background);

    // Step simulation 10 times
    for _ in 0..10 {
        sim.step(0.033);
    }

    // Rasterize
    rasterize_simulation(&sim, &mut fb, &palette, config.simulation.threshold);

    // Test Half-Block rendering
    let mut hb_renderer = HalfBlockRenderer::new();
    let mut hb_out = Vec::new();
    hb_renderer
        .render(&fb, &mut hb_out)
        .expect("HalfBlock render succeeds");
    let hb_str = String::from_utf8_lossy(&hb_out);
    assert!(
        hb_str.contains("▀"),
        "HalfBlock output must contain upper half block characters"
    );
    assert!(
        hb_str.contains("\x1b[38;2;"),
        "Output must contain TrueColor foreground escape sequences"
    );

    // Test Block rendering
    let mut block_renderer = BlockRenderer::new();
    let mut block_out = Vec::new();
    block_renderer
        .render(&fb, &mut block_out)
        .expect("Block render succeeds");
    let block_str = String::from_utf8_lossy(&block_out);
    assert!(
        block_str.contains("█"),
        "Block output must contain full block characters"
    );

    // Test Braille rendering
    let mut braille_fb = VirtualFramebuffer::new(80, 40, palette.background);
    rasterize_simulation(&sim, &mut braille_fb, &palette, config.simulation.threshold);
    let mut braille_renderer = BrailleRenderer::new();
    let mut braille_out = Vec::new();
    braille_renderer
        .render(&braille_fb, &mut braille_out)
        .expect("Braille render succeeds");
    let braille_str = String::from_utf8_lossy(&braille_out);
    assert!(
        braille_str
            .chars()
            .any(|c| ('\u{2800}'..='\u{28FF}').contains(&c)),
        "Braille output must contain Unicode Braille patterns"
    );
}
