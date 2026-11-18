use crate::solids::*;

/// solidcraft-kernel has no cone: the right triangle of the half section, apex on the z axis, turned
/// about it.
fn cone(r: f64, h: f64) -> R<Body> {
    revolve_z(polygon(&[(0.0, 0.0), (r, 0.0), (0.0, h)]), 360.0)
}

pub fn build() -> R<Body> {
    cut(&cone(5.0, 10.0)?, &cone(4.999, 10.0)?)
}
