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
    let json_theme_path = temp_dir.join(format!("lavaterm_test_theme_{}.json", std::process::id()));
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

#[test]
fn test_phase10_stress_and_rapid_interaction() {
    use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
    use lavaterm::input::MouseTracker;
    use lavaterm::widget::{CompactProfile, CompactScaler};

    let mut sim = Simulation::new(PhysicsParams::default(), 8, 12345);
    let mut mouse_tracker = MouseTracker::new();

    // 1. Rapid shockwaves across random spots
    for i in 0..100 {
        let col = (i * 7) % 80;
        let row = (i * 13) % 24;
        let click = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: col as u16,
            row: row as u16,
            modifiers: crossterm::event::KeyModifiers::NONE,
        };
        if let Some(inter) = mouse_tracker.handle_event(click, 80, 24, 5.0, 5.0) {
            sim.apply_interaction(&inter);
        }
        sim.step(0.016);
    }

    // 2. Rapid continuous drag sequences
    for i in 0..50 {
        let col = ((i * 3) % 80) as u16;
        let row = ((i * 5) % 24) as u16;
        let drag = MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Left),
            column: col,
            row,
            modifiers: crossterm::event::KeyModifiers::NONE,
        };
        if let Some(inter) = mouse_tracker.handle_event(drag, 80, 24, 5.0, 5.0) {
            sim.apply_interaction(&inter);
        }
        sim.step(0.016);
    }

    // 3. Compact mode adaptation
    let compact_profile = CompactProfile {
        blob_count: 3,
        radius_scale: 0.5,
        buoyancy_scale: 1.2,
        noise_scale: 0.8,
    };
    CompactScaler::adapt_simulation(&compact_profile, &mut sim);

    // Verify all blobs remain finite and within physical bounds
    for blob in &sim.blobs {
        assert!(blob.x.is_finite() && blob.x >= 0.0 && blob.x <= 1.0);
        assert!(blob.y.is_finite() && blob.y >= 0.0 && blob.y <= 1.0);
        assert!(blob.vx.is_finite() && blob.vx >= -2.0 && blob.vx <= 2.0);
        assert!(blob.vy.is_finite() && blob.vy >= -2.0 && blob.vy <= 2.0);
        assert!(blob.temperature.is_finite() && blob.temperature >= 0.0 && blob.temperature <= 1.0);
        assert!(blob.radius.is_finite() && blob.radius > 0.0);
    }
}

#[test]
fn test_phase11_cross_platform_system_provider_contract() {
    use lavaterm::reactive::default_system_provider;

    let mut provider = default_system_provider();
    let signals1 = provider.poll_signals();

    assert!(
        signals1.cpu_load.is_finite() && signals1.cpu_load >= 0.0 && signals1.cpu_load <= 1.0,
        "CPU load must be normalized within [0.0, 1.0]"
    );
    assert!(
        signals1.memory_usage.is_finite()
            && signals1.memory_usage >= 0.0
            && signals1.memory_usage <= 1.0,
        "Memory usage must be normalized within [0.0, 1.0]"
    );
    assert!(
        signals1.battery_level.is_finite()
            && signals1.battery_level >= 0.0
            && signals1.battery_level <= 1.0,
        "Battery level must be normalized within [0.0, 1.0]"
    );
    assert!(
        signals1.io_activity.is_finite()
            && signals1.io_activity >= 0.0
            && signals1.io_activity <= 1.0,
        "IO activity must be normalized within [0.0, 1.0]"
    );

    // Second poll to exercise delta tick calculations (CPU & I/O)
    let signals2 = provider.poll_signals();
    assert!(
        signals2.cpu_load.is_finite() && signals2.cpu_load >= 0.0 && signals2.cpu_load <= 1.0,
        "Second poll CPU load must remain normalized within [0.0, 1.0]"
    );
    assert!(
        signals2.io_activity.is_finite()
            && signals2.io_activity >= 0.0
            && signals2.io_activity <= 1.0,
        "Second poll IO activity must remain normalized within [0.0, 1.0]"
    );
}

