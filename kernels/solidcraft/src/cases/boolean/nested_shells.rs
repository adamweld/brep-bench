use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = |c: Vec3, r: f64| solidcraft_kernel::sphere(c, r);
    let outer = cut(&ball(Vec3::ZERO, 10.0)?, &ball(v3(1.0, 0.0, 0.0), 8.0)?)?;
    let middle = cut(&ball(v3(1.0, 1.0, 0.0), 6.0)?, &ball(v3(1.0, 1.0, 1.0), 4.0)?)?;
    union(&union(&outer, &middle)?, &ball(v3(1.0, 1.5, 1.5), 2.0)?)
}
