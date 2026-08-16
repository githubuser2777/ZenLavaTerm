//! LavaTerm — CLI Entry point & Terminal Event Loop.

use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use lavaterm::{
    config::load_config,
    core::{PhysicsParams, Simulation},
    input::{map_key_event, Action},
    reactive::default_system_provider,
    render::{
        rasterize_simulation, BlockRenderer, BrailleRenderer, ColorPalette, HalfBlockRenderer,
        Renderer, VirtualFramebuffer,
    },
    LavaError, Result,
};
use std::{
    io::{stdout, BufWriter, Write},
    path::PathBuf,
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

    /// Enable ambient system-reactive visualizer mode (CPU/RAM/Battery)
    #[arg(long)]
    system: bool,

    /// Run headless simulation without taking over TTY (useful for testing/CI)
    #[arg(long)]
    headless: bool,

    /// Number of frames to step when in headless mode
    #[arg(long, default_value_t = 60)]
    frames: usize,
}

fn setup_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
        original_hook(panic_info);
    }));
}

fn run_headless(
    sim: &mut Simulation,
    palette: &ColorPalette,
    threshold: f32,
    frames: usize,
    system_reactive: bool,
) -> Result<()> {
    println!(
        "Starting LavaTerm headless simulation ({} frames, reactive={})...",
        frames, system_reactive
    );
    let mut fb = VirtualFramebuffer::new(80, 48, palette.background);
    let dt = 1.0 / 30.0;
    let mut provider = if system_reactive {
        Some(default_system_provider())
    } else {
        None
    };

    for frame in 0..frames {
        if let Some(ref mut p) = provider {
            let signals = p.poll_signals();
            sim.step_reactive(dt, &signals);
        } else {
            sim.step(dt);
        }
        rasterize_simulation(sim, &mut fb, palette, threshold);
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

fn run_interactive(
    mut sim: Simulation,
    palette: ColorPalette,
    threshold: f32,
    fps: u32,
    renderer_type: &str,
    system_reactive: bool,
    poll_interval_ms: u64,
) -> Result<()> {
    // 1. Initialize Terminal
    setup_panic_hook();
    terminal::enable_raw_mode()?;
    let mut out = BufWriter::with_capacity(64 * 1024, stdout());
    execute!(out, EnterAlternateScreen, cursor::Hide)?;

    // 2. Query initial terminal dimensions
    let (mut cols, mut rows) = terminal::size()?;
    let (v_width, v_height) = framebuffer_dimensions(cols, rows, renderer_type);
    let mut fb = VirtualFramebuffer::new(v_width, v_height, palette.background);

    let mut renderer: Box<dyn Renderer> = match renderer_type {
        "block" => Box::new(BlockRenderer::new()),
        "braille" => Box::new(BrailleRenderer::new()),
        _ => Box::new(HalfBlockRenderer::new()),
    };

    let target_frame_duration = Duration::from_secs_f32(1.0 / fps as f32);
    let mut last_instant = Instant::now();
    let mut paused = false;

    // Optional Reactive Metrics Poller
    let mut provider = if system_reactive {
        Some(default_system_provider())
    } else {
        None
    };
    let mut last_metric_poll = Instant::now();
    let poll_duration = Duration::from_millis(poll_interval_ms.max(100));
    let mut current_signals = provider.as_mut().map(|p| p.poll_signals());

    // 3. Event and Render Loop
    let loop_result = (|| -> Result<()> {
        loop {
            let now = Instant::now();
            let delta = now.duration_since(last_instant);
            last_instant = now;

            // Poll input events
            while event::poll(Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key) => match map_key_event(key) {
                        Action::Quit => return Ok(()),
                        Action::TogglePause => paused = !paused,
                        Action::SpeedUp => {
                            sim.params.buoyancy = (sim.params.buoyancy + 0.1).min(3.0)
                        }
                        Action::SlowDown => {
                            sim.params.buoyancy = (sim.params.buoyancy - 0.1).max(0.1)
                        }
                        Action::Reset => {
                            let count = sim.blobs.len();
                            sim = Simulation::new(sim.params, count, 42);
                        }
                        Action::None => {}
                    },
                    Event::Resize(new_cols, new_rows) => {
                        cols = new_cols;
                        rows = new_rows;
                        let (new_w, new_h) = framebuffer_dimensions(cols, rows, renderer_type);
                        fb.resize(new_w, new_h, palette.background);
                    }
                    _ => {}
                }
            }

            // Poll background OS metrics if reactive mode active
            if let Some(ref mut p) = provider {
                if last_metric_poll.elapsed() >= poll_duration {
                    current_signals = Some(p.poll_signals());
                    last_metric_poll = Instant::now();
                }
            }

            // Step Simulation
            if !paused {
                if let Some(ref signals) = current_signals {
                    sim.step_reactive(delta.as_secs_f32(), signals);
                } else {
                    sim.step(delta.as_secs_f32());
                }
            }

            // Rasterize into Framebuffer
            rasterize_simulation(&sim, &mut fb, &palette, threshold);

            // Render to Terminal
            renderer
                .render(&fb, &mut out)
                .map_err(|e| LavaError::Render(e.to_string()))?;
            out.flush()?;

            // Frame rate capping
            let elapsed = now.elapsed();
            if elapsed < target_frame_duration {
                std::thread::sleep(target_frame_duration - elapsed);
            }
        }
    })();

    // 4. Cleanup Terminal
    let _ = execute!(out, LeaveAlternateScreen, cursor::Show);
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

    let blob_count = cli.blobs.unwrap_or(config.simulation.blobs);
    let fps = cli.fps.unwrap_or(config.render.fps);
    let renderer_type = cli.renderer.unwrap_or(config.render.renderer);
    let system_reactive = cli.system || config.reactive.enabled;
    let poll_interval_ms = config.reactive.poll_interval_ms;

    let physics = PhysicsParams {
        gravity: config.simulation.gravity,
        buoyancy: config.simulation.buoyancy,
        viscosity: config.simulation.viscosity,
        noise: config.simulation.noise,
        thermal_transfer_rate: 0.40,
    };

    let mut sim = Simulation::new(physics, blob_count, 1337);
    let palette = ColorPalette::from(config.palette);
    let threshold = config.simulation.threshold;

    let result = if cli.headless {
        run_headless(&mut sim, &palette, threshold, cli.frames, system_reactive)
    } else {
        run_interactive(
            sim,
            palette,
            threshold,
            fps,
            &renderer_type,
            system_reactive,
            poll_interval_ms,
        )
    };

    if let Err(e) = result {
        eprintln!("LavaTerm error: {e}");
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}