#[test]
fn test_phase11_cross_platform_theme_paths_discovery() {
    use lavaterm::theme::pywal::default_pywal_paths;
    use lavaterm::theme::wallust::default_wallust_paths;

    let pywal_candidates = default_pywal_paths();
    let wallust_candidates = default_wallust_paths();

    // Verify candidate lists contain valid non-empty path buffers
    for p in pywal_candidates {
        assert!(!p.as_os_str().is_empty());
    }
    for p in wallust_candidates {
        assert!(!p.as_os_str().is_empty());
    }
}

#[test]
fn test_phase11_cross_platform_audio_provider_contract() {
    use lavaterm::audio::default_audio_provider;

    let mut provider = default_audio_provider();
    let signals = provider.poll_signals();

    assert!(
        signals.bass.is_finite() && signals.bass >= 0.0 && signals.bass <= 1.0,
        "Bass signal must be normalized within [0.0, 1.0]"
    );
    assert!(
        signals.mid.is_finite() && signals.mid >= 0.0 && signals.mid <= 1.0,
        "Mid signal must be normalized within [0.0, 1.0]"
    );
    assert!(
        signals.treble.is_finite() && signals.treble >= 0.0 && signals.treble <= 1.0,
        "Treble signal must be normalized within [0.0, 1.0]"
    );
}

#[test]
fn test_phase11_cross_platform_headless_execution() {
    use lavaterm::config::Config;
    use lavaterm::core::{PhysicsParams, Simulation};
    use lavaterm::reactive::default_system_provider;
    use lavaterm::render::{rasterize_simulation, ColorPalette, VirtualFramebuffer};

    let config = Config::default();
    let physics = PhysicsParams::default();
    let mut sim = Simulation::new(physics, config.simulation.blobs, 42);
    let palette = ColorPalette::from(config.palette);
    let mut fb = VirtualFramebuffer::new(60, 30, palette.background);
    let mut provider = default_system_provider();

    let dt = 1.0 / 30.0;
    for _ in 0..30 {
        let signals = provider.poll_signals();
        sim.step_reactive(dt, &signals);
        rasterize_simulation(&sim, &mut fb, &palette, config.simulation.threshold);
    }

    assert!(sim.elapsed_time > 0.9);
    for blob in &sim.blobs {
        assert!(blob.x.is_finite());
        assert!(blob.y.is_finite());
    }
}

#[test]
fn test_phase11_signal_shutdown_lifecycle_transition() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&shutdown_flag);

    let mut loop_iterations = 0;
    // Simulate event loop checking shutdown_flag
    while !flag_clone.load(Ordering::SeqCst) {
        loop_iterations += 1;
        if loop_iterations == 5 {
            // Signal arrives
            shutdown_flag.store(true, Ordering::SeqCst);
        }
        if loop_iterations > 100 {
            panic!("Shutdown flag failed to terminate loop");
        }
    }

    assert_eq!(loop_iterations, 5);
    assert!(shutdown_flag.load(Ordering::SeqCst));
}

