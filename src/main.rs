//! LavaTerm — CLI Entry point & Terminal Event Loop.

use clap::Parser;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use lavaterm::{
    audio::default_audio_provider,
    config::load_config,
    core::{Interaction, PhysicsParams, Simulation},
    input::{map_key_event_with_ripple, Action, MouseTracker},
    reactive::default_system_provider,
    render::{
        rasterize_simulation_options, BlockRenderer, BrailleRenderer, ColorPalette,
        HalfBlockRenderer, Renderer, VirtualFramebuffer,
    },
    theme::{load_custom_theme_file, resolve_theme},
    widget::{
        render_snapshot_options, resolve_policy, should_compact, CompactScaler, ExecutionMode,
        PolicyInput,
    },
    LavaError, Result,
};
use std::{
    io::{stdout, BufWriter, Write},
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    time::{Duration, Instant},
};

/// LavaTerm: Terminal-native ambient lava lamp visualizer.
#[derive(Parser, Debug)]
#[command(name = "lavaterm", author, version, about, long_about = None)]
struct Cli {
    /// Path to custom TOML configuration file
    #[arg(short, long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Renderer backend: halfblock | block | braille
    #[arg(short, long, value_name = "TYPE")]
    renderer: Option<String>,

    /// Target frames per second
    #[arg(long, value_name = "FPS")]
    fps: Option<u32>,

    /// Number of metaball blobs
    #[arg(long, value_name = "COUNT")]
    blobs: Option<usize>,

    /// Theme preset, auto-detect, or theme file path (e.g. ocean, cyberpunk, synthwave, auto, pywal, wallust)
    #[arg(short, long, value_name = "THEME")]
    theme: Option<String>,

    /// Force compact geometry & profile scaling
    #[arg(long)]
    compact: bool,

    /// Run as low-overhead ambient widget (default 15 FPS, compact physics)
    #[arg(long)]
    widget: bool,

    /// Run inline in terminal without entering alternate screen
    #[arg(long)]
    inline: bool,

    /// Render a single ANSI frame to stdout and exit
    #[arg(long)]
    snapshot: bool,

    /// Explicit viewport width (columns)
    #[arg(long, value_name = "COLS")]
    width: Option<u16>,

    /// Explicit viewport height (rows)
    #[arg(long, value_name = "ROWS")]
    height: Option<u16>,

    /// Enable ambient system-reactive visualizer mode (CPU/RAM/Battery)
    #[arg(long)]
    system: bool,

    /// Enable audio-reactive visualizer mode (FFT bass/mid/treble)
    #[arg(long)]
    audio: bool,

    /// Disable mouse click shockwaves, dragging, and scroll pressure
    #[arg(long)]
    no_mouse: bool,

    /// Disable keyboard ripples on character keypresses
    #[arg(long)]
    no_ripple: bool,

    /// Multiplier for mouse click shockwave force (default: 1.0)
    #[arg(long, value_name = "FORCE")]
    shockwave_force: Option<f32>,

    /// Multiplier for mouse drag stirring force (default: 1.0)
    #[arg(long, value_name = "FORCE")]
    stir_force: Option<f32>,

    /// Run headless simulation without taking over TTY (useful for testing/CI)
    #[arg(long)]
    headless: bool,

    /// Number of frames to step when in headless mode
    #[arg(long, default_value_t = 60)]
    frames: usize,
}

fn restore_terminal(alternate_screen: bool, mouse_enabled: bool) {
    let _ = terminal::disable_raw_mode();
    if mouse_enabled {
        let _ = execute!(stdout(), DisableMouseCapture);
    }
    if alternate_screen {
        let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
    } else {
        let _ = execute!(stdout(), cursor::Show);
    }
}

fn setup_panic_hook(alternate_screen: bool, mouse_enabled: bool) {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore_terminal(alternate_screen, mouse_enabled);
        original_hook(panic_info);
    }));
}

#[cfg(unix)]
fn setup_signal_handler(term_flag: Arc<AtomicBool>) {
    let _ = signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term_flag));
    let _ = signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term_flag));
}

#[cfg(windows)]
fn setup_signal_handler(term_flag: Arc<AtomicBool>) {
    use std::sync::OnceLock;
    static GLOBAL_TERM_FLAG: OnceLock<Arc<AtomicBool>> = OnceLock::new();
    let _ = GLOBAL_TERM_FLAG.set(term_flag);

    unsafe extern "system" fn ctrl_handler(_ctrl_type: u32) -> i32 {
        if let Some(flag) = GLOBAL_TERM_FLAG.get() {
            flag.store(true, Ordering::SeqCst);
        }
        1
    }

    extern "system" {
        fn SetConsoleCtrlHandler(
            HandlerRoutine: Option<unsafe extern "system" fn(u32) -> i32>,
            Add: i32,
        ) -> i32;
    }

    unsafe {
        SetConsoleCtrlHandler(Some(ctrl_handler), 1);
    }
}

