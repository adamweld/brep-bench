//! The block from (−3, −3, −3) to (3, 3, 3), less everything the cube from (−1, −1, −1) to
//! (1, 1, 1) passes through as it turns a whole turn about its long diagonal.
//!
//! next.BREP's sweep of a solid builds a turn only where both its ends stay clear of the region
//! asked for, and a whole turn starts and stops in the cube. A whole turn about an axis sweeps
//! exactly the solid of revolution of the distances from the axis the solid reaches at each place
//! along it. Along d = (1, 1, 1)/√3, at s from the centre, the cube's section is a convex polygon
//! about the axis, so it reaches every distance up to that of its farthest corner, which lies on a
//! cube edge: for |s| ≥ 1/√3 on an edge from a diagonal end, (1, 1, 1 − 2λ) and its images, at
//! ρ = √2(√3 − |s|), a cone line from (±√3, 0) to (±1/√3, √(8/3)); for |s| ≤ 1/√3 on a skew
//! edge, (1, t, −1) and its images under the cube's turns, at ρ² = 2 + 2s², the hyperbola of
//! half axes √2 and 1 (the kernel's exact hyperbola, s = sinh t, ρ = √2 cosh t). That meridian,
//! revolved a whole turn by the kernel, is cut from the block, leaving it a sealed cavity.
use crate::solids::fillet::subtract;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_hyperbola, make_line, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-3.0, -3.0, -3.0), 6.0, 6.0, 6.0)?;
    let r3 = 3f64.sqrt();
    let d = p(1.0, 1.0, 1.0).scale(1.0 / r3);
    let out = p(1.0, -1.0, 0.0).scale(1.0 / 2f64.sqrt());
    let at = |s: f64, rho: f64| d.scale(s).add(out.scale(rho));
    let (s1, rho1) = (1.0 / r3, (8.0f64 / 3.0).sqrt());
    let t1 = s1.asinh();
    let origin = p(0.0, 0.0, 0.0);
    let meridian = vec![
        make_line(at(r3, 0.0), at(-r3, 0.0))?,
        make_line(at(-r3, 0.0), at(-s1, rho1))?,
        make_hyperbola(origin, out, d, 2f64.sqrt(), 1.0, -t1, t1)?,
        make_line(at(s1, rho1), at(r3, 0.0))?,
    ];
    let spun = revolve_profile_brep(&meridian, origin, d, TAU)?;
    Ok(subtract(&block, &spun)?)
}
