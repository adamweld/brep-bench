//! The plate from (−10, −10, 0) to (10, 10, 4), less everything a core drill passes through as it
//! slides by 24 along its own leaning axis u = (sin 30°, 0, cos 30°) through (0, 0, 2): the tube
//! between 1.5 and 2.5 from that axis, from 18 to 6 before (0, 0, 2) where it starts.
//!
//! next.BREP's sweep of a solid takes a prism only in motions square to its own direction. A tube
//! slid along its own axis passes through exactly the tube that runs from where its back end
//! starts to where its front end stops (each of its positions lies in it, and every point of it
//! lies in one of them, the slide being longer than the tube): here from 18 before (0, 0, 2) to 18
//! past it. That tube, the kernel's cylinder of radius 2.5 less one of radius 1.5 running past its
//! ends, is cut from the plate, which leaves the plate with its hole and the core inside it.
use crate::solids::fillet::subtract;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 4.0)?;
    let u = p(30f64.to_radians().sin(), 0.0, 30f64.to_radians().cos());
    let centre = p(0.0, 0.0, 2.0);
    let outer = make_cylinder_brep(centre.sub(u.scale(18.0)), u, 2.5, 36.0)?;
    let inner = make_cylinder_brep(centre.sub(u.scale(20.0)), u, 1.5, 40.0)?;
    let tube = subtract(&outer, &inner)?;
    Ok(subtract(&plate, &tube)?)
}
