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

#[test]
fn test_system_reactive_integration() {
    use lavaterm::reactive::{MockSystemProvider, SystemProvider, SystemSignals};

    let mut provider = MockSystemProvider::new(SystemSignals::new(0.85, 0.60, 0.95, 0.40));
    let mut sim = Simulation::new(PhysicsParams::default(), 6, 123);
    let palette = ColorPalette::default();
    let mut fb = VirtualFramebuffer::new(40, 20, palette.background);

    for _ in 0..10 {
        let signals = provider.poll_signals();
        sim.step_reactive(0.033, &signals);
        rasterize_simulation(&sim, &mut fb, &palette, 1.0);
    }

    assert!(sim.elapsed_time > 0.0);
    let active_pixels = fb
        .as_slice()
        .iter()
        .filter(|c| **c != palette.background)
        .count();
    assert!(
        active_pixels > 0,
        "Reactive simulation must rasterize active pixels"
    );
}

#[test]
fn test_audio_reactive_integration() {
    use lavaterm::audio::{AudioProvider, SyntheticAudioGenerator};

    let mut provider = SyntheticAudioGenerator::new(128.0);
    let mut sim = Simulation::new(PhysicsParams::default(), 6, 456);
    let palette = ColorPalette::default();
    let mut fb = VirtualFramebuffer::new(40, 20, palette.background);

    for _ in 0..15 {
        let audio_sig = provider.poll_signals();
        sim.step_audio(0.033, &audio_sig);
        rasterize_simulation(&sim, &mut fb, &palette, 1.0);
    }

    assert!(sim.elapsed_time > 0.0);
    let active_pixels = fb
        .as_slice()
        .iter()
        .filter(|c| **c != palette.background)
        .count();
    assert!(
        active_pixels > 0,
        "Audio reactive simulation must rasterize active pixels"
    );
}

#[test]
fn test_theme_engine_integration() {
    use lavaterm::theme::{list_presets, load_custom_theme_file, resolve_theme};
    use std::io::Write;

    // 1. Verify all presets resolve successfully and produce distinct palettes
    let presets = list_presets();
    assert!(presets.len() >= 8);
    for preset in &presets {
        let pal =
            resolve_theme(preset).unwrap_or_else(|_| panic!("Failed to resolve preset {preset}"));
        assert_ne!(pal.bottom, pal.background);
    }

    // 2. Test custom temporary JSON theme file
    let temp_dir = std::env::temp_dir();
    let json_theme_path = temp_dir.join("lavaterm_test_theme.json");
    let json_content = r##"{
        "bottom": "#00ffcc",
        "middle": "#0077ff",
        "top": "#ff0077",
        "background": "#000011"
    }"##;
    std::fs::File::create(&json_theme_path)
        .expect("create test theme json")
        .write_all(json_content.as_bytes())
        .expect("write test theme json");

    let pal_json = load_custom_theme_file(&json_theme_path).expect("load json theme");
    assert_eq!(
        pal_json.bottom,
        lavaterm::render::Rgb::new(0x00, 0xFF, 0xCC)
    );
    assert_eq!(pal_json.top, lavaterm::render::Rgb::new(0xFF, 0x00, 0x77));

    // Test rasterization with resolved theme
    let mut sim = Simulation::new(PhysicsParams::default(), 6, 999);
    let mut fb = VirtualFramebuffer::new(40, 20, pal_json.background);
    sim.step(0.033);
    rasterize_simulation(&sim, &mut fb, &pal_json, 1.0);
    assert_eq!(fb.get_pixel(0, 0), Some(pal_json.background));

    // Clean up
    let _ = std::fs::remove_file(json_theme_path);
}

#[test]
fn test_phase9_snapshot_mode_integration() {
    use lavaterm::theme::resolve_theme;
    use lavaterm::widget::render_snapshot;

    let palette = resolve_theme("cyberpunk").expect("Resolve theme");
    let mut sim = Simulation::new(PhysicsParams::default(), 6, 777);

    // Test 1-row snapshot for status line
    let snapshot_1row = render_snapshot(&mut sim, &palette, 20, 1, "braille", 1.0, 5)
        .expect("Snapshot 1-row succeeds");
    assert!(snapshot_1row.ends_with("\x1b[0m"));
    assert!(!snapshot_1row.contains('\n'));
    assert!(snapshot_1row.contains("\x1b[38;2;"));

    // Test multi-row snapshot for mini pane
    let snapshot_multirow = render_snapshot(&mut sim, &palette, 24, 6, "halfblock", 1.0, 5)
        .expect("Snapshot multi-row succeeds");
    assert!(snapshot_multirow.ends_with("\x1b[0m"));
    assert_eq!(snapshot_multirow.matches('\n').count(), 5);
}

