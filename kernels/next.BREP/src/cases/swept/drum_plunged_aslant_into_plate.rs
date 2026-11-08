//! A plate pocketed from below by a drum lying on its side, pushed up aslant and stopping inside:
//! the plate from (0, −5, 0) to (20, 5, 4), less everything the drum (radius 2, its axis the line
//! y = 0, z = −5 from x = 6 to x = 14, its ends flat) passes through as it slides without turning
//! by v = 8·(0.3, 0, √(1 − 0.3²)).
//!
//! next.BREP has no sweep of a solid along a line. The drum is convex, and for a convex solid P
//! slid by v the sweep is P where it starts, P + v where it stops, and Σ slid along v, for any
//! surface Σ inside P that meets every line along v that meets P. For a point q in the sweep but
//! in neither end, the s in [0, 1] with q − sv in P form one interval (P is convex), and the line
//! meets Σ at a point of P on that segment, so q lies on Σ slid by that s; and Σ slid stays in
//! the sweep, Σ being inside P. Here Σ spans the drum's outline seen along v: the side's lines at
//! y = ±2 (where its normal is square to v), the upper half of the rim at x = 6 and the lower half
//! at x = 14 (where the side and the end face opposite ways along v). Σ is the drum's axial
//! rectangle, z = −5, with the upper half disc of the end x = 6 and the lower half disc of the end
//! x = 14. Each piece is flat and none of them lies along v, so each one, extruded along v, is a
//! prism: the rectangle into an oblique slab whose sides are the planes y = ±2, and the half discs
//! into the oblique half elliptic cylinders the rims sweep. The drum where it starts lies wholly
//! below the plate (its top is z = −3), so it is left out of the cut.
use crate::solids::p;
use crate::solids::sweep::moved;
use crate::solids::swept::slide;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, extrude_profile_brep, make_arc, make_box_brep, make_line,
    revolve_profile_brep, BooleanOperation, BrepSolid,
};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 4.0)?;
    let (radius, x0, x1, axis_z) = (2.0, 6.0, 14.0, -5.0);
    let along = p(0.3, 0.0, (1.0f64 - 0.09).sqrt());
    let length = 8.0;
    let v = along.scale(length);
    let (x, y, z) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    let at = |px: f64, py: f64, pz: f64| p(px, py, pz);

    // The drum where it stops: the revolve about its axis of the rectangle out to its radius.
    let section = vec![
        make_line(at(x0, 0.0, axis_z), at(x1, 0.0, axis_z))?,
        make_line(at(x1, 0.0, axis_z), at(x1, 0.0, axis_z + radius))?,
        make_line(at(x1, 0.0, axis_z + radius), at(x0, 0.0, axis_z + radius))?,
        make_line(at(x0, 0.0, axis_z + radius), at(x0, 0.0, axis_z))?,
    ];
    let drum = revolve_profile_brep(&section, at(x0, 0.0, axis_z), x, TAU)?;
    let stop = moved(&drum, slide(v)?)?;

    // Σ: the axial rectangle and the two half discs, each extruded along v.
    let rectangle = vec![
        make_line(at(x0, -radius, axis_z), at(x1, -radius, axis_z))?,
        make_line(at(x1, -radius, axis_z), at(x1, radius, axis_z))?,
        make_line(at(x1, radius, axis_z), at(x0, radius, axis_z))?,
        make_line(at(x0, radius, axis_z), at(x0, -radius, axis_z))?,
    ];
    // the upper half of the end x = 6: from y = 2 over the top to y = −2, then back along y
    let upper = vec![
        make_arc(at(x0, 0.0, axis_z), y, z, radius, 0.0, PI)?,
        make_line(at(x0, -radius, axis_z), at(x0, radius, axis_z))?,
    ];
    // the lower half of the end x = 14: from y = −2 under the bottom to y = 2, then back along y
    let lower = vec![
        make_arc(at(x1, 0.0, axis_z), y, z, radius, PI, TAU)?,
        make_line(at(x1, radius, axis_z), at(x1, -radius, axis_z))?,
    ];
    let mut operands = vec![plate, stop];
    for piece in [&rectangle, &upper, &lower] {
        operands.push(extrude_profile_brep(piece, along, length)?);
    }
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
