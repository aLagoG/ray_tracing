use std::time::Duration;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
};
use ray_tracing::RunConfig;

fn get_config() -> RunConfig<'static> {
    let mut config = RunConfig::default();
    config.quiet = true;

    config.img_config.aspect_ratio = 3.0 / 2.0;
    config.img_config.width = 150;
    config.img_config.samples_per_pixel = 10;
    config.img_config.max_depth = 10;

    config.scene_config.small_sphere_count = 50;

    config
}

fn set_up_group(group: &mut BenchmarkGroup<WallTime>) {
    group.warm_up_time(Duration::from_secs(5)).sample_size(50);
}

pub fn samples_per_pixel(c: &mut Criterion) {
    let mut config = get_config();

    let mut group = c.benchmark_group("samples_per_pixel");
    set_up_group(&mut group);

    config.img_config.samples_per_pixel = 1;
    group.bench_with_input(BenchmarkId::from_parameter(1), &config, |b, c| {
        b.iter(|| ray_tracing::run(&c))
    });

    for i in (10..=100).step_by(20) {
        config.img_config.samples_per_pixel = i;
        group.bench_with_input(BenchmarkId::from_parameter(i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });
    }

    group.finish();
}

pub fn ray_depth(c: &mut Criterion) {
    let mut config = get_config();

    let mut group = c.benchmark_group("ray_depth");
    set_up_group(&mut group);

    config.img_config.max_depth = 1;
    group.bench_with_input(BenchmarkId::from_parameter(1), &config, |b, c| {
        b.iter(|| ray_tracing::run(&c))
    });

    for i in (5..=50).step_by(5) {
        config.img_config.max_depth = i;
        group.bench_with_input(BenchmarkId::from_parameter(i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });
    }

    group.finish();
}

pub fn img_width(c: &mut Criterion) {
    let mut config = get_config();
    config.img_config.samples_per_pixel = 10;
    config.img_config.max_depth = 10;

    let mut group = c.benchmark_group("img_width");
    set_up_group(&mut group);

    for i in (50..=500).step_by(50) {
        config.img_config.width = i;
        group.bench_with_input(BenchmarkId::from_parameter(i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });
    }

    group.finish();
}

pub fn sphere_count(c: &mut Criterion) {
    let mut config = get_config();
    config.img_config.samples_per_pixel = 10;
    config.img_config.max_depth = 10;

    let mut group = c.benchmark_group("sphere_count");
    set_up_group(&mut group);

    for i in (50..=400).step_by(50) {
        config.scene_config.small_sphere_count = i;
        group.bench_with_input(BenchmarkId::from_parameter(i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });
    }

    group.finish();
}

pub fn materials(c: &mut Criterion) {
    let mut config = get_config();

    let mut group = c.benchmark_group("materials");
    set_up_group(&mut group);

    for i in (0..=100).step_by(10) {
        let leading_prob = i as f64 / 100.0;
        let other_probs = (1.0 - leading_prob) / 2.0;

        config.scene_config.diffuse_prob = leading_prob;
        config.scene_config.metal_prob = other_probs;
        group.bench_with_input(BenchmarkId::new("Diffuse", i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });

        config.scene_config.metal_prob = leading_prob;
        config.scene_config.diffuse_prob = other_probs;
        group.bench_with_input(BenchmarkId::new("Metal", i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });

        config.scene_config.diffuse_prob = other_probs;
        config.scene_config.metal_prob = other_probs;
        group.bench_with_input(BenchmarkId::new("Glass", i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });
    }

    group.finish();
}

pub fn bvh(c: &mut Criterion) {
    let mut config = get_config();
    config.img_config.samples_per_pixel = 10;
    config.img_config.max_depth = 10;

    let mut group = c.benchmark_group("bvh");
    set_up_group(&mut group);

    for i in (50..=500).step_by(50) {
        config.scene_config.small_sphere_count = i;

        config.use_bvh = true;
        group.bench_with_input(BenchmarkId::new("bvh", i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });

        config.use_bvh = false;
        group.bench_with_input(BenchmarkId::new("plain", i), &config, |b, c| {
            b.iter(|| ray_tracing::run(&c))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    samples_per_pixel,
    ray_depth,
    img_width,
    sphere_count,
    materials,
    bvh,
);
criterion_main!(benches);
