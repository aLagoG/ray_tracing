use std::default::Default;

use crate::{Point, Vec3};

pub struct RunConfig<'a> {
    pub img_config: ImgConfig,
    pub cam_config: CameraConfig,
    pub scene_config: SceneConfig,
    pub filename: &'a str,
    pub quiet: bool,
    pub use_bvh: bool,
}

impl<'a> Default for RunConfig<'a> {
    fn default() -> Self {
        Self {
            img_config: ImgConfig::default(),
            cam_config: CameraConfig::default(),
            scene_config: SceneConfig::default(),
            filename: "res.png",
            quiet: false,
            use_bvh: true,
        }
    }
}

pub struct ImgConfig {
    pub aspect_ratio: f64,
    pub width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
}

impl Default for ImgConfig {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            width: 1200,
            samples_per_pixel: 500,
            max_depth: 50,
        }
    }
}

pub struct CameraConfig {
    pub lookfrom: Point,
    pub lookat: Point,
    pub vec_up: Vec3,
    pub vert_fov: f64,
    pub aperture: f64,
    pub focus_dist: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            lookfrom: Point::new(13.0, 2.0, 3.0),
            lookat: Point::new(0.0, 0.0, 0.0),
            vec_up: Vec3::new(0.0, 1.0, 0.0),
            vert_fov: 20.0,
            aperture: 0.1,
            focus_dist: 10.0,
        }
    }
}

pub struct SceneConfig {
    pub small_sphere_count: u32,
    pub diffuse_prob: f64,
    pub metal_prob: f64,
}

impl SceneConfig {
    pub fn validate(&self) {
        assert!(
            (self.diffuse_prob - self.metal_prob).abs() <= 1.0,
            "The probabilities for diffuse and metal materials should be <= 1\n(the remainder will be used as the probability for glass)"
        );
    }
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            small_sphere_count: 484,
            diffuse_prob: 0.8,
            metal_prob: 0.1,
        }
    }
}
