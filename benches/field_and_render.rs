//! Comprehensive performance benchmarks for LavaTerm field evaluation, renderers, audio FFT, and pipeline.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lavaterm::{
    audio::{resample_linear, AudioSignals, SpectrumAnalyzer},
    core::{PhysicsParams, Simulation},
    reactive::SystemSignals,
    render::{
        rasterize_simulation, rasterize_simulation_options, BlockRenderer, BrailleRenderer,
        ColorPalette, HalfBlockRenderer, Renderer, VirtualFramebuffer,
    },
    widget::{CompactProfile, CompactScaler},
};

fn bench_field_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("field_evaluation");

    for blob_count in [6, 12, 24] {
        let sim = Simulation::new(PhysicsParams::default(), blob_count, 42);
        group.bench_with_input(
            BenchmarkId::from_parameter(blob_count),
            &blob_count,
            |b, _| {
                b.iter(|| {
                    for y in 0..20 {
                        for x in 0..40 {
                            let px = black_box(x as f32 / 40.0);
                            let py = black_box(y as f32 / 20.0);
                            let _ = sim.evaluate_at(px, py);
                        }
                    }
                });
            },
        );
    }
    group.finish();
}

fn bench_rasterization(c: &mut Criterion) {
    let mut group = c.benchmark_group("rasterization");
    let sim = Simulation::new(PhysicsParams::default(), 12, 42);
    let palette = ColorPalette::default();

    let mut fb_80x48 = VirtualFramebuffer::new(80, 48, palette.background);
    group.bench_function("rasterize_80x48", |b| {
        b.iter(|| {
            rasterize_simulation(&sim, &mut fb_80x48, &palette, black_box(1.0));
        });
    });

    let mut fb_120x60 = VirtualFramebuffer::new(120, 60, palette.background);
    group.bench_function("rasterize_120x60", |b| {
        b.iter(|| {
            rasterize_simulation(&sim, &mut fb_120x60, &palette, black_box(1.0));
        });
    });

    group.bench_function("rasterize_stepped_gradient_80x48", |b| {
        b.iter(|| {
            rasterize_simulation_options(&sim, &mut fb_80x48, &palette, black_box(1.0), false);
        });
    });

    group.finish();
}

fn bench_renderers(c: &mut Criterion) {
    let mut group = c.benchmark_group("renderers");
    let palette = ColorPalette::default();

    let mut hb_fb = VirtualFramebuffer::new(80, 48, palette.background);
    let sim = Simulation::new(PhysicsParams::default(), 12, 42);
    rasterize_simulation(&sim, &mut hb_fb, &palette, 1.0);

    let mut hb_renderer = HalfBlockRenderer::new();
    let mut sink = Vec::with_capacity(64 * 1024);

    group.bench_function("halfblock", |b| {
        b.iter(|| {
            sink.clear();
            hb_renderer
                .render(&hb_fb, &mut sink)
                .expect("render succeeds");
            black_box(sink.len());
        });
    });

    let mut block_renderer = BlockRenderer::new();
    group.bench_function("block", |b| {
        b.iter(|| {
            sink.clear();
            block_renderer
                .render(&hb_fb, &mut sink)
                .expect("render succeeds");
            black_box(sink.len());
        });
    });

    let mut braille_fb = VirtualFramebuffer::new(160, 96, palette.background);
    rasterize_simulation(&sim, &mut braille_fb, &palette, 1.0);
    let mut braille_renderer = BrailleRenderer::new();
    group.bench_function("braille", |b| {
        b.iter(|| {
            sink.clear();
            braille_renderer
                .render(&braille_fb, &mut sink)
                .expect("render succeeds");
            black_box(sink.len());
        });
    });

    group.finish();
}

fn bench_fft_and_audio(c: &mut Criterion) {
    let mut group = c.benchmark_group("fft_and_audio");

    for size in [512, 1024, 2048] {
        let mut samples = vec![0.0f32; size];
        for (i, s) in samples.iter_mut().enumerate() {
            *s = (2.0 * std::f32::consts::PI * 440.0 * (i as f32) / 44100.0).sin();
        }

        group.bench_with_input(
            BenchmarkId::new("compute_fft", size),
            &size,
            |b, _| {
                let mut real = samples.clone();
                let mut imag = vec![0.0f32; size];
                b.iter(|| {
                    real.copy_from_slice(&samples);
                    imag.fill(0.0);
                    let _ = SpectrumAnalyzer::compute_fft(&mut real, &mut imag);
                    black_box(real[0]);
                });
            },
        );
    }

    let analyzer = SpectrumAnalyzer::new(44100, 1024);
    let pcm_1024 = vec![0.3f32; 1024];
    group.bench_function("spectrum_analyze_1024", |b| {
        b.iter(|| {
            let sig = analyzer.analyze(&pcm_1024);
            black_box(sig.bass);
        });
    });


    let pcm_48k = vec![0.5f32; 1024];
    let mut resample_out = Vec::with_capacity(1024);
    group.bench_function("resample_linear_48k_to_44k", |b| {
        b.iter(|| {
            resample_linear(&pcm_48k, 48000, 44100, &mut resample_out);
            black_box(resample_out.len());
        });
    });

    group.finish();
}

fn bench_pipeline_and_adaptation(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline_and_adaptation");
    let palette = ColorPalette::default();
    let mut sim = Simulation::new(PhysicsParams::default(), 12, 42);
    let mut fb = VirtualFramebuffer::new(80, 48, palette.background);
    let mut hb_renderer = HalfBlockRenderer::new();
    let mut sink = Vec::with_capacity(64 * 1024);
    let audio_sig = AudioSignals::new(0.5, 0.4, 0.3, 0.6);
    let sys_sig = SystemSignals::new(0.4, 0.5, 0.8, 0.2);

    group.bench_function("full_frame_audio_halfblock", |b| {
        b.iter(|| {
            sim.step_audio(1.0 / 30.0, &audio_sig);
            rasterize_simulation(&sim, &mut fb, &palette, 1.0);
            sink.clear();
            hb_renderer.render(&fb, &mut sink).unwrap();
            black_box(sink.len());
        });
    });

    group.bench_function("full_frame_reactive_halfblock", |b| {
        b.iter(|| {
            sim.step_reactive(1.0 / 30.0, &sys_sig);
            rasterize_simulation(&sim, &mut fb, &palette, 1.0);
            sink.clear();
            hb_renderer.render(&fb, &mut sink).unwrap();
            black_box(sink.len());
        });
    });

    let profile = CompactProfile {
        blob_count: 4,
        radius_scale: 0.6,
        buoyancy_scale: 1.2,
        noise_scale: 0.9,
    };
    group.bench_function("compact_adapt_simulation", |b| {
        b.iter(|| {
            CompactScaler::adapt_simulation(&profile, &mut sim);
            black_box(sim.blobs.len());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_field_evaluation,
    bench_rasterization,
    bench_renderers,
    bench_fft_and_audio,
    bench_pipeline_and_adaptation
);
criterion_main!(benches);
