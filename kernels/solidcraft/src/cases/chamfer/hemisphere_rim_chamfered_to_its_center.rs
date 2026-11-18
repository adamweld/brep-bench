use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let half = common(&ball, &block((-6.0, -6.0, 0.0), (6.0, 6.0, 6.0))?)?;
    // 5, the whole way to its center, and 1 on the ball on the flat face: the first face flipped, the ball facing up more at the rim
    let rim = v3(0.0, 5.0, 0.0);
    solidcraft_kernel::chamfer_sides(&half, &[rim], 5.0, ChamferSide::Distance(1.0), true)
}
