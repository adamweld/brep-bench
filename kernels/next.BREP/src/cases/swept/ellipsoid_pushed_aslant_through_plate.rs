//! The plate from (−15, −15, 0) to (15, 15, 4), less everything the ellipsoid of semi-axes 3, 2
//! and 1.5 centred at (−2, −1.5, −4) passes through as it slides without turning along (4, 3, 12).
//!
//! next.BREP has no ellipsoid and no sweep of a solid along a line. The ellipsoid is the ball of
//! radius 1 under the affine map A (stretch by 3, 2, 1.5, then move to its centre), and a slide
//! commutes with A: A(K + tw) = A(K) + t·A w. So the sweep is A of the unit ball's sweep along
//! w = A⁻¹(4, 3, 12) = (4/3, 3/2, 8), which is, by elementary geometry, the ball at each end and
//! the cylinder of radius 1 about the segment between them. Each piece is mapped by A (the kernel
//! applies an affine map exactly to rational poles) and cut from the plate.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_box_brep, make_cylinder_brep, make_sphere_brep, transform_brep,
    AffineTransform, BooleanOperation, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-15.0, -15.0, 0.0), 30.0, 30.0, 4.0)?;
    let w = p(4.0 / 3.0, 1.5, 8.0);
    let origin = p(0.0, 0.0, 0.0);
    let pieces = [
        make_sphere_brep(origin, 1.0, w)?,
        make_cylinder_brep(origin, w, 1.0, w.length())?,
        make_sphere_brep(w, 1.0, w)?,
    ];
    let a = AffineTransform::new([
        3.0, 0.0, 0.0, -2.0, //
        0.0, 2.0, 0.0, -1.5, //
        0.0, 0.0, 1.5, -4.0, //
        0.0, 0.0, 0.0, 1.0,
    ])?;
    let mut operands = vec![plate];
    for piece in &pieces {
        operands.push(transform_brep(piece, a, false)?);
    }
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
