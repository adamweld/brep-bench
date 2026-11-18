use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let plate = zcyl(0.0, 0.0, 8.0, -6.0, 0.0)?;
    let joined = union(&plate, &solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?)?;
    // 1 out across the plate's top (the first face: the one facing up most), at 50° to it
    let edge = v3(0.0, 5.0, 0.0);
    solidcraft_kernel::chamfer_sides(&joined, &[edge], 1.0, ChamferSide::Angle(50f64.to_radians()), false)
}
