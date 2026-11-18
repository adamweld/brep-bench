use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let segment = cut(&ball, &block((-6.0, -6.0, 3.0), (6.0, 6.0, 6.0))?)?;
    // 4, the whole way to its center, on the flat face (the first: the one facing up most), 1 on the ball
    let rim = v3(0.0, 4.0, 3.0);
    solidcraft_kernel::chamfer_sides(&segment, &[rim], 4.0, ChamferSide::Distance(1.0), false)
}
