//! Performance benchmarks for LavaTerm field evaluation and terminal renderers.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lavaterm::{
    core::{PhysicsParams, Simulation},
    render::{
        rasterize_simulation, BlockRenderer, BrailleRenderer, ColorPalette, HalfBlockRenderer,
        Renderer, VirtualFramebuffer,
    },
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
    let sim = Simulation::new(PhysicsParams::default(), 12, 42);
    let palette = ColorPalette::default();
    let mut fb = VirtualFramebuffer::new(80, 48, palette.background);

    c.bench_function("rasterize_80x48", |b| {
        b.iter(|| {
            rasterize_simulation(&sim, &mut fb, &palette, black_box(1.0));
        });
    });
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

criterion_group!(
    benches,
    bench_field_evaluation,
    bench_rasterization,
    bench_renderers
);
criterion_main!(benches);
