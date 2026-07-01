use instant::Instant;

use crate::plantain::elements::element::DimD2;

pub trait Lerpable: Copy + Clone {
    fn lerp(&self, b: Self, t: f32) -> Self;
}

impl Lerpable for f32 {
    fn lerp(&self, b: Self, t: f32) -> Self {
        self + (b - self) * t
    }
}

impl Lerpable for DimD2 {
    fn lerp(&self, b: Self, t: f32) -> Self {
        DimD2::new(
            self.scale_x.lerp(b.scale_x, t),
            self.scale_y.lerp(b.scale_y, t),
            self.offset_x.lerp(b.offset_x, t), 
            self.offset_y.lerp(b.offset_y, t)
        )
    }
}

impl Lerpable for [f32; 4] {
    fn lerp(&self, b: Self, t: f32) -> Self {
        [
            self[0].lerp(b[0], t),
            self[1].lerp(b[1], t),
            self[2].lerp(b[2], t),
            self[3].lerp(b[3], t)
        ]
    }
}

impl Lerpable for nalgebra::Vector3<f32> {
    fn lerp(&self, b: Self, t: f32) -> Self {
        self + (b - self) * t
    }
}

pub struct DynAnimation<T: Lerpable> {
    origin: T,
    target: T,
    duration: f64,
    started: Instant
}

impl<T: Lerpable> DynAnimation<T> {
    pub fn new(origin: T, target: T, duration: f64) -> Self {
        Self {
            origin, target, duration,
            started: Instant::now()
        }
    }
    pub fn get(&self) -> T {
        let difference = Instant::now() - self.started;

        let alpha = (difference.as_secs_f64() / self.duration).clamp(0.0, 1.0) as f32;

        self.origin.lerp(self.target, alpha)
    }
}

pub struct DynValue<T: Lerpable> {
    value: T,
    animation: Option<DynAnimation<T>>
}

impl<T: Lerpable> DynValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            animation: None
        }
    }
    pub fn get(&mut self) -> &T {
        match &self.animation {
            Some(u) => self.value = u.get(),
            None => {}
        }
        &self.value
    }
    pub fn set(&mut self, v: T) {
        self.value = v;
    }

    pub fn animate_to(&mut self, target: T, duration: f64) {
        self.animation = Some(DynAnimation::new(self.value, target, duration));
    }
}