#[test]
fn test_phase9_compact_mode_integration() {
    use lavaterm::widget::{should_compact, CompactScaler};

    let cols = 20;
    let rows = 8;
    assert!(should_compact(cols, rows, false));

    let base_blobs = 12;
    let base_physics = PhysicsParams::default();
    let profile = CompactScaler::calculate_profile(cols, rows, base_blobs);
    assert_eq!(profile.blob_count, 4);
    assert_eq!(profile.radius_scale, 0.65);

    let mut sim = Simulation::new(base_physics, profile.blob_count, 42);
    let initial_radius = sim.blobs[0].radius;
    CompactScaler::adapt_simulation(&profile, &mut sim);

    let palette = ColorPalette::default();
    let mut fb = VirtualFramebuffer::new(20, 16, palette.background);

    for _ in 0..10 {
        sim.step(0.033);
        rasterize_simulation(&sim, &mut fb, &palette, 1.0);
    }

    assert!(sim.elapsed_time > 0.0);
    assert_eq!(sim.blobs.len(), 4);
    assert_eq!(sim.radius_scale, 0.65);
    assert!((sim.blobs[0].radius - initial_radius * 0.65).abs() < 1e-5);
}

#[test]
fn test_phase9_policy_and_multiplexer_integration() {
    use lavaterm::widget::{
        detect_multiplexer_with, resolve_policy, ExecutionMode, MultiplexerKind, PolicyInput,
    };
    use std::collections::HashMap;

    // Test environment detection
    let mut tmux_env = HashMap::new();
    tmux_env.insert("TMUX", "1".to_string());
    assert_eq!(
        detect_multiplexer_with(|k| tmux_env.get(k).cloned()),
        MultiplexerKind::Tmux
    );

    // Test policy resolution: --widget implies compact & 15 FPS
    let input = PolicyInput {
        cli_widget: true,
        toml_render_fps: 60,
        toml_widget_fps: 15,
        ..Default::default()
    };
    let policy = resolve_policy(&input).expect("Resolve widget policy");
    assert_eq!(policy.mode, ExecutionMode::Widget);
    assert_eq!(policy.target_fps, 15);
    assert!(policy.force_compact);

    // Test CLI FPS override
    let input_fps_override = PolicyInput {
        cli_widget: true,
        cli_fps: Some(45),
        ..Default::default()
    };
    let policy_override = resolve_policy(&input_fps_override).expect("Resolve override");
    assert_eq!(policy_override.target_fps, 45);
}

#[test]
fn test_phase10_interactive_physics_integration() {
    use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
    use lavaterm::core::Interaction;
    use lavaterm::input::MouseTracker;

    let mut sim = Simulation::new(PhysicsParams::default(), 6, 888);
    let palette = ColorPalette::default();
    let mut fb = VirtualFramebuffer::new(40, 20, palette.background);

    // 1. Test mouse click event mapping and shockwave application
    let mut mouse_tracker = MouseTracker::new();
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 20,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::NONE,
    };
    let interaction = mouse_tracker
        .handle_event(click, 40, 20, 1.5, 1.0)
        .expect("Emits shockwave");
    assert!(matches!(interaction, Interaction::Shockwave { .. }));
    sim.apply_interaction(&interaction);

    // 2. Test mouse drag event mapping and stir velocity application
    let drag = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 25,
        row: 8,
        modifiers: crossterm::event::KeyModifiers::NONE,
    };
    let drag_interaction = mouse_tracker
        .handle_event(drag, 40, 20, 1.5, 1.2)
        .expect("Emits stir");
    assert!(matches!(drag_interaction, Interaction::Stir { .. }));
    sim.apply_interaction(&drag_interaction);

    // 3. Test mouse scroll pressure modulation
    let scroll_up = MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 20,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::NONE,
    };
    let pressure = mouse_tracker
        .handle_event(scroll_up, 40, 20, 1.0, 1.0)
        .expect("Emits pressure");
    assert_eq!(pressure, Interaction::Pressure { delta: 1.0 });
    let initial_buoyancy = sim.params.buoyancy;
    sim.apply_interaction(&pressure);
    assert!(sim.params.buoyancy > initial_buoyancy);

    // 4. Test keyboard ripple perturbation
    sim.apply_interaction(&Interaction::Ripple { intensity: 1.0 });

    // Step and rasterize
    for _ in 0..10 {
        sim.step(0.033);
        rasterize_simulation(&sim, &mut fb, &palette, 1.0);
    }

    assert!(sim.elapsed_time > 0.0);
    let active_pixels = fb
        .as_slice()
        .iter()
        .filter(|c| **c != palette.background)
        .count();
    assert!(
        active_pixels > 0,
        "Interactive simulation must rasterize active pixels"
    );
}
