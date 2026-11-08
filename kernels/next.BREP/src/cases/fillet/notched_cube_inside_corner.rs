//! The cube from the origin to (20, 20, 20) less the cube from (10, 10, 10) to (20, 20, 20), the
//! notch's three concave edges from its inside corner (10, 10, 10) rounded to radius 3 in one
//! fillet, which closes the corner where they meet with the ball's eighth. Each edge is selected
//! by its middle.
use crate::solids::fillet::subtract;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 20.0, 20.0)?;
    let notch = make_box_brep(p(10.0, 10.0, 10.0), 10.0, 10.0, 10.0)?;
    let notched = subtract(&cube, &notch)?;
    let edges = [p(15.0, 10.0, 10.0), p(10.0, 15.0, 10.0), p(10.0, 10.0, 15.0)];
    Ok(fillet_edges(&notched, &edges, None, 3.0, false, None)?)
}
