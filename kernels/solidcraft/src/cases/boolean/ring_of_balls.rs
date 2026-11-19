use crate::solids::*;

pub fn build() -> R<Body> {
    let ball = |k: usize| {
        let a = (30.0 * k as f64).to_radians();
        solidcraft_kernel::sphere(v3(10.0 * a.cos(), 10.0 * a.sin(), 0.0), 3.0)
    };
    let mut ring = ball(0)?;
    for k in 1..12 {
        ring = union(&ring, &ball(k)?)?;
    }
    Ok(ring)
}
