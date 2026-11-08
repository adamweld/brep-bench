//! Everything a ball of radius 2 centred at (1, 0, 0) passes through as it turns rigidly about
//! the z axis by 120°, right-handed.
//!
//! next.BREP's sweep of a solid builds a turn only where both its ends stay clear of the region
//! asked for, and here the result is the whole sweep. By elementary geometry: in each level the
//! ball is a disc about its centre's foot, and a circle about the z axis meets a disc in one arc
//! about the disc's centre's angle, holding it whenever it meets the disc at all (or in the whole
//! circle). So the angles at which the turning ball covers a point run from the start ball's arc to
//! the stop ball's, and between them are exactly the points of its section by the half plane at
//! angle 0, the disc of radius 2 about (1, 0) there cut off by the axis. The sweep is that section
//! turned by 120° (the kernel's revolve) and the kernel's ball where it starts and where it stops.
use crate::solids::p;
use crate::solids::swept::{planar, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_sphere_brep, revolve_profile_brep, BooleanOperation, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let (turn, h) = (120f64.to_radians(), 3f64.sqrt());
    let section = planar(&[
        Piece::Arc { centre: [1.0, 0.0], radius: 2.0, start: -turn, end: turn },
        Piece::Line { from: [0.0, h], to: [0.0, -h] },
    ], p(0.0, 0.0, 0.0), x, z)?;
    let swung = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), z, turn)?;
    let start = make_sphere_brep(p(1.0, 0.0, 0.0), 2.0, z)?;
    let stop = make_sphere_brep(p(turn.cos(), turn.sin(), 0.0), 2.0, z)?;
    Ok(boolean_operation_nary(&[swung, start, stop], BooleanOperation::Union)?)
}
