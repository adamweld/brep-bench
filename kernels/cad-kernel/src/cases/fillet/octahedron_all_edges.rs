//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges rounded to radius 2, four rounds meeting
//! at each corner, which the ball touching all four faces there closes with a patch of its sphere:
//! the octahedron shrunk by 2 and grown by 2.
//!
//! Built as that composition, the case's own definition, by cad-kernel's offsets. Rounding the
//! edges one at a time with `fillet::roll` is refused by the kernel: the first edge is rolled and
//! taken off (its run cut off at each end by the corner's other two faces), but the second, which
//! shares a corner with it, is not — "the fillet of `face 2` with `face 3` runs out onto `face 8`
//! (bspline), which cuts nothing off yet": a run ends only on faces that cut it off, planes and the
//! solids of cylinders, cones and spheres, and the first round is a fitted canal. Nor could the
//! corner be closed by the kernel once the runs were rolled: `fillet::corner_among`, the ball's
//! patch where rounds meet, stands between three faces, and four meet here.
use crate::sketch::block;
use cad_kernel::build::kept;
use cad_kernel::offset::{grown, shrunk, Join};
use cad_kernel::topo::Brep;

const REACH: f64 = 10.;
const ROUND: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    // the octahedron: a block round it kept within the plane of each face
    let mut solid = block([-REACH; 3], [REACH; 3])?;
    let k = 1. / 3f64.sqrt();
    for sx in [-1., 1.] {
        for sy in [-1., 1.] {
            for sz in [-1., 1.] {
                solid = kept(&solid, [sx * REACH, 0., 0.], [-sx * k, -sy * k, -sz * k], 0.)?;
            }
        }
    }
    // the union of every ball of radius ROUND inside it: shrunk by ROUND, then grown by it
    grown(&shrunk(&solid, ROUND, Join::Round)?, ROUND, Join::Round)
}
