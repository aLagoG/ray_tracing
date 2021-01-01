use ray_tracing::{run, RunConfig};

fn main() {
    let mut config = RunConfig::default();
    config.scene_config.small_sphere_count = 100;
    config.img_config.samples_per_pixel = 10;
    run(&config);
}