#[cfg(not(any(unix, windows)))]
fn setup_signal_handler(_term_flag: Arc<AtomicBool>) {}

struct RuntimeOptions {
    renderer_type: String,
    fps: u32,
    threshold: f32,
    gradient: bool,
    system_reactive: bool,
    audio_reactive: bool,
    poll_interval_ms: u64,
    mouse_enabled: bool,
    keyboard_ripple: bool,
    shockwave_force: f32,
    stir_force: f32,
}

fn run_headless(
    sim: &mut Simulation,
    palette: &ColorPalette,
    frames: usize,
    opts: &RuntimeOptions,
) -> Result<()> {
    println!(
        "Starting LavaTerm headless simulation ({} frames, system={}, audio={})...",
        frames, opts.system_reactive, opts.audio_reactive
    );
    let mut fb = VirtualFramebuffer::new(80, 48, palette.background);
    let dt = 1.0 / 30.0;
    let mut sys_provider = if opts.system_reactive {
        Some(default_system_provider())
    } else {
        None
    };
    let mut audio_provider = if opts.audio_reactive {
        Some(default_audio_provider())
    } else {
        None
    };

    for frame in 0..frames {
        if let Some(ref mut ap) = audio_provider {
            let audio_sig = ap.poll_signals();
            sim.step_audio(dt, &audio_sig);
        } else if let Some(ref mut sp) = sys_provider {
            let signals = sp.poll_signals();
            sim.step_reactive(dt, &signals);
        } else {
            sim.step(dt);
        }
        rasterize_simulation_options(sim, &mut fb, palette, opts.threshold, opts.gradient);
        if frame % 20 == 0 || frame == frames - 1 {
            println!(
                "  [Frame {:03}/{:03}] Sim Time: {:.2}s | Blobs: {} | Active pixels in canvas: {}",
                frame + 1,
                frames,
                sim.elapsed_time,
                sim.blobs.len(),
                fb.as_slice()
                    .iter()
                    .filter(|c| **c != palette.background)
                    .count()
            );
        }
    }
    println!("Headless simulation completed successfully.");
    Ok(())
}

