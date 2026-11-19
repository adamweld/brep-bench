use crate::solids::*;

pub fn build() -> R<Body> {
    let foot = block((0.0, 0.0, 0.0), (20.0, 10.0, 6.0))?;
    let upright = block((0.0, 0.0, 0.0), (6.0, 10.0, 20.0))?;
    let l = union(&foot, &upright)?;
    // first the concave edge along y
    let rounded = solidcraft_kernel::fillet(&l, &[v3(6.0, 5.0, 6.0)], 2.0)?;
    // then the end face's chain it leaves: picked up the upright, and followed tangentially round
    // the concave round's end and along the foot
    let chain = solidcraft_kernel::tangent_chain(&rounded, &[v3(6.0, 0.0, 14.0)])?;
    solidcraft_kernel::fillet(&rounded, &chain, 2.0)
}
