use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    // the kernel has no cone: the frustum's half section turned about the z axis
    let frustum = revolve_z(polygon(&[(0.0, 0.0), (2.0, 0.0), (4.0, 3.0), (0.0, 3.0)]), 360.0)?;
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let cap = common(&ball, &block((-6.0, -6.0, 3.0), (6.0, 6.0, 6.0))?)?;
    let capped = union(&frustum, &cap)?;
    // 1 down the cone, at 60° to it: the first face flipped, the ball facing up more
    let rim = v3(0.0, 4.0, 3.0);
    solidcraft_kernel::chamfer_sides(&capped, &[rim], 1.0, ChamferSide::Angle(60f64.to_radians()), true)
}
