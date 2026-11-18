use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    // solidcraft-kernel has no cone: the seat's half section, the plate's with the cone z = 1 + r/2
    // taken off its top, turned about the z axis
    let seat = revolve_z(polygon(&[(0.0, -6.0), (8.0, -6.0), (8.0, 5.0), (0.0, 1.0)]), 360.0)?;
    let joined = union(&seat, &solidcraft_kernel::sphere(Vec3::ZERO, 5.0)?)?;
    // 2 up the seat's cone (the first face: it faces up more than the ball there), at 30° to it
    let edge = v3(0.0, 4.0, 3.0);
    solidcraft_kernel::chamfer_sides(&joined, &[edge], 2.0, ChamferSide::Angle(30f64.to_radians()), false)
}
