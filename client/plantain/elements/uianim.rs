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
    duration: f32,
    current: T,
    elapsed: f32,
    finished: bool
}

impl<T: Lerpable> DynAnimation<T> {
    pub fn new(origin: T, target: T, duration: f32) -> Self {
        Self {
            origin, target, duration,
            elapsed: 0.,
            current: origin,
            finished: false
        }
    }
    pub fn update(&mut self, dt: f32) {
        if self.finished {return;}

        self.elapsed = (self.elapsed + dt).clamp(0.0, self.duration);

        let alpha = self.elapsed / self.duration;

        self.current = self.origin.lerp(self.target, alpha);

        if self.elapsed >= self.duration {
            self.finished = true;
        }
    }
    pub fn get(&self) -> &T {
        &self.current
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
    pub fn get(&self) -> &T {
        match &self.animation {
            Some(u) => if u.finished {&self.value} else {u.get()},
            None => &self.value
        }
    }
    pub fn set(&mut self, v: T) {
        self.value = v;
    }

    pub fn animate_to(&mut self, target: T, duration: f32) {
        self.animation = Some(DynAnimation::new(self.value, target, duration));
    }
}