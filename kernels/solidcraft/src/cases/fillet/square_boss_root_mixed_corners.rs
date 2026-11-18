use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((0.0, 0.0, 0.0), (40.0, 30.0, 5.0))?;
    let boss = block((12.0, 7.0, 5.0), (28.0, 23.0, 15.0))?;
    let part = union(&plate, &boss)?;
    // first the boss's four upright edges, at their midpoints
    let uprights = [v3(12.0, 7.0, 10.0), v3(28.0, 7.0, 10.0), v3(28.0, 23.0, 10.0), v3(12.0, 23.0, 10.0)];
    let rounded = solidcraft_kernel::fillet(&part, &uprights, 3.0)?;
    // then the root, now one tangent chain: picked on one side and followed all round
    let root = solidcraft_kernel::tangent_chain(&rounded, &[v3(20.0, 7.0, 5.0)])?;
    solidcraft_kernel::fillet(&rounded, &root, 2.0)
}
