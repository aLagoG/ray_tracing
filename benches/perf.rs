use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use std::time::Duration;

use ray_tracing::RunConfig;

fn get_config() -> RunConfig<'static> {
    let mut config = RunConfig::default();
    config.img_config.aspect_ratio = 3.0 / 2.0;
    config.quiet = true;
    config.use_bvh = false;

    config
}

fn set_up_group(group: &mut BenchmarkGroup<WallTime>) {
    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(30))
        .sample_size(10);
}

pub fn small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Small");
    set_up_group(&mut group);

    let mut config = get_config();
    config.img_config.width = 100;

    group.bench_function("Small", |b| b.iter(|| ray_tracing::run(&config)));

    group.finish();
}

pub fn medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("Medium");
    set_up_group(&mut group);

    let mut config = get_config();
    config.img_config.width = 200;

    group.bench_function("Meduim", |b| b.iter(|| ray_tracing::run(&config)));

    group.finish();
}

pub fn large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Large");
    set_up_group(&mut group);

    let mut config = get_config();
    config.img_config.width = 500;

    group.bench_function("Large", |b| b.iter(|| ray_tracing::run(&config)));

    group.finish();
}

criterion_group!(benches, small, medium, large,);
criterion_main!(benches);
