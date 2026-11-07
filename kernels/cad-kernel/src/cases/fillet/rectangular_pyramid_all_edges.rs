//! The pyramid on the rectangle from (−10, −6, 0) to (10, 6, 0), its apex at (0, 0, 12), all
//! eight edges rounded to radius 3/2: a ball's patch at each corner of the base, and the apex split
//! into a short round between the long sides closed by two patches. The pyramid shrunk by 3/2 and
//! grown by 3/2.
//!
//! Built as that composition, the case's own definition, by cad-kernel's offsets. Rounding the
//! edges one at a time with `fillet::roll` is refused by the kernel: the first base edge is rolled
//! and taken off, but the second, which meets it at a corner, is not — "the ball of `face 0` with
//! `face 2` is larger than `face 2` can hold": the first round's piece has left the end face an
//! edge inside the band the second ball would roll over. The base's corners would want
//! `fillet::corner_among` placed by the caller, and the split apex, where four faces meet and no
//! ball touches all four, has no operation.
use crate::sketch::polygon;
use cad_kernel::build::kept;
use cad_kernel::offset::{grown, shrunk, Join};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{cross, dot, scale, sub, unit};

const HALF: [f64; 2] = [10., 6.];
const APEX: V = [0., 0., 12.];
const ROUND: f64 = 1.5;

pub fn build() -> Result<Brep, String> {
    let [a, b] = HALF;
    let base = [[-a, -b, 0.], [a, -b, 0.], [a, b, 0.], [-a, b, 0.]];
    // the pyramid: the prism on its base up to the apex's height, kept inside the plane of each
    // side (through a base edge and the apex)
    let mut solid = cad_kernel::build::prism(&polygon(&base), 0., APEX[2])?;
    for k in 0..4 {
        let (p, q) = (base[k], base[(k + 1) % 4]);
        let mut n = unit(cross(sub(q, p), sub(APEX, p)));
        if dot(n, scale(p, -1.)) < 0. { n = scale(n, -1.); }
        solid = kept(&solid, p, n, 0.)?;
    }
    // the union of every ball of radius ROUND inside it: shrunk by ROUND, then grown by it
    grown(&shrunk(&solid, ROUND, Join::Round)?, ROUND, Join::Round)
}
