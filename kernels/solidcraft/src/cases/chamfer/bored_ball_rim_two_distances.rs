use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let ball = solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?;
    let bored = cut(&ball, &zcyl(0.0, 0.0, 2.0, -6.0, 6.0)?)?;
    // 1 on the ball (the first face: it faces up there, the bore sideways), 2 down the bore
    let rim = v3(0.0, 2.0, 21f64.sqrt());
    solidcraft_kernel::chamfer_sides(&bored, &[rim], 1.0, ChamferSide::Distance(2.0), false)
}