fn framebuffer_dimensions(cols: u16, rows: u16, renderer_type: &str) -> (usize, usize) {
    match renderer_type {
        "block" => (cols as usize, rows as usize),
        "braille" => (cols as usize * 2, rows as usize * 4),
        _ => (cols as usize, (rows as usize) * 2),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InteractiveMode {
    Fullscreen,
    Inline { target_height: u16 },
}

fn run_event_loop(
    mut sim: Simulation,
    palette: ColorPalette,
    opts: RuntimeOptions,
    fixed_dim: Option<(u16, u16)>,
    mode: InteractiveMode,
) -> Result<()> {
    let is_fullscreen = matches!(mode, InteractiveMode::Fullscreen);
    setup_panic_hook(is_fullscreen, opts.mouse_enabled);

    let shutdown_flag = Arc::new(AtomicBool::new(false));
    setup_signal_handler(Arc::clone(&shutdown_flag));

    terminal::enable_raw_mode()?;
    let mut out = BufWriter::with_capacity(64 * 1024, stdout());

    if is_fullscreen {
        execute!(out, EnterAlternateScreen, cursor::Hide)?;
    } else {
        execute!(out, cursor::Hide)?;
    }

    if opts.mouse_enabled {
        let _ = execute!(out, EnableMouseCapture);
    }

    let (term_cols, term_rows) = terminal::size().unwrap_or((80, 24));
    let (mut cols, mut rows) = match (fixed_dim, mode) {
        (Some((w, h)), _) => (w, h),
        (None, InteractiveMode::Fullscreen) => (term_cols, term_rows),
        (None, InteractiveMode::Inline { target_height }) => {
            (term_cols, target_height.min(term_rows))
        }
    };

    let (v_width, v_height) = framebuffer_dimensions(cols, rows, &opts.renderer_type);
    let mut fb = VirtualFramebuffer::new(v_width, v_height, palette.background);

    let mut renderer: Box<dyn Renderer> = match opts.renderer_type.as_str() {
        "block" => Box::new(BlockRenderer::new()),
        "braille" => Box::new(BrailleRenderer::new()),
        _ => Box::new(HalfBlockRenderer::new()),
    };

    let mut mouse_tracker = MouseTracker::new();
    let target_frame_duration = Duration::from_secs_f32(1.0 / opts.fps as f32);
    let mut last_instant = Instant::now();
    let mut paused = false;

    let mut sys_provider = if opts.system_reactive {
        Some(default_system_provider())
    } else {
        None
    };
    let mut audio_provider = if opts.audio_reactive {
        Some(default_audio_provider())
    } else {
        None
    };

    let mut last_metric_poll = Instant::now();
    let poll_duration = Duration::from_millis(opts.poll_interval_ms.max(100));
    let mut current_sys_signals = sys_provider.as_mut().map(|p| p.poll_signals());

    let loop_result = (|| -> Result<()> {
        loop {
            if shutdown_flag.load(Ordering::Relaxed) {
                return Ok(());
            }

            let now = Instant::now();
            let delta = now.duration_since(last_instant);
            last_instant = now;

            while event::poll(Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key) => match map_key_event_with_ripple(key, opts.keyboard_ripple) {
                        Action::Quit => return Ok(()),
                        Action::TogglePause => paused = !paused,
                        Action::SpeedUp => {
                            sim.params.buoyancy = (sim.params.buoyancy + 0.1).min(3.0);
                        }
                        Action::SlowDown => {
                            sim.params.buoyancy = (sim.params.buoyancy - 0.1).max(0.1);
                        }
                        Action::Reset => {
                            let count = sim.blobs.len();
                            let radius_scale = sim.radius_scale;
                            sim = Simulation::new(sim.params, count, 42);
                            sim.apply_radius_scale(radius_scale);
                            mouse_tracker.reset();
                        }
                        Action::Ripple(intensity) => {
                            sim.apply_interaction(&Interaction::Ripple { intensity });
                        }
                        Action::None => {}
                    },
                    Event::Mouse(mouse_event) if opts.mouse_enabled => {
                        if let Some(interaction) = mouse_tracker.handle_event(
                            mouse_event,
                            cols,
                            rows,
                            opts.shockwave_force,
                            opts.stir_force,
                        ) {
                            sim.apply_interaction(&interaction);
                        }
                    }
                    Event::Resize(new_cols, new_rows) if fixed_dim.is_none() => {
                        cols = new_cols;
                        rows = match mode {
                            InteractiveMode::Fullscreen => new_rows,
                            InteractiveMode::Inline { target_height } => {
                                target_height.min(new_rows)
                            }
                        };
                        let (new_w, new_h) =
                            framebuffer_dimensions(cols, rows, &opts.renderer_type);
                        fb.resize(new_w, new_h, palette.background);
                        mouse_tracker.reset();
                    }
                    _ => {}
                }
            }

            if let Some(ref mut sp) = sys_provider {
                if last_metric_poll.elapsed() >= poll_duration {
                    current_sys_signals = Some(sp.poll_signals());
                    last_metric_poll = Instant::now();
                }
            }

            if !paused {
                if let Some(ref mut ap) = audio_provider {
                    let audio_sig = ap.poll_signals();
                    sim.step_audio(delta.as_secs_f32(), &audio_sig);
                } else if let Some(ref signals) = current_sys_signals {
                    sim.step_reactive(delta.as_secs_f32(), signals);
                } else {
                    sim.step(delta.as_secs_f32());
                }
            }

            rasterize_simulation_options(&sim, &mut fb, &palette, opts.threshold, opts.gradient);

            renderer
                .render(&fb, &mut out)
                .map_err(|e| LavaError::Render(e.to_string()))?;
            out.flush()?;

            let elapsed = now.elapsed();
            if elapsed < target_frame_duration {
                std::thread::sleep(target_frame_duration - elapsed);
            }
        }
    })();

    if opts.mouse_enabled {
        let _ = execute!(out, DisableMouseCapture);
    }
    if is_fullscreen {
        let _ = execute!(out, LeaveAlternateScreen, cursor::Show);
    } else {
        let _ = execute!(out, cursor::Show);
    }
    let _ = out.flush();
    let _ = terminal::disable_raw_mode();

    loop_result
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();

    let config = match load_config(cli.config.as_deref()) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error loading configuration: {err}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let policy_input = PolicyInput {
        cli_fps: cli.fps,
        cli_compact: cli.compact,
        cli_widget: cli.widget,
        cli_inline: cli.inline,
        cli_snapshot: cli.snapshot,
        cli_headless: cli.headless,
        cli_width: cli.width,
        cli_height: cli.height,

        toml_render_fps: config.render.fps,
        toml_widget_fps: config.widget.fps,
        toml_widget_compact: config.widget.compact,
        toml_widget_inline: config.widget.inline,
        toml_widget_width: config.widget.width,
        toml_widget_height: config.widget.height,
        toml_widget_adapt_blobs: config.widget.adapt_blobs,
    };

    let policy = match resolve_policy(&policy_input) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Configuration/Policy error: {err}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let base_blob_count = cli.blobs.unwrap_or(config.simulation.blobs);
    let renderer_type = cli.renderer.unwrap_or(config.render.renderer);
    let system_reactive = cli.system || config.reactive.enabled;
    let audio_reactive = cli.audio || config.audio.enabled;
    let poll_interval_ms = config.reactive.poll_interval_ms;
    let threshold = config.simulation.threshold;

    let mouse_enabled = if cli.no_mouse {
        false
    } else {
        config.interaction.mouse
    };
    let keyboard_ripple = if cli.no_ripple {
        false
    } else {
        config.interaction.keyboard_ripple
    };
    let shockwave_force = cli
        .shockwave_force
        .unwrap_or(config.interaction.shockwave_force);
    let stir_force = cli.stir_force.unwrap_or(config.interaction.stir_force);

    let base_physics = PhysicsParams {
        gravity: config.simulation.gravity,
        buoyancy: config.simulation.buoyancy,
        viscosity: config.simulation.viscosity,
        noise: config.simulation.noise,
        thermal_transfer_rate: config.simulation.thermal_transfer_rate,
    };

    let palette = if let Some(ref theme_spec) = cli.theme {
        match resolve_theme(theme_spec) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error resolving theme '{theme_spec}': {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    } else if let Some(ref path) = config.theme.path {
        match load_custom_theme_file(path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error loading theme file '{}': {e}", path.display());
                return std::process::ExitCode::FAILURE;
            }
        }
    } else if let Some(ref theme_name) = config.theme.name {
        match resolve_theme(theme_name) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error resolving theme '{theme_name}': {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    } else {
        ColorPalette::from(config.palette)
    };

    let (initial_cols, initial_rows) = match policy.explicit_dimensions {
        Some((w, h)) => (w, h),
        None => terminal::size().unwrap_or((80, 24)),
    };

    let is_compact = should_compact(initial_cols, initial_rows, policy.force_compact);
    let (blob_count, physics, profile_opt) = if is_compact && policy.adapt_blobs {
        let profile = CompactScaler::calculate_profile(initial_cols, initial_rows, base_blob_count);
        let adapted_physics = CompactScaler::adapt_physics(&profile, base_physics);
        (profile.blob_count, adapted_physics, Some(profile))
    } else {
        (base_blob_count, base_physics, None)
    };

    let mut sim = Simulation::new(physics, blob_count, 1337);
    if let Some(profile) = profile_opt {
        sim.apply_radius_scale(profile.radius_scale);
    }

    let opts = RuntimeOptions {
        renderer_type: renderer_type.clone(),
        fps: policy.target_fps,
        threshold,
        gradient: config.render.gradient,
        system_reactive,
        audio_reactive,
        poll_interval_ms,
        mouse_enabled,
        keyboard_ripple,
        shockwave_force,
        stir_force,
    };

    let result = match policy.mode {
        ExecutionMode::Snapshot => {
            match render_snapshot_options(
                &mut sim,
                &palette,
                initial_cols,
                initial_rows,
                &renderer_type,
                threshold,
                5,
                config.render.gradient,
            ) {
                Ok(snapshot_str) => {
                    print!("{snapshot_str}");
                    let _ = stdout().flush();
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        ExecutionMode::Headless => run_headless(&mut sim, &palette, cli.frames, &opts),
        ExecutionMode::Inline => {
            let target_height = policy
                .explicit_dimensions
                .map(|(_, h)| h)
                .or(config.widget.height)
                .unwrap_or(10);
            run_event_loop(
                sim,
                palette,
                opts,
                policy.explicit_dimensions,
                InteractiveMode::Inline { target_height },
            )
        }
        ExecutionMode::Interactive | ExecutionMode::Widget => run_event_loop(
            sim,
            palette,
            opts,
            policy.explicit_dimensions,
            InteractiveMode::Fullscreen,
        ),
    };

    if let Err(e) = result {
        eprintln!("LavaTerm error: {e}");
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}
