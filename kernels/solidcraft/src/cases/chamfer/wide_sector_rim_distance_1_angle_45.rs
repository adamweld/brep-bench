use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    // solidcraft-kernel has no cone: its half section, apex at the origin, turned about the z axis
    let cone = revolve_z(polygon(&[(0.0, 0.0), (24.0, 10.0), (0.0, 10.0)]), 360.0)?;
    let sector = common(&ball, &cone)?;
    // 1, at 45° to it, down the cone: the first face flipped, the ball facing up more at the rim
    let rim = v3(0.0, 60.0 / 13.0, 25.0 / 13.0);
    solidcraft_kernel::chamfer_sides(&sector, &[rim], 1.0, ChamferSide::Angle(45f64.to_radians()), true)
}
