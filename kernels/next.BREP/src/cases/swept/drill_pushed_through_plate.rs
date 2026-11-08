//! The plate from (0, −5, 0) to (20, 5, 4), less everything a drill passes through as it slides
//! straight down its own axis by 20: about the upright line through (10, 0), a cylinder of radius 2
//! from z = 6 + 2/tan 59° to 14, ended below by a cone of 118° with its point at (10, 0, 6).
//!
//! next.BREP's sweep of a solid builds a screw or a turn of a solid of revolution only where its
//! ends stay clear of the region asked for, and a slide is neither. A solid of revolution slid along
//! its own axis passes, by elementary geometry, through the drill where it starts and where it
//! stops and, between them, the cylinder of its greatest radius from the cylinder's lowest reach
//! (6 + 2/tan 59° − 20) to its start (14): its cylinder slid sweeps that cylinder, and its cone,
//! never wider than 2, lies in the start's cone and that cylinder at every step. The drill is the
//! kernel's revolve of its meridian; all of it is cut from the plate.
use crate::solids::p;
use crate::solids::sweep::moved;
use crate::solids::swept::{slide, Meridian, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_box_brep, make_cylinder_brep, BooleanOperation, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 4.0)?;
    let shoulder = 6.0 + 2.0 / 59f64.to_radians().tan();
    let drill = Meridian(vec![
        Piece::Line { from: [0.0, 6.0], to: [2.0, shoulder] },
        Piece::Line { from: [2.0, shoulder], to: [2.0, 14.0] },
        Piece::Line { from: [2.0, 14.0], to: [0.0, 14.0] },
    ]);
    let start = drill.upright_solid(p(10.0, 0.0, 0.0))?;
    let stop = moved(&start, slide(p(0.0, 0.0, -20.0))?)?;
    let bore = make_cylinder_brep(p(10.0, 0.0, shoulder - 20.0), p(0.0, 0.0, 1.0), 2.0,
        14.0 - (shoulder - 20.0))?;
    Ok(boolean_operation_nary(&[plate, start, bore, stop], BooleanOperation::Subtract)?)
}
