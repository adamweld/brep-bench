use crate::solids::*;

/// solidcraft-kernel has no cone: the right triangle of the half section, apex on the z axis, turned
/// about it.
fn cone(r: f64, h: f64) -> R<Body> {
    revolve_z(polygon(&[(0.0, 0.0), (r, 0.0), (0.0, h)]), 360.0)
}

pub fn build() -> R<Body> {
    let (r, h) = (0.00075, 0.0015);
    cut(&cone(r * 1.01, h)?, &cone(r, h)?)
}