#[test]
fn test_phase11_linux_provider_delta_transition() {
    use lavaterm::reactive::linux::LinuxSystemProvider;
    use lavaterm::reactive::provider::SystemProvider;
    use std::fs;

    let temp_dir = std::env::temp_dir();
    let test_prefix = format!("lavaterm_delta_test_{}", std::process::id());
    let stat_file = temp_dir.join(format!("{}_stat", test_prefix));
    let mem_file = temp_dir.join(format!("{}_mem", test_prefix));
    let bat_dir = temp_dir.join(format!("{}_bat", test_prefix));
    let disk_file = temp_dir.join(format!("{}_disk", test_prefix));

    let _ = fs::create_dir_all(&bat_dir);
    let _ = fs::write(&stat_file, "cpu  1000 200 300 8000 100 0 0 0 0 0\n");
    let _ = fs::write(
        &mem_file,
        "MemTotal:       16000000 kB\nMemAvailable:    8000000 kB\n",
    );
    let _ = fs::write(&disk_file, "   8       0 sda 100 0 2000 50 0 0 0 0 0 0 0\n");

    let mut provider =
        LinuxSystemProvider::new_with_paths(&stat_file, &mem_file, &bat_dir, &disk_file);

    // Initial baseline poll
    let s1 = provider.poll_signals();
    assert_eq!(s1.cpu_load, 0.15); // Fallback on first sample before delta
    assert_eq!(s1.memory_usage, 0.50);

    // Second poll with higher CPU & disk activity
    let _ = fs::write(&stat_file, "cpu  1500 200 300 8500 100 0 0 0 0 0\n"); // 500 active delta, 1000 total delta -> 50%
    let _ = fs::write(
        &disk_file,
        "   8       0 sda 200 0 4000 100 0 0 0 0 0 0 0\n",
    ); // 2000 sector delta
    let s2 = provider.poll_signals();

    assert!(
        (s2.cpu_load - 0.50).abs() < 0.05,
        "CPU load delta should calculate ~0.50, got {}",
        s2.cpu_load
    );
    assert!(s2.io_activity > 0.0, "IO activity delta should be non-zero");

    // Cleanup
    let _ = fs::remove_file(&stat_file);
    let _ = fs::remove_file(&mem_file);
    let _ = fs::remove_file(&disk_file);
    let _ = fs::remove_dir_all(&bat_dir);
}

#[test]
fn test_phase12_native_audio_architecture_and_resampling() {
    use lavaterm::audio::{AudioProvider, LiveAudioProvider, PcmRingBuffer, SpectrumAnalyzer};

    let ring = PcmRingBuffer::new(2048);
    let analyzer = SpectrumAnalyzer::new(44100, 512);
    let mut provider = LiveAudioProvider::new(ring.clone(), analyzer);

    assert!(provider.is_live());
    assert_eq!(provider.provider_name(), "live");
    assert_eq!(provider.sample_rate(), 44100);

    // 1. Ingest 48kHz stereo signal resampled to 44.1kHz
    let mut stereo_48k = Vec::with_capacity(960);
    for i in 0..480 {
        let sample = (2.0 * std::f32::consts::PI * 80.0 * (i as f32) / 48000.0).sin();
        stereo_48k.push(sample); // L
        stereo_48k.push(sample); // R
    }

    // Downmix to mono and resample to 44.1kHz
    let mut mono_48k = Vec::with_capacity(480);
    for chunk in stereo_48k.chunks_exact(2) {
        mono_48k.push((chunk[0] + chunk[1]) * 0.5);
    }
    ring.push_resampled(&mono_48k, 48000, 44100);

    let signals = provider.poll_signals();
    assert!(
        signals.bass > 0.0,
        "80Hz pulse resampled from 48kHz should register bass energy"
    );
    assert!(signals.volume > 0.0, "Volume should be non-zero");
}

#[test]
fn test_phase12_unified_audio_runtime_and_device_enumeration() {
    use lavaterm::audio::{create_audio_provider, list_audio_devices};
    use lavaterm::config::AudioConfig;

    // 1. Device enumeration
    let devices = list_audio_devices();
    assert!(!devices.is_empty(), "Audio devices list must not be empty");
    assert!(
        devices.iter().any(|d| d.is_default),
        "Must contain a default device"
    );

    // 2. Synthetic fallback when disabled
    let disabled_cfg = AudioConfig {
        enabled: false,
        bpm: 130.0,
        device: None,
    };
    let mut disabled_provider = create_audio_provider(&disabled_cfg);
    assert!(!disabled_provider.is_live());
    assert_eq!(disabled_provider.provider_name(), "synthetic");
    let disabled_signals = disabled_provider.poll_signals();
    assert!(disabled_signals.bass.is_finite());

    // 3. Live capture provider when enabled
    let enabled_cfg = AudioConfig {
        enabled: true,
        bpm: 120.0,
        device: None,
    };
    let mut enabled_provider = create_audio_provider(&enabled_cfg);
    let signals = enabled_provider.poll_signals();
    assert!(signals.bass.is_finite());
    assert!(signals.mid.is_finite());
    assert!(signals.treble.is_finite());
    assert!(signals.volume.is_finite());
}
