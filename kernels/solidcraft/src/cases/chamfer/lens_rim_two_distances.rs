use crate::solids::*;
use solidcraft_kernel::ChamferSide;

pub fn build() -> R<Body> {
    let upper = solidcraft_kernel::sphere(v3(0.0, 0.0, -3.0), 5.0)?;
    let lower = solidcraft_kernel::sphere(v3(0.0, 0.0, 3.0), 5.0)?;
    let lens = common(&upper, &lower)?;
    // 2 on the upper face (the first: the one facing up most), 1 on the lower
    let rim = v3(0.0, 4.0, 0.0);
    solidcraft_kernel::chamfer_sides(&lens, &[rim], 2.0, ChamferSide::Distance(1.0), false)
}
