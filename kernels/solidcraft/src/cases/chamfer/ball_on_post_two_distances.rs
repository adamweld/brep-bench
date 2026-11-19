use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let post = zcyl(0.0, 0.0, 3.0, 0.0, 10.0)?;
    let joined = union(&post, &solidcraft_kernel::sphere(v3(0.0, 0.0, 10.0), 5.0)?)?;
    // 2 down the post (the first face: the ball faces down there, the post sideways), 1 on the ball
    let edge = v3(0.0, 3.0, 6.0);
    solidcraft_kernel::chamfer_sides(&joined, &[edge], 2.0, ChamferSide::Distance(1.0), false)
}
