use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rand::{thread_rng, Rng};

macro_rules! float_eq {
    ($lhs:expr, $rhs:expr) => {
        float_eq!($lhs, $rhs, std::f64::EPSILON)
    };
    ($lhs:expr, $rhs:expr, $epsilon:expr) => {
        ($lhs - $rhs).abs() < $epsilon
    };
}

macro_rules! float_eq_cero {
    ($lhs:expr) => {
        float_eq_cero!($lhs, std::f64::EPSILON)
    };
    ($lhs:expr, $epsilon:expr) => {
        $lhs.abs() < $epsilon
    };
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point = Vec3;
pub type Color = Vec3;

const COLOR_MAX: f64 = 255.999;

fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val > max {
        max
    } else if val < min {
        min
    } else {
        val
    }
}

fn get_color_value(v: f64, samples: u32) -> u8 {
    (clamp((v / samples as f64).sqrt(), 0.0, 1.0) * COLOR_MAX) as u8
}

// TODO: add a flag `by_hand` and use it to toggle the opperations being done by hand or with iterators and stuff like that
// Just for satisfying my curiosity
impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e: [e1, e2, e3] }
    }

    pub fn ones() -> Self {
        Self { e: [1.0, 1.0, 1.0] }
    }

    pub fn ceros() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn random_in_unit_cube() -> Self {
        let mut rng = thread_rng();
        Self {
            e: [rng.gen(), rng.gen(), rng.gen()],
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        let distr = rand::distributions::Uniform::new(min, max);
        Self {
            e: [rng.sample(distr), rng.sample(distr), rng.sample(distr)],
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random_in_range(-1.0, 1.0);
            if v.len2() >= 1.0 {
                continue;
            }
            return v;
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let mut p = Vec3::random_in_range(-1.0, 1.0);
            p[2] = 0.0;
            if p.len2() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn r(&self, samples: u32) -> u8 {
        get_color_value(self[0], samples)
    }
    pub fn g(&self, samples: u32) -> u8 {
        get_color_value(self[1], samples)
    }
    pub fn b(&self, samples: u32) -> u8 {
        get_color_value(self[2], samples)
    }

    pub fn len2(&self) -> f64 {
        self.e.iter().fold(0.0, |a, v| v * v + a)
    }

    pub fn len(&self) -> f64 {
        self.len2().sqrt()
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.len();
        for v in &mut self.e {
            *v *= k;
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.e
            .iter()
            .zip(rhs.e.iter())
            .fold(0.0, |a, (v0, v1)| v0 * v1 + a)
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            e: [
                self[1] * rhs[2] - self[2] * rhs[1],
                -(self[0] * rhs[2] - self[2] * rhs[0]),
                self[0] * rhs[1] - self[1] * rhs[0],
            ],
        }
    }

    pub fn approx_eq(&self, rhs: Self) -> bool {
        float_eq!(self[0], rhs[0]) && float_eq!(self[1], rhs[1]) && float_eq!(self[2], rhs[2])
    }

    pub fn approx_eq_epsilon(&self, rhs: Self, epsilon: f64) -> bool {
        float_eq!(self[0], rhs[0], epsilon)
            && float_eq!(self[1], rhs[1], epsilon)
            && float_eq!(self[2], rhs[2], epsilon)
    }

    pub fn approx_cero(&self) -> bool {
        float_eq_cero!(self[0]) && float_eq_cero!(self[1]) && float_eq_cero!(self[2])
    }

    pub fn refract(uv: Vec3, n: Vec3, ei_over_et: f64) -> Vec3 {
        let cos_theta = n.dot(-uv).min(1.0);

        let perpendicular_ray = (uv + n * cos_theta) * ei_over_et;
        let parallel_ray = n * -(1.0 - perpendicular_ray.len2()).abs().sqrt();

        perpendicular_ray + parallel_ray
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * (v.dot(n) * 2.0)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            e: [self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]],
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            e: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl From<&str> for Vec3 {
    fn from(s: &str) -> Self {
        let values: Vec<f64> = s
            .split_ascii_whitespace()
            .map(|v| v.parse().expect("Invalid float literal in string"))
            .collect();
        assert!(values.len() == 3);

        Self {
            e: [values[0], values[1], values[2]],
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);

        assert!(vec[0] == 1.0);
        assert!(vec[1] == 2.0);
        assert!(vec[2] == 3.0);

        assert!(vec[0] == vec.x());
        assert!(vec[1] == vec.y());
        assert!(vec[2] == vec.z());

        assert!((vec.x() * COLOR_MAX) as u8 == vec.r(1));
        assert!((vec.y() * COLOR_MAX) as u8 == vec.g(1));
        assert!((vec.z() * COLOR_MAX) as u8 == vec.b(1));

        vec[0] = 4.0;
        assert!(vec[0] == 4.0);
        assert!(vec[1] == 2.0);
        assert!(vec[2] == 3.0);

        vec[1] = 5.0;
        assert!(vec[0] == 4.0);
        assert!(vec[1] == 5.0);
        assert!(vec[2] == 3.0);

        vec[2] = 6.0;
        assert!(vec[0] == 4.0);
        assert!(vec[1] == 5.0);
        assert!(vec[2] == 6.0);
    }

    #[test]
    fn approx_eq() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1.clone();

        assert!(v1.approx_eq(v2));

        let v3 = v2 * 1.00001;

        assert!(v1.approx_eq_epsilon(v3, 0.0001));
        assert!(!v1.approx_eq_epsilon(v3, 0.00001));
        assert!(!v1.approx_eq(v3));
    }

    #[test]
    fn len() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);

        let res = 14.0;

        assert!(vec.len2() == res);
        assert!(float_eq!(vec.len(), res.sqrt()));

        vec[1] *= -1.0;

        assert!(vec.len2() == res);
        assert!(float_eq!(vec.len(), res.sqrt()));
    }

    #[test]
    fn unit_vector() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);

        let unit = vec.unit_vector();
        let epsilon = 0.000001;

        assert!(float_eq!(unit[0], 0.267261, epsilon));
        assert!(float_eq!(unit[1], 0.534522, epsilon));
        assert!(float_eq!(unit[2], 0.801784, epsilon));

        vec.make_unit_vector();

        assert!(float_eq!(vec[0], 0.267261, epsilon));
        assert!(float_eq!(vec[1], 0.534522, epsilon));
        assert!(float_eq!(vec[2], 0.801784, epsilon));
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert!(float_eq!(v1.dot(v2), 32.0));
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let res = v1.cross(v2);

        assert!(float_eq!(res[0], -3.0));
        assert!(float_eq!(res[1], 6.0));
        assert!(float_eq!(res[2], -3.0));
    }

    #[test]
    fn neg() {
        let vec = -Vec3::new(1.0, 2.0, 3.0);

        assert!(float_eq!(vec[0], -1.0));
        assert!(float_eq!(vec[1], -2.0));
        assert!(float_eq!(vec[2], -3.0));
    }

    #[test]
    fn add() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let res = v1 + v2;

        assert!(float_eq!(res[0], 5.0));
        assert!(float_eq!(res[1], 7.0));
        assert!(float_eq!(res[2], 9.0));

        v1 += v2;
        assert!(v1.approx_eq(res));
    }

    #[test]
    fn sub() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(6.0, 5.0, 4.0);

        let res = v1 - v2;

        assert!(float_eq!(res[0], -5.0));
        assert!(float_eq!(res[1], -3.0));
        assert!(float_eq!(res[2], -1.0));

        v1 -= v2;
        assert!(v1.approx_eq(res));
    }

    #[test]
    fn mul() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let res = v1 * v2;

        assert!(float_eq!(res[0], 4.0));
        assert!(float_eq!(res[1], 10.0));
        assert!(float_eq!(res[2], 18.0));

        v1 *= v2;
        assert!(v1.approx_eq(res));
    }

    #[test]
    fn mul_float() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let t = 3.0;

        let res = v1 * t;

        assert!(float_eq!(res[0], 3.0));
        assert!(float_eq!(res[1], 6.0));
        assert!(float_eq!(res[2], 9.0));

        v1 *= t;
        assert!(v1.approx_eq(res));
    }

    #[test]
    fn div() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let res = v1 / v2;

        assert!(float_eq!(res[0], 1.0 / 4.0));
        assert!(float_eq!(res[1], 2.0 / 5.0));
        assert!(float_eq!(res[2], 3.0 / 6.0));

        v1 /= v2;
        assert!(v1.approx_eq(res));
    }

    #[test]
    fn div_float() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let t = 3.0;

        let res = v1 / t;

        assert!(float_eq!(res[0], 1.0 / t));
        assert!(float_eq!(res[1], 2.0 / t));
        assert!(float_eq!(res[2], 3.0 / t));

        v1 /= t;
        assert!(v1.approx_eq(res));
    }

    #[test]
    fn from() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let s = "1 2 3";

        assert!(vec.approx_eq(Vec3::from(s)));
    }

    #[test]
    fn display() {
        let vec = Vec3::new(1.0, 2.0, 3.0);

        assert!(vec.to_string() == "1 2 3");
    }
}
