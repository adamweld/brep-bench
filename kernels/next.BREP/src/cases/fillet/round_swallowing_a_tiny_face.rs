//! The pentagon (0, 0), (10, 0), (10, 9.9999), (9.9999, 10), (0, 10) extruded along +z from z = 0
//! to z = 5: the block from the origin to (10, 10, 5) with its vertical edge at x = 10, y = 10
//! broken by a flat at 45°, 0.0001 back along each side. The flat's two vertical edges, selected by
//! their middles, rounded together to radius 1: the ball cannot touch so narrow a flat, so the
//! round runs from side to side and the flat is gone.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = polygon(&[
        p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 9.9999, 0.0),
        p(9.9999, 10.0, 0.0), p(0.0, 10.0, 0.0),
    ])?;
    let block = extrude_profile_brep(&section, p(0.0, 0.0, 1.0), 5.0)?;
    let edges = [p(10.0, 9.9999, 2.5), p(9.9999, 10.0, 2.5)];
    Ok(fillet_edges(&block, &edges, None, 1.0, false, None)?)
}
