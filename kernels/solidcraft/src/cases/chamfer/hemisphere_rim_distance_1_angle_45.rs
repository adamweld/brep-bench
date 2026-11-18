use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let half = common(&ball, &block((-6.0, -6.0, 0.0), (6.0, 6.0, 6.0))?)?;
    // 1, at 45° to it on the flat face: the first face flipped, the ball facing up more at the rim
    let rim = v3(0.0, 5.0, 0.0);
    solidcraft_kernel::chamfer_sides(&half, &[rim], 1.0, ChamferSide::Angle(45f64.to_radians()), true)
}
