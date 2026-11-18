use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    // solidcraft-kernel has no cone: its half section, apex at the origin, turned about the z axis
    let cone = revolve_z(polygon(&[(0.0, 0.0), (8.0, 6.0), (0.0, 6.0)]), 360.0)?;
    let sector = common(&ball, &cone)?;
    // 2, at 60° to it, down the cone: the first face flipped, the ball facing up more at the rim
    let rim = v3(0.0, 4.0, 3.0);
    solidcraft_kernel::chamfer_sides(&sector, &[rim], 2.0, ChamferSide::Angle(60f64.to_radians()), true)
}